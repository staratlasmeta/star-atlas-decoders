use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum LicenseType {
    None,
    Burn { quantity: u64 },
    Vault { quantity: u64 },
}
