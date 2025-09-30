use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe4f11fb635a93bc7")]
pub struct Harvest {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct HarvestInstructionAccounts {
    pub user: solana_pubkey::Pubkey,
    pub reward_mint: solana_pubkey::Pubkey,
    pub registered_stake: solana_pubkey::Pubkey,
    pub staking_account: solana_pubkey::Pubkey,
    pub user_reward_account: solana_pubkey::Pubkey,
    pub reward_ata: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Harvest {
    type ArrangedAccounts = HarvestInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let user = next_account(&mut iter)?;
        let reward_mint = next_account(&mut iter)?;
        let registered_stake = next_account(&mut iter)?;
        let staking_account = next_account(&mut iter)?;
        let user_reward_account = next_account(&mut iter)?;
        let reward_ata = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let rent = next_account(&mut iter)?;
        let associated_token_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(HarvestInstructionAccounts {
            user,
            reward_mint,
            registered_stake,
            staking_account,
            user_reward_account,
            reward_ata,
            token_program,
            rent,
            associated_token_program,
            system_program,
        })
    }
}
