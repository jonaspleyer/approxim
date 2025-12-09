//! This module defines a custom hasher, which can be used in a `#![no_std]` environment.
//!
//! The code is taken from [indexmap's `#![no_std]` integration test](https://github.com/indexmap-rs/indexmap/blob/main/test-nostd/src/lib.rs).
#[cfg(feature = "indexmap_impl")]
pub mod indexmap {
    use core::hash::BuildHasherDefault;
    use core::hash::Hasher;

    #[derive(Default)]
    pub struct BadHasher(u64);

    impl Hasher for BadHasher {
        fn finish(&self) -> u64 {
            self.0
        }

        fn write(&mut self, bytes: &[u8]) {
            for &byte in bytes {
                self.0 += byte as u64
            }
        }
    }

    pub type IndexMap<K, V> = indexmap::IndexMap<K, V, BuildHasherDefault<BadHasher>>;
}
