/*
 * CREATE PUZZLE - Creates and funds a puzzle on Liquid testnet
 *
 * Usage:
 *   cargo run --bin create_puzzle -- <secret> <prize_amount> [hint]
 *
 * Examples:
 *   cargo run --bin create_puzzle -- "satoshi" 0.1
 *   cargo run --bin create_puzzle -- "bitcoin" 0.5 "The creator of Bitcoin"
 *   cargo run --bin create_puzzle -- "moon" 0.2 "Where Bitcoin is going 🚀"
 *
 * This will:
 * 1. Calculate the SHA256 of the secret
 * 2. Create a Simplicity contract with that hash
 * 3. Fund it with the specified amount
 * 4. Save puzzle information with hint
 * 5. Print the address and puzzle details
 *
 * The hint parameter is optional. If not provided, it defaults to
 * showing the character count of the secret.
 */

use anyhow::{Context, Result};
use chrono;
use elements::secp256k1_zkp as secp256k1;
use elements::{Address, AddressParams};
use secp256k1::XOnlyPublicKey;
use sha2::{Digest, Sha256};
use simplicityhl::{Arguments, CompiledProgram, Value};
use simplicityhl::value::ValueConstructible;
use std::collections::HashMap;
use std::env;
use std::process::Command;
use std::str::FromStr;

const PUZZLE_CONTRACT: &str = include_str!("../../../SimplicityHL/examples/puzzle_jackpot.simf");

fn main() -> Result<()> {
    // Parse arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 || args.len() > 4 {
        eprintln!("Usage: {} <secret> <amount_in_btc> [hint]", args[0]);
        eprintln!("\nExamples:");
        eprintln!("  {} \"satoshi\" 0.1", args[0]);
        eprintln!("  {} \"bitcoin\" 0.5 \"The creator of Bitcoin\"", args[0]);
        eprintln!("  {} \"moon\" 0.2 \"Where Bitcoin is going 🚀\"", args[0]);
        eprintln!("\nThe hint is optional and will help participants guess the secret.");
        std::process::exit(1);
    }

    let secret = &args[1];
    let amount = &args[2];
    let hint = if args.len() == 4 {
        args[3].clone()
    } else {
        format!("The secret has {} characters", secret.len())
    };

    println!("╔══════════════════════════════════════╗");
    println!("║       🎯 CREATING PUZZLE HUNT 🎯     ║");
    println!("╚══════════════════════════════════════╝");
    println!();

    // 1. Calculate hash of the secret
    println!("📋 Puzzle Configuration:");
    println!("   📝 Secret: {}", secret);
    println!("   💰 Amount: {} L-BTC", amount);
    println!("   💡 Hint: \"{}\"", hint);
    println!();

    println!("🔐 Processing secret and value...");
    // Convert secret to u256 (32 bytes) with right-padding
    let mut secret_bytes = [0u8; 32];
    let secret_raw = secret.as_bytes();
    let len = secret_raw.len().min(32);
    secret_bytes[32 - len..].copy_from_slice(&secret_raw[..len]);

    // Parse amount to validate it's a valid number
    let _amount_btc: f64 = amount.parse()
        .context("Invalid amount format")?;

    // Calculate SHA256 of just the secret
    let mut hasher = Sha256::new();
    hasher.update(&secret_bytes);
    let hash = hasher.finalize();
    let hash_hex = hex::encode(hash);

    // Convert to u256 (32 bytes)
    let mut hash_bytes = [0u8; 32];
    hash_bytes.copy_from_slice(&hash);
    let target_hash = simplicityhl::num::U256::from_byte_array(hash_bytes);

    println!("✅ Target Hash computed: 0x{}", hash_hex);
    println!("   Formula: SHA256(secret)");
    println!();

    // 2. Compile the contract with the hash
    let mut arguments = HashMap::new();
    arguments.insert(
        simplicityhl::str::WitnessName::from_str_unchecked("TARGET_HASH"),
        Value::u256(target_hash),
    );
    let args = Arguments::from(arguments);

    println!("⚙️  Compiling Simplicity contract...");
    let compiled = CompiledProgram::new(PUZZLE_CONTRACT, args, false)
        .map_err(|e| anyhow::anyhow!("Failed to compile contract: {}", e))?;
    println!("✅ Contract compiled!");
    println!();

    // 3. Create Taproot address
    let internal_key = XOnlyPublicKey::from_str(
        "50929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac0",
    )?;

    let script = elements::Script::from(compiled.commit().cmr().as_ref().to_vec());
    let builder = elements::taproot::TaprootBuilder::new();
    // Convert LeafVersion between elements versions
    let leaf_ver_inner: u8 = simplicity::leaf_version().into();
    let leaf_ver = elements::taproot::LeafVersion::from_u8(leaf_ver_inner)
        .expect("valid leaf version");
    let builder = builder
        .add_leaf_with_ver(0, script, leaf_ver)
        .expect("tap tree should be valid");

    let spend_info = builder
        .finalize(secp256k1::SECP256K1, internal_key)
        .expect("tap tree should be valid");

    let address = Address::p2tr(
        secp256k1::SECP256K1,
        spend_info.internal_key(),
        spend_info.merkle_root(),
        None,
        &AddressParams::LIQUID_TESTNET,
    );

    println!("📍 Puzzle Address:");
    println!("   {}", address);
    println!();

    // 4. Send funds using elements-cli
    println!("💰 Funding puzzle with {} L-BTC...", amount);

    // NOTE: elementsd must be running! Check with: ps aux | grep elementsd
    let elements_cli = "/Users/felipe/Desktop/hub/blockchain/elements/src/elements-cli";
    let output = Command::new(elements_cli)
        .args(&[
            "-chain=liquidtestnet",
            "-rpcwallet=my_wallet",  // Use wallet with funds
            "sendtoaddress",
            &address.to_string(),
            amount
        ])
        .output()
        .context("Failed to execute elements-cli")?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("Failed to send funds: {}", error));
    }

    let txid = String::from_utf8_lossy(&output.stdout).trim().to_string();
    println!("✅ Puzzle funded!");
    println!("   TXID: {}", txid);
    println!();

    // 5. Save puzzle information
    let public_info = serde_json::json!({
        "target_hash": format!("0x{}", hash_hex),
        "address": address.to_string(),
        "txid": txid,
        "vout": 0,
        "amount": amount,
        "hint": hint.clone(),
        "created_at": chrono::Local::now().to_rfc3339(),
    });

    let filename = format!("puzzle_{}.json", &hash_hex[..8]);
    std::fs::write(&filename, serde_json::to_string_pretty(&public_info)?)?;

    // Save private info for creator only
    let private_info = serde_json::json!({
        "secret": secret,
        "hash": format!("0x{}", hash_hex),
        "txid": txid,
        "amount": amount,
        "hint": hint.clone(),
        "address": address.to_string(),
        "created_at": chrono::Local::now().to_rfc3339(),
    });

    let private_filename = format!("puzzle_{}_SECRET.json", &hash_hex[..8]);
    std::fs::write(&private_filename, serde_json::to_string_pretty(&private_info)?)?;

    println!("💾 Files saved:");
    println!("   📄 Public file: {}", filename);
    println!("   🔒 Private file: {}", private_filename);
    println!();
    println!("╔══════════════════════════════════════╗");
    println!("║    🎉 PUZZLE CREATED SUCCESSFULLY!    ║");
    println!("╚══════════════════════════════════════╝");
    println!();
    println!("📢 Share with participants:");
    println!("   📍 Address: {}", address);
    println!("   💰 Prize: {} L-BTC", amount);
    println!("   💡 Hint: \"{}\"", hint);
    println!("   🔐 Target Hash: 0x{}", hash_hex);
    println!("   📄 Puzzle file: {}", filename);
    println!();
    println!("⚠️  IMPORTANT:");
    println!("   - DO NOT share the _SECRET.json file!");
    println!("   - The secret is case-sensitive");
    println!("   - Share the {} file with participants", filename);

    Ok(())
}
