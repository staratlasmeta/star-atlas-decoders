
use super::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct SerializableInstruction {
    pub program_id: solana_pubkey::Pubkey,
    pub accounts: Vec<SerializableAccount>,
    pub data: Vec<u8>,
}
