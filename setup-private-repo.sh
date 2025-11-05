#!/bin/bash
# Script para criar repositório privado do hackathon_puzzle

set -e  # Para em caso de erro

echo "🚀 CRIANDO REPOSITÓRIO PRIVADO"
echo "=============================="
echo ""

# Nome do repositório
REPO_NAME="bitcoin-puzzle-hunt"
REPO_DIR="$HOME/Desktop/bitcoin-puzzle-hunt"

echo "📂 Criando diretório: $REPO_DIR"
mkdir -p "$REPO_DIR"

echo "📋 Copiando arquivos..."
cp -r . "$REPO_DIR/"

cd "$REPO_DIR"

echo "🗑️  Removendo arquivos desnecessários..."
rm -rf .git
rm -f setup-private-repo.sh

echo "📝 Criando .gitignore..."
cat > .gitignore << 'EOF'
# Rust
/target/
**/*.rs.bk
*.pdb
Cargo.lock

# Puzzle files (contém secrets!)
puzzle_*.json

# OS
.DS_Store
.vscode/
.idea/

# Logs
*.log
EOF

echo "🔧 Inicializando git..."
git init
git branch -M main

echo "➕ Adicionando arquivos..."
git add .

echo "💾 Criando commit inicial..."
git commit -m "🎯 Initial commit: Bitcoin Puzzle Hunt

Hackathon project implementing on-chain treasure hunt using Simplicity smart contracts on Liquid Network.

Features:
- ✅ Create hash-locked puzzles
- ✅ Verify puzzles on blockchain
- ✅ CLI tools and helper scripts
- ⏳ Solve puzzles and claim prizes (WIP)

Tech stack: Rust, Simplicity, Elements/Liquid, Taproot"

echo ""
echo "✅ Repositório local criado!"
echo "📍 Localização: $REPO_DIR"
echo ""
echo "🌐 PRÓXIMOS PASSOS:"
echo "==================="
echo ""
echo "1. Vá para: https://github.com/new"
echo ""
echo "2. Configure:"
echo "   - Repository name: $REPO_NAME"
echo "   - Description: Bitcoin Puzzle Hunt - On-chain treasure hunt using Simplicity"
echo "   - ✅ Private (IMPORTANTE!)"
echo "   - ❌ NÃO adicione README, .gitignore, ou license"
echo ""
echo "3. Clique em 'Create repository'"
echo ""
echo "4. Volte aqui e execute:"
echo ""
echo "   cd $REPO_DIR"
echo "   git remote add origin https://github.com/SEU_USERNAME/$REPO_NAME.git"
echo "   git push -u origin main"
echo ""
echo "Ou se preferir SSH:"
echo ""
echo "   cd $REPO_DIR"
echo "   git remote add origin git@github.com:SEU_USERNAME/$REPO_NAME.git"
echo "   git push -u origin main"
echo ""
