use carbon_core::borsh::{self, BorshDeserialize};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CargoPod {
    pub version: u8,
    pub stats_definition: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub open_token_accounts: u8,
    pub pod_seeds: [u8; 32],
    pub pod_bump: u8,
    pub seq_id: u16,
    pub unupdated_token_accounts: u8,
    pub cargo_contents: Vec<u64>,
}

impl borsh::de::BorshDeserialize for CargoPod
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
            authority: borsh::BorshDeserialize::deserialize_reader(reader)?,
            open_token_accounts: borsh::BorshDeserialize::deserialize_reader(reader)?,
            pod_seeds: borsh::BorshDeserialize::deserialize_reader(reader)?,
            pod_bump: borsh::BorshDeserialize::deserialize_reader(reader)?,
            seq_id: borsh::BorshDeserialize::deserialize_reader(reader)?,
            unupdated_token_accounts: borsh::BorshDeserialize::deserialize_reader(reader)?,
            cargo_contents: Vec::new(),
        })
    }
}

#[automatically_derived]
impl carbon_core::deserialize::CarbonDeserialize for CargoPod {
    const DISCRIMINATOR: &'static [u8] = &[165u8, 33u8, 118u8, 235u8, 252u8, 188u8, 244u8, 93u8];
    fn deserialize(data: &[u8]) -> Option<Self> {
        if data.len() < Self::DISCRIMINATOR.len() {
            return None;
        }

        let (disc, mut rest) = data.split_at(Self::DISCRIMINATOR.len());

        if disc != Self::DISCRIMINATOR {
            return None;
        }

        // CargoPod has RemainingData = List<PackedValue<u64>>
        // Contains the contents of the cargo pod as a list of u64 (no length prefix in account)
        // Byte layout: version(1) + stats_definition(32) + authority(32) + open_token_accounts(1) + pod_seeds(32) + pod_bump(1) + seq_id(2) + unupdated_token_accounts(1) = 102 bytes
        // All remaining bytes after fixed fields are u64 cargo values

        let cargo_pod: CargoPod = match BorshDeserialize::deserialize(&mut rest) {
            Ok(res) => res,
            Err(_) => return None,
        };

        // Read all remaining bytes as u64 values (each u64 is 8 bytes)
        let num_u64_values = rest.len() / 8;
        let mut cargo_contents = Vec::with_capacity(num_u64_values);

        for _ in 0..num_u64_values {
            match BorshDeserialize::deserialize(&mut rest) {
                Ok(value) => cargo_contents.push(value),
                Err(_) => return None,
            }
        }

        let mut final_cargo_pod = cargo_pod;
        final_cargo_pod.cargo_contents = cargo_contents;

        if !rest.is_empty() {
            carbon_core::log::debug!(
                "Not all bytes were read when deserializing {}: {} bytes remaining",
                stringify!(CargoPod),
                rest.len(),
            );
        }

        Some(final_cargo_pod)
    }
}
