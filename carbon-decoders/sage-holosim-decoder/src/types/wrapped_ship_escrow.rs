use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct WrappedShipEscrow {
    pub ship: solana_pubkey::Pubkey,
    pub amount: u64,
    pub update_id: u64,
}
