/*
 * SOLVE PUZZLE - Resolve e reclama o prêmio de um puzzle
 *
 * Uso:
 *   cargo run --bin solve-puzzle -- <puzzle_json_file> <secret> <destination_address>
 *
 * Exemplo:
 *   cargo run --bin solve-puzzle -- puzzle_2cf24dba.json "satoshi" tex1q...
 *
 * Isso vai:
 * 1. Ler as informações do puzzle
 * 2. Criar uma transação gastando o UTXO do puzzle
 * 3. Fornecer o secret como witness
 * 4. Transmitir e ganhar o prêmio!
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
    // Parse argumentos
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Uso: {} <puzzle_json> <secret> <destination_address>", args[0]);
        eprintln!("\nExemplo:");
        eprintln!("  {} puzzle_2cf24dba.json \"satoshi\" tex1q...", args[0]);
        std::process::exit(1);
    }

    let puzzle_file = &args[1];
    let secret = &args[2];
    let dest_address = &args[3];

    println!("🎯 RESOLVENDO PUZZLE");
    println!("===================");
    println!();

    // 1. Ler informações do puzzle
    println!("📖 Lendo puzzle de: {}", puzzle_file);
    let puzzle_data = std::fs::read_to_string(puzzle_file)?;
    let puzzle: serde_json::Value = serde_json::from_str(&puzzle_data)?;

    let expected_hash = puzzle["hash"].as_str().unwrap();
    let puzzle_address = puzzle["address"].as_str().unwrap();

    println!("   Endereço do puzzle: {}", puzzle_address);
    println!("   Hash esperado: {}", expected_hash);
    println!();

    // 2. Verificar se o secret está correto
    println!("🔍 Verificando secret...");
    let mut hasher = Sha256::new();
    hasher.update(secret.as_bytes());
    let hash = hasher.finalize();
    let hash_hex = format!("0x{}", hex::encode(hash));

    if hash_hex != expected_hash {
        eprintln!("❌ ERRO: Secret incorreto!");
        eprintln!("   Esperado: {}", expected_hash);
        eprintln!("   Obtido:   {}", hash_hex);
        std::process::exit(1);
    }

    println!("✅ Secret correto!");
    println!();

    // 3. Compilar o contrato
    println!("⚙️  Compilando contrato...");
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
        .map_err(|e| anyhow::anyhow!("Falha ao compilar contrato: {}", e))?;
    println!("✅ Contrato compilado!");
    println!();

    // 4. Conectar ao elementsd (deve estar rodando!)
    let mut daemon = ElementsD::new("/Users/felipe/Desktop/hub/blockchain/elements/src/elementsd")
        .map_err(|e| anyhow::anyhow!("Falha ao criar cliente elementsd: {:?}", e))?;
    daemon.chain = Some("liquidtestnet".to_string());

    // 5. Encontrar o UTXO do puzzle
    println!("🔎 Procurando UTXO do puzzle...");
    let address = Address::from_str(puzzle_address)?;

    // Aqui você precisaria implementar a busca pelo UTXO
    // Por simplicidade, vamos assumir que você tem o txid e vout
    // TODO: Implementar busca automática de UTXO

    println!("⚠️  NOTA: Você precisa fornecer o TXID e VOUT manualmente");
    println!("   Use: elements-cli listunspent");
    println!("   E procure pelo endereço: {}", puzzle_address);
    println!();
    println!("   Depois, edite este script para incluir:");
    println!("   - txid: o ID da transação");
    println!("   - vout: o índice do output");
    println!("   - value: o valor em satoshis");
    println!("   - asset: o asset ID");
    println!();

    // Exemplo de como seria (você precisa preencher):
    let txid_str = "SEU_TXID_AQUI";
    let vout = 0u32;
    let value_sats = 10_000_000u64; // 0.1 BTC = 10 milhões de sats

    if txid_str == "SEU_TXID_AQUI" {
        eprintln!("❌ ERRO: Você precisa editar o script e adicionar o TXID/VOUT");
        eprintln!("   Execute: elements-cli -chain=liquidtestnet listunspent");
        std::process::exit(1);
    }

    let txid = elements::Txid::from_str(txid_str)?;
    let outpoint = OutPoint::new(txid, vout);

    println!("✅ UTXO encontrado: {}:{}", txid, vout);
    println!();

    // 6. Criar transação de gasto
    println!("💸 Criando transação de gasto...");

    let dest_addr = Address::from_str(dest_address)?;
    let fee_sats = 3_000u64;
    let output_value = value_sats - fee_sats;

    // Obter asset ID da testnet (você precisa pegar isso do UTXO real)
    // Este é um placeholder - use o asset do seu UTXO real
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

    // 7. Criar witness com o secret
    println!("🔐 Criando witness com secret...");

    let secret_value = Value::u256(target_hash);
    let mut witness_map = HashMap::new();
    witness_map.insert(
        simplicityhl::str::WitnessName::from_str_unchecked("SECRET"),
        secret_value,
    );
    let witness_values = WitnessValues::from(witness_map);

    // 8. Satisfazer o programa e criar witness final
    let satisfied = compiled
        .satisfy(witness_values)
        .context("Falha ao satisfazer programa")?;

    let (program_bytes, witness_bytes) = satisfied.redeem().encode_to_vec();

    // 9. Adicionar witness à transação
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

    // 10. Transmitir transação
    println!("📡 Transmitindo transação...");

    let tx = psbt
        .extract_tx()
        .expect("transaction should be extractable");

    match daemon.send_raw_transaction(&tx) {
        txid => {
            println!();
            println!("🎉🎉🎉 SUCESSO! 🎉🎉🎉");
            println!();
            println!("✅ Transação transmitida!");
            println!("   TXID: {}", txid);
            println!();
            println!("💰 Prêmio enviado para: {}", dest_address);
            println!("   Valor: {} sats", output_value);
            println!();
            println!("🏆 VOCÊ GANHOU O PUZZLE!");
        }
    }

    Ok(())
}
