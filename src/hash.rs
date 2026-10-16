//! A copy of the 64-bit `FxHasher` of `rustc-hash` 2.1.3 (MIT or Apache-2.0, by the Rust
//! Project Developers), with the state fixed to `u64` and every integer write widened to `u64`.
//! `rustc-hash` adjusts its state and constants to the pointer width, so wasm32 and the server
//! would hash the same view differently, which breaks subtree skipping in cabin.js and duplicates
//! styles.

use std::hash::Hasher;

const K: u64 = 0xf1357aea2e62a9c5;

// Nothing special, digits of pi.
const SEED1: u64 = 0x243f6a8885a308d3;
const SEED2: u64 = 0x13198a2e03707344;
const PREVENT_TRIVIAL_ZERO_COLLAPSE: u64 = 0xa4093822299f31d0;

#[derive(Default, Clone)]
pub struct FxHasher {
    hash: u64,
}

impl FxHasher {
    #[inline]
    fn add_to_hash(&mut self, i: u64) {
        self.hash = self.hash.wrapping_add(i).wrapping_mul(K);
    }

    /// Folds the hash into 32 bits. The lower bits of the polynomial hash are weak, so they are
    /// mixed with the upper half instead of being truncated.
    #[inline]
    pub fn finish_u32(&self) -> u32 {
        let hash = self.finish();
        (hash >> 32) as u32 ^ hash as u32
    }
}

impl Hasher for FxHasher {
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        self.write_u64(hash_bytes(bytes));
    }

    #[inline]
    fn write_u8(&mut self, i: u8) {
        self.add_to_hash(i as u64);
    }

    #[inline]
    fn write_u16(&mut self, i: u16) {
        self.add_to_hash(i as u64);
    }

    #[inline]
    fn write_u32(&mut self, i: u32) {
        self.add_to_hash(i as u64);
    }

    #[inline]
    fn write_u64(&mut self, i: u64) {
        self.add_to_hash(i);
    }

    #[inline]
    fn write_u128(&mut self, i: u128) {
        self.add_to_hash(i as u64);
        self.add_to_hash((i >> 64) as u64);
    }

    #[inline]
    fn write_usize(&mut self, i: usize) {
        self.add_to_hash(i as u64);
    }

    #[inline]
    fn write_isize(&mut self, i: isize) {
        self.add_to_hash(i as i64 as u64);
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.hash.rotate_left(26)
    }
}

#[inline]
fn multiply_mix(x: u64, y: u64) -> u64 {
    // Always the full u64 x u64 -> u128 product, also on wasm32, where it is slower but gives the
    // same result as on the server.
    let full = (x as u128).wrapping_mul(y as u128);
    let lo = full as u64;
    let hi = (full >> 64) as u64;
    lo ^ hi
}

/// A wyhash-inspired non-collision-resistant hash for strings/slices designed by Orson Peters.
#[inline]
fn hash_bytes(bytes: &[u8]) -> u64 {
    let len = bytes.len();
    let mut s0 = SEED1;
    let mut s1 = SEED2;

    if len <= 16 {
        // XOR the input into s0, s1.
        if len >= 8 {
            s0 ^= u64::from_le_bytes(bytes[0..8].try_into().unwrap());
            s1 ^= u64::from_le_bytes(bytes[len - 8..].try_into().unwrap());
        } else if len >= 4 {
            s0 ^= u32::from_le_bytes(bytes[0..4].try_into().unwrap()) as u64;
            s1 ^= u32::from_le_bytes(bytes[len - 4..].try_into().unwrap()) as u64;
        } else if len > 0 {
            let lo = bytes[0];
            let mid = bytes[len / 2];
            let hi = bytes[len - 1];
            s0 ^= lo as u64;
            s1 ^= ((hi as u64) << 8) | mid as u64;
        }
    } else {
        // Handle bulk (can partially overlap with suffix).
        let mut bulk = &bytes[..(len - 1)];
        while let Some((chunk, rest)) = bulk.split_first_chunk::<16>() {
            let x = u64::from_le_bytes((&chunk[..8]).try_into().unwrap());
            let y = u64::from_le_bytes((&chunk[8..]).try_into().unwrap());

            // Since zeroes are a common input we prevent an immediate trivial collapse of the
            // hash function by XOR'ing a constant with y.
            let t = multiply_mix(s0 ^ x, PREVENT_TRIVIAL_ZERO_COLLAPSE ^ y);
            s0 = s1;
            s1 = t;
            bulk = rest;
        }

        let suffix = &bytes[len - 16..];
        s0 ^= u64::from_le_bytes(suffix[0..8].try_into().unwrap());
        s1 ^= u64::from_le_bytes(suffix[8..16].try_into().unwrap());
    }

    multiply_mix(s0, s1) ^ (len as u64)
}

#[cfg(test)]
mod tests {
    use std::hash::{Hash, Hasher as _};

    use super::FxHasher;

    // Same output as `rustc_hash::FxHasher` on 64-bit, pinned so wasm32 is known to match.
    #[test]
    fn stable_output() {
        let mut h = FxHasher::default();
        (
            1u8,
            2u32,
            -3isize,
            4usize,
            "short",
            "a string longer than sixteen bytes",
        )
            .hash(&mut h);
        assert_eq!(h.finish(), EXPECTED);
    }

    const EXPECTED: u64 = 0xba22ce7c7f0a4aa5;
}
