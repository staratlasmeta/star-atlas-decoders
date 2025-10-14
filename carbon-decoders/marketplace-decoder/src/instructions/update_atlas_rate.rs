use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf8539e287daecbd4")]
pub struct UpdateAtlasRate {
    pub rate: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct UpdateAtlasRateInstructionAccounts {
    pub funder: solana_pubkey::Pubkey,
    pub update_authority_account: solana_pubkey::Pubkey,
    pub market_vars_account: solana_pubkey::Pubkey,
    pub atlas_rate: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateAtlasRate {
    type ArrangedAccounts = UpdateAtlasRateInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let funder = next_account(&mut iter)?;
        let update_authority_account = next_account(&mut iter)?;
        let market_vars_account = next_account(&mut iter)?;
        let atlas_rate = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(UpdateAtlasRateInstructionAccounts {
            funder,
            update_authority_account,
            market_vars_account,
            atlas_rate,
            system_program,
        })
    }
}
