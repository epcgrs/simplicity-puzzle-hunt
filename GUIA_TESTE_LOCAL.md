# 🎯 Guia de Teste Local - Bitcoin Puzzle Hunt

## 📋 O Que Foi Feito

Este projeto implementa um **jogo de caça ao tesouro on-chain** usando contratos inteligentes Simplicity na Liquid Network.

### Como Funciona

1. **Você bloqueia Bitcoin com uma senha secreta**
   - Escolhe uma senha (ex: "satoshi")
   - O sistema calcula o hash SHA256 da senha
   - Cria um contrato inteligente que só libera os fundos se alguém fornecer a senha correta

2. **Publica hints sobre a senha**
   - "A senha tem 7 letras"
   - "É o nome do criador do Bitcoin"
   - etc.

3. **Primeira pessoa que descobrir a senha ganha TUDO!**
   - O contrato verifica matematicamente se a senha está correta
   - Não tem intermediário, não tem como trapacear
   - É pura matemática + blockchain

### Componentes Criados

✅ **3 binários Rust:**
- `create-puzzle` - Cria e financia puzzles
- `solve-puzzle` - Resolve puzzles e reclama prêmios
- `add-to-pot` - Adiciona mais fundos ao jackpot

✅ **Contrato Simplicity** (`examples/puzzle_jackpot.simf`):
```rust
fn main() {
    let secret: u256 = witness::SECRET;          // Senha fornecida
    let target_hash: u256 = param::TARGET_HASH;  // Hash esperado
    let computed_hash: u256 = sha2(secret);      // Calcula hash da senha
    assert!(jet::eq_256(computed_hash, target_hash)); // Verifica se bate
}
```

✅ **Sistema de Taproot + Simplicity** para segurança máxima

---

## 🚀 Como Testar no Seu elementsd Local

### Pré-requisitos

1. **elementsd rodando em localhost**
2. **Rust instalado** (para compilar)
3. **Alguns L-BTC na carteira** (testnet ou regtest)

### Passo 1: Verificar elementsd

```bash
# Ver se está rodando
ps aux | grep elementsd

# Se não estiver, iniciar
cd $HOME/Desktop/hub/blockchain/elements
./src/elementsd -chain=liquidtestnet -daemon

# OU em regtest (mais rápido para testes)
./src/elementsd -chain=liquidregtest -daemon

# Verificar conexão
./src/elements-cli -chain=liquidtestnet getblockchaininfo
```

### Passo 2: Garantir que Tem Fundos

**Se estiver em testnet:**
```bash
# Gerar endereço
./src/elements-cli -chain=liquidtestnet getnewaddress

# Pegar fundos no faucet
# https://liquidtestnet.com/faucet

# Verificar saldo
./src/elements-cli -chain=liquidtestnet getbalance
```

**Se estiver em regtest:**
```bash
# Gerar blocos e ganhar coinbase
./src/elements-cli -chain=liquidregtest generatetoaddress 101 $(./src/elements-cli -chain=liquidregtest getnewaddress)

# Verificar saldo
./src/elements-cli -chain=liquidregtest getbalance
```

### Passo 3: Ajustar Paths no Código

Os binários têm paths hardcoded. Vamos ajustar para o seu sistema:

```bash
cd /Users/felipe/Desktop/hub/blockchain/SimplicityHL/hackathon_puzzle

# Verificar o path atual do seu elementsd
which elementsd
# OU
ls $HOME/Desktop/hub/blockchain/elements/src/elementsd
```

**Editar os 3 arquivos:**

1. `src/bin/create_puzzle.rs` (linha ~75)
2. `src/bin/solve_puzzle.rs` (linha ~90)
3. `src/bin/add_to_pot.rs` (linha ~49)

Trocar o path para o seu elementsd:
```rust
let daemon = ElementsD::new(
    "/Users/felipe/Desktop/hub/blockchain/elements/src/elementsd",  // ← SEU PATH
    "/Users/felipe/Desktop/hub/blockchain/elements",                 // ← DIR BASE
)?;
```

E a chain (se quiser usar regtest):
```rust
daemon.chain = Some("liquidtestnet".to_string());  // ou "liquidregtest"
```

### Passo 4: Compilar o Projeto

```bash
cd /Users/felipe/Desktop/hub/blockchain/SimplicityHL/hackathon_puzzle

# Compilar
cargo build --release

# Vai demorar um pouco na primeira vez
```

### Passo 5: Criar Seu Primeiro Puzzle! 🎯

```bash
# Criar puzzle com secret "hello" e prêmio de 0.001 L-BTC
cargo run --release --bin create-puzzle -- "hello" 0.001
```

**Você vai ver algo assim:**
```
🎯 CRIANDO PUZZLE HUNT
====================

📝 Secret: hello
🔐 Hash (SHA256): 0x2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824

⚙️  Compilando contrato Simplicity...
✅ Contrato compilado!

📍 Endereço do Puzzle:
   tex1qjr5yzs... (ou lq1q... se regtest)

💰 Financiando puzzle com 0.001 L-BTC...
✅ Puzzle financiado!
   TXID: a1b2c3d4e5f6...

💾 Informações salvas em: puzzle_2cf24dba.json

🎉 PUZZLE CRIADO COM SUCESSO!
```

**IMPORTANTE:** Guarde o arquivo `puzzle_2cf24dba.json`!

### Passo 6: Verificar o Puzzle On-Chain

```bash
# Pegar o endereço do puzzle
cat puzzle_2cf24dba.json | grep address

# Listar UTXOs desse endereço
cd $HOME/Desktop/hub/blockchain/elements
./src/elements-cli -chain=liquidtestnet listunspent 0 9999999 '["<ENDERECO_DO_PUZZLE>"]'
```

Você vai ver algo assim:
```json
[
  {
    "txid": "a1b2c3d4e5f6789...",
    "vout": 0,
    "amount": 0.00100000,
    "asset": "144c654344aa716d6f3abcc1ca90e5641e4e2a7f633bc09fe3baf64585819a49",
    "scriptPubKey": "5120abcd...",
    ...
  }
]
```

**COPIE ESSAS INFORMAÇÕES!** Você vai precisar para resolver o puzzle.

### Passo 7: (Opcional) Adicionar Mais Fundos ao Jackpot

```bash
cd /Users/felipe/Desktop/hub/blockchain/SimplicityHL/hackathon_puzzle

# Adicionar mais 0.002 L-BTC ao prêmio
cargo run --release --bin add-to-pot -- puzzle_2cf24dba.json 0.002
```

### Passo 8: Resolver o Puzzle

Agora vem a parte crítica. Você precisa editar o código com as informações do UTXO.

**8.1 - Editar `src/bin/solve_puzzle.rs`:**

Abra o arquivo:
```bash
code src/bin/solve_puzzle.rs
# OU
nano src/bin/solve_puzzle.rs
```

Vá até aproximadamente a **linha 115-125** e encontre este trecho:
```rust
// ⚠️ HARD-CODED UTXO - EDIT THIS!
let txid_str = "REPLACE_WITH_YOUR_TXID";
let vout = 0u32;
let value_sats = 100_000u64; // 0.001 BTC = 100,000 sats
```

**Substitua com os valores que você copiou do `listunspent`:**
```rust
let txid_str = "a1b2c3d4e5f6789..."; // Cole o TXID aqui
let vout = 0u32;                      // O vout (geralmente 0)
let value_sats = 100_000u64;         // Valor em satoshis (0.001 = 100,000)
```

**TAMBÉM ajuste o asset ID** (um pouco abaixo):
```rust
// L-BTC testnet asset ID
let asset_bytes = hex::decode("144c654344aa716d6f3abcc1ca90e5641e4e2a7f633bc09fe3baf64585819a49").unwrap();
// OU para regtest:
// let asset_bytes = hex::decode("5ac9f65c0efcc4775e0baec4ec03abdde22473cd3cf33c0419ca290e0751b225").unwrap();
```

Salve o arquivo!

**8.2 - Criar endereço para receber o prêmio:**
```bash
cd $HOME/Desktop/hub/blockchain/elements
./src/elements-cli -chain=liquidtestnet getnewaddress
```

Copie esse endereço!

**8.3 - RESOLVER O PUZZLE! 🎉**
```bash
cd /Users/felipe/Desktop/hub/blockchain/SimplicityHL/hackathon_puzzle

# Rodar o solver
cargo run --release --bin solve-puzzle -- puzzle_2cf24dba.json "hello" <SEU_ENDERECO_NOVO>
```

Se tudo der certo:
```
🎯 RESOLVENDO PUZZLE
===================

📖 Lendo puzzle de: puzzle_2cf24dba.json
🔍 Verificando secret...
✅ Secret correto!

⚙️  Compilando contrato...
✅ Contrato compilado!

🔨 Criando transação...
✅ Transação criada!

🚀 Transmitindo transação...

🎉🎉🎉 SUCESSO! 🎉🎉🎉

✅ Transação transmitida!
   TXID: f1e2d3c4b5a6...

💰 Prêmio enviado para: <SEU_ENDERECO>
   Valor: ~99,700 sats (descontando fees)

🏆 VOCÊ GANHOU O PUZZLE!
```

**8.4 - Verificar no seu saldo:**
```bash
cd $HOME/Desktop/hub/blockchain/elements

# Se testnet, aguardar confirmação
./src/elements-cli -chain=liquidtestnet getbalance

# Se regtest, gerar um bloco
./src/elements-cli -chain=liquidregtest generatetoaddress 1 $(./src/elements-cli -chain=liquidregtest getnewaddress)

# Verificar saldo novamente
./src/elements-cli -chain=liquidregtest getbalance
```

---

## 🎮 Cenários de Teste

### Teste 1: Senha Correta
```bash
cargo run --release --bin create-puzzle -- "bitcoin" 0.005
cargo run --release --bin solve-puzzle -- puzzle_*.json "bitcoin" <endereco>
```
**Esperado:** ✅ Transação aceita, fundos transferidos

### Teste 2: Senha Errada
```bash
cargo run --release --bin solve-puzzle -- puzzle_*.json "satoshi" <endereco>
```
**Esperado:** ❌ Erro antes de transmitir (hash não bate)

### Teste 3: Aumentar Jackpot
```bash
cargo run --release --bin create-puzzle -- "hodl" 0.001
cargo run --release --bin add-to-pot -- puzzle_*.json 0.002
cargo run --release --bin add-to-pot -- puzzle_*.json 0.003
# Agora tem 0.006 L-BTC no total!
```

### Teste 4: Múltiplos Puzzles Simultâneos
```bash
cargo run --release --bin create-puzzle -- "easy" 0.001
cargo run --release --bin create-puzzle -- "medium" 0.005
cargo run --release --bin create-puzzle -- "hard" 0.010

# Tentar resolver cada um
cargo run --release --bin solve-puzzle -- puzzle_*.json "easy" <endereco1>
cargo run --release --bin solve-puzzle -- puzzle_*.json "medium" <endereco2>
# etc...
```

---

## 🐛 Troubleshooting

### ❌ "Failed to connect to daemon"
**Causa:** elementsd não está rodando ou path errado

**Solução:**
```bash
# Verificar se está rodando
ps aux | grep elementsd

# Iniciar se necessário
cd $HOME/Desktop/hub/blockchain/elements
./src/elementsd -chain=liquidtestnet -daemon

# Verificar conexão
./src/elements-cli -chain=liquidtestnet getblockchaininfo
```

### ❌ "Insufficient funds"
**Causa:** Não tem L-BTC suficiente

**Solução:**
- **Testnet:** Use o faucet https://liquidtestnet.com/faucet
- **Regtest:** `./src/elements-cli -chain=liquidregtest generatetoaddress 101 $(./src/elements-cli -chain=liquidregtest getnewaddress)`

### ❌ "Parameter TARGET_HASH is missing"
**Causa:** Tentou compilar o contrato diretamente

**Solução:** Sempre use os binários `create-puzzle` ou `solve-puzzle`

### ❌ "Secret incorreto"
**Causa:** O hash SHA256 não bate

**Solução:**
- Verifique se o secret é exatamente igual (case-sensitive!)
- Confira o hash no arquivo JSON
- Teste localmente:
```bash
echo -n "hello" | sha256sum
# Deve dar: 2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824
```

### ❌ "UTXO not found"
**Causa:** Informações do UTXO erradas ou transação não confirmada

**Solução:**
1. Verificar com `listunspent` se o UTXO existe
2. Aguardar confirmação (testnet) ou gerar bloco (regtest)
3. Verificar se editou o código com TXID/vout corretos

### ❌ Erro ao compilar: "could not find simplicityhl"
**Causa:** O projeto depende do SimplicityHL do diretório pai

**Solução:**
```bash
# Garantir que está no workspace correto
cd /Users/felipe/Desktop/hub/blockchain/SimplicityHL

# Compilar o SimplicityHL primeiro
cargo build

# Depois compilar o puzzle
cd hackathon_puzzle
cargo build
```

### ❌ "Invalid address"
**Causa:** Endereço de destino não é válido para a network

**Solução:**
- Testnet: endereços começam com "tex1" (Taproot) ou "ert1" (SegWit)
- Regtest: endereços começam com "lq1" ou similar
- Use `getnewaddress` da mesma network

---

## 📊 Estrutura dos Arquivos Gerados

Cada puzzle cria um arquivo JSON:

**puzzle_2cf24dba.json:**
```json
{
  "secret": "hello",
  "hash": "0x2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824",
  "address": "tex1qjr5yzs...",
  "amount": "0.001",
  "txid": "a1b2c3d4e5f6..."
}
```

⚠️ **NUNCA compartilhe esse arquivo!** Ele contém o secret.

Para compartilhar o puzzle:
```
📢 PUZZLE ATIVO!
Endereço: tex1qjr5yzs...
Prêmio: 0.001 L-BTC
Hash: 0x2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824

Hints:
- A senha tem 5 letras
- É uma saudação comum em inglês
- Começa com "h"
```

---

## 🎯 Próximos Passos

Agora que testou localmente, você pode:

1. **Criar puzzles com diferentes dificuldades**
   - Fácil: "hello" (5 letras)
   - Médio: "satoshinakamoto" (15 letras)
   - Difícil: Hash de algo complexo

2. **Testar na testnet pública**
   - Compartilhar puzzles com amigos
   - Ver quem resolve primeiro

3. **Adicionar features:**
   - [ ] Aceitar TXID/vout como argumentos CLI
   - [ ] Suporte a múltiplos secrets (AND/OR)
   - [ ] Time-lock (puzzle expira)
   - [ ] Web interface

4. **Apresentar no hackathon! 🚀**

---

## 📚 Recursos Úteis

- **Simplicity Docs:** https://github.com/BlockstreamResearch/simplicity
- **Elements/Liquid:** https://elementsproject.org/
- **Taproot:** https://bitcoinops.org/en/topics/taproot/
- **Liquid Testnet Explorer:** https://liquid.network/

---

## ✨ Resumo do Fluxo

```
1. create-puzzle "senha" 0.1
   └─> Calcula hash
   └─> Compila contrato
   └─> Financia endereço
   └─> Salva puzzle_*.json

2. (Opcional) add-to-pot puzzle_*.json 0.05
   └─> Aumenta jackpot

3. listunspent <endereco>
   └─> Pega TXID/vout

4. Edita solve_puzzle.rs com TXID/vout

5. solve-puzzle puzzle_*.json "senha" <destino>
   └─> Verifica hash
   └─> Cria transação
   └─> Fornece senha como witness
   └─> Transmite
   └─> 🎉 GANHA!
```

---

**Boa sorte e bons testes! 🚀**

Se tiver dúvidas ou problemas, abra uma issue no GitHub!
