
use super::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct ThreadResponse {
    pub close_to: Option<solana_pubkey::Pubkey>,
    pub dynamic_instruction: Option<SerializableInstruction>,
    pub trigger: Option<Trigger>,
}
