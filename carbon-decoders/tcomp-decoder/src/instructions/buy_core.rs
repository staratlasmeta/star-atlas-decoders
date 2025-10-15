use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa9e357ff4c56ff19")]
pub struct BuyCore {
    pub max_amount: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct BuyCoreInstructionAccounts {
    pub tcomp: solana_pubkey::Pubkey,
    pub list_state: solana_pubkey::Pubkey,
    pub asset: solana_pubkey::Pubkey,
    pub collection: Option<solana_pubkey::Pubkey>,
    pub buyer: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub taker_broker: Option<solana_pubkey::Pubkey>,
    pub maker_broker: Option<solana_pubkey::Pubkey>,
    pub rent_dest: solana_pubkey::Pubkey,
    pub mpl_core_program: solana_pubkey::Pubkey,
    pub tcomp_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for BuyCore {
    type ArrangedAccounts = BuyCoreInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let tcomp = next_account(&mut iter)?;
        let list_state = next_account(&mut iter)?;
        let asset = next_account(&mut iter)?;
        let collection = next_account(&mut iter);
        let buyer = next_account(&mut iter)?;
        let payer = next_account(&mut iter)?;
        let owner = next_account(&mut iter)?;
        let taker_broker = next_account(&mut iter);
        let maker_broker = next_account(&mut iter);
        let rent_dest = next_account(&mut iter)?;
        let mpl_core_program = next_account(&mut iter)?;
        let tcomp_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(BuyCoreInstructionAccounts {
            tcomp,
            list_state,
            asset,
            collection,
            buyer,
            payer,
            owner,
            taker_broker,
            maker_broker,
            rent_dest,
            mpl_core_program,
            tcomp_program,
            system_program,
        })
    }
}
