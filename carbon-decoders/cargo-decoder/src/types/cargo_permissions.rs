use bitflags::bitflags;

bitflags! {
    /// Permissions for the cargo program.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct CargoPermissions: u64 {
        /// Key can init and update cargo definitions.
        const MANAGE_DEFINITION = 1 << 0;
        /// Key can create cargo types.
        const CREATE_CARGO_TYPE = 1 << 1;
        /// Key can update cargo types.
        const MANAGE_CARGO_TYPE = 1 << 2;
    }
}

impl CargoPermissions {
    /// Create CargoPermissions from a u64 value, truncating any unknown bits.
    pub fn from_u64(bits: u64) -> Self {
        Self::from_bits_truncate(bits)
    }

    /// Convert CargoPermissions to a u64 value.
    pub fn to_u64(self) -> u64 {
        self.bits()
    }

    /// Create CargoPermissions from little-endian bytes.
    pub fn from_le_bytes(bytes: [u8; 8]) -> Self {
        Self::from_u64(u64::from_le_bytes(bytes))
    }

    /// Convert CargoPermissions to little-endian bytes.
    pub fn to_le_bytes(self) -> [u8; 8] {
        self.to_u64().to_le_bytes()
    }
}
