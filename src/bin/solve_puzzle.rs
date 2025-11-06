/*
 * SOLVE PUZZLE - Solves and claims the prize from a puzzle
 *
 * Usage:
 *   cargo run --bin solve-puzzle -- <txid> <vout> <target_hash> <secret> <destination_address>
 *
 * Example:
 *   cargo run --bin solve-puzzle -- abc123...def 0 0x2cf24dba... "satoshi" tex1q...
 *
 * This will:
 * 1. Get UTXO info from txid:vout
 * 2. Verify the secret matches target_hash
 * 3. Create a transaction spending the puzzle UTXO
 * 4. Provide the secret as witness
 * 5. Broadcast and win the prize!
 */

use anyhow::{Context, Result};
use chrono;
use elements::pset::PartiallySignedTransaction as Psbt;
use elements::{confidential, secp256k1_zkp as secp256k1, Address, OutPoint, TxIn, TxInWitness, TxOut};
use secp256k1::XOnlyPublicKey;
use sha2::{Digest, Sha256};
use simplicityhl::{Arguments, CompiledProgram, Value, WitnessValues};
use simplicityhl::value::ValueConstructible;
use std::collections::HashMap;
use std::env;
use std::process::Command;
use std::str::FromStr;

const PUZZLE_CONTRACT: &str = include_str!("../../../SimplicityHL/examples/puzzle_jackpot.simf");

fn get_utxo_info(txid: &str, vout: u32, fallback_amount: Option<f64>) -> Result<(f64, String)> {
    let elements_cli = "/Users/felipe/Desktop/hub/blockchain/elements/src/elements-cli";

    // First, try gettxout
    let output = Command::new(elements_cli)
        .args(&[
            "-chain=liquidtestnet",
            "gettxout",
            txid,
            &vout.to_string(),
        ])
        .output()
        .context("Failed to execute gettxout")?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("Failed to get UTXO info: {}", error));
    }

    let output_str = String::from_utf8_lossy(&output.stdout);
    let utxo_data: serde_json::Value = serde_json::from_str(&output_str)
        .context("Failed to parse gettxout output")?;

    // Check if UTXO exists
    if utxo_data.is_null() {
        return Err(anyhow::anyhow!(
            "❌ UTXO {}:{} not found or already spent!\n   The puzzle may have already been solved by someone else.",
            txid, vout
        ));
    }

    // Try to get value - it might be confidential
    let amount = if let Some(value) = utxo_data["value"].as_f64() {
        value
    } else {
        // Value might be confidential, try to get raw transaction
        println!("   ⚠️  Value is confidential, fetching raw transaction...");

        let raw_tx_output = Command::new(elements_cli)
            .args(&[
                "-chain=liquidtestnet",
                "getrawtransaction",
                txid,
                "true",  // verbose
            ])
            .output()
            .context("Failed to get raw transaction")?;

        if !raw_tx_output.status.success() {
            let error = String::from_utf8_lossy(&raw_tx_output.stderr);
            return Err(anyhow::anyhow!("Failed to get raw transaction: {}", error));
        }

        let raw_tx_str = String::from_utf8_lossy(&raw_tx_output.stdout);
        let tx_data: serde_json::Value = serde_json::from_str(&raw_tx_str)
            .context("Failed to parse raw transaction")?;

        // Try to get the value from the specific output
        if let Some(vout_data) = tx_data["vout"].as_array()
            .and_then(|vouts| vouts.get(vout as usize)) {

            if let Some(value) = vout_data["value"].as_f64() {
                println!("   ✓ Found unblinded value from transaction");
                value
            } else {
                // If still no value, it might be fully confidential
                // Use the amount from the puzzle file as fallback
                if let Some(fallback) = fallback_amount {
                    println!("   ⚠️  Value is fully confidential, using amount from puzzle file: {} L-BTC", fallback);
                    fallback
                } else {
                    return Err(anyhow::anyhow!("Value is confidential and no fallback amount provided. Please check the puzzle file."));
                }
            }
        } else {
            return Err(anyhow::anyhow!("Cannot find output {} in transaction", vout));
        }
    };

    let asset = utxo_data["asset"].as_str()
        .unwrap_or_else(|| utxo_data["assetcommitment"].as_str()
            .unwrap_or("144c654344aa716d6f3abcc1ca90e5641e4e2a7f633bc09fe3baf64585819a49"))
        .to_string();

    // Get confirmations if available
    if let Some(confs) = utxo_data["confirmations"].as_u64() {
        println!("   ✓ UTXO has {} confirmations", confs);
    }

    Ok((amount, asset))
}

fn main() -> Result<()> {
    // Parse arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: {} <puzzle_file.json> <secret> <destination_address>", args[0]);
        eprintln!("\nExample:");
        eprintln!("  {} puzzle_2cf24dba.json \"satoshi\" tex1q...", args[0]);
        eprintln!("\nSupported secret formats:");
        eprintln!("  - String: \"satoshi\", \"bitcoin\"");
        eprintln!("  - Hex number: \"0x00000001\" (32-bit), \"0x0000000000000001\" (64-bit)");
        eprintln!("  - Hex bytes: \"0xdeadbeef\" (arbitrary hex string)");
        std::process::exit(1);
    }

    let puzzle_file = &args[1];
    let secret = &args[2];
    let dest_address = &args[3];

    println!("╔══════════════════════════════════════╗");
    println!("║      🎯 SOLVING PUZZLE HUNT 🎯       ║");
    println!("╚══════════════════════════════════════╝");
    println!();
    println!("🎮 Starting puzzle solver...");
    println!("📅 Time: {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"));
    println!();

    // Read puzzle info from JSON
    println!("📂 Step 1: Loading puzzle information");
    println!("   File: {}", puzzle_file);
    let puzzle_data = std::fs::read_to_string(puzzle_file)
        .context("Failed to read puzzle file")?;
    let puzzle: serde_json::Value = serde_json::from_str(&puzzle_data)
        .context("Failed to parse puzzle JSON")?;

    let txid_str = puzzle["txid"].as_str()
        .ok_or_else(|| anyhow::anyhow!("Missing txid in puzzle file"))?;
    let vout: u32 = puzzle["vout"].as_u64()
        .ok_or_else(|| anyhow::anyhow!("Missing vout in puzzle file"))? as u32;
    let target_hash = puzzle["target_hash"].as_str()
        .ok_or_else(|| anyhow::anyhow!("Missing target_hash in puzzle file"))?;
    let puzzle_address = puzzle["address"].as_str()
        .ok_or_else(|| anyhow::anyhow!("Missing address in puzzle file"))?;
    let hint = puzzle["hint"].as_str().unwrap_or("No hint provided");
    let puzzle_amount = puzzle["amount"].as_str().unwrap_or("Unknown");

    println!("✅ Puzzle loaded successfully!");
    println!("   📍 Puzzle address: {}", puzzle_address);
    println!("   📝 Transaction ID: {}", txid_str);
    println!("   🔢 Output index: {}", vout);
    println!("   🎯 Target hash: {}", target_hash);
    println!("   💰 Original amount: {} L-BTC", puzzle_amount);
    println!("   💡 Hint: \"{}\"", hint);
    println!();

    // 1. Get UTXO information FIRST (we need the value for the hash!)
    println!("📊 Step 2: Fetching UTXO information from blockchain");
    println!("   Connecting to Elements daemon...");

    // Parse amount from puzzle file as fallback for confidential values
    let fallback_amount = puzzle_amount.parse::<f64>().ok();
    let (amount_btc, asset_id_str) = get_utxo_info(txid_str, vout, fallback_amount)?;
    let value_sats = (amount_btc * 100_000_000.0) as u64;

    println!("✅ UTXO verified on-chain!");
    println!("   💰 Current prize: {} L-BTC", amount_btc);
    println!("   💵 In satoshis: {} sats", value_sats);
    println!("   🪙 Asset ID: {}", &asset_id_str[..16]);
    println!("   📈 Full Asset: {}", asset_id_str);
    println!();

    // 2. Verify the secret is correct WITH THE CURRENT VALUE
    println!("🔐 Step 3: Processing and verifying your secret");
    println!("   Your secret: \"{}\"", secret);

    // Convert secret to u256 (32 bytes)
    let mut secret_bytes = [0u8; 32];

    // Check if secret is a hex number (0x prefix)
    if secret.starts_with("0x") || secret.starts_with("0X") {
        // Parse as hex number
        let hex_str = &secret[2..];

        if hex_str.len() == 8 {
            // 32-bit number (8 hex chars)
            let num = u32::from_str_radix(hex_str, 16)
                .context("Invalid hex number format")?;
            secret_bytes[28..32].copy_from_slice(&num.to_be_bytes());
            println!("   🔢 Format detected: 32-bit hex number");
            println!("      Hex: 0x{:08x}", num);
            println!("      Decimal: {}", num);
        } else if hex_str.len() == 16 {
            // 64-bit number (16 hex chars)
            let num = u64::from_str_radix(hex_str, 16)
                .context("Invalid hex number format")?;
            secret_bytes[24..32].copy_from_slice(&num.to_be_bytes());
            println!("   🔢 Format detected: 64-bit hex number");
            println!("      Value: 0x{:016x}", num);
        } else {
            // Generic hex string, parse as bytes
            let bytes = hex::decode(hex_str)
                .context("Invalid hex string")?;
            let len = bytes.len().min(32);
            secret_bytes[32 - len..].copy_from_slice(&bytes[..len]);
            println!("   🔤 Format detected: Hex byte string");
            println!("      Length: {} bytes", bytes.len());
            println!("      Hex: 0x{}", hex_str);
        }
    } else {
        // Regular string with right-padding
        let secret_raw = secret.as_bytes();
        let len = secret_raw.len().min(32);
        secret_bytes[32 - len..].copy_from_slice(&secret_raw[..len]);
        println!("   📝 Format detected: Text string");
        println!("      Length: {} characters", secret.len());
        println!("      ASCII bytes: {:?}", &secret_raw[..len.min(8)]);
    }

    println!();
    println!("🧮 Step 4: Computing hash");
    println!("   Hash formula: SHA256(secret)");
    println!();
    println!("   📥 Hash input:");
    println!("      Secret (32 bytes): 0x{}", hex::encode(&secret_bytes));

    // Calculate SHA256 of just the secret
    let mut hasher = Sha256::new();
    hasher.update(&secret_bytes);
    let hash = hasher.finalize();
    let hash_hex = format!("0x{}", hex::encode(hash));

    println!();
    println!("   🔄 Computing: SHA256(secret)");
    println!("   📤 Result: {}", hash_hex);
    println!();
    println!("🔍 Step 5: Verifying hash matches target");
    println!("   Expected: {}", target_hash);
    println!("   Computed: {}", hash_hex);

    if hash_hex != target_hash {
        println!();
        println!("╔══════════════════════════════════════════════╗");
        println!("║         ❌ VERIFICATION FAILED! ❌            ║");
        println!("╚══════════════════════════════════════════════╝");
        eprintln!();
        eprintln!("🚫 The computed hash does not match the target!");
        eprintln!();
        eprintln!("📊 Debug information:");
        eprintln!("   Expected hash: {}", target_hash);
        eprintln!("   Your hash:     {}", hash_hex);
        eprintln!("   Your secret:   \"{}\"", secret);
        eprintln!();
        eprintln!("⚠️  Possible reasons for failure:");
        eprintln!("   1. ❌ Wrong secret - check your spelling");
        eprintln!("   2. 📝 Check if the secret format is correct");
        eprintln!("   3. 🔤 Try different formats (string vs hex)");
        eprintln!();
        eprintln!("💡 Tips:");
        eprintln!("   - The secret is case-sensitive");
        eprintln!("   - Try different secret formats (string vs hex)");
        eprintln!("   - Contact the puzzle creator if stuck");
        std::process::exit(1);
    }

    println!();
    println!("╔══════════════════════════════════════════════╗");
    println!("║       ✅ SECRET VERIFIED SUCCESSFULLY! ✅     ║");
    println!("╚══════════════════════════════════════════════╝");
    println!();

    // 3. Compile the contract
    println!("🛠️ Step 6: Compiling Simplicity smart contract");
    println!("   Contract: puzzle_jackpot.simf");
    println!("   Preparing contract parameters...");

    let mut hash_bytes = [0u8; 32];
    hash_bytes.copy_from_slice(&hash);
    let target_hash_u256 = simplicityhl::num::U256::from_byte_array(hash_bytes);

    let mut arguments = HashMap::new();
    arguments.insert(
        simplicityhl::str::WitnessName::from_str_unchecked("TARGET_HASH"),
        Value::u256(target_hash_u256),
    );
    let args = Arguments::from(arguments);

    println!("   Compiling with TARGET_HASH parameter...");
    let compiled = CompiledProgram::new(PUZZLE_CONTRACT, args, false)
        .map_err(|e| anyhow::anyhow!("Failed to compile contract: {}", e))?;

    let cmr = compiled.commit().cmr();
    println!("✅ Contract compiled successfully!");
    println!("   📝 CMR (Commitment Merkle Root): 0x{}", hex::encode(cmr.as_ref()));
    println!();

    // 4. Create spending transaction
    println!("💸 Step 7: Creating spending transaction");
    println!("   Parsing destination address...");

    let dest_addr = Address::from_str(dest_address)?;
    let fee_sats = 1_000u64;

    if value_sats <= fee_sats {
        return Err(anyhow::anyhow!(
            "UTXO value ({} sats) is too small to pay fee ({} sats)",
            value_sats, fee_sats
        ));
    }

    let output_value = value_sats - fee_sats;
    println!("   📊 Transaction economics:");
    println!("      Input amount:  {} sats ({} L-BTC)", value_sats, value_sats as f64 / 100_000_000.0);
    println!("      Output amount: {} sats ({} L-BTC)", output_value, output_value as f64 / 100_000_000.0);
    println!("      Network fee:   {} sats ({} L-BTC)", fee_sats, fee_sats as f64 / 100_000_000.0);
    println!("      Destination:   {}", dest_address);
    println!();

    let txid = elements::Txid::from_str(txid_str)?;
    let outpoint = OutPoint::new(txid, vout);

    let asset_id = elements::AssetId::from_str(&asset_id_str)?;
    let asset = confidential::Asset::Explicit(asset_id);

    let psbt = Psbt::from_tx(elements::Transaction {
        version: 2,
        lock_time: elements::LockTime::ZERO,
        input: vec![TxIn {
            previous_output: outpoint,
            is_pegin: false,
            script_sig: elements::Script::new(),
            sequence: elements::Sequence::ZERO,
            asset_issuance: elements::AssetIssuance::null(),
            witness: TxInWitness::empty(),
        }],
        output: vec![
            TxOut {
                value: confidential::Value::Explicit(output_value),
                script_pubkey: dest_addr.script_pubkey(),
                asset,
                nonce: confidential::Nonce::Null,
                witness: elements::TxOutWitness::empty(),
            },
            TxOut::new_fee(fee_sats, asset.explicit().unwrap()),
        ],
    });

    // 5. Create witness with the secret
    println!("🔐 Step 8: Creating witness data with your secret");
    println!("   Preparing witness values...");

    // Use the same secret_bytes we calculated earlier
    let secret_u256 = simplicityhl::num::U256::from_byte_array(secret_bytes);

    let mut witness_map = HashMap::new();
    witness_map.insert(
        simplicityhl::str::WitnessName::from_str_unchecked("SECRET"),
        Value::u256(secret_u256),
    );
    let witness_values = WitnessValues::from(witness_map);
    println!("   Witness map created with SECRET parameter");

    // 6. Satisfy the program and create final witness
    println!();
    println!("🔓 Step 9: Satisfying the Simplicity program");
    println!("   Running contract with witness data...");
    let satisfied = compiled
        .satisfy(witness_values)
        .map_err(|e| anyhow::anyhow!("Failed to satisfy program: {}", e))?;

    let (program_bytes, witness_bytes) = satisfied.redeem().to_vec_with_witness();
    println!("✅ Program satisfied successfully!");
    println!("   📏 Program size: {} bytes", program_bytes.len());
    println!("   📏 Witness size: {} bytes", witness_bytes.len());
    println!("   📏 Total script: {} bytes", program_bytes.len() + witness_bytes.len());
    println!();

    // 7. Add witness to transaction
    println!("🔧 Step 10: Building Taproot witness structure");
    println!("   Creating Taproot script tree...");
    let internal_key = XOnlyPublicKey::from_str(
        "50929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac0",
    )?;
    println!("   Using unspendable internal key");

    let script = elements::Script::from(compiled.commit().cmr().as_ref().to_vec());
    let builder = elements::taproot::TaprootBuilder::new();

    let leaf_ver_inner: u8 = simplicity::leaf_version().into();
    let leaf_ver = elements::taproot::LeafVersion::from_u8(leaf_ver_inner)
        .expect("valid leaf version");
    println!("   Leaf version: 0x{:02x}", leaf_ver_inner);

    let builder = builder
        .add_leaf_with_ver(0, script.clone(), leaf_ver)
        .expect("tap tree should be valid");
    println!("   Script leaf added to Taproot tree");

    let spend_info = builder
        .finalize(secp256k1::SECP256K1, internal_key)
        .expect("tap tree should be valid");
    println!("   Taproot tree finalized");

    let control_block = spend_info
        .control_block(&(script.clone(), leaf_ver))
        .expect("control block should exist");
    println!("   Control block created ({} bytes)", control_block.serialize().len());

    // Build the final witness
    println!();
    println!("   📦 Assembling final witness stack:");
    println!("      1. Witness data: {} bytes", witness_bytes.len());
    println!("      2. Program code: {} bytes", program_bytes.len());
    println!("      3. Script: {} bytes", script.as_bytes().len());
    println!("      4. Control block: {} bytes", control_block.serialize().len());

    let witness_stack = vec![
        witness_bytes,
        program_bytes,
        script.as_bytes().to_vec(),
        control_block.serialize(),
    ];

    let total_witness_size: usize = witness_stack.iter().map(|v| v.len()).sum();
    println!("   📊 Total witness size: {} bytes", total_witness_size);

    let mut tx = psbt.extract_tx()
        .map_err(|e| anyhow::anyhow!("Failed to extract transaction: {:?}", e))?;
    tx.input[0].witness = TxInWitness {
        script_witness: witness_stack,
        pegin_witness: vec![],
        amount_rangeproof: None,
        inflation_keys_rangeproof: None,
    };
    println!("✅ Transaction witness attached");
    println!();

    // 8. Broadcast transaction
    println!("📡 Step 11: Broadcasting transaction to the network");
    let tx_size = elements::encode::serialize(&tx).len();
    println!("   📦 Transaction size: {} bytes", tx_size);
    println!("   💵 Fee rate: ~{:.2} sats/byte", fee_sats as f64 / tx_size as f64);

    let tx_hex = hex::encode(elements::encode::serialize(&tx));
    println!("   🔤 Transaction hex: {}...{}", &tx_hex[..16], &tx_hex[tx_hex.len()-16..]);
    println!();
    println!("   🌐 Connecting to Elements daemon...");
    println!("   📤 Sending transaction to network...");

    let elements_cli = "/Users/felipe/Desktop/hub/blockchain/elements/src/elements-cli";
    let output = Command::new(elements_cli)
        .args(&[
            "-chain=liquidtestnet",
            "sendrawtransaction",
            &tx_hex,
        ])
        .output()
        .context("Failed to broadcast transaction")?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        println!();
        println!("╔══════════════════════════════════════════════╗");
        println!("║         ❌ BROADCAST FAILED! ❌              ║");
        println!("╚══════════════════════════════════════════════╝");
        eprintln!();
        eprintln!("🚫 Failed to broadcast transaction!");
        eprintln!();
        eprintln!("Error message: {}", error);
        eprintln!();
        eprintln!("Possible reasons:");
        eprintln!("   - Network connectivity issues");
        eprintln!("   - Transaction already in mempool");
        eprintln!("   - Invalid witness data");
        eprintln!("   - UTXO already spent (someone else won!)");
        return Err(anyhow::anyhow!("Failed to broadcast transaction: {}", error));
    }

    let broadcast_txid = String::from_utf8_lossy(&output.stdout).trim().to_string();

    println!();
    println!("╔══════════════════════════════════════════════╗");
    println!("║                                              ║");
    println!("║        🎉🎉🎉 SUCCESS! 🎉🎉🎉              ║");
    println!("║                                              ║");
    println!("║         YOU WON THE PUZZLE!                  ║");
    println!("║                                              ║");
    println!("╚══════════════════════════════════════════════╝");
    println!();
    println!("✅ Transaction successfully broadcasted!");
    println!();
    println!("📊 Transaction Details:");
    println!("   🆔 TXID: {}", broadcast_txid);
    println!("   💰 Prize sent to: {}", dest_address);
    println!("   💵 Amount: {} sats", output_value);
    println!("   💸 In L-BTC: {} L-BTC", output_value as f64 / 100_000_000.0);
    println!("   ⏱️  Time: {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"));
    println!();
    println!("🔍 Next steps:");
    println!("   1. Wait for confirmation (usually 1-2 minutes)");
    println!("   2. Check your wallet balance");
    println!("   3. View transaction details:");
    println!("      ./elements-cli -chain=liquidtestnet gettransaction {}", broadcast_txid);
    println!();
    println!("🏆 Congratulations on solving the puzzle hunt!");
    println!("   Share your victory with #SimplicityCTF");
    println!();

    Ok(())
}