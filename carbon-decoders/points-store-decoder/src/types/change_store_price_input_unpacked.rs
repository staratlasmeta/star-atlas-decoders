use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct ChangeStorePriceInputUnpacked {
    pub new_price: u64,
    pub key_index: u16,
}
