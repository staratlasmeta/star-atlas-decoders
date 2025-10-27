use super::super::types::*;

use crate::types::RecipeStatus;
use carbon_core::borsh::{self, BorshDeserialize};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Recipe {
    pub version: u8,
    pub domain: solana_pubkey::Pubkey,
    pub category: solana_pubkey::Pubkey,
    pub creator: solana_pubkey::Pubkey,
    pub duration: i64,
    pub min_duration: i64,
    pub namespace: [u8; 32],
    pub status: RecipeStatus,
    pub fee_amount: u64,
    pub fee_recipient: OptionalNonSystemPubkey,
    pub usage_count: u64,
    pub usage_limit: u64,
    pub value: u64,
    pub consumables_count: u8,
    pub non_consumables_count: u8,
    pub outputs_count: u8,
    pub total_count: u16,
    pub recipe_items: Vec<RecipeInputsOutputs>,
}

impl borsh::de::BorshDeserialize for Recipe
where
    u8: borsh::BorshDeserialize,
    solana_pubkey::Pubkey: borsh::BorshDeserialize,
    i64: borsh::BorshDeserialize,
    RecipeStatus: borsh::BorshDeserialize,
    u64: borsh::BorshDeserialize,
    OptionalNonSystemPubkey: borsh::BorshDeserialize,
    u16: borsh::BorshDeserialize,
{
    fn deserialize_reader<R: borsh::maybestd::io::Read>(
        reader: &mut R,
    ) -> Result<Self, borsh::maybestd::io::Error> {
        Ok(Self {
            version: borsh::BorshDeserialize::deserialize_reader(reader)?,
            domain: borsh::BorshDeserialize::deserialize_reader(reader)?,
            category: borsh::BorshDeserialize::deserialize_reader(reader)?,
            creator: borsh::BorshDeserialize::deserialize_reader(reader)?,
            duration: borsh::BorshDeserialize::deserialize_reader(reader)?,
            min_duration: borsh::BorshDeserialize::deserialize_reader(reader)?,
            namespace: borsh::BorshDeserialize::deserialize_reader(reader)?,
            status: borsh::BorshDeserialize::deserialize_reader(reader)?,
            fee_amount: borsh::BorshDeserialize::deserialize_reader(reader)?,
            fee_recipient: borsh::BorshDeserialize::deserialize_reader(reader)?,
            usage_count: borsh::BorshDeserialize::deserialize_reader(reader)?,
            usage_limit: borsh::BorshDeserialize::deserialize_reader(reader)?,
            value: borsh::BorshDeserialize::deserialize_reader(reader)?,
            consumables_count: borsh::BorshDeserialize::deserialize_reader(reader)?,
            non_consumables_count: borsh::BorshDeserialize::deserialize_reader(reader)?,
            outputs_count: borsh::BorshDeserialize::deserialize_reader(reader)?,
            total_count: borsh::BorshDeserialize::deserialize_reader(reader)?,
            recipe_items: Vec::new(),
        })
    }
}

#[automatically_derived]
impl carbon_core::deserialize::CarbonDeserialize for Recipe {
    const DISCRIMINATOR: &'static [u8] = &[10u8, 162u8, 156u8, 100u8, 56u8, 193u8, 205u8, 77u8];
    fn deserialize(data: &[u8]) -> Option<Self> {
        if data.len() < Self::DISCRIMINATOR.len() {
            return None;
        }

        let (disc, mut rest) = data.split_at(Self::DISCRIMINATOR.len());

        if disc != Self::DISCRIMINATOR {
            return None;
        }

        // Recipe has RemainingData = UnorderedList<RecipeInputsOutputs>
        // Contains recipe inputs/outputs with amount and mint for each item
        // Byte layout: version(1) + domain(32) + category(32) + creator(32) + duration(8) + min_duration(8) + namespace(32) + status(1) + fee_amount(8) + fee_recipient(33) + usage_count(8) + usage_limit(8) + value(8) + consumables_count(1) + non_consumables_count(1) + outputs_count(1) + total_count(2) = 216 bytes
        // After fixed fields: total_count RecipeInputsOutputs structs (40 bytes each: 8 bytes for u64 amount + 32 bytes for Pubkey mint)

        let recipe: Recipe = match BorshDeserialize::deserialize(&mut rest) {
            Ok(res) => res,
            Err(_) => return None,
        };

        // Read total_count RecipeInputsOutputs from remaining data
        let total_count = recipe.total_count as usize;
        let mut recipe_items = Vec::with_capacity(total_count);

        for _ in 0..total_count {
            match BorshDeserialize::deserialize(&mut rest) {
                Ok(item) => recipe_items.push(item),
                Err(_) => return None,
            }
        }

        let mut final_recipe = recipe;
        final_recipe.recipe_items = recipe_items;

        if !rest.is_empty() {
            carbon_core::log::debug!(
                "Not all bytes were read when deserializing {}: {} bytes remaining",
                stringify!(Recipe),
                rest.len(),
            );
        }

        Some(final_recipe)
    }
}
