use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa7bfe73f9229731b")]
pub struct Domain {
    pub version: u8,
    pub profile: solana_pubkey::Pubkey,
    pub namespace: [u8; 32],
}
