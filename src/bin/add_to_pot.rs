/*
 * ADD TO POT - Adds more funds to the puzzle jackpot
 *
 * Usage:
 *   cargo run --bin add-to-pot -- <puzzle_json> <amount>
 *
 * Example:
 *   cargo run --bin add-to-pot -- puzzle_2cf24dba.json 0.05
 *
 * This will add more funds to the puzzle address, increasing the prize!
 */

use anyhow::{Context, Result};
use elements::Address;
use elementsd::ElementsD;
use std::env;
use std::str::FromStr;

fn main() -> Result<()> {
    // Parse arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <puzzle_json> <amount_in_btc>", args[0]);
        eprintln!("\nExample:");
        eprintln!("  {} puzzle_2cf24dba.json 0.05", args[0]);
        std::process::exit(1);
    }

    let puzzle_file = &args[1];
    let amount = &args[2];

    println!("💰 INCREASING JACKPOT");
    println!("=====================");
    println!();

    // 1. Read puzzle information
    let puzzle_data = std::fs::read_to_string(puzzle_file)?;
    let puzzle: serde_json::Value = serde_json::from_str(&puzzle_data)?;

    let puzzle_address = puzzle["address"].as_str().unwrap();
    let current_amount = puzzle["amount"].as_str().unwrap();

    println!("📍 Puzzle address: {}", puzzle_address);
    println!("💵 Current prize: {} L-BTC", current_amount);
    println!("➕ Adding: {} L-BTC", amount);
    println!();

    // 2. Connect to elementsd (must be running!)
    let mut daemon = ElementsD::new("/Users/felipe/Desktop/hub/blockchain/elements/src/elementsd")
        .map_err(|e| anyhow::anyhow!("Failed to create elementsd client: {:?}", e))?;
    daemon.chain = Some("liquidtestnet".to_string());

    // 3. Send funds
    let address = Address::from_str(puzzle_address)?;

    println!("📤 Sending funds...");
    match daemon.send_to_address(&address, amount) {
        txid => {
            println!("✅ Funds added!");
            println!("   TXID: {}", txid);
            println!();
        }
    }

    // 4. Update JSON file (estimate)
    let new_amount: f64 = current_amount.parse::<f64>()? + amount.parse::<f64>()?;
    let mut updated_puzzle = puzzle.as_object().unwrap().clone();
    updated_puzzle.insert(
        "amount".to_string(),
        serde_json::Value::String(format!("{:.8}", new_amount)),
    );

    std::fs::write(puzzle_file, serde_json::to_string_pretty(&updated_puzzle)?)?;

    println!("💾 File updated: {}", puzzle_file);
    println!("🎉 New estimated prize: {:.8} L-BTC", new_amount);
    println!();
    println!("📢 Share with participants that the jackpot has increased!");

    Ok(())
}
