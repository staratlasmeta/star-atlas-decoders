use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xbb7344745398ee15")]
pub struct RegisterStake {
    pub reward_multiplier: u64,
    pub cooldown_period: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct RegisterStakeInstructionAccounts {
    pub authority: solana_pubkey::Pubkey,
    pub staking_vars_account: solana_pubkey::Pubkey,
    pub stake_mint: solana_pubkey::Pubkey,
    pub reward_mint: solana_pubkey::Pubkey,
    pub registered_stake: solana_pubkey::Pubkey,
    pub reward_ata: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RegisterStake {
    type ArrangedAccounts = RegisterStakeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let authority = next_account(&mut iter)?;
        let staking_vars_account = next_account(&mut iter)?;
        let stake_mint = next_account(&mut iter)?;
        let reward_mint = next_account(&mut iter)?;
        let registered_stake = next_account(&mut iter)?;
        let reward_ata = next_account(&mut iter)?;
        let rent = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let associated_token_program = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;

        Some(RegisterStakeInstructionAccounts {
            authority,
            staking_vars_account,
            stake_mint,
            reward_mint,
            registered_stake,
            reward_ata,
            rent,
            system_program,
            associated_token_program,
            token_program,
        })
    }
}
