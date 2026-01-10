
#[repr(transparent)]
#[derive(Clone,Copy,Debug,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub struct Hash128(u128);

#[repr(transparent)]
#[derive(Clone,Copy,Debug,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub struct Hash64(u64);

pub fn hash128(data: &[u8]) -> Hash128 {
    let hash = blake3::hash(data);
    let bytes = hash.as_bytes();
    let mut arr = [0u8; 16];
    arr.copy_from_slice(&bytes[0..16]);
    Hash128(u128::from_le_bytes(arr))
}

pub fn hash64(data: &[u8]) -> Hash64 {
    let hash = blake3::hash(data);
    let bytes = hash.as_bytes();
    let mut arr = [0u8; 8];
    arr.copy_from_slice(&bytes[0..8]);
    Hash64(u64::from_le_bytes(arr))
}

pub fn hash_str128(s: &str) -> Hash128 {
    hash128(s.as_bytes())
}

pub fn hash_str64(s: &str) -> Hash64 {
    hash64(s.as_bytes())
}