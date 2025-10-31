use crate::types::RedemptionEpoch;
use carbon_core::borsh::{self, BorshDeserialize};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct RedemptionConfig {
    pub version: u8,
    pub point_category: solana_pubkey::Pubkey,
    pub profile: solana_pubkey::Pubkey,
    pub faction: u8,
    pub bank: solana_pubkey::Pubkey,
    pub signer_bump: u8,
    pub allow_only_current_epoch: u8,
    pub redemption_epochs: Vec<RedemptionEpoch>,
}

impl borsh::de::BorshDeserialize for RedemptionConfig
where
    u8: borsh::BorshDeserialize,
    solana_pubkey::Pubkey: borsh::BorshDeserialize,
{
    fn deserialize_reader<R: borsh::maybestd::io::Read>(
        reader: &mut R,
    ) -> Result<Self, borsh::maybestd::io::Error> {
        Ok(Self {
            version: borsh::BorshDeserialize::deserialize_reader(reader)?,
            point_category: borsh::BorshDeserialize::deserialize_reader(reader)?,
            profile: borsh::BorshDeserialize::deserialize_reader(reader)?,
            faction: borsh::BorshDeserialize::deserialize_reader(reader)?,
            bank: borsh::BorshDeserialize::deserialize_reader(reader)?,
            signer_bump: borsh::BorshDeserialize::deserialize_reader(reader)?,
            allow_only_current_epoch: borsh::BorshDeserialize::deserialize_reader(reader)?,
            redemption_epochs: Vec::new(),
        })
    }
}

#[automatically_derived]
impl carbon_core::deserialize::CarbonDeserialize for RedemptionConfig {
    const DISCRIMINATOR: &'static [u8] = &[173u8, 1u8, 86u8, 47u8, 27u8, 204u8, 146u8, 185u8];
    fn deserialize(data: &[u8]) -> Option<Self> {
        if data.len() < Self::DISCRIMINATOR.len() {
            return None;
        }

        let (disc, mut rest) = data.split_at(Self::DISCRIMINATOR.len());

        if disc != Self::DISCRIMINATOR {
            return None;
        }

        // RedemptionConfig has RemainingData = UnorderedList<RedemptionEpoch, u32>
        // Contains a u32 length prefix followed by that many RedemptionEpoch structs
        // Byte layout: version(1) + point_category(32) + profile(32) + faction(1) +
        //              bank(32) + signer_bump(1) + allow_only_current_epoch(1) = 100 bytes
        // After fixed fields: u32 length + (length * RedemptionEpoch)

        let redemption_config: RedemptionConfig = match BorshDeserialize::deserialize(&mut rest) {
            Ok(res) => res,
            Err(_) => return None,
        };

        // Read u32 length prefix
        let epochs_count: u32 = match BorshDeserialize::deserialize(&mut rest) {
            Ok(count) => count,
            Err(_) => return None,
        };

        let mut redemption_epochs = Vec::with_capacity(epochs_count as usize);

        for _ in 0..epochs_count {
            match BorshDeserialize::deserialize(&mut rest) {
                Ok(epoch) => redemption_epochs.push(epoch),
                Err(_) => return None,
            }
        }

        let mut final_redemption_config = redemption_config;
        final_redemption_config.redemption_epochs = redemption_epochs;

        if !rest.is_empty() {
            carbon_core::log::debug!(
                "Not all bytes were read when deserializing {}: {} bytes remaining",
                stringify!(RedemptionConfig),
                rest.len(),
            );
        }

        Some(final_redemption_config)
    }
}
