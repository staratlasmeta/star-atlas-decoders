

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x12fa71f21ff41396")]
pub struct TakeBidT22{
    pub min_amount: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct TakeBidT22InstructionAccounts {
    pub tcomp: solana_pubkey::Pubkey,
    pub seller: solana_pubkey::Pubkey,
    pub bid_state: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub taker_broker: solana_pubkey::Pubkey,
    pub maker_broker: solana_pubkey::Pubkey,
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
}

impl carbon_core::deserialize::ArrangeAccounts for TakeBidT22 {
    type ArrangedAccounts = TakeBidT22InstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
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
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(TakeBidT22InstructionAccounts {
            tcomp: tcomp.pubkey,
            seller: seller.pubkey,
            bid_state: bid_state.pubkey,
            owner: owner.pubkey,
            taker_broker: taker_broker.pubkey,
            maker_broker: maker_broker.pubkey,
            margin_account: margin_account.pubkey,
            whitelist: whitelist.pubkey,
            nft_seller_acc: nft_seller_acc.pubkey,
            nft_mint: nft_mint.pubkey,
            owner_ata_acc: owner_ata_acc.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            system_program: system_program.pubkey,
            tcomp_program: tcomp_program.pubkey,
            tensorswap_program: tensorswap_program.pubkey,
            cosigner: cosigner.pubkey,
            mint_proof: mint_proof.pubkey,
            rent_dest: rent_dest.pubkey,
        })
    }
}