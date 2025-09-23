use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum PlanetType {
    Terrestrial,
    Volcanic,
    Barren,
    AsteroidBelt,
    GasGiant,
    IceGiant,
    Dark,
}
