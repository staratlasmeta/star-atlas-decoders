use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0bfbcb97b491272e")]
pub struct RegisteredStake {
    pub authority: solana_pubkey::Pubkey,
    pub stake_mint: solana_pubkey::Pubkey,
    pub reward_mint: solana_pubkey::Pubkey,
    pub current_period: u16,
    pub reward_multiplier: u64,
    pub cooldown_period: u64,
    pub bump: u8,
}
