# Carbon ATLAS Staking Decoder

<p align="center">
  <a href="https://crates.io/crates/carbon-atlas-staking-decoder">
    <img src="https://img.shields.io/crates/v/carbon-atlas-staking-decoder?logo=rust" />
  </a>
  <a href="https://docs.rs/carbon-atlas-staking-decoder">
    <img src="https://img.shields.io/docsrs/carbon-atlas-staking-decoder?logo=docsdotrs" />
  </a>
  <a href="https://github.com/staratlasmeta/star-atlas-decoders/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/license-Apache%202.0-blue" />
  </a>
</p>

Rust decoder for the Star Atlas ATLAS staking program on Solana, generated using [Carbon CLI](https://github.com/sevenlabs-hq/carbon).

## Program Information

- **Program ID**: `ATLocKpzDbTokxgvnLew3d7drZkEzLzDpzwgrgWKDbmc`
- **Network**: Solana Mainnet
- **Description**: Star Atlas ATLAS token staking program with configurable rewards and cooldown periods for participating in the Star Atlas ecosystem.

## Features

- Decodes all ATLAS staking account types
- Full instruction parsing support
- Integration with Carbon indexing framework
- Support for staking pools, user stakes, and reward distribution

## Usage

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
carbon-atlas-staking-decoder = "0.10.0"
```

### Decoding Accounts

```rust
use carbon_atlas_staking_decoder::AtlasStakingDecoder;
use carbon_core::account::AccountDecoder;

let decoder = AtlasStakingDecoder;
let decoded_account = decoder.decode_account(&account);

if let Some(decoded) = decoded_account {
    match decoded.data {
        AtlasStakingAccount::StakePool(pool) => {
            println!("Stake Pool: {:?}", pool);
        }
        AtlasStakingAccount::StakeAccount(stake) => {
            println!("User Stake: {:?}", stake);
        }
        // ... handle other account types
        _ => {}
    }
}
```

### Account Types

This decoder supports ATLAS staking account types including:
- `StakePool` - Staking pool configuration and state
- `StakeAccount` - Individual user stake positions
- `StakeDepositReceipt` - Deposit tracking
- And more...

## Documentation

Full documentation is available at [docs.rs](https://docs.rs/carbon-atlas-staking-decoder).

## Repository

See the [main repository](https://github.com/staratlasmeta/star-atlas-decoders) for build instructions and contribution guidelines.

## License

Licensed under the [Apache-2.0](https://github.com/staratlasmeta/star-atlas-decoders/blob/main/LICENSE) license.
