#!/bin/bash
# Script para verificar status de um puzzle

if [ -z "$1" ]; then
    echo "Uso: $0 <puzzle_file.json>"
    echo ""
    echo "Exemplo:"
    echo "  $0 puzzle_6b88c087.json"
    echo ""
    echo "Puzzles disponíveis:"
    ls puzzle_*.json 2>/dev/null || echo "  Nenhum puzzle encontrado"
    exit 1
fi

PUZZLE_FILE="$1"

if [ ! -f "$PUZZLE_FILE" ]; then
    echo "❌ Arquivo não encontrado: $PUZZLE_FILE"
    exit 1
fi

echo "🔍 VERIFICANDO PUZZLE"
echo "===================="
echo ""

# Extrair informações
ADDRESS=$(grep -o '"address": "[^"]*"' "$PUZZLE_FILE" | cut -d'"' -f4)
AMOUNT=$(grep -o '"amount": "[^"]*"' "$PUZZLE_FILE" | cut -d'"' -f4)
HASH=$(grep -o '"hash": "[^"]*"' "$PUZZLE_FILE" | cut -d'"' -f4)
SECRET=$(grep -o '"secret": "[^"]*"' "$PUZZLE_FILE" | cut -d'"' -f4)

echo "📍 Endereço: $ADDRESS"
echo "💰 Prêmio: $AMOUNT L-BTC"
echo "🔐 Hash: $HASH"
echo "🤫 Secret: $SECRET"
echo ""

# Verificar transação de funding
echo "🔎 Verificando transações..."
ELEMENTS_CLI="$HOME/Desktop/hub/blockchain/elements/src/elements-cli"

# Buscar UTXOs no endereço
UTXOS=$($ELEMENTS_CLI -chain=liquidtestnet scantxoutset start "[\"addr($ADDRESS)\"]" 2>/dev/null)

if echo "$UTXOS" | grep -q "success.*true"; then
    echo "✅ Puzzle encontrado na blockchain!"

    # Extrair total_amount do JSON
    TOTAL=$(echo "$UTXOS" | grep -o '"total_amount":[^,}]*' | head -1 | cut -d':' -f2 | tr -d ' ')

    if [ -n "$TOTAL" ] && [ "$TOTAL" != "0" ]; then
        echo "💵 Total no endereço: $TOTAL L-BTC"
    else
        echo "💵 Total no endereço: $AMOUNT L-BTC (conforme JSON)"
    fi

    # Contar UTXOs
    UNSPENT_COUNT=$(echo "$UTXOS" | grep -o '"txid"' | wc -l | tr -d ' ')
    echo "📦 UTXOs encontrados: $UNSPENT_COUNT"
else
    echo "⏳ Puzzle pode estar no mempool (não confirmado ainda)"
    echo "   Aguarde alguns segundos e tente novamente"
fi

echo ""
echo "📊 Para ver detalhes completos:"
echo "   ./elements-cli.sh scantxoutset start '[\"addr($ADDRESS)\"]'"
