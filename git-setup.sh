#!/bin/bash
# Script para preparar e fazer push do repositório

set -e

echo "🚀 CONFIGURANDO REPOSITÓRIO GIT"
echo "================================"
echo ""

# Verificar se já existe .git
if [ -d ".git" ]; then
    echo "⚠️  Diretório .git já existe!"
    echo "   Removendo para começar limpo..."
    rm -rf .git
fi

echo "🔧 Inicializando git..."
git init
git branch -M main

echo ""
echo "➕ Adicionando arquivos..."
git add .

echo ""
echo "📋 Arquivos que serão commitados:"
echo "================================="
git status --short
echo ""

# Verificar se puzzle_*.json está sendo ignorado
if git status --short | grep -q "puzzle_"; then
    echo "⚠️  ATENÇÃO: Arquivos puzzle_*.json estão sendo commitados!"
    echo "   Isso NÃO deveria acontecer (contêm secrets)"
    echo ""
    read -p "Deseja continuar mesmo assim? (s/N): " confirm
    if [[ ! $confirm =~ ^[Ss]$ ]]; then
        echo "Abortado."
        exit 1
    fi
else
    echo "✅ Arquivos puzzle_*.json estão sendo ignorados (correto!)"
fi

echo ""
echo "💾 Criando commit inicial..."
git commit -m "🎯 Initial commit: Bitcoin Puzzle Hunt

Hackathon project implementing on-chain treasure hunt using Simplicity smart contracts on Liquid Network.

## Features
- ✅ Create hash-locked puzzles with Simplicity contracts
- ✅ Verify puzzles on blockchain
- ✅ CLI tools and helper scripts
- ✅ Taproot + Simplicity integration
- ⏳ Solve puzzles and claim prizes (WIP)

## Tech Stack
- Rust
- Simplicity (formally verifiable smart contract language)
- Elements/Liquid Network
- Taproot
- SHA256 hash locks

## How It Works
1. Create puzzle with secret password
2. Compile Simplicity contract with SHA256(secret) as parameter
3. Lock funds in Taproot address containing the contract
4. First person to provide correct secret claims all funds!

## Usage
\`\`\`bash
# Create puzzle
cargo run --release --bin create-puzzle -- \"secret\" 0.001

# Verify on blockchain
./check-puzzle.sh puzzle_*.json

# Check balance
./elements-cli.sh getbalance
\`\`\`

See COMO_USAR.md for full documentation."

echo ""
echo "✅ Git configurado!"
echo ""
echo "📊 Estatísticas:"
git log --oneline
echo ""
git diff --stat HEAD~1 HEAD 2>/dev/null || git show --stat

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🌐 PRÓXIMO PASSO: Criar repositório no GitHub"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "1️⃣  Acesse: https://github.com/new"
echo ""
echo "2️⃣  Preencha:"
echo "    Nome: bitcoin-puzzle-hunt"
echo "    Descrição: Bitcoin Puzzle Hunt - On-chain treasure hunt using Simplicity"
echo "    Visibilidade: ✅ Public"
echo "    NÃO marque: Add README, .gitignore, ou license"
echo ""
echo "3️⃣  Clique em 'Create repository'"
echo ""
echo "4️⃣  Copie o comando que o GitHub mostrar, algo como:"
echo ""
echo "    git remote add origin https://github.com/SEU_USERNAME/bitcoin-puzzle-hunt.git"
echo "    git push -u origin main"
echo ""
echo "Ou execute agora (substitua SEU_USERNAME):"
echo ""
echo "    git remote add origin https://github.com/SEU_USERNAME/bitcoin-puzzle-hunt.git"
echo "    git push -u origin main"
echo ""
