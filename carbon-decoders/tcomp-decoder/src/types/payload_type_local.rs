use super::*;

use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum PayloadTypeLocal {
    Pubkey(solana_pubkey::Pubkey),
    Seeds(SeedsVecLocal),
    MerkleProof(ProofInfoLocal),
    Number(u64),
}
