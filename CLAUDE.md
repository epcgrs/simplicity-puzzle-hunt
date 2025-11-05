# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**Bitcoin Puzzle Hunt** is a hackathon project that implements an on-chain treasure hunt game using Simplicity smart contracts on the Liquid Network. Players compete to find a secret password that unlocks Bitcoin prizes, with all rules enforced cryptographically on-chain.

**Key concepts:**
- Hash-locked puzzles where funds are secured with SHA256 of a secret
- First person to discover the secret preimage wins ALL the locked funds
- Trustless - no intermediaries, math guarantees the rules
- Built on Liquid testnet using Simplicity + Taproot
- Uses the SimplicityHL compiler from parent directory (`../`)

## Common Commands

### Building
```bash
# Build all binaries (debug)
cargo build

# Build in release mode
cargo build --release
```

### Running the Binaries

There are three main binaries:

**1. Create a puzzle:**
```bash
# Create puzzle with secret "satoshi" and 0.1 L-BTC prize
cargo run --bin create-puzzle -- "satoshi" 0.1

# This will:
# - Calculate SHA256 of the secret
# - Compile Simplicity contract with the hash
# - Fund the puzzle address with L-BTC
# - Save details to puzzle_<hash_prefix>.json
```

**2. Add funds to increase jackpot:**
```bash
# Add 0.05 L-BTC to existing puzzle
cargo run --bin add-to-pot -- puzzle_2cf24dba.json 0.05
```

**3. Solve a puzzle:**
```bash
# Solve puzzle by providing correct secret
cargo run --bin solve-puzzle -- puzzle_2cf24dba.json "satoshi" <destination_address>

# IMPORTANT: Before running, you must edit src/bin/solve_puzzle.rs to include:
# - TXID and vout of the puzzle UTXO
# - Value in satoshis
# - Asset ID
```

### Finding UTXO Information

To solve a puzzle, you need the UTXO details:

```bash
# From the Elements directory
cd $HOME/Desktop/hub/blockchain/elements

# List UTXOs for puzzle address
./src/elements-cli -chain=liquidtestnet listunspent 0 9999999 '["<PUZZLE_ADDRESS>"]'

# Get a new address to receive winnings
./src/elements-cli -chain=liquidtestnet getnewaddress
```

### Starting Elements Node

The project requires a running Elements daemon:

```bash
# Check if running
ps aux | grep elementsd

# Start if not running
cd $HOME/Desktop/hub/blockchain/elements
./src/elementsd -chain=liquidtestnet -daemon

# Verify connection
./src/elements-cli -chain=liquidtestnet getblockchaininfo
```

## Architecture

### How It Works

1. **Contract Creation** (`examples/puzzle_jackpot.simf`):
   - Organizer chooses a secret (e.g., "satoshi")
   - Computes SHA256 hash of secret
   - Compiles Simplicity contract with hash as `TARGET_HASH` parameter
   - Contract accepts `SECRET` as witness data

2. **Funding**:
   - Contract is embedded in Taproot script tree
   - Funds are sent to Taproot address containing contract
   - Multiple transactions can fund same address (jackpot grows)

3. **Solving**:
   - Solver provides secret as witness data
   - Simplicity contract verifies `sha256(SECRET) == TARGET_HASH` on-chain
   - If true: transaction valid, solver claims funds
   - If false: transaction rejected by network

### Project Structure

- **src/bin/create_puzzle.rs** - Creates and funds new puzzles
- **src/bin/solve_puzzle.rs** - Attempts to solve puzzles and claim prizes
- **src/bin/add_to_pot.rs** - Adds more funds to existing puzzles
- **examples/puzzle_jackpot.simf** - The Simplicity smart contract (in parent repo)
- **puzzle_*.json** - Generated files storing puzzle metadata (address, hash, amount)

### Dependencies

- **simplicityhl** - The SimplicityHL compiler from parent directory (`path = ".."`)
- **simplicity-lang** - Core Simplicity implementation
- **elements** - Elements/Liquid blockchain library
- **elementsd** - RPC client for Elements daemon
- **secp256k1-zkp** - Cryptographic primitives

### Taproot Structure

```
Taproot Output
    │
    ├── Internal Key (placeholder - unspendable)
    └── Script Tree
            └── Leaf: Simplicity Program (CMR of compiled contract)
```

## Development Notes

### Hard-coded Paths

The binaries contain hard-coded paths to the Elements daemon:
- Located in `src/bin/*.rs` files
- Default: `/Users/felipe/Desktop/hub/blockchain/elements/src/elementsd`
- Update these paths if Elements is installed elsewhere

### Manual UTXO Configuration

**CRITICAL**: The `solve_puzzle.rs` binary requires manual editing before use:
- Lines ~120: Set `txid_str`, `vout`, and `value_sats` from `listunspent` output
- This is by design for the hackathon demo
- Future versions could accept these as CLI arguments

### Puzzle File Format

Generated `puzzle_*.json` files contain:
```json
{
  "secret": "satoshi",         // DO NOT share!
  "hash": "0xa0dc65ff...",     // Share with participants
  "address": "tex1q...",       // Share with participants
  "amount": "0.1",             // Share with participants
  "txid": "abc123..."          // Funding transaction
}
```

### Network

- **Default**: Liquid testnet (`-chain=liquidtestnet`)
- **Asset**: L-BTC (Liquid Bitcoin)
- **Faucet**: Use Liquid testnet faucet if you need test coins

### The Simplicity Contract

The contract in `examples/puzzle_jackpot.simf` is minimal:
1. Takes `TARGET_HASH` as compile-time parameter
2. Takes `SECRET` as runtime witness
3. Computes `sha256(SECRET)`
4. Asserts equality with `TARGET_HASH`
5. If assertion passes, spending is authorized

## Common Patterns

When working with this codebase:
- All binaries share the same Simplicity contract source via `include_str!`
- Hash computation uses `sha2` crate, must match Simplicity's SHA256 jet
- Elements RPC requires exact paths to daemon binary and RPC cookies
- Addresses are Taproot (begin with "tex1" on testnet)
- Amounts are in BTC units (e.g., 0.1) but converted to satoshis internally

## Use Cases Beyond Games

- **Digital Inheritance**: Family members combine secret fragments
- **Educational CTFs**: Teach cryptography with real rewards
- **Marketing Campaigns**: Viral puzzles for brand engagement
- **Proof of Knowledge**: Prove you know something without revealing it
- **Dead Man's Switch**: Time-locked secret release

## Troubleshooting

### "Parameter TARGET_HASH is missing"
You're compiling the contract directly instead of using the binaries. Always use `create-puzzle` or `solve-puzzle`.

### "Failed to connect to daemon"
Elements daemon isn't running. Start with: `./src/elementsd -chain=liquidtestnet -daemon`

### "Insufficient funds"
Wallet doesn't have L-BTC. Use Liquid testnet faucet or mine regtest blocks.

### "Secret incorrect"
The SHA256 of your secret doesn't match the puzzle hash. Secrets are case-sensitive.

### "UTXO not found"
- Check address with `listunspent`
- Ensure funding transaction is confirmed
- Verify you're on the correct network (testnet/regtest)
