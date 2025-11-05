# 🎯 Bitcoin Puzzle Hunt - Hackathon Project

**Caça ao tesouro on-chain usando Simplicity na Liquid Network!**

## 🎬 Demonstração Rápida

Este projeto implementa um jogo de "caça ao tesouro" onde:
1. 💰 Você bloqueia fundos com uma senha secreta (hash SHA256)
2. 📢 Publica hints sobre a senha
3. 🏆 Primeira pessoa que descobrir a senha ganha TODO o prêmio!

## ⚡ Quick Start

### 1. Instalar Dependências

```bash
cd hackathon_puzzle

# Compilar o projeto
cargo build --release
```

### 2. Garantir que seu elementsd está rodando

```bash
# Verificar se está rodando
ps aux | grep elementsd

# Se não estiver, iniciar:
cd $HOME/Desktop/hub/blockchain/elements
./src/elementsd -chain=liquidtestnet -daemon
```

### 3. Criar um Puzzle

```bash
# Criar puzzle com secret "satoshi" e prêmio de 0.1 L-BTC
cargo run --bin create-puzzle -- "satoshi" 0.1
```

**Output esperado:**
```
🎯 CRIANDO PUZZLE HUNT
====================

📝 Secret: satoshi
🔐 Hash (SHA256): 0xa0dc65ffca799873cbea0ac274015b9526505daaaed385155425f7337704883e

⚙️  Compilando contrato Simplicity...
✅ Contrato compilado!

📍 Endereço do Puzzle:
   tex1qjr5yzs...

💰 Financiando puzzle com 0.1 L-BTC...
✅ Puzzle financiado!
   TXID: a1b2c3d4...

💾 Informações salvas em: puzzle_a0dc65ff.json

🎉 PUZZLE CRIADO COM SUCESSO!

📢 Compartilhe com os participantes:
   Endereço: tex1qjr5yzs...
   Prêmio: 0.1 L-BTC
   Hash do Secret: 0xa0dc65ff...

🔍 Hint: A senha tem 7 caracteres

⚠️  GUARDAR O SECRET EM SEGREDO!
   Secret: satoshi (não compartilhe isso!)
```

### 4. Adicionar Mais Fundos ao Jackpot (Opcional)

```bash
# Aumentar o prêmio para deixar mais atrativo
cargo run --bin add-to-pot -- puzzle_a0dc65ff.json 0.05
```

### 5. Resolver o Puzzle (Como Participante)

**ATENÇÃO:** O script `solve-puzzle` precisa ser editado manualmente para incluir o TXID e VOUT do UTXO.

Passos:
1. **Encontrar o UTXO do puzzle:**

```bash
cd $HOME/Desktop/hub/blockchain/elements
./src/elements-cli -chain=liquidtestnet listunspent 0 9999999 '["tex1qjr5yzs..."]'
```

Você verá algo como:
```json
[
  {
    "txid": "a1b2c3d4e5f6...",
    "vout": 0,
    "amount": 0.10000000,
    "asset": "144c654344aa716d6f3abcc1ca90e5641e4e2a7f633bc09fe3baf64585819a49",
    ...
  }
]
```

2. **Editar `src/bin/solve_puzzle.rs` e substituir:**

```rust
// Linha ~120
let txid_str = "a1b2c3d4e5f6..."; // Seu TXID real
let vout = 0u32; // Seu vout real
let value_sats = 10_000_000u64; // Valor em satoshis (0.1 BTC = 10M sats)
```

3. **Rodar o solver:**

```bash
cargo run --bin solve-puzzle -- puzzle_a0dc65ff.json "satoshi" <SEU_ENDERECO_DESTINO>
```

Se o secret estiver correto:
```
🎉🎉🎉 SUCESSO! 🎉🎉🎉

✅ Transação transmitida!
   TXID: f1e2d3c4...

💰 Prêmio enviado para: <SEU_ENDERECO>
   Valor: 9997000 sats

🏆 VOCÊ GANHOU O PUZZLE!
```

## 🎓 Como Funciona Tecnicamente

### Contrato Simplicity (`puzzle_jackpot.simf`)

```rust
fn main() {
    let secret: u256 = witness::SECRET;
    let target_hash: u256 = param::TARGET_HASH;
    let computed_hash: u256 = sha2(secret);

    // Verifica se o hash do secret fornecido bate com o hash esperado
    assert!(jet::eq_256(computed_hash, target_hash));
}
```

**Como funciona:**
1. O contrato é compilado com um `TARGET_HASH` (parâmetro fixo)
2. Fundos são enviados para um endereço Taproot que inclui este contrato
3. Para gastar, você precisa fornecer um `SECRET` cujo SHA256 seja igual ao `TARGET_HASH`
4. O Simplicity verifica on-chain se `sha256(SECRET) == TARGET_HASH`
5. Se verdadeiro → transação válida, você ganha os fundos!
6. Se falso → transação rejeitada pela network

### Taproot + Simplicity

```
Taproot Output
    │
    ├── Internal Key (placeholder)
    └── Script Tree
            └── Leaf: Simplicity Program (CMR do contrato compilado)
```

## 🎪 Ideias para Apresentação no Hackathon

### 1. Demo Ao Vivo
- Criar 3 puzzles com dificuldades diferentes:
  - **Fácil**: "hello" (5 letras) - 0.01 BTC
  - **Médio**: "satoshi" (7 letras) - 0.05 BTC
  - **Difícil**: Hash de algo complexo - 0.1 BTC

### 2. Website Simples
Criar uma landing page com:
- Lista de puzzles ativos
- Hints progressivos
- Contador de tentativas
- Leaderboard

### 3. Gamificação
- **Hints progressivos**: A cada 10 minutos, libera uma dica
- **Multiple puzzles**: Vários puzzles simultâneos
- **Team competition**: Times competindo

### 4. Casos de Uso Reais
- **Herança digital**: Família precisa juntar partes do secret
- **Dead man's switch**: Puzzle se auto-resolve após X tempo
- **Proof of work alternativo**: Quebrar hash em vez de minerar
- **Educational games**: Ensinar criptografia

## 📊 Pitch para Jurados

**"Bitcoin Puzzle Hunt - Gamificando Contratos Inteligentes"**

**Problema:**
- Contratos inteligentes são complexos e intimidadores
- Difícil demonstrar o valor de smart contracts para o público geral
- Falta engajamento com blockchain além de especulação

**Solução:**
- Jogo on-chain onde qualquer um pode participar
- Demonstra propriedades únicas de blockchain:
  - ✅ Trustless (sem intermediários)
  - ✅ Transparente (qualquer um vê as regras)
  - ✅ Imutável (regras não mudam)
  - ✅ Permissionless (qualquer um pode tentar)

**Tech Stack:**
- ⚡ **Simplicity**: Linguagem de contratos verificável formalmente
- 🌊 **Liquid Network**: Sidechain do Bitcoin
- 🔐 **Taproot**: Privacy e eficiência
- 🦀 **Rust**: Performance e segurança

**Diferencial:**
- Primeiro jogo educacional usando Simplicity
- On-chain verification (não depende de oráculos)
- Código aberto e educacional

## 🚀 Próximos Passos (Após Hackathon)

1. **Web Interface** - Frontend para criar/resolver puzzles
2. **Time locks** - Puzzles que expiram
3. **Multi-step puzzles** - Resolver vários desafios em sequência
4. **NFT rewards** - Ganhar NFTs por resolver puzzles
5. **ZK Proofs** - Resolver puzzle sem revelar o secret publicamente

## 🐛 Troubleshooting

### Erro: "Parameter TARGET_HASH is missing"
- Você está compilando o contrato sem fornecer argumentos
- Use os scripts `create-puzzle` ou `solve-puzzle`

### Erro: "Falha ao conectar com elementsd"
- Verificar se elementsd está rodando: `ps aux | grep elementsd`
- Verificar path correto em cada script

### Erro: "Secret incorreto"
- Verifique se está usando o secret exato (case-sensitive)
- Confira o hash SHA256

### Erro: "UTXO não encontrado"
- Use `elements-cli -chain=liquidtestnet listunspent` para verificar
- Aguarde confirmação da transação de funding

## 📚 Recursos Adicionais

- [Simplicity Docs](https://github.com/BlockstreamResearch/simplicity)
- [Elements/Liquid Docs](https://elementsproject.org/)
- [Taproot Explained](https://bitcoinops.org/en/topics/taproot/)

## 🏆 Licença

MIT - Use à vontade, só não se esqueça de dar os créditos!

---

**Criado para Hackathon 2025 - Boa sorte! 🚀**
