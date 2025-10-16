use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb865a5bc5f3f7fbc")]
pub struct Profile {
    pub version: u8,
    pub auth_key_count: u16,
    pub key_threshold: u8,
    pub next_seq_id: u64,
    pub created_at: i64,
}
