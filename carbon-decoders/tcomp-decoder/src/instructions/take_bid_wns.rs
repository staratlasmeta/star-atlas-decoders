

use carbon_core::{CarbonDeserialize, borsh, account_utils::next_account};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x58057a58fa8b23d8")]
pub struct TakeBidWns{
    pub min_amount: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct TakeBidWnsInstructionAccounts {
    pub tcomp: solana_pubkey::Pubkey,
    pub seller: solana_pubkey::Pubkey,
    pub bid_state: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub taker_broker: Option<solana_pubkey::Pubkey>,
    pub maker_broker: Option<solana_pubkey::Pubkey>,
    pub margin_account: solana_pubkey::Pubkey,
    pub whitelist: solana_pubkey::Pubkey,
    pub nft_seller_acc: solana_pubkey::Pubkey,
    pub nft_mint: solana_pubkey::Pubkey,
    pub owner_ata_acc: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub tcomp_program: solana_pubkey::Pubkey,
    pub tensorswap_program: solana_pubkey::Pubkey,
    pub cosigner: solana_pubkey::Pubkey,
    pub mint_proof: solana_pubkey::Pubkey,
    pub rent_dest: solana_pubkey::Pubkey,
    pub approve_account: solana_pubkey::Pubkey,
    pub distribution: solana_pubkey::Pubkey,
    pub wns_program: solana_pubkey::Pubkey,
    pub distribution_program: solana_pubkey::Pubkey,
    pub extra_metas: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for TakeBidWns {
    type ArrangedAccounts = TakeBidWnsInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let tcomp = next_account(&mut iter)?;
        let seller = next_account(&mut iter)?;
        let bid_state = next_account(&mut iter)?;
        let owner = next_account(&mut iter)?;
        let taker_broker = next_account(&mut iter);
        let maker_broker = next_account(&mut iter);
        let margin_account = next_account(&mut iter)?;
        let whitelist = next_account(&mut iter)?;
        let nft_seller_acc = next_account(&mut iter)?;
        let nft_mint = next_account(&mut iter)?;
        let owner_ata_acc = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let associated_token_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let tcomp_program = next_account(&mut iter)?;
        let tensorswap_program = next_account(&mut iter)?;
        let cosigner = next_account(&mut iter)?;
        let mint_proof = next_account(&mut iter)?;
        let rent_dest = next_account(&mut iter)?;
        let approve_account = next_account(&mut iter)?;
        let distribution = next_account(&mut iter)?;
        let wns_program = next_account(&mut iter)?;
        let distribution_program = next_account(&mut iter)?;
        let extra_metas = next_account(&mut iter)?;

        Some(TakeBidWnsInstructionAccounts {
            tcomp,
            seller,
            bid_state,
            owner,
            taker_broker,
            maker_broker,
            margin_account,
            whitelist,
            nft_seller_acc,
            nft_mint,
            owner_ata_acc,
            token_program,
            associated_token_program,
            system_program,
            tcomp_program,
            tensorswap_program,
            cosigner,
            mint_proof,
            rent_dest,
            approve_account,
            distribution,
            wns_program,
            distribution_program,
            extra_metas,
        })
    }
}