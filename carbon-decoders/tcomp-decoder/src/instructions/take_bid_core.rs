

use carbon_core::{CarbonDeserialize, borsh, account_utils::next_account};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xfa29f8143da11b8d")]
pub struct TakeBidCore{
    pub min_amount: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct TakeBidCoreInstructionAccounts {
    pub tcomp: solana_pubkey::Pubkey,
    pub seller: solana_pubkey::Pubkey,
    pub bid_state: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub taker_broker: Option<solana_pubkey::Pubkey>,
    pub maker_broker: Option<solana_pubkey::Pubkey>,
    pub margin_account: solana_pubkey::Pubkey,
    pub whitelist: solana_pubkey::Pubkey,
    pub asset: solana_pubkey::Pubkey,
    pub collection: Option<solana_pubkey::Pubkey>,
    pub mpl_core_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub tcomp_program: solana_pubkey::Pubkey,
    pub tensorswap_program: solana_pubkey::Pubkey,
    pub cosigner: solana_pubkey::Pubkey,
    pub mint_proof: solana_pubkey::Pubkey,
    pub rent_dest: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for TakeBidCore {
    type ArrangedAccounts = TakeBidCoreInstructionAccounts;

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
        let asset = next_account(&mut iter)?;
        let collection = next_account(&mut iter);
        let mpl_core_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let tcomp_program = next_account(&mut iter)?;
        let tensorswap_program = next_account(&mut iter)?;
        let cosigner = next_account(&mut iter)?;
        let mint_proof = next_account(&mut iter)?;
        let rent_dest = next_account(&mut iter)?;

        Some(TakeBidCoreInstructionAccounts {
            tcomp,
            seller,
            bid_state,
            owner,
            taker_broker,
            maker_broker,
            margin_account,
            whitelist,
            asset,
            collection,
            mpl_core_program,
            system_program,
            tcomp_program,
            tensorswap_program,
            cosigner,
            mint_proof,
            rent_dest,
        })
    }
}