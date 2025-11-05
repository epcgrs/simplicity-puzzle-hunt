# ⚡ Quick Start - 5 Minutos para Começar

## Passo 1: Garantir que seu node Elements está rodando

```bash
# Verificar se está rodando
ps aux | grep elementsd

# Se não estiver, iniciar:
cd $HOME/Desktop/hub/blockchain/elements
./src/elementsd -chain=liquidtestnet -daemon

# Aguardar alguns segundos e verificar
./src/elements-cli -chain=liquidtestnet getblockchaininfo
```

## Passo 2: Criar seu primeiro puzzle

```bash
cd /Users/felipe/Desktop/hub/blockchain/SimplicityHL/hackathon_puzzle

# Criar puzzle com secret "hello" e prêmio de 0.001 L-BTC
cargo run --bin create-puzzle -- "hello" 0.001
```

**IMPORTANTE**: Guarde o arquivo JSON que foi criado (ex: `puzzle_2cf24dba.json`)!

## Passo 3: Testar resolver seu próprio puzzle

### 3.1 - Encontrar o UTXO

```bash
# Pegar o endereço do arquivo JSON criado
cat puzzle_*.json | grep address

# Listar UTXOs desse endereço
cd $HOME/Desktop/hub/blockchain/elements
./src/elements-cli -chain=liquidtestnet listunspent 1 9999999 '["<ENDERECO_DO_PUZZLE>"]'
```

Você verá algo assim:
```json
[
  {
    "txid": "a1b2c3d4...",
    "vout": 0,
    "amount": 0.00100000,
    "asset": "144c654344aa...",
    ...
  }
]
```

### 3.2 - Editar o solver com as informações do UTXO

Abra o arquivo:
```bash
code hackathon_puzzle/src/bin/solve_puzzle.rs
```

E substitua (aproximadamente linha 120):
```rust
let txid_str = "a1b2c3d4..."; // Cole seu TXID aqui
let vout = 0u32; // Cole seu vout aqui
let value_sats = 100_000u64; // Valor em sats (0.001 = 100,000 sats)

// Também precisa do asset ID - pegue do listunspent
let asset_bytes = hex::decode("144c654344aa...").unwrap();
let mut asset_array = [0u8; 32];
asset_array.copy_from_slice(&asset_bytes);
let asset = confidential::Asset::Explicit(elements::AssetId::from_inner(asset_array));
```

### 3.3 - Criar endereço para receber o prêmio

```bash
cd $HOME/Desktop/hub/blockchain/elements
./src/elements-cli -chain=liquidtestnet getnewaddress
```

Guarde esse endereço!

### 3.4 - Resolver o puzzle

```bash
cd /Users/felipe/Desktop/hub/blockchain/SimplicityHL/hackathon_puzzle

cargo run --bin solve-puzzle -- puzzle_2cf24dba.json "hello" <SEU_ENDERECO_NOVO>
```

Se tudo der certo, você verá:
```
🎉🎉🎉 SUCESSO! 🎉🎉🎉
TXID: ...
```

## Passo 4: Adicionar mais fundos ao jackpot

```bash
cargo run --bin add-to-pot -- puzzle_2cf24dba.json 0.002
```

Isso adiciona mais 0.002 L-BTC ao prêmio!

---

## 🐛 Problemas Comuns

### "failed to connect to daemon"
- Seu elementsd não está rodando
- Solução: `./src/elementsd -chain=liquidtestnet -daemon`

### "insufficient funds"
- Você não tem L-BTC na testnet
- Solução: Use um faucet: https://liquidtestnet.com/faucet

### "Parameter TARGET_HASH is missing"
- Você está tentando compilar o contrato diretamente
- Solução: Use os scripts `create-puzzle` ou `solve-puzzle`

### "Secret incorreto"
- Você está usando o secret errado
- Solução: Verifique o arquivo JSON do puzzle

---

## 🎯 Para Apresentação no Hackathon

1. **Crie 3 puzzles diferentes:**
```bash
cargo run --bin create-puzzle -- "bitcoin" 0.005
cargo run --bin create-puzzle -- "satoshi" 0.010
cargo run --bin create-puzzle -- "hackathon2025" 0.020
```

2. **Prepare slides mostrando:**
   - Endereços dos puzzles
   - Valores dos prêmios
   - Hints progressivos

3. **Durante apresentação:**
   - Mostre o código do contrato (`examples/puzzle_jackpot.simf`)
   - Demonstre criando um puzzle ao vivo
   - Deixe alguém tentar resolver
   - Mostre a transação no explorer

---

## 📱 Próximos Passos Depois do Hackathon

- [ ] Web interface para criar/resolver puzzles
- [ ] Suporte a múltiplos puzzles simultâneos
- [ ] Time-lock (puzzles que expiram)
- [ ] Leaderboard
- [ ] Sistema de hints progressivos automáticos
- [ ] Integração com Discord/Telegram para notificações

---

**Boa sorte! 🚀**

Se tiver dúvidas, abra uma issue ou me mande mensagem!
