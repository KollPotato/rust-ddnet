use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Uuid([u8; 16]);

impl Uuid {
    pub const fn from_u128(value: u128) -> Self {
        Self(value.to_le_bytes())
    }

    pub fn from_slice(bytes: &[u8]) -> Self {
        Uuid(bytes.try_into().expect("invalid uuid"))
    }

    pub fn as_bytes(&self) -> &[u8; 16] {
        &self.0
    }
}
