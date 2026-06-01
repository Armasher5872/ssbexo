//Credit to HDR
#[repr(C)]
pub struct HashedString {
    pub length: u32,
    pub hash: u32,
    pub string: [u8; 64],
}