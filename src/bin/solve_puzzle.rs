/*
 * SOLVE PUZZLE - Solves and claims the prize from a puzzle
 *
 * Usage:
 *   cargo run --bin solve-puzzle -- <puzzle_json_file> <secret> <destination_address>
 *
 * Example:
 *   cargo run --bin solve-puzzle -- puzzle_2cf24dba.json "satoshi" tex1q...
 *
 * This will:
 * 1. Read the puzzle information
 * 2. Create a transaction spending the puzzle UTXO
 * 3. Provide the secret as witness
 * 4. Broadcast and win the prize!
 */

use anyhow::{Context, Result};
use elements::hashes::Hash;
use elements::pset::PartiallySignedTransaction as Psbt;
use elements::{confidential, secp256k1_zkp as secp256k1, Address, OutPoint, TxIn, TxInWitness, TxOut};
use elementsd::ElementsD;
use secp256k1::XOnlyPublicKey;
use sha2::{Digest, Sha256};
use simplicity::jet::elements::{ElementsEnv, ElementsUtxo};
use simplicityhl::{Arguments, CompiledProgram, Value, WitnessValues};
use simplicityhl::value::ValueConstructible;
use std::collections::HashMap;
use std::env;
use std::str::FromStr;

const PUZZLE_CONTRACT: &str = include_str!("../../../examples/puzzle_jackpot.simf");

fn main() -> Result<()> {
    // Parse arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: {} <puzzle_json> <secret> <destination_address>", args[0]);
        eprintln!("\nExample:");
        eprintln!("  {} puzzle_2cf24dba.json \"satoshi\" tex1q...", args[0]);
        std::process::exit(1);
    }

    let puzzle_file = &args[1];
    let secret = &args[2];
    let dest_address = &args[3];

    println!("🎯 SOLVING PUZZLE");
    println!("=================");
    println!();

    // 1. Read puzzle information
    println!("📖 Reading puzzle from: {}", puzzle_file);
    let puzzle_data = std::fs::read_to_string(puzzle_file)?;
    let puzzle: serde_json::Value = serde_json::from_str(&puzzle_data)?;

    let expected_hash = puzzle["hash"].as_str().unwrap();
    let puzzle_address = puzzle["address"].as_str().unwrap();

    println!("   Puzzle address: {}", puzzle_address);
    println!("   Expected hash: {}", expected_hash);
    println!();

    // 2. Verify the secret is correct
    println!("🔍 Verifying secret...");
    let mut hasher = Sha256::new();
    hasher.update(secret.as_bytes());
    let hash = hasher.finalize();
    let hash_hex = format!("0x{}", hex::encode(hash));

    if hash_hex != expected_hash {
        eprintln!("❌ ERROR: Incorrect secret!");
        eprintln!("   Expected: {}", expected_hash);
        eprintln!("   Got:      {}", hash_hex);
        std::process::exit(1);
    }

    println!("✅ Secret is correct!");
    println!();

    // 3. Compile the contract
    println!("⚙️  Compiling contract...");
    let mut hash_bytes = [0u8; 32];
    hash_bytes.copy_from_slice(&hash);
    let target_hash = simplicityhl::num::U256::from_byte_array(hash_bytes);

    let mut arguments = HashMap::new();
    arguments.insert(
        simplicityhl::str::WitnessName::from_str_unchecked("TARGET_HASH"),
        Value::u256(target_hash),
    );
    let args = Arguments::from(arguments);

    let compiled = CompiledProgram::new(PUZZLE_CONTRACT, args, false)
        .map_err(|e| anyhow::anyhow!("Failed to compile contract: {}", e))?;
    println!("✅ Contract compiled!");
    println!();

    // 4. Connect to elementsd (must be running!)
    let mut daemon = ElementsD::new("/Users/felipe/Desktop/hub/blockchain/elements/src/elementsd")
        .map_err(|e| anyhow::anyhow!("Failed to create elementsd client: {:?}", e))?;
    daemon.chain = Some("liquidtestnet".to_string());

    // 5. Find the puzzle UTXO
    println!("🔎 Looking for puzzle UTXO...");
    let address = Address::from_str(puzzle_address)?;

    // Here you would need to implement UTXO search
    // For simplicity, we'll assume you have the txid and vout
    // TODO: Implement automatic UTXO search

    println!("⚠️  NOTE: You need to provide the TXID and VOUT manually");
    println!("   Use: elements-cli listunspent");
    println!("   And search for the address: {}", puzzle_address);
    println!();
    println!("   Then, edit this script to include:");
    println!("   - txid: the transaction ID");
    println!("   - vout: the output index");
    println!("   - value: the value in satoshis");
    println!("   - asset: the asset ID");
    println!();

    // Example of how it would be (you need to fill in):
    let txid_str = "YOUR_TXID_HERE";
    let vout = 0u32;
    let value_sats = 10_000_000u64; // 0.1 BTC = 10 million sats

    if txid_str == "YOUR_TXID_HERE" {
        eprintln!("❌ ERROR: You need to edit the script and add the TXID/VOUT");
        eprintln!("   Run: elements-cli -chain=liquidtestnet listunspent");
        std::process::exit(1);
    }

    let txid = elements::Txid::from_str(txid_str)?;
    let outpoint = OutPoint::new(txid, vout);

    println!("✅ UTXO found: {}:{}", txid, vout);
    println!();

    // 6. Create spending transaction
    println!("💸 Creating spending transaction...");

    let dest_addr = Address::from_str(dest_address)?;
    let fee_sats = 3_000u64;
    let output_value = value_sats - fee_sats;

    // Get testnet asset ID (you need to get this from the real UTXO)
    // This is a placeholder - use the asset from your real UTXO
    let asset = confidential::Asset::Explicit(elements::AssetId::default());

    let mut psbt = Psbt::from_tx(elements::Transaction {
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

    // 7. Create witness with the secret
    println!("🔐 Creating witness with secret...");

    let secret_value = Value::u256(target_hash);
    let mut witness_map = HashMap::new();
    witness_map.insert(
        simplicityhl::str::WitnessName::from_str_unchecked("SECRET"),
        secret_value,
    );
    let witness_values = WitnessValues::from(witness_map);

    // 8. Satisfy the program and create final witness
    let satisfied = compiled
        .satisfy(witness_values)
        .context("Failed to satisfy program")?;

    let (program_bytes, witness_bytes) = satisfied.redeem().encode_to_vec();

    // 9. Add witness to transaction
    let internal_key = XOnlyPublicKey::from_str(
        "50929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac0",
    )?;

    let script = elements::Script::from(compiled.commit().cmr().as_ref().to_vec());
    let builder = elements::taproot::TaprootBuilder::new();
    let builder = builder
        .add_leaf_with_ver(0, script.clone(), simplicity::leaf_version())
        .expect("tap tree should be valid");

    let spend_info = builder
        .finalize(secp256k1::SECP256K1, internal_key)
        .expect("tap tree should be valid");

    let control_block = spend_info
        .control_block(&(script, simplicity::leaf_version()))
        .expect("control block should exist");

    psbt.inputs_mut()[0].final_script_witness = Some(vec![
        witness_bytes,
        program_bytes,
        control_block.serialize(),
    ]);

    // 10. Broadcast transaction
    println!("📡 Broadcasting transaction...");

    let tx = psbt
        .extract_tx()
        .expect("transaction should be extractable");

    match daemon.send_raw_transaction(&tx) {
        txid => {
            println!();
            println!("🎉🎉🎉 SUCCESS! 🎉🎉🎉");
            println!();
            println!("✅ Transaction broadcasted!");
            println!("   TXID: {}", txid);
            println!();
            println!("💰 Prize sent to: {}", dest_address);
            println!("   Amount: {} sats", output_value);
            println!();
            println!("🏆 YOU WON THE PUZZLE!");
        }
    }

    Ok(())
}
