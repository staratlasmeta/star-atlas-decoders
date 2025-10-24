use carbon_core::borsh::{self, BorshDeserialize};

use crate::types::LocationType;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CraftingFacility {
    pub version: u8,
    pub domain: solana_pubkey::Pubkey,
    pub location: solana_pubkey::Pubkey,
    pub location_type: LocationType,
    pub max_concurrent_processes: u32,
    pub num_concurrent_processes: u32,
    pub efficiency: u32,
    pub num_recipe_categories: u32,
    pub recipe_categories: Vec<solana_pubkey::Pubkey>,
}

impl borsh::de::BorshDeserialize for CraftingFacility
where
    u8: borsh::BorshDeserialize,
    solana_pubkey::Pubkey: borsh::BorshDeserialize,
    LocationType: borsh::BorshDeserialize,
    u32: borsh::BorshDeserialize,
{
    fn deserialize_reader<R: borsh::maybestd::io::Read>(
        reader: &mut R,
    ) -> Result<Self, borsh::maybestd::io::Error> {
        Ok(Self {
            version: borsh::BorshDeserialize::deserialize_reader(reader)?,
            domain: borsh::BorshDeserialize::deserialize_reader(reader)?,
            location: borsh::BorshDeserialize::deserialize_reader(reader)?,
            location_type: borsh::BorshDeserialize::deserialize_reader(reader)?,
            max_concurrent_processes: borsh::BorshDeserialize::deserialize_reader(reader)?,
            num_concurrent_processes: borsh::BorshDeserialize::deserialize_reader(reader)?,
            efficiency: borsh::BorshDeserialize::deserialize_reader(reader)?,
            num_recipe_categories: borsh::BorshDeserialize::deserialize_reader(reader)?,
            recipe_categories: Vec::new(),
        })
    }
}

#[automatically_derived]
impl carbon_core::deserialize::CarbonDeserialize for CraftingFacility {
    const DISCRIMINATOR: &'static [u8] = &[58u8, 73u8, 35u8, 17u8, 92u8, 247u8, 49u8, 30u8];
    fn deserialize(data: &[u8]) -> Option<Self> {
        if data.len() < Self::DISCRIMINATOR.len() {
            return None;
        }

        let (disc, mut rest) = data.split_at(Self::DISCRIMINATOR.len());

        if disc != Self::DISCRIMINATOR {
            return None;
        }

        // CraftingFacility has RemainingData = UnorderedList<WrappedRecipeCategory>
        // Contains a list of recipe category pubkeys (no length prefix in account)
        // Byte layout: version(1) + domain(32) + location(32) + location_type(1) + max_concurrent_processes(4) + num_concurrent_processes(4) + efficiency(4) + num_recipe_categories(4) = 82 bytes
        // All remaining bytes after fixed fields are Pubkeys (32 bytes each)

        let crafting_facility: CraftingFacility = match BorshDeserialize::deserialize(&mut rest) {
            Ok(res) => res,
            Err(_) => return None,
        };

        // Read num_recipe_categories pubkeys from remaining data
        let num_categories = crafting_facility.num_recipe_categories as usize;
        let mut recipe_categories = Vec::with_capacity(num_categories);

        for _ in 0..num_categories {
            match BorshDeserialize::deserialize(&mut rest) {
                Ok(pubkey) => recipe_categories.push(pubkey),
                Err(_) => return None,
            }
        }

        let mut final_crafting_facility = crafting_facility;
        final_crafting_facility.recipe_categories = recipe_categories;

        if !rest.is_empty() {
            carbon_core::log::debug!(
                "Not all bytes were read when deserializing {}: {} bytes remaining",
                stringify!(CraftingFacility),
                rest.len(),
            );
        }

        Some(final_crafting_facility)
    }
}
