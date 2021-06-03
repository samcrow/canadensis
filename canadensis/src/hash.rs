use hash32::{BuildHasherDefault, Hasher};
use heapless::IndexMap;

/// Map that uses a trivial hash
pub type TrivialIndexMap<K, V, const N: usize> =
    IndexMap<K, V, BuildHasherDefault<TrivialHasher>, N>;

/// A very simple hasher that only performs well on keys that are smaller than 32 bits
#[derive(Default)]
pub struct TrivialHasher(u32);

impl Hasher for TrivialHasher {
    fn finish(&self) -> u32 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.0 = (self.0 << 8) | u32::from(byte);
        }
    }
}
