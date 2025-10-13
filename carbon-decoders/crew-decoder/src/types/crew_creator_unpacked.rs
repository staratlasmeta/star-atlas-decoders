

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct CrewCreatorUnpacked {
    pub key: solana_pubkey::Pubkey,
    pub share: u8,
}
