#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct AddrPacked {
    ip_address: [u8; 16],
    port: u16,
}

pub trait AddrPackedSliceExt {
    fn from_bytes<'a>(bytes: &'a [u8]) -> &'a Self;
    fn as_bytes(&self) -> &[u8];
}

impl AddrPackedSliceExt for [AddrPacked] {
    fn from_bytes<'a>(bytes: &'a [u8]) -> &'a [AddrPacked] {
        let remainder = bytes.len() % std::mem::size_of::<AddrPacked>();

        let actual_len = bytes.len() - remainder;
        unsafe { std::mem::transmute(&bytes[..actual_len]) }
    }
    fn as_bytes(&self) -> &[u8] {
        unsafe { std::mem::transmute(self) }
    }
}
