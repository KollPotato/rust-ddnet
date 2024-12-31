#[derive(Clone, Copy, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct Sha256([u8; 32]);

impl Sha256 {
    pub fn from_slice(bytes: &[u8]) -> Self {
        Sha256(bytes.try_into().expect("not enough bytes for sha256"))
    }
}
