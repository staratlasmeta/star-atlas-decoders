
use super::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct RegisterCrewConfigArgs {
    pub name_prefix: String,
    pub symbol: String,
    pub uri_prefix: String,
    pub seller_fee_basis_points: u16,
    pub collection: solana_pubkey::Pubkey,
    pub creators: Vec<CrewCreatorUnpacked>,
}
