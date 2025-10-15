# Carbon Tcomp Decoder

<p align="center">
  <a href="https://crates.io/crates/carbon-tcomp-decoder">
    <img src="https://img.shields.io/crates/v/carbon-tcomp-decoder?logo=rust" />
  </a>
  <a href="https://docs.rs/carbon-tcomp-decoder">
    <img src="https://img.shields.io/docsrs/carbon-tcomp-decoder?logo=docsdotrs" />
  </a>
  <a href="https://github.com/staratlasmeta/star-atlas-decoders/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/license-Apache%202.0-blue" />
  </a>
</p>

Rust decoder for the Tensor cNFT Compressed program on Solana, generated using [Carbon CLI](https://github.com/sevenlabs-hq/carbon).

## Program Information

- **Program ID**: `TCMPhJdwDryooaGtiocG1u3xcYbRpiJzb283XfCZsDp`
- **Network**: Solana Mainnet
- **Description**: Tensor cNFT Compressed marketplace program for trading compressed NFTs with support for listings, bids, and core asset management.

## Features

- Decodes all Tensor cNFT account types
- Full instruction parsing support
- Integration with Carbon indexing framework
- Support for compressed NFT marketplace operations including listings, bids, and core asset transfers

## Usage

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
carbon-tcomp-decoder = "0.10.0"
```

### Decoding Accounts

```rust
use carbon_tcomp_decoder::TcompDecoder;
use carbon_core::account::AccountDecoder;

let decoder = TcompDecoder;
let decoded_account = decoder.decode_account(&account);

if let Some(decoded) = decoded_account {
    match decoded.data {
        TcompAccount::ListState(list_state) => {
            println!("List State: {:?}", list_state);
        }
        TcompAccount::BidState(bid_state) => {
            println!("Bid State: {:?}", bid_state);
        }
    }
}
```

### Account Types

This decoder supports all Tensor cNFT Compressed account types:
- `ListState` - NFT listing state and configuration
- `BidState` - Active bid state for NFT purchases

## Documentation

Full documentation is available at [docs.rs](https://docs.rs/carbon-tcomp-decoder).

## Repository

See the [main repository](https://github.com/staratlasmeta/star-atlas-decoders) for build instructions and contribution guidelines.

## License

Licensed under the [Apache-2.0](https://github.com/staratlasmeta/star-atlas-decoders/blob/main/LICENSE) license.
