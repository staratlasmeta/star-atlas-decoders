# Decoder Explorer

CLI tool for testing and debugging Carbon decoders against live Solana accounts. Fetches all accounts for a program and analyzes decoder success rates, account type distributions, and failure patterns.

## Purpose

- **Validate decoders**: Verify that decoders correctly handle all on-chain account variants
- **Debug failures**: Identify accounts that fail to decode with detailed error information
- **Analyze distributions**: Understand account type breakdowns and size distributions
- **Inspect accounts**: View decoded account data as JSON (debug mode)

## Prerequisites

- Rust 1.85+ (edition 2024)
- Access to a Solana RPC endpoint

## Installation

```bash
cd tools/decoder-explorer
cargo build --release --features <decoder>
```

## Usage

### Environment Configuration

Create a `.env` file or set environment variables:

```bash
# Required: Solana RPC URL
RPC_URL=https://api.mainnet-beta.solana.com

# Optional: Override default program ID
PROGRAM_ID=CustomProgramId...
```

### Running

```bash
# Run with environment variable
RPC_URL="https://api.mainnet-beta.solana.com" cargo run --release --features cargo

# Run with .env file
cargo run --release --features sage-starbased

# Debug mode (show sample accounts as JSON)
RUST_LOG=debug cargo run --release --features cargo
```

### Available Features

Each decoder has a corresponding feature flag:

| Feature | Program |
|---------|---------|
| `sage-starbased` | SAGE Starbase |
| `sage-holosim` | SAGE Holosim |
| `atlas-staking` | Atlas Staking |
| `locked-voter` | Locked Voter |
| `marketplace` | Galactic Marketplace |
| `atlas-fee-payer` | Atlas Fee Payer |
| `cargo` | Cargo |
| `crafting` | Crafting |
| `crew` | Crew Management |
| `profile-vault` | Profile Vault |
| `srsly` | Fleet Rentals |
| `tcomp` | Tensor cNFT |
| `player-profile` | Player Profile |
| `points` | Points |
| `points-store` | Points Store |
| `profile-faction` | Profile Faction |

## Output

### Account Size Distribution

Histogram of account sizes across all program accounts:

```
=== Account Size Distribution ===
Total accounts: 15432
Total data size: 2345678 bytes (2.24 MB)
Average account size: 152 bytes

Size Distribution:
  < 100 bytes              430 ( 2.79%)
  < 500 bytes            12000 (77.76%)
  < 1000 bytes            3000 (19.44%)
  >= 1000000 bytes           2 ( 0.01%)

Size Range:
  Minimum: 56 bytes
  Maximum: 1234567 bytes (1205.63 KB)
```

### Account Type Classification

Decode statistics and type breakdown:

```
=== Account Type Classification ===
Total accounts: 15432
Successfully decoded: 15430
Decode failures: 2
Decode failure rate: 0.01%

Account Type Distribution:
  CargoPod                       12000 (77.76%)
  CargoType                       3000 (19.44%)
  CargoStatsDefinition             430 ( 2.79%)
```

### Decode Failures

Detailed information for accounts that failed to decode:

```
=== Decode Failures ===
[1] Account: 7xKXt...
    Owner: Cargo2VNTPPTi9c1vq1Jw5d3BWUNr18MjRtSupAghKEk
    Data size: 156 bytes
    First 16 bytes: a5 21 76 eb fc bc f4 5d 01 00 00 00 00 00 00 00
```

### Sample Accounts (Debug Mode)

With `RUST_LOG=debug`, shows decoded JSON for the first 5 accounts:

```
=== Sample Accounts ===
Showing first 5 accounts:

--- Account 1 ---
Pubkey: 7xKXt...
Owner: Cargo2VNTPPTi9c1vq1Jw5d3BWUNr18MjRtSupAghKEk
Lamports: 1234567
Data length: 256 bytes
Account type: CargoPod
Decoded data:
{
  "authority": "...",
  "cargo_contents": [...]
}
```

## Architecture

```
src/
├── main.rs              # Entry point, feature-gated imports, macro invocations
├── config.rs            # Environment config (RPC_URL, PROGRAM_ID)
├── decoder.rs           # Macros: impl_type_identifier!, run_decoder_analysis!
├── rpc.rs               # RPC client + get_program_accounts
└── analysis/
    ├── mod.rs                  # AnalysisContext, AccountTypeIdentifier trait
    ├── type_classification.rs  # Decode stats + failure reporting
    ├── size_distribution.rs    # Account size histogram
    └── sample_accounts.rs      # JSON output (debug mode)
```

## License

Apache-2.0
