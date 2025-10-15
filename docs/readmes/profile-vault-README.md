# Carbon Profile Vault Decoder

<p align="center">
  <a href="https://crates.io/crates/carbon-profile-vault-decoder">
    <img src="https://img.shields.io/crates/v/carbon-profile-vault-decoder?logo=rust" />
  </a>
  <a href="https://docs.rs/carbon-profile-vault-decoder">
    <img src="https://img.shields.io/docsrs/carbon-profile-vault-decoder?logo=docsdotrs" />
  </a>
  <a href="https://github.com/staratlasmeta/star-atlas-decoders/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/license-Apache%202.0-blue" />
  </a>
</p>

Rust decoder for the Star Atlas Profile Vault program on Solana, generated using [Carbon CLI](https://github.com/sevenlabs-hq/carbon).

## Program Information

- **Program ID**: `pv1ttom8tbyh83C1AVh6QH2naGRdVQUVt3HY1Yst5sv`
- **Network**: Solana Mainnet
- **Description**: Star Atlas Profile Vault program for managing vault authorities and secure asset storage in the Star Atlas ecosystem.

## Features

- Decodes all Profile Vault account types
- Full instruction parsing support
- Integration with Carbon indexing framework
- Support for vault authority management and vault operations

## Usage

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
carbon-profile-vault-decoder = "0.10.0"
```

### Decoding Accounts

```rust
use carbon_profile_vault_decoder::ProfileVaultDecoder;
use carbon_core::account::AccountDecoder;

let decoder = ProfileVaultDecoder;
let decoded_account = decoder.decode_account(&account);

if let Some(decoded) = decoded_account {
    match decoded.data {
        ProfileVaultAccount::VaultAuthority(authority) => {
            println!("Vault Authority: {:?}", authority);
        }
    }
}
```

### Account Types

This decoder supports all Profile Vault account types:
- `VaultAuthority` - Vault authority configuration and permissions

## Documentation

Full documentation is available at [docs.rs](https://docs.rs/carbon-profile-vault-decoder).

## Repository

See the [main repository](https://github.com/staratlasmeta/star-atlas-decoders) for build instructions and contribution guidelines.

## License

Licensed under the [Apache-2.0](https://github.com/staratlasmeta/star-atlas-decoders/blob/main/LICENSE) license.
