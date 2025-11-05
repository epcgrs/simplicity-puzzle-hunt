/*
 * ADD TO POT - Adiciona mais fundos ao jackpot do puzzle
 *
 * Uso:
 *   cargo run --bin add-to-pot -- <puzzle_json> <amount>
 *
 * Exemplo:
 *   cargo run --bin add-to-pot -- puzzle_2cf24dba.json 0.05
 *
 * Isso vai adicionar mais fundos ao endereço do puzzle, aumentando o prêmio!
 */

use anyhow::{Context, Result};
use elements::Address;
use elementsd::ElementsD;
use std::env;
use std::str::FromStr;

fn main() -> Result<()> {
    // Parse argumentos
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Uso: {} <puzzle_json> <amount_in_btc>", args[0]);
        eprintln!("\nExemplo:");
        eprintln!("  {} puzzle_2cf24dba.json 0.05", args[0]);
        std::process::exit(1);
    }

    let puzzle_file = &args[1];
    let amount = &args[2];

    println!("💰 AUMENTANDO JACKPOT");
    println!("====================");
    println!();

    // 1. Ler informações do puzzle
    let puzzle_data = std::fs::read_to_string(puzzle_file)?;
    let puzzle: serde_json::Value = serde_json::from_str(&puzzle_data)?;

    let puzzle_address = puzzle["address"].as_str().unwrap();
    let current_amount = puzzle["amount"].as_str().unwrap();

    println!("📍 Endereço do puzzle: {}", puzzle_address);
    println!("💵 Prêmio atual: {} L-BTC", current_amount);
    println!("➕ Adicionando: {} L-BTC", amount);
    println!();

    // 2. Conectar ao elementsd (deve estar rodando!)
    let mut daemon = ElementsD::new("/Users/felipe/Desktop/hub/blockchain/elements/src/elementsd")
        .map_err(|e| anyhow::anyhow!("Falha ao criar cliente elementsd: {:?}", e))?;
    daemon.chain = Some("liquidtestnet".to_string());

    // 3. Enviar fundos
    let address = Address::from_str(puzzle_address)?;

    println!("📤 Enviando fundos...");
    match daemon.send_to_address(&address, amount) {
        txid => {
            println!("✅ Fundos adicionados!");
            println!("   TXID: {}", txid);
            println!();
        }
    }

    // 4. Atualizar arquivo JSON (estimativa)
    let new_amount: f64 = current_amount.parse::<f64>()? + amount.parse::<f64>()?;
    let mut updated_puzzle = puzzle.as_object().unwrap().clone();
    updated_puzzle.insert(
        "amount".to_string(),
        serde_json::Value::String(format!("{:.8}", new_amount)),
    );

    std::fs::write(puzzle_file, serde_json::to_string_pretty(&updated_puzzle)?)?;

    println!("💾 Arquivo atualizado: {}", puzzle_file);
    println!("🎉 Novo prêmio estimado: {:.8} L-BTC", new_amount);
    println!();
    println!("📢 Compartilhe com os participantes que o jackpot aumentou!");

    Ok(())
}
