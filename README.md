# Star Atlas Decoders

<p align="center">
  <a href="https://github.com/staratlasmeta/star-atlas-decoders/actions/workflows/ci.yml">
    <img src="https://img.shields.io/github/actions/workflow/status/staratlasmeta/star-atlas-decoders/ci.yml?logo=GitHub" />
  </a>
  <a href="LICENSE">
    <img src="https://img.shields.io/badge/license-Apache%202.0-blue" />
  </a>
</p>

<p align="center">
  <a href="https://crates.io/crates/carbon-sage-starbased-decoder">
    <img src="https://img.shields.io/crates/v/carbon-sage-starbased-decoder?logo=rust&label=sage-starbased" />
  </a>
  <a href="https://crates.io/crates/carbon-sage-holosim-decoder">
    <img src="https://img.shields.io/crates/v/carbon-sage-holosim-decoder?logo=rust&label=sage-holosim" />
  </a>
  <a href="https://crates.io/crates/carbon-atlas-staking-decoder">
    <img src="https://img.shields.io/crates/v/carbon-atlas-staking-decoder?logo=rust&label=atlas-staking" />
  </a>
  <a href="https://crates.io/crates/carbon-locked-voter-decoder">
    <img src="https://img.shields.io/crates/v/carbon-locked-voter-decoder?logo=rust&label=locked-voter" />
  </a>
  <a href="https://crates.io/crates/carbon-marketplace-decoder">
    <img src="https://img.shields.io/crates/v/carbon-marketplace-decoder?logo=rust&label=marketplace" />
  </a>
  <a href="https://crates.io/crates/carbon-atlas-fee-payer-decoder">
    <img src="https://img.shields.io/crates/v/carbon-atlas-fee-payer-decoder?logo=rust&label=atlas-fee-payer" />
  </a>
  <a href="https://crates.io/crates/carbon-crew-decoder">
    <img src="https://img.shields.io/crates/v/carbon-crew-decoder?logo=rust&label=crew" />
  </a>
  <a href="https://crates.io/crates/carbon-profile-vault-decoder">
    <img src="https://img.shields.io/crates/v/carbon-profile-vault-decoder?logo=rust&label=profile-vault" />
  </a>
  <a href="https://crates.io/crates/carbon-srsly-decoder">
    <img src="https://img.shields.io/crates/v/carbon-srsly-decoder?logo=rust&label=srsly" />
  </a>
  <a href="https://crates.io/crates/carbon-tcomp-decoder">
    <img src="https://img.shields.io/crates/v/carbon-tcomp-decoder?logo=rust&label=tcomp" />
  </a>
  <a href="https://crates.io/crates/carbon-player-profile-decoder">
    <img src="https://img.shields.io/crates/v/carbon-player-profile-decoder?logo=rust&label=player-profile" />
  </a>
</p>

Rust decoders for Star Atlas Solana programs, generated from IDLs using Carbon CLI with custom patches for complex account deserialization.

## Overview

This project generates and maintains Rust decoders for Star Atlas programs on Solana. It uses [Carbon CLI](https://github.com/sevenlabs-hq/carbon) to generate initial decoder code from program IDLs, then applies custom patches to handle complex account structures that require manual deserialization logic.

### Supported Decoders

- **sage-starbased**: SAGE Starbase program (`SAGE2HAwep459SNq61LHvjxPk4pLPEJLoMETef7f7EE`)
  - Fetches IDL directly from Solana mainnet
  - Custom deserialization for Fleet and StarbasePlayer accounts

- **sage-holosim**: SAGE Holosim program (`SAgEeT8u14TE69JXtanGSgNkEdoPUcLabeyZD2uw8x9`)
  - Uses local IDL file from `./idl/` directory
  - Custom deserialization for Fleet and StarbasePlayer accounts

- **atlas-staking**: Atlas Staking program (`ATLocKpzDbTokxgvnLew3d7drZkEzLzDpzwgrgWKDbmc`)
  - Fetches IDL directly from Solana mainnet
  - ATLAS token staking with configurable rewards and cooldown periods
  - Minimal patches needed - adds serialization support for account types

- **locked-voter**: Locked Voter program (`Lock7kBijGCQLEFAmXcengzXKA88iDNQPriQ7TbgeyG`)
  - Fetches IDL directly from Solana mainnet
  - POLIS governance and voting with escrow and whitelist controls
  - Minimal patches needed - adds serialization support for account types

- **marketplace**: Galactic Marketplace program (`traderDnaR5w6Tcoi3NFm53i48FTDNbGjBSZwWXDRrg`)
  - Fetches IDL directly from Solana mainnet
  - NFT marketplace with order books, currency management, and royalty tiers
  - Minimal patches needed - adds serialization support for account types

- **atlas-fee-payer**: ATLAS Fee Payer program (`APR1MEny25pKupwn72oVqMH4qpDouArsX8zX4VwwfoXD`)
  - Fetches IDL directly from Solana mainnet
  - Fee payment management for Star Atlas transactions
  - Minimal patches needed - adds serialization support for account types

- **crew**: Crew Management program (`CREWiq8qbxvo4SKkAFpVnc6t7CRQC4tAAscsNAENXgrJ`)
  - Fetches IDL directly from Solana mainnet
  - Crew management for Star Atlas ships and operations
  - Uses serde_big_array for large byte arrays
  - Minimal patches needed - adds serialization support for account types

- **profile-vault**: Profile Vault program (`pv1ttom8tbyh83C1AVh6QH2naGRdVQUVt3HY1Yst5sv`)
  - Fetches IDL directly from Solana mainnet
  - Profile vault management for Star Atlas player profiles
  - Minimal patches needed - adds serialization support for account types

- **srsly**: Fleet Rentals (SRSLY) program (`SRSLY1fq9TJqCk1gNSE7VZL2bztvTn9wm4VR8u8jMKT`)
  - Fetches IDL directly from Solana mainnet
  - Fleet rental contracts and automated payment processing
  - Custom patches - adds serialization support and f64 rate field workaround

- **tcomp**: Tensor cNFT Compressed program (`TCMPhJdwDryooaGtiocG1u3xcYbRpiJzb283XfCZsDp`)
  - Fetches IDL directly from Solana mainnet
  - Compressed NFT marketplace for trading cNFTs
  - Uses serde_big_array for large byte arrays
  - Minimal patches needed - adds serialization support for account types

- **player-profile**: Player Profile program (`pprofELXjL5Kck7Jn5hCpwAL82DpTkSYBENzahVtbc9`)
  - Fetches IDL directly from Solana mainnet
  - Player identity and role-based access control for Star Atlas
  - Custom patches - adds serialization and ergonomic permission bitflags with helper methods
  - Includes permission checking, key expiration validation, and role management

## Prerequisites

Run `./scripts/check-tools.sh` to verify all required tools are installed:

- **Rust** (1.75+ for edition 2024)
- **Carbon CLI**: `cargo install --git https://github.com/sevenlabs-hq/carbon.git carbon-cli`
- **Just**: `cargo install just` or `brew install just`
- **Git** and standard Unix tools (sed, find)

## Quick Start

```bash
# Check required tools
./scripts/check-tools.sh

# Build all decoders and run CI checks
./scripts/ci.sh
```

## Project Structure

```
star-atlas-decoders/
├── carbon-decoders/         # Published decoder crates
│   ├── sage-starbased-decoder/
│   ├── sage-holosim-decoder/
│   ├── atlas-staking-decoder/
│   ├── locked-voter-decoder/
│   ├── marketplace-decoder/
│   ├── atlas-fee-payer-decoder/
│   ├── crew-decoder/
│   ├── profile-vault-decoder/
│   ├── srsly-decoder/
│   ├── tcomp-decoder/
│   └── player-profile-decoder/
├── dist/                    # Temporary build directory (gitignored)
├── patches/                 # Custom patches for decoders
│   ├── sage-starbased-01-accounts.patch
│   ├── sage-holosim-01-disable-ix-combat-log-event.patch
│   ├── atlas-staking-01-accounts-serialize.patch
│   ├── locked-voter-01-accounts-serialize.patch
│   ├── marketplace-01-accounts-serialize.patch
│   ├── atlas-fee-payer-01-accounts-serialize.patch
│   ├── crew-01-accounts-serialize.patch
│   ├── profile-vault-01-accounts-serialize.patch
│   ├── srsly-01-accounts-serialize.patch
│   ├── srsly-02-rate-f64-workaround.patch
│   ├── tcomp-01-accounts-serialize.patch
│   ├── player-profile-01-accounts-serialize.patch
│   ├── player-profile-02-permissions-helpers.patch
│   └── player-profile-03-remaining-data.patch
├── idl/                     # Local IDL files
├── scripts/                 # CI and utility scripts
│   ├── ci.sh               # Full CI pipeline
│   └── check-tools.sh      # Tool verification
├── docs/                    # Documentation
│   ├── patch-development-workflow.md
│   └── readmes/            # Individual decoder READMEs
└── justfile                # Build automation

```

## Development Workflow

### Building Decoders

The build process follows these stages:

1. **Generate**: Fetch IDL and generate initial decoder code
2. **Prepare**: Fix compilation issues (array sizes, workspace refs)
3. **Patch**: Apply custom implementations for complex accounts
4. **Publish**: Move to workspace and verify compilation

```bash
# Full pipeline for a decoder
just all-sage-starbased

# Individual steps
just generate-sage-starbased   # Generate from IDL
just build-sage-starbased      # Clean + generate + prepare
just apply-patches-sage-starbased  # Apply custom patches
just publish-sage-starbased    # Move to workspace
```

### Creating Custom Patches

Some accounts require custom deserialization for "remaining data" fields or dynamic arrays not in the IDL:

```bash
# 1. Build clean decoder
just build-sage-starbased

# 2. Apply existing patches
just apply-patches-sage-starbased

# 3. Edit files in dist/sage-starbased/
cd dist/sage-starbased
# Make your changes
vim src/accounts/fleet.rs

# 4. Test changes
cargo check

# 5. Create patch
just create-patch-sage-starbased my-change

# 6. Publish to workspace
just publish-sage-starbased
```

See [docs/patch-development-workflow.md](docs/patch-development-workflow.md) for detailed instructions.

### Patch Naming Convention

When multiple patches need specific ordering, use numbered prefixes:
- `sage-starbased-01-accounts.patch` (applied first)
- `sage-starbased-02-instructions.patch` (applied second)

## Common Commands

### CI and Testing

```bash
# Run full CI pipeline
./scripts/ci.sh

# Run clippy
cargo clippy --all-targets --all-features -- -D warnings

# Run tests
cargo test --all
```

### Maintenance

```bash
# Clean build artifacts
just clean-all

# List available patches
just list-patches
```

## Custom Implementations

The decoders include custom deserialization for accounts with:

- **Variable-length "remaining data" fields**: e.g., Fleet's `fleet_state`
- **Dynamic arrays not in IDL**: e.g., StarbasePlayer's `ship_escrows`
- **Complex nested structures**: Custom BorshDeserialize implementations

Example accounts with custom deserialization:
- `Fleet`: Includes `fleet_state` enum for current fleet activity
- `StarbasePlayer`: Includes dynamic `ship_escrows` list

## Technical Details

- **Rust Edition**: 2024 (requires Rust 1.85.0+)
- **Carbon Version**: 0.10.0 (available on [crates.io](https://crates.io/crates/carbon-core))
- **Solana SDK**: 2.x
- **Platform**: macOS and Linux compatible

## Contributing

1. Follow the existing patch workflow for modifications
2. Ensure all changes pass `./scripts/ci.sh`
3. Document any new custom deserializations
4. Use numbered patch prefixes when order matters

## License

This project is licensed under the [Apache-2.0](LICENSE) license.

## Resources

- [Carbon CLI Documentation](https://github.com/sevenlabs-hq/carbon)
- [Star Atlas Build](https://build.staratlas.com/)
