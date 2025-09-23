use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd4f185ac895331b1")]
pub struct RedeemCertificate {
    pub key_index: u16,
    pub amount: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct RedeemCertificateInstructionAccounts {
    pub starbase_and_starbase_player: solana_pubkey::Pubkey,
    pub game_accounts_and_profile: solana_pubkey::Pubkey,
    pub funds_to: solana_pubkey::Pubkey,
    pub cargo_mint: solana_pubkey::Pubkey,
    pub certificate_mint: solana_pubkey::Pubkey,
    pub certificate_token_from: solana_pubkey::Pubkey,
    pub cargo_token_from: solana_pubkey::Pubkey,
    pub cargo_token_to: solana_pubkey::Pubkey,
    pub cargo_pod: solana_pubkey::Pubkey,
    pub cargo_type: solana_pubkey::Pubkey,
    pub cargo_stats_definition: solana_pubkey::Pubkey,
    pub cargo_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub token2022_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RedeemCertificate {
    type ArrangedAccounts = RedeemCertificateInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let starbase_and_starbase_player = next_account(&mut iter)?;
        let game_accounts_and_profile = next_account(&mut iter)?;
        let funds_to = next_account(&mut iter)?;
        let cargo_mint = next_account(&mut iter)?;
        let certificate_mint = next_account(&mut iter)?;
        let certificate_token_from = next_account(&mut iter)?;
        let cargo_token_from = next_account(&mut iter)?;
        let cargo_token_to = next_account(&mut iter)?;
        let cargo_pod = next_account(&mut iter)?;
        let cargo_type = next_account(&mut iter)?;
        let cargo_stats_definition = next_account(&mut iter)?;
        let cargo_program = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let token2022_program = next_account(&mut iter)?;

        Some(RedeemCertificateInstructionAccounts {
            starbase_and_starbase_player,
            game_accounts_and_profile,
            funds_to,
            cargo_mint,
            certificate_mint,
            certificate_token_from,
            cargo_token_from,
            cargo_token_to,
            cargo_pod,
            cargo_type,
            cargo_stats_definition,
            cargo_program,
            token_program,
            token2022_program,
        })
    }
}
