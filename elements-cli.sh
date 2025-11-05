#!/bin/bash
# Wrapper para elements-cli com defaults configurados

ELEMENTS_CLI="$HOME/Desktop/hub/blockchain/elements/src/elements-cli"

# Executa elements-cli com chain e wallet pré-configurados
$ELEMENTS_CLI -chain=liquidtestnet -rpcwallet=my_wallet "$@"
