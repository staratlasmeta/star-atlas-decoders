# Star Atlas Decoders

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

# Or build individual decoders
just all-sage-starbased
just all-sage-holosim
```

## Project Structure

```
star-atlas-decoders/
├── carbon-decoders/         # Published decoder crates
│   ├── sage-starbased-decoder/
│   └── sage-holosim-decoder/
├── dist/                    # Temporary build directory (gitignored)
├── patches/                 # Custom patches for decoders
│   ├── sage-starbased-01-accounts.patch
│   └── sage-holosim-01-disable-ix-combat-log-event.patch
├── idl/                     # Local IDL files
├── scripts/                 # CI and utility scripts
│   ├── ci.sh               # Full CI pipeline
│   └── check-tools.sh      # Tool verification
├── docs/                    # Documentation
│   └── patch-development-workflow.md
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

# Check specific decoder
cargo check -p sage-starbased-decoder
cargo check -p sage-holosim-decoder

# Run clippy
cargo clippy --all-targets --all-features -- -D warnings

# Run tests
cargo test --all
```

### Maintenance

```bash
# Clean build artifacts
just clean-sage-starbased
just clean-sage-holosim
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

- **Rust Edition**: 2024
- **Carbon Version**: 0.10.0
- **Solana SDK**: 2.x
- **Platform**: macOS compatible (BSD sed)

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
