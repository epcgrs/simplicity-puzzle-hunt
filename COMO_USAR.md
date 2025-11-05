# 🎯 Como Usar o Bitcoin Puzzle Hunt

## Quick Start - 3 Comandos

```bash
# 1. Criar puzzle
cargo run --release --bin create-puzzle -- "meusecreto" 0.0001

# 2. Verificar se está na blockchain
./check-puzzle.sh puzzle_*.json

# 3. Ver seus puzzles
ls puzzle_*.json
```

---

## 📖 Guia Completo

### 1️⃣ Criar um Puzzle

```bash
cargo run --release --bin create-puzzle -- "<SECRET>" <VALOR>
```

**Exemplos:**
```bash
# Puzzle fácil (secret curto)
cargo run --release --bin create-puzzle -- "hello" 0.0001

# Puzzle médio
cargo run --release --bin create-puzzle -- "satoshi" 0.0005

# Puzzle difícil (secret longo)
cargo run --release --bin create-puzzle -- "satoshinakamoto" 0.001
```

**O que acontece:**
1. ✅ Calcula SHA256 do secret
2. ✅ Compila contrato Simplicity com o hash
3. ✅ Cria endereço Taproot
4. ✅ Envia fundos da sua wallet `my_wallet`
5. ✅ Salva informações em `puzzle_<hash>.json`

**Output:**
```
🎯 CRIANDO PUZZLE HUNT
====================

📝 Secret: hello
🔐 Hash (SHA256): 0x2cf24dba...

⚙️  Compilando contrato Simplicity...
✅ Contrato compilado!

📍 Endereço do Puzzle:
   tex1p...xyz

💰 Financiando puzzle com 0.0001 L-BTC...
✅ Puzzle financiado!
   TXID: abc123...

💾 Informações salvas em: puzzle_2cf24dba.json
```

---

### 2️⃣ Verificar Puzzle na Blockchain

#### Método A: Script Automático (Recomendado)

```bash
./check-puzzle.sh puzzle_2cf24dba.json
```

**Output:**
```
🔍 VERIFICANDO PUZZLE
====================

📍 Endereço: tex1p...xyz
💰 Prêmio: 0.0001 L-BTC
🔐 Hash: 0x2cf24dba...
🤫 Secret: hello

🔎 Verificando transações...
✅ Puzzle encontrado na blockchain!
💵 Total no endereço: 0.0001 L-BTC
📦 UTXOs encontrados: 1
```

#### Método B: Manual via RPC

```bash
# 1. Ver informações do puzzle
cat puzzle_2cf24dba.json

# 2. Pegar o endereço
ADDRESS=$(cat puzzle_2cf24dba.json | grep address | cut -d'"' -f4)

# 3. Verificar UTXOs
./elements-cli.sh scantxoutset start "[\"addr($ADDRESS)\"]"
```

---

### 3️⃣ Listar Todos os Puzzles

```bash
# Ver todos os arquivos
ls -lh puzzle_*.json

# Ver conteúdo de todos
cat puzzle_*.json

# Verificar cada um
for file in puzzle_*.json; do
    echo "Verificando $file..."
    ./check-puzzle.sh "$file"
    echo ""
done
```

---

### 4️⃣ Ver Detalhes de uma Transação

```bash
# Pegar TXID do JSON
TXID=$(cat puzzle_2cf24dba.json | grep txid | cut -d'"' -f4)

# Ver detalhes completos
./elements-cli.sh gettransaction $TXID

# Ver só confirmações
./elements-cli.sh gettransaction $TXID | grep confirmations
```

---

## 🛠️ Scripts Úteis Disponíveis

### `elements-cli.sh`
Wrapper para executar `elements-cli` sem navegar de diretório

```bash
# Sintaxe
./elements-cli.sh <comando> <argumentos>

# Exemplos
./elements-cli.sh getbalance
./elements-cli.sh getnewaddress
./elements-cli.sh listunspent
./elements-cli.sh gettransaction <txid>
```

### `check-puzzle.sh`
Verifica status de um puzzle na blockchain

```bash
# Sintaxe
./check-puzzle.sh <puzzle_file.json>

# Exemplos
./check-puzzle.sh puzzle_2cf24dba.json
./check-puzzle.sh puzzle_da2876b3.json

# Ver todos
ls puzzle_*.json
```

---

## 📊 Comandos Úteis

### Ver Seu Saldo
```bash
./elements-cli.sh getbalance
```

### Gerar Novo Endereço (para receber prêmios)
```bash
./elements-cli.sh getnewaddress
```

### Ver Mempool (transações não confirmadas)
```bash
./elements-cli.sh getrawmempool
```

### Ver Info da Blockchain
```bash
./elements-cli.sh getblockchaininfo
```

---

## 🎮 Fluxo Completo de Teste

### Cenário: Criar e Verificar Puzzle

```bash
# 1. Criar puzzle
cargo run --release --bin create-puzzle -- "bitcoin" 0.0002

# 2. Aguardar alguns segundos (para confirmação)
sleep 5

# 3. Verificar
./check-puzzle.sh puzzle_6b88c087.json

# 4. Ver na blockchain
./elements-cli.sh scantxoutset start '["addr(tex1p...)"]'
```

### Cenário: Múltiplos Puzzles

```bash
# Criar 3 puzzles
cargo run --release --bin create-puzzle -- "easy" 0.0001
cargo run --release --bin create-puzzle -- "medium" 0.0003
cargo run --release --bin create-puzzle -- "hard" 0.0005

# Verificar todos
for file in puzzle_*.json; do
    ./check-puzzle.sh "$file"
    echo "---"
done

# Ver saldo restante
./elements-cli.sh getbalance
```

---

## ⚠️ Importante

### Guardando Secrets
- ⚠️ **NUNCA compartilhe os arquivos `.json`** - eles contém o secret!
- ✅ Compartilhe apenas: endereço, hash, valor, hints

### Para Compartilhar um Puzzle:
```bash
# Pegar informações do JSON
cat puzzle_2cf24dba.json

# Compartilhar apenas:
📍 Endereço: tex1p...xyz
💰 Prêmio: 0.0001 L-BTC
🔐 Hash: 0x2cf24dba...
💡 Hint: A senha tem 5 letras
```

### Backup
```bash
# Fazer backup dos JSONs (tem os secrets!)
mkdir -p ~/puzzles_backup
cp puzzle_*.json ~/puzzles_backup/
```

---

## 🐛 Troubleshooting

### "Insufficient funds"
```bash
# Verificar saldo
./elements-cli.sh getbalance

# Se não tem fundos, usar faucet:
# https://liquidtestnet.com/faucet
```

### "Puzzle não encontrado na blockchain"
```bash
# Verificar se transação está no mempool
./elements-cli.sh getrawmempool

# Ou verificar diretamente
./elements-cli.sh gettransaction <TXID>
```

### Script não executa
```bash
# Dar permissão de execução
chmod +x elements-cli.sh
chmod +x check-puzzle.sh
```

---

## 🚀 Próximos Passos

Atualmente funcionando:
- ✅ Criar puzzles
- ✅ Verificar na blockchain

Em desenvolvimento:
- ⏳ `add-to-pot` - Aumentar jackpot
- ⏳ `solve-puzzle` - Resolver e ganhar prêmio

---

## 📚 Arquivos no Projeto

```
hackathon_puzzle/
├── elements-cli.sh          # Wrapper para elements-cli
├── check-puzzle.sh          # Verifica puzzle na blockchain
├── puzzle_*.json            # Dados dos puzzles (GUARDAR!)
├── src/bin/
│   ├── create_puzzle.rs     # ✅ Funcionando
│   ├── add_to_pot.rs        # ⏳ Precisa ajustes
│   └── solve_puzzle.rs      # ⏳ Precisa ajustes
└── examples/
    └── puzzle_jackpot.simf  # Contrato Simplicity
```

---

**Boa sorte e bom jogo! 🎯🚀**
