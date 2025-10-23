use carbon_core::borsh::{self, BorshDeserialize};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CargoType {
    pub version: u8,
    pub stats_definition: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub creator: solana_pubkey::Pubkey,
    pub bump: u8,
    pub stats_count: u16,
    pub seq_id: u16,
    pub cargo_stats: Vec<u64>,
}

impl borsh::de::BorshDeserialize for CargoType
where
    u8: borsh::BorshDeserialize,
    solana_pubkey::Pubkey: borsh::BorshDeserialize,
    u16: borsh::BorshDeserialize,
{
    fn deserialize_reader<R: borsh::maybestd::io::Read>(
        reader: &mut R,
    ) -> Result<Self, borsh::maybestd::io::Error> {
        Ok(Self {
            version: borsh::BorshDeserialize::deserialize_reader(reader)?,
            stats_definition: borsh::BorshDeserialize::deserialize_reader(reader)?,
            mint: borsh::BorshDeserialize::deserialize_reader(reader)?,
            creator: borsh::BorshDeserialize::deserialize_reader(reader)?,
            bump: borsh::BorshDeserialize::deserialize_reader(reader)?,
            stats_count: borsh::BorshDeserialize::deserialize_reader(reader)?,
            seq_id: borsh::BorshDeserialize::deserialize_reader(reader)?,
            cargo_stats: Vec::new(),
        })
    }
}

#[automatically_derived]
impl carbon_core::deserialize::CarbonDeserialize for CargoType {
    const DISCRIMINATOR: &'static [u8] = &[152u8, 102u8, 188u8, 230u8, 200u8, 14u8, 164u8, 224u8];
    fn deserialize(data: &[u8]) -> Option<Self> {
        if data.len() < Self::DISCRIMINATOR.len() {
            return None;
        }

        let (disc, mut rest) = data.split_at(Self::DISCRIMINATOR.len());

        if disc != Self::DISCRIMINATOR {
            return None;
        }

        // CargoType has RemainingData = List<PackedValue<u64>>
        // The stats_count field (u16) indicates how many u64 values follow in remaining data
        // Byte layout: version(1) + stats_definition(32) + mint(32) + creator(32) + bump(1) + stats_count(2) + seq_id(2) = 102 bytes
        // After fixed fields: stats_count u64 values (8 bytes each)

        let cargo_type: CargoType = match BorshDeserialize::deserialize(&mut rest) {
            Ok(res) => res,
            Err(_) => return None,
        };

        let stats_count = cargo_type.stats_count as usize;
        let mut cargo_stats = Vec::with_capacity(stats_count);

        for _ in 0..stats_count {
            match BorshDeserialize::deserialize(&mut rest) {
                Ok(stat) => cargo_stats.push(stat),
                Err(_) => return None,
            }
        }

        let mut final_cargo_type = cargo_type;
        final_cargo_type.cargo_stats = cargo_stats;

        if !rest.is_empty() {
            carbon_core::log::debug!(
                "Not all bytes were read when deserializing {}: {} bytes remaining",
                stringify!(CargoType),
                rest.len(),
            );
        }

        Some(final_cargo_type)
    }
}
