/*
 * CREATE PUZZLE - Cria e financia um puzzle na Liquid testnet
 *
 * Uso:
 *   cargo run --bin create-puzzle -- <secret> <prize_amount>
 *
 * Exemplo:
 *   cargo run --bin create-puzzle -- "satoshi" 0.1
 *
 * Isso vai:
 * 1. Calcular o SHA256 do secret
 * 2. Criar um contrato Simplicity com esse hash
 * 3. Financiar com o valor especificado
 * 4. Printar o endereço e informações
 */

use anyhow::{Context, Result};
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

const PUZZLE_CONTRACT: &str = include_str!("../../../examples/puzzle_jackpot.simf");

fn main() -> Result<()> {
    // Parse argumentos
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Uso: {} <secret> <amount_in_btc>", args[0]);
        eprintln!("\nExemplo:");
        eprintln!("  {} \"satoshi\" 0.1", args[0]);
        std::process::exit(1);
    }

    let secret = &args[1];
    let amount = &args[2];

    println!("🎯 CRIANDO PUZZLE HUNT");
    println!("====================");
    println!();

    // 1. Calcular hash do secret
    let mut hasher = Sha256::new();
    hasher.update(secret.as_bytes());
    let hash = hasher.finalize();
    let hash_hex = hex::encode(hash);

    // Converter para u256 (32 bytes)
    let mut hash_bytes = [0u8; 32];
    hash_bytes.copy_from_slice(&hash);
    let target_hash = simplicityhl::num::U256::from_byte_array(hash_bytes);

    println!("📝 Secret: {}", secret);
    println!("🔐 Hash (SHA256): 0x{}", hash_hex);
    println!();

    // 2. Compilar o contrato com o hash
    let mut arguments = HashMap::new();
    arguments.insert(
        simplicityhl::str::WitnessName::from_str_unchecked("TARGET_HASH"),
        Value::u256(target_hash),
    );
    let args = Arguments::from(arguments);

    println!("⚙️  Compilando contrato Simplicity...");
    let compiled = CompiledProgram::new(PUZZLE_CONTRACT, args, false)
        .map_err(|e| anyhow::anyhow!("Falha ao compilar contrato: {}", e))?;
    println!("✅ Contrato compilado!");
    println!();

    // 3. Criar endereço Taproot
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

    println!("📍 Endereço do Puzzle:");
    println!("   {}", address);
    println!();

    // 4. Enviar fundos usando elements-cli
    println!("💰 Financiando puzzle com {} L-BTC...", amount);

    // NOTA: O elementsd deve estar rodando! Verificar com: ps aux | grep elementsd
    let elements_cli = "/Users/felipe/Desktop/hub/blockchain/elements/src/elements-cli";
    let output = Command::new(elements_cli)
        .args(&[
            "-chain=liquidtestnet",
            "-rpcwallet=my_wallet",  // Usar a wallet com fundos
            "sendtoaddress",
            &address.to_string(),
            amount
        ])
        .output()
        .context("Falha ao executar elements-cli")?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("Falha ao enviar fundos: {}", error));
    }

    let txid = String::from_utf8_lossy(&output.stdout).trim().to_string();
    println!("✅ Puzzle financiado!");
    println!("   TXID: {}", txid);
    println!();

    // 5. Salvar informações
    let info = serde_json::json!({
        "secret": secret,
        "hash": format!("0x{}", hash_hex),
        "address": address.to_string(),
        "amount": amount,
        "hint": format!("A senha tem {} caracteres", secret.len()),
    });

    let filename = format!("puzzle_{}.json", &hash_hex[..8]);
    std::fs::write(&filename, serde_json::to_string_pretty(&info)?)?;

    println!("💾 Informações salvas em: {}", filename);
    println!();
    println!("🎉 PUZZLE CRIADO COM SUCESSO!");
    println!();
    println!("📢 Compartilhe com os participantes:");
    println!("   Endereço: {}", address);
    println!("   Prêmio: {} L-BTC", amount);
    println!("   Hash do Secret: 0x{}", hash_hex);
    println!();
    println!("🔍 Hint: A senha tem {} caracteres", secret.len());
    println!();
    println!("⚠️  GUARDAR O SECRET EM SEGREDO!");
    println!("   Secret: {} (não compartilhe isso!)", secret);

    Ok(())
}
