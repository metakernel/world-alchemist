use rand::{CryptoRng, RngCore, SeedableRng};
use rand_chacha::ChaCha12Rng;

#[derive(Clone, Debug)]
pub struct DetRng(ChaCha12Rng);

impl DetRng {
    pub fn from_seed_u64(seed: u64) -> Self {
        let mut s = [0u8; 32];
        s[..8].copy_from_slice(&seed.to_le_bytes());
        Self(ChaCha12Rng::from_seed(s))
    }

    // Deterministically splits the RNG into two independent RNGs
    pub fn split(&mut self, tag: u64) -> Self {
        let mut seed = self.next_u64() ^ tag;
        DetRng::from_seed_u64(seed)
    }
}

impl RngCore for DetRng {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.0.next_u32()
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        self.0.next_u64()
    }

    #[inline]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.0.fill_bytes(dest)
    }
}

impl CryptoRng for DetRng {}
