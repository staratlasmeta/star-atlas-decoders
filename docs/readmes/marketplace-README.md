# Carbon Marketplace Decoder

<p align="center">
  <a href="https://crates.io/crates/carbon-marketplace-decoder">
    <img src="https://img.shields.io/crates/v/carbon-marketplace-decoder?logo=rust" />
  </a>
  <a href="https://docs.rs/carbon-marketplace-decoder">
    <img src="https://img.shields.io/docsrs/carbon-marketplace-decoder?logo=docsdotrs" />
  </a>
  <a href="https://github.com/staratlasmeta/star-atlas-decoders/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/license-Apache%202.0-blue" />
  </a>
</p>

Rust decoder for the Star Atlas Galactic Marketplace program on Solana, generated using [Carbon CLI](https://github.com/sevenlabs-hq/carbon).

## Program Information

- **Program ID**: `traderDnaR5w6Tcoi3NFm53i48FTDNbGjBSZwWXDRrg`
- **Network**: Solana Mainnet
- **Description**: Star Atlas Galactic Marketplace program for NFT trading with order books, currency management, and royalty distribution.

## Features

- Decodes all Marketplace account types
- Full instruction parsing support
- Integration with Carbon indexing framework
- Support for orders, currencies, and marketplace configuration

## Usage

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
carbon-marketplace-decoder = "0.10.0"
```

### Decoding Accounts

```rust
use carbon_marketplace_decoder::MarketplaceDecoder;
use carbon_core::account::AccountDecoder;

let decoder = MarketplaceDecoder;
let decoded_account = decoder.decode_account(&account);

if let Some(decoded) = decoded_account {
    match decoded.data {
        MarketplaceAccount::Market(market) => {
            println!("Market: {:?}", market);
        }
        MarketplaceAccount::OpenOrdersAccount(orders) => {
            println!("Open Orders: {:?}", orders);
        }
        // ... handle other account types
        _ => {}
    }
}
```

### Account Types

This decoder supports Marketplace account types including:
- `Market` - Marketplace configuration
- `OpenOrdersAccount` - Active buy/sell orders
- `Currency` - Supported currencies
- `ProductType` - Product categorization
- And more...

## Documentation

Full documentation is available at [docs.rs](https://docs.rs/carbon-marketplace-decoder).

## Repository

See the [main repository](https://github.com/staratlasmeta/star-atlas-decoders) for build instructions and contribution guidelines.

## License

Licensed under the [Apache-2.0](https://github.com/staratlasmeta/star-atlas-decoders/blob/main/LICENSE) license.
