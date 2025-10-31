use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct CreateRedemptionConfigInput {
    pub faction: u8,
    pub allow_only_current_epoch: bool,
}
