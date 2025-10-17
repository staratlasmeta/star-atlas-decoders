use carbon_core::account::AccountDecoder;
use carbon_core::deserialize::CarbonDeserialize;

use super::CraftingDecoder;
pub mod craftable_item;
pub mod crafting_facility;
pub mod crafting_process;
pub mod domain;
pub mod recipe;
pub mod recipe_category;

#[derive(Debug, serde::Serialize)]
pub enum CraftingAccount {
    CraftableItem(craftable_item::CraftableItem),
    CraftingFacility(crafting_facility::CraftingFacility),
    CraftingProcess(crafting_process::CraftingProcess),
    Domain(domain::Domain),
    Recipe(recipe::Recipe),
    RecipeCategory(recipe_category::RecipeCategory),
}

impl<'a> AccountDecoder<'a> for CraftingDecoder {
    type AccountType = CraftingAccount;
    fn decode_account(
        &self,
        account: &solana_account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if let Some(decoded_account) =
            craftable_item::CraftableItem::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: CraftingAccount::CraftableItem(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            crafting_facility::CraftingFacility::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: CraftingAccount::CraftingFacility(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            crafting_process::CraftingProcess::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: CraftingAccount::CraftingProcess(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = domain::Domain::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: CraftingAccount::Domain(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = recipe::Recipe::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: CraftingAccount::Recipe(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            recipe_category::RecipeCategory::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: CraftingAccount::RecipeCategory(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
