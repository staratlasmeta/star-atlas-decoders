use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct SetFeePayerData {
    pub owning_profile: bool,
    pub token_owner: bool,
    pub token_limit: Option<u64>,
    pub conversion_rate: Option<u64>,
}
