

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct SerializableAccount {
    pub pubkey: solana_pubkey::Pubkey,
    pub is_signer: bool,
    pub is_writable: bool,
}
