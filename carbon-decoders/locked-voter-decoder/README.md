# Carbon Locked Voter Decoder

<p align="center">
  <a href="https://crates.io/crates/carbon-locked-voter-decoder">
    <img src="https://img.shields.io/crates/v/carbon-locked-voter-decoder?logo=rust" />
  </a>
  <a href="https://docs.rs/carbon-locked-voter-decoder">
    <img src="https://img.shields.io/docsrs/carbon-locked-voter-decoder?logo=docsdotrs" />
  </a>
  <a href="https://github.com/staratlasmeta/star-atlas-decoders/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/license-Apache%202.0-blue" />
  </a>
</p>

Rust decoder for the Star Atlas Locked Voter governance program on Solana, generated using [Carbon CLI](https://github.com/sevenlabs-hq/carbon).

## Program Information

- **Program ID**: `Lock7kBijGCQLEFAmXcengzXKA88iDNQPriQ7TbgeyG`
- **Network**: Solana Mainnet
- **Description**: Star Atlas POLIS governance and voting program with token locking, escrow management, and whitelist controls for DAO participation.

## Features

- Decodes all Locked Voter account types
- Full instruction parsing support
- Integration with Carbon indexing framework
- Support for escrows, lockers, and governance voting

## Usage

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
carbon-locked-voter-decoder = "0.10.0"
```

### Decoding Accounts

```rust
use carbon_locked_voter_decoder::LockedVoterDecoder;
use carbon_core::account::AccountDecoder;

let decoder = LockedVoterDecoder;
let decoded_account = decoder.decode_account(&account);

if let Some(decoded) = decoded_account {
    match decoded.data {
        LockedVoterAccount::Locker(locker) => {
            println!("Locker: {:?}", locker);
        }
        LockedVoterAccount::Escrow(escrow) => {
            println!("Escrow: {:?}", escrow);
        }
        // ... handle other account types
        _ => {}
    }
}
```

### Account Types

This decoder supports Locked Voter account types including:
- `Locker` - Token locking configuration
- `Escrow` - User token escrow accounts
- `LockerWhitelistEntry` - Whitelist management
- And more...

## Documentation

Full documentation is available at [docs.rs](https://docs.rs/carbon-locked-voter-decoder).

## Repository

See the [main repository](https://github.com/staratlasmeta/star-atlas-decoders) for build instructions and contribution guidelines.

## License

Licensed under the [Apache-2.0](https://github.com/staratlasmeta/star-atlas-decoders/blob/main/LICENSE) license.
