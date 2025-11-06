#!/bin/bash
# Wrapper for elements-cli with configured defaults

ELEMENTS_CLI="$HOME/Desktop/hub/blockchain/elements/src/elements-cli"

# Executes elements-cli with pre-configured chain and wallet
$ELEMENTS_CLI -chain=liquidtestnet -rpcwallet=my_wallet "$@"
