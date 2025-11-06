# 🎯 Bitcoin Puzzle Hunt - Hackathon Project

**On-chain treasure hunt using Simplicity on Liquid Network!**

## 🎬 Quick Overview

This project implements a "treasure hunt" game where:
1. 💰 You lock funds with a secret password (SHA256 hash)
2. 📢 Publish hints about the password
3. 🏆 First person to discover the password wins ALL the prize!

## ⚡ Quick Start

### 1. Install Dependencies

```bash
cd hackathon_puzzle

# Build the project
cargo build --release
```

### 2. Ensure elementsd is running

```bash
# Check if running
ps aux | grep elementsd

# If not, start it:
cd $HOME/Desktop/hub/blockchain/elements
./src/elementsd -chain=liquidtestnet -daemon
```

### 3. Create a Puzzle

```bash
# Create puzzle with secret "satoshi" and prize of 0.1 L-BTC
cargo run --bin create-puzzle -- "satoshi" 0.1
```

**Expected output:**
```
🎯 CREATING PUZZLE HUNT
========================

📝 Secret: satoshi
🔐 Hash (SHA256): 0xa0dc65ff...

⚙️  Compiling Simplicity contract...
✅ Contract compiled!

📍 Puzzle Address:
   tex1qjr5yzs...

💰 Funding puzzle with 0.1 L-BTC...
✅ Puzzle funded!
   TXID: a1b2c3d4...

💾 Information saved to: puzzle_a0dc65ff.json

🎉 PUZZLE CREATED SUCCESSFULLY!

📢 Share with participants:
   Address: tex1qjr5yzs...
   Prize: 0.1 L-BTC
   Secret Hash: 0xa0dc65ff...

🔍 Hint: The password has 7 characters

⚠️  KEEP THE SECRET SAFE!
   Secret: satoshi (don't share this!)
```

### 4. Add More Funds to Jackpot (Optional)

```bash
# Increase the prize to make it more attractive
cargo run --bin add-to-pot -- puzzle_a0dc65ff.json 0.05
```

**Output:**
```
💰 INCREASING JACKPOT
=====================

📍 Puzzle address: tex1qjr5yzs...
💵 Current prize: 0.1 L-BTC
➕ Adding: 0.05 L-BTC

📤 Sending funds...
✅ Funds added!
   TXID: xyz123...

💾 File updated: puzzle_a0dc65ff.json
🎉 New estimated prize: 0.15000000 L-BTC

📢 Share with participants that the jackpot has increased!
```

### 5. Solve the Puzzle

When you know the secret, you can claim the prize:

```bash
# First, find the UTXO
./elements-cli.sh listunspent 0 9999999 '["tex1qjr5yzs..."]'

# Then edit src/bin/solve_puzzle.rs with UTXO info (txid, vout, amount, asset)

# Get destination address
./elements-cli.sh getnewaddress

# Solve the puzzle
cargo run --bin solve-puzzle -- puzzle_a0dc65ff.json "satoshi" <your_address>
```

**If correct:**
```
🎯 SOLVING PUZZLE
=================

📖 Reading puzzle from: puzzle_a0dc65ff.json
   Puzzle address: tex1qjr5yzs...
   Expected hash: 0xa0dc65ff...

🔍 Verifying secret...
✅ Secret is correct!

⚙️  Compiling contract...
✅ Contract compiled!

💸 Creating spending transaction...
🔐 Creating witness with secret...
📡 Broadcasting transaction...

🎉🎉🎉 SUCCESS! 🎉🎉🎉

✅ Transaction broadcasted!
   TXID: def456...

💰 Prize sent to: <your_address>
   Amount: 14997000 sats

🏆 YOU WON THE PUZZLE!
```

## 📚 How It Works

### The Smart Contract

The puzzle uses a Simplicity contract (`examples/puzzle_jackpot.simf`) that:

1. Takes a `TARGET_HASH` as a compile-time parameter
2. Takes a `SECRET` as runtime witness data
3. Computes `sha256(SECRET)`
4. Verifies that the computed hash matches `TARGET_HASH`
5. If correct, the transaction is valid and the prize is claimed!

```simplicity
fn main() {
    let secret: u256 = witness::SECRET;
    let target_hash: u256 = param::TARGET_HASH;
    let computed_hash: u256 = sha2(secret);

    // If this passes, you win!
    assert!(jet::eq_256(computed_hash, target_hash));
}
```

### Security Model

- **Trustless**: No intermediaries - the blockchain validates everything
- **Transparent**: Contract code is open source
- **Atomic**: Either you have the correct secret and win, or transaction fails
- **First-come-first-served**: First valid transaction to be mined wins

### Taproot Structure

The puzzle uses Taproot script paths:

```
Taproot Output
    │
    ├── Internal Key (placeholder - unspendable)
    └── Script Tree
            └── Leaf: Simplicity Program (CMR of contract)
```

## 🏗️ Project Structure

```
hackathon_puzzle/
├── src/bin/
│   ├── create_puzzle.rs    # Create and fund puzzles
│   ├── solve_puzzle.rs     # Solve puzzles and claim prizes
│   ├── add_to_pot.rs       # Add more funds to jackpot
│   └── export_program.rs   # Export compiled contract for analysis
├── puzzle_*.json           # Generated puzzle files
├── elements-cli.sh         # Wrapper script for Elements CLI
├── check-puzzle.sh         # Verify puzzle and check UTXO status
├── Cargo.toml              # Project configuration
├── README.md               # This file
└── CLAUDE.md               # Development guide
```

## 🔧 Requirements

- **Rust**: 1.78.0 or higher
- **Elements daemon**: Running on Liquid testnet
- **Wallet**: With L-BTC for funding puzzles
- **hal-simplicity**: For contract analysis (optional)

## 💡 Use Cases

Beyond games, this technology enables:

- **Digital Inheritance**: Family members combine secret fragments
- **Educational CTFs**: Teach cryptography with real rewards
- **Marketing Campaigns**: Viral puzzles for brand engagement
- **Proof of Knowledge**: Prove you know something without revealing it
- **Dead Man's Switch**: Time-locked secret release

## 🔒 Security Considerations

### For Organizers

- Use strong, random secrets (not dictionary words)
- Never reuse secrets across puzzles
- Keep the secret file secure until the game ends
- Consider using high-entropy secrets (random hex strings)

### For Solvers

- First to broadcast a valid transaction wins
- Use high fees or RBF for priority
- The secret becomes public once you broadcast
- Race condition: multiple solvers may find the secret simultaneously

## 🛠️ Troubleshooting

### "Failed to compile contract"
- Check that the parent SimplicityHL directory exists
- Verify `../examples/puzzle_jackpot.simf` is accessible

### "Failed to connect to daemon"
- Ensure elementsd is running: `ps aux | grep elementsd`
- Start it with: `./src/elementsd -chain=liquidtestnet -daemon`

### "Insufficient funds"
- Check wallet balance: `./elements-cli.sh getbalance`
- Use Liquid testnet faucet for test coins

### "Transaction rejected"
- Wrong secret provided
- UTXO info incorrect in solve_puzzle.rs
- Insufficient fees
- Asset ID mismatch

## 📖 Additional Resources

- **Simplicity Language**: https://github.com/BlockstreamResearch/simplicity
- **SimplicityHL**: https://github.com/BlockstreamResearch/rust-simplicity
- **Elements**: https://elementsproject.org/
- **Liquid Network**: https://liquid.net/

## 🤝 Contributing

This is a hackathon project. Feel free to:
- Report issues
- Suggest improvements
- Fork and experiment
- Share your puzzles!

## 📄 License

MIT License - See LICENSE file for details

## 🎉 Credits

Built with:
- **Simplicity** - Blockchain programming language
- **SimplicityHL** - High-level Simplicity compiler
- **Elements** - Sidechain platform
- **Rust** - Systems programming language

---

**Have fun and happy puzzle hunting!** 🎯
