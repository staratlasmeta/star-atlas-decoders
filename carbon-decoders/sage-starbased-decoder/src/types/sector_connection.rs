use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct SectorConnection {
    pub connection_sector: solana_pubkey::Pubkey,
    pub sub_coordinates: [i64; 2],
    pub flags: u8,
}
