# Rust Blockchain

A minimal, fully-functional blockchain implemented in Rust as a command-line application.

---

## Features

| Feature | Details |
|---|---|
| Genesis block | Auto-created on startup |
| SHA-256 hashing | Each block hashes `index + timestamp + data + previous_hash` |
| Chain linking | Every block stores the hash of the previous block |
| Validation | Detects both hash corruption and broken chain links |
| Tamper detection | Demo shows chain becomes INVALID after data is mutated |
| Interactive CLI | Add blocks and inspect the chain live |

---

## Prerequisites

- [Rust + Cargo](https://rustup.rs/) (stable, 1.70+)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

---

## Setup

```bash
# Clone the repository
git clone https://github.com/<your-username>/blockchain.git
cd blockchain

# Build (debug)
cargo build

# Build (release — faster binary)
cargo build --release
```

---

## Running the Application

### Interactive mode

```bash
cargo run
```

You will see a prompt (`>`) where you can type commands:

| Command | Description |
|---|---|
| `add <data>` | Add a new block with the given transaction string |
| `display` | Print every block in the chain |
| `validate` | Check the chain for tampering |
| `tamper <index> <data>` | Overwrite a block's data without updating its hash |
| `demo` | Run a scripted walkthrough automatically |
| `quit` | Exit |

**Example session:**

```
> add X sent 100 tokens to Y
Block #1 added → hash: 3a9f1c2b...
> add Y sent 50 tokens to Z
Block #2 added → hash: 7e4d8a11...
> display
> validate
✅  Blockchain is VALID
> tamper 1 Alice sent 999999 tokens to Bob
> validate
❌  Blockchain is INVALID
    Reason: Block #1 has an invalid hash. Data may have been tampered with.
```

### Scripted demo mode

Runs the full demo (add blocks → display → validate → tamper → validate again) then exits:

```bash
 
```

### Tests

```bash
cargo test
```

---

## Project Structure

```
src/
├── main.rs         # Entry point; wires modules together
├── block.rs        # Block struct, SHA-256 hashing, display
├── blockchain.rs   # Chain management: add, display, validate, tamper
└── cli.rs          # Interactive REPL and demo runner
Cargo.toml          # Dependencies: sha2, hex, chrono, serde
```

---

## Blockchain Implementation

### Block structure

Each block contains five fields:

```
index         — sequential position in the chain
timestamp     — Unix timestamp (UTC) at creation time
data          — arbitrary string (transaction or message)
previous_hash — SHA-256 hash of the preceding block
hash          — SHA-256 hash of this block's own contents
```

### Hashing

The hash is derived from:

```
SHA-256( index || timestamp || data || previous_hash )
```

All fields are concatenated as UTF-8 strings before hashing.

### Genesis block

The first block (index 0) uses a string of 64 zeros as its `previous_hash` and is created automatically when the application starts.

### Validation

`Blockchain::validate()` walks the chain and checks two invariants per block:

1. **Hash integrity** — the stored hash equals a freshly computed hash of the block's own fields.  
2. **Chain linkage** — the block's `previous_hash` equals the `hash` of the immediately preceding block.

Any violation is reported with a descriptive error message, and the chain is declared **INVALID**.

### Tamper detection

`Blockchain::tamper(index, new_data)` mutates a block's `data` field *without* recomputing its hash, simulating an attacker editing the ledger. The next call to `validate()` will catch the discrepancy and report the corrupted block.

---

## Dependencies

| Crate | Purpose |
|---|---|
| `sha2` | SHA-256 digest |
| `hex` | Encode hash bytes as a hex string |
| `chrono` | UTC timestamps |
| `serde` + `serde_json` | Serialisation (ready for JSON export) |
