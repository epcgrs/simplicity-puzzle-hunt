# 🎯 Simplicity Puzzle Hunt

**On-chain treasure hunt using Simplicity smart contracts on Liquid Network!**

> ⚠️ **CRITICAL BUG**: All puzzle types are currently unsolvable due to a hash computation mismatch between contracts and scripts. See [Known Issues](#-known-issues) section for details.

## 📋 Table of Contents
- [Overview](#-overview)
- [Quick Start](#-quick-start)
- [How It Works](#-how-it-works)
- [Advanced Puzzle Types](#-advanced-puzzle-types)
- [Project Structure](#-project-structure)
- [Security Considerations](#-security-considerations)
- [Troubleshooting](#-troubleshooting)
- [Contributing](#-contributing)

## 🎬 Overview

This project implements a cryptographic "treasure hunt" game on the Liquid Network where:

1. 💰 **Lock funds** with a secret password (SHA256 hash)
2. 📢 **Publish hints** about the password to create a challenge
3. 🏆 **Winner takes all** - First person to discover the password wins the entire prize!

### Key Features

- **Multiple puzzle types**: Simple, time-locked, chained, and consolidation puzzles
- **Fixed prize pools**: Prize amount is set at puzzle creation
- **Transparent rules**: All logic is in the Simplicity smart contract
- **Trustless execution**: No intermediaries - blockchain validates everything
- **Educational tool**: Perfect for teaching cryptography and blockchain concepts

## ⚡ Quick Start

### Prerequisites

- **Rust** 1.78.0 or higher
- **Elements daemon** running on Liquid testnet
- **Wallet** with L-BTC for funding puzzles

### 1. Clone and Build

```bash
git clone https://github.com/yourusername/simplicity-puzzle-hunt
cd simplicity-puzzle-hunt

# Build the project
cargo build --release
```

### 2. Start Elements Daemon

```bash
# Check if running
ps aux | grep elementsd

# If not running, start it:
cd $HOME/Desktop/hub/blockchain/elements
./src/elementsd -chain=liquidtestnet -daemon

# Create or load wallet
./src/elements-cli -chain=liquidtestnet createwallet "my_wallet"
```

### 3. Create Your First Puzzle

```bash
# Create puzzle with secret "satoshi" and 0.1 L-BTC prize
cargo run --bin create-puzzle -- "satoshi" 0.1 "Hint: Bitcoin's creator"
```

**Expected output:**
```
╔══════════════════════════════════════╗
║       🎯 CREATING PUZZLE HUNT 🎯     ║
╚══════════════════════════════════════╝

📋 Puzzle Configuration:
   📝 Secret: satoshi
   💰 Amount: 0.1 L-BTC
   💡 Hint: "Bitcoin's creator"

🔐 Processing secret and value...
✅ Target Hash computed: 0xa0dc65ff...
   Formula: SHA256(secret)

⚙️  Compiling Simplicity contract...
✅ Contract compiled!

📍 Puzzle Address:
   tex1qjr5yzs...

💰 Funding puzzle with 0.1 L-BTC...
✅ Puzzle funded!
   TXID: a1b2c3d4...

💾 Files saved:
   📄 Public file: puzzle_a0dc65ff.json
   🔒 Private file: puzzle_a0dc65ff_SECRET.json

╔══════════════════════════════════════╗
║    🎉 PUZZLE CREATED SUCCESSFULLY!    ║
╚══════════════════════════════════════╝
```

### 4. List Active Puzzles

```bash
# Interactive mode - asks to archive solved puzzles
./list-puzzles.sh

# Auto-archive mode (for automation)
./list-puzzles.sh --auto
```

### 5. Solve a Puzzle

```bash
# Using the puzzle file and secret
cargo run --bin solve-puzzle -- puzzle_a0dc65ff.json "satoshi" <your_liquid_address>
```

**Success output:**
```
╔══════════════════════════════════════╗
║       🎯 SOLVING PUZZLE HUNT 🎯       ║
╚══════════════════════════════════════╝

📖 Reading puzzle: puzzle_a0dc65ff.json
🎯 Target hash: 0xa0dc65ff...
💡 Hint: "Bitcoin's creator"

🔍 Verifying secret...
✅ Secret "satoshi" is CORRECT!

💸 Creating spending transaction...
📡 Broadcasting transaction...

╔══════════════════════════════════════╗
║       🎉 PUZZLE SOLVED! 🎉           ║
╚══════════════════════════════════════╝

💰 Prize: 0.09997000 L-BTC
📍 Sent to: tex1q...
📦 TXID: def456...
```

## 📚 How It Works

### The Simplicity Smart Contract

The core puzzle logic is implemented in Simplicity (`SimplicityHL/examples/puzzle_jackpot.simf`):

```simplicity
// PUZZLE WITH VALUE-BASED ENTROPY
// Note: This design has a critical bug with the current implementation
param TARGET_HASH: u256;
witness SECRET: u256;

fn main() {
    // Get the current UTXO value (in satoshis)
    let input_value: u64 = jet::current_value();

    // Convert u64 value to u256 for hashing
    let value_u256: u256 = u256::from(input_value);

    // Compute hash = SHA256(SECRET || VALUE)
    // WARNING: This means the hash changes if UTXO value changes!
    let hasher = jet::sha_256_ctx_8_init();
    let hasher = jet::sha_256_ctx_8_add_32(hasher, SECRET);
    let hasher = jet::sha_256_ctx_8_add_32(hasher, value_u256);  // <-- Bug: scripts don't account for this
    let computed_hash = jet::sha_256_ctx_8_finalize(hasher);

    // Verify the hash matches the target
    assert!(jet::eq_256(computed_hash, TARGET_HASH));
}
```

⚠️ **See "Known Issues" section below for critical bug details**

### Taproot Structure

Puzzles use Bitcoin's Taproot for enhanced privacy and efficiency:

```
Taproot Output
    │
    ├── Internal Key (unspendable placeholder)
    └── Script Tree
            └── Leaf: Simplicity Program (Contract Merkle Root)
```

## 🎮 Advanced Puzzle Types

> ⚠️ **Note**: All puzzle types currently include UTXO value in hash computation, making them incompatible with the creation/solving scripts. See Known Issues section.

### 1. **Basic Puzzle** (`puzzle_jackpot.simf`)
- SHA256(secret || value) verification
- Once created, the prize amount cannot be changed

### 2. **Time-Locked Puzzle** (`puzzle_chain_timelock.simf`)
- Adds minimum block height requirement
- Puzzle can only be solved after specific time
- Perfect for scheduled reveals
- Also uses SHA256(secret || value) formula

### 3. **Chained Puzzles** (`puzzle_chain.simf`)
- Multiple puzzles that must be solved in sequence
- Each solution reveals the next challenge
- Great for multi-stage challenges or treasure hunts
- Also uses SHA256(secret || value) formula

### 4. **Consolidation Puzzle** (`puzzle_consolidation.simf`)
- Requires multiple secrets to unlock
- Can implement M-of-N schemes
- Useful for group challenges or multi-sig scenarios
- Also uses SHA256(secret || value) formula

### 5. **Jackpot Consolidation** (`puzzle_jackpot_consolidation.simf`)
- Combines value-based hash with consolidation requirements
- Multiple unlock conditions with fixed prize pool
- Uses SHA256(secret || initial_value) formula

## 🏗️ Project Structure

```
simplicity-puzzle-hunt/
├── src/bin/
│   ├── create_puzzle.rs        # Create and fund new puzzles
│   └── solve_puzzle.rs         # Solve puzzles and claim prizes
├── SimplicityHL/examples/
│   ├── puzzle_jackpot.simf              # Basic puzzle contract
│   ├── puzzle_chain.simf                # Chained puzzles
│   ├── puzzle_chain_timelock.simf       # Time-locked puzzles
│   ├── puzzle_consolidation.simf        # Multi-secret puzzles
│   └── puzzle_jackpot_consolidation.simf # Combined mechanics
├── puzzle_*.json               # Generated puzzle files (public)
├── puzzle_*_SECRET.json        # Secret files (keep private!)
├── archived_puzzles/           # Solved puzzles archive
├── list-puzzles.sh            # List and manage puzzles
├── elements-cli               # Elements CLI wrapper
├── Cargo.toml                 # Rust project configuration
└── README.md                  # This file
```

## 🔧 Implemented Functions

### 1. **create_puzzle** (`src/bin/create_puzzle.rs`)

**Purpose**: Creates and funds new puzzle hunts on the Liquid testnet.

**Key Functions**:
- **SHA256 Hash Generation**: Computes SHA256(secret) as the target hash
- **Simplicity Contract Compilation**: Compiles the puzzle contract with the target hash
- **Taproot Address Creation**: Creates a P2TR address using the compiled contract
- **Automatic Funding**: Sends L-BTC to the puzzle address via Elements CLI
- **File Generation**: Creates both public and private JSON files

**Usage**:
```bash
cargo run --bin create-puzzle -- <secret> <amount> [hint]
```

**Outputs**:
- `puzzle_<hash>.json` - Public puzzle file with target hash, address, TXID
- `puzzle_<hash>_SECRET.json` - Private file with secret (keep secure!)

---

### 2. **solve_puzzle** (`src/bin/solve_puzzle.rs`)

**Purpose**: Solves puzzles and claims the prize by providing the correct secret.

**Key Functions**:
- **UTXO Verification** (`get_utxo_info`):
  - Fetches UTXO information from blockchain
  - Handles confidential values
  - Returns amount and asset ID
- **Secret Processing**:
  - Supports multiple formats: strings, hex numbers (32/64-bit), hex byte strings
  - Right-pads strings to 32 bytes
  - Converts to U256 format
- **Hash Verification**:
  - Computes SHA256(secret)
  - Validates against target hash
- **Contract Satisfaction**:
  - Compiles Simplicity contract with target hash
  - Creates witness values with secret
  - Satisfies the program
- **Transaction Building**:
  - Creates spending transaction with proper inputs/outputs
  - Calculates fees
  - Builds Taproot witness structure
- **Broadcasting**:
  - Sends transaction to network via Elements CLI
  - Reports success/failure

**Usage**:
```bash
cargo run --bin solve-puzzle -- <puzzle_file.json> <secret> <destination_address>
```

**Secret Formats Supported**:
- Text strings: `"satoshi"`
- 32-bit hex: `"0x00000001"`
- 64-bit hex: `"0x0000000000000001"`
- Hex bytes: `"0xdeadbeef"`

---

### 3. **list-puzzles.sh** (Shell Script)

**Purpose**: Lists, verifies, and manages puzzle status with archiving capabilities.

**Key Functions**:
- **UTXO Status Checking** (`check_utxo_status`):
  - Queries blockchain for UTXO existence
  - Determines if puzzle is active or solved
- **Puzzle Scanning**:
  - Reads all `puzzle_*.json` files
  - Extracts metadata (address, amount, hint, hash)
  - Categorizes as active/solved/unknown
- **Archiving System**:
  - Interactive mode: prompts before archiving
  - Auto mode: archives solved puzzles automatically
  - Moves both public and SECRET files
  - Timestamps archived files
- **Statistics Reporting**:
  - Total active prize pool calculation
  - Count of active/solved/invalid puzzles
  - Lists active puzzles ready to solve

**Usage**:
```bash
./list-puzzles.sh           # Interactive mode
./list-puzzles.sh --auto    # Auto-archive mode (for cron jobs)
./list-puzzles.sh --help    # Show help
```

**Features**:
- Color-coded output (active=green, solved=yellow, error=red)
- Archive management with timestamp preservation
- Cron-friendly auto mode for automation

---

### 4. **Helper Functions**

**elements-cli wrapper**:
- Provides interface to Elements daemon
- Used for:
  - Sending funds (`sendtoaddress`)
  - Checking UTXOs (`gettxout`)
  - Broadcasting transactions (`sendrawtransaction`)
  - Getting transaction details (`getrawtransaction`)

**JSON File Management**:
- Stores puzzle metadata
- Separates public and private information
- Enables puzzle sharing and tracking

---

### 5. **Simplicity Contracts** (`SimplicityHL/examples/`)

While not functions per se, these are the smart contract templates:

- **puzzle_jackpot.simf**: Basic SHA256(secret || value) verification
- **puzzle_chain.simf**: Sequential multi-puzzle challenges
- **puzzle_chain_timelock.simf**: Time-locked puzzles with block height requirements
- **puzzle_consolidation.simf**: Multi-secret unlock requirements
- **puzzle_jackpot_consolidation.simf**: Combined value-based and multi-secret mechanics

**Note**: All contracts currently include UTXO value in hash computation, causing incompatibility with the creation/solving scripts (see Known Issues).

## 🔒 Security Considerations

### For Puzzle Creators

- **Use strong secrets**: Avoid dictionary words, use random strings
- **Never reuse secrets**: Each puzzle should have a unique secret
- **Secure the SECRET files**: Delete or encrypt after puzzle is live
- **Consider entropy**: Mix random data with human-readable secrets
- **Test on testnet first**: Always verify contracts before mainnet

### For Puzzle Solvers

- **Race conditions exist**: Multiple solvers may find the secret simultaneously
- **Use competitive fees**: Higher fees = higher priority in mempool
- **Secret becomes public**: Once you broadcast, everyone sees the secret
- **Verify puzzle data**: Check the contract matches expected behavior
- **Monitor the mempool**: Watch for competing transactions

### Contract Security

- **Immutable rules**: Contract logic cannot be changed after deployment
- **No backdoors**: Simplicity's design prevents hidden behavior
- **Transparent validation**: Anyone can verify the contract logic
- **Atomic execution**: Either the secret is correct or transaction fails

## ⚠️ Known Issues

### Critical Bug: Hash Computation Mismatch
There's currently a fundamental mismatch between ALL contracts and the creation/solving scripts:

**What the contracts do** (all `.simf` files):
- Compute: `SHA256(SECRET || UTXO_VALUE)`
- This includes the UTXO's value in satoshis in the hash
- **All 5 puzzle types** have this same behavior

**What the scripts do**:
- `create_puzzle.rs` computes: `SHA256(SECRET)` only
- `solve_puzzle.rs` also verifies: `SHA256(SECRET)` only
- Neither script accounts for the UTXO value

**Impact**:
- **All puzzles created with current code are unsolvable**
- The contracts will compute a different hash than what's stored as TARGET_HASH
- This affects every single puzzle type in the project
- Adding funds would change the UTXO value, making puzzles permanently unsolvable
- The intended "jackpot" feature cannot work

**Possible fixes**:
1. **Fix the contracts**: Remove value from hash computation in all `.simf` files
2. **Fix the scripts**: Update both scripts to include the UTXO value in hash calculations
3. **Create new puzzle types**: Make simple versions that don't include value in the hash

## 🛠️ Troubleshooting

### Common Issues

#### "Failed to compile contract"
```bash
# Ensure SimplicityHL directory exists
ls -la SimplicityHL/examples/

# Check file permissions
chmod +r SimplicityHL/examples/*.simf
```

#### "Failed to connect to daemon"
```bash
# Start elementsd
./elements/src/elementsd -chain=liquidtestnet -daemon

# Check it's running
./elements/src/elements-cli -chain=liquidtestnet getblockchaininfo
```

#### "Insufficient funds"
```bash
# Check wallet balance
./elements-cli -chain=liquidtestnet getbalance

# Get testnet L-BTC from faucet
# Visit: https://liquidtestnet.com/faucet
```

#### "Transaction rejected"
Possible causes:
- Wrong secret provided
- UTXO already spent (puzzle solved)
- Insufficient transaction fees
- Network congestion

## 💡 Use Cases

Beyond gaming, this technology enables:

- **Educational CTFs**: Teach cryptography with real incentives
- **Marketing Campaigns**: Viral puzzles for brand engagement
- **Proof of Knowledge**: Prove knowledge without revealing it
- **Time Capsules**: Scheduled secret reveals
- **Group Escrow**: Multi-party unlocking mechanisms
- **Dead Man's Switch**: Automatic release after timeout

## 📖 Resources

- **Simplicity Language**: [GitHub](https://github.com/BlockstreamResearch/simplicity)
- **Elements Platform**: [elementsproject.org](https://elementsproject.org/)
- **Liquid Network**: [liquid.net](https://liquid.net/)
- **Liquid Testnet Faucet**: [liquidtestnet.com](https://liquidtestnet.com/faucet)

## 🤝 Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Ideas for Contribution

- Fix the critical hash computation bug
- New puzzle types (e.g., merkle tree puzzles, multi-sig puzzles)
- Web interface for puzzle creation/solving
- Mobile app integration
- Analytics dashboard for tracking puzzle statistics
- Automated testing suite
- Documentation improvements

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

Built with amazing technologies:
- **Simplicity** - Next-generation smart contract language
- **Elements** - Blockchain platform with confidential transactions
- **Liquid Network** - Bitcoin sidechain for digital assets
- **Rust** - Systems programming language

## 📞 Contact & Support

- **Issues**: [GitHub Issues](https://github.com/yourusername/simplicity-puzzle-hunt/issues)
- **Discussions**: [GitHub Discussions](https://github.com/yourusername/simplicity-puzzle-hunt/discussions)
- **Email**: your.email@example.com

---

**Happy Puzzle Hunting! May the best cryptographer win!** 🎯🏆