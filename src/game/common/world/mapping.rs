use std::hash::{BuildHasher, Hasher};

#[derive(Default)]
pub struct ChunkPosHasher {
    hash: u64,
}

impl Hasher for ChunkPosHasher {
    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.hash ^= u64::from(byte).wrapping_mul(0x9e3779b97f4a7c15);
            self.hash = self.hash.rotate_left(5);
        }
    }

    fn write_u64(&mut self, i: u64) {
        self.hash ^= i.wrapping_mul(0x9e3779b97f4a7c15);
        self.hash = self.hash.rotate_left(5);
    }

    fn finish(&self) -> u64 {
        self.hash
    }
}

#[derive(Default)]
pub struct ChunkPosHasherBuilder;

impl BuildHasher for ChunkPosHasherBuilder {
    type Hasher = ChunkPosHasher;

    fn build_hasher(&self) -> Self::Hasher {
        ChunkPosHasher::default()
    }
}