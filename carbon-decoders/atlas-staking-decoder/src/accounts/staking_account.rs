use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x34b2fb9db4ba62ea")]
pub struct StakingAccount {
    pub owner: solana_pubkey::Pubkey,
    pub registered_stake: solana_pubkey::Pubkey,
    pub stake_mint: solana_pubkey::Pubkey,
    pub total_stake: u64,
    pub active_stake: u64,
    pub pending_rewards: u64,
    pub paid_rewards: u64,
    pub current_period: u16,
    pub staked_at_ts: i64,
    pub last_pending_reward_calc_ts: i64,
    pub last_harvest_ts: i64,
    pub unstaked_ts: i64,
    pub bump: u8,
}
