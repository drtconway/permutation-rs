use std::hash::{BuildHasher, Hasher};


/// A basic Feistel Network cipher.
/// 
/// The cipher requires a series of hashes which are built using
/// a supplied [`std::hash::BuildHasher`] (passed with the parameter
/// name `bob`, coz it'z a builder, right?)
pub struct Feistel<B>
where
    B: BuildHasher,
{
    bob: B,
    bits: usize,
    keys: Vec<u64>,
}

impl<B> Feistel<B>
where
    B: BuildHasher,
{
    /// Construct a new Feistel cipher.
    pub fn new(bob: B, bits: usize, keys: &[u64]) -> Feistel<B> {
        // Insist that there are an even number of bits.
        assert_eq!(bits & 1, 0);
        Feistel {
            bob,
            bits,
            keys: Vec::from(keys),
        }
    }

    /// Encrypt a value.
    pub fn encrypt(&self, x: u64) -> u64 {
        let (mut l, mut r) = self.split(x);
        for k in self.keys.iter() {
            l ^= self.hash(*k, r);
            let t = l;
            l = r;
            r = t;
        }
        self.combine(r, l)
    }

    /// Decrypt a value.
    pub fn decrypt(&self, x: u64) -> u64 {
        let (mut l, mut r) = self.split(x);
        for k in self.keys.iter().rev() {
            l ^= self.hash(*k, r);
            let t = l;
            l = r;
            r = t;
        }
        self.combine(r, l)
    }

    fn split(&self, x: u64) -> (u64, u64) {
        let n = self.bits >> 1;
        let m = (1u64 << n) - 1;
        let hi = x >> n;
        let lo = x & m;
        (hi, lo)
    }

    fn combine(&self, hi: u64, lo: u64) -> u64 {
        let n = self.bits >> 1;
        (hi << n) | lo
    }

    fn hash(&self, k: u64, x: u64) -> u64 {
        let mut h: <B as BuildHasher>::Hasher = self.bob.build_hasher();
        h.write_u64(k);
        h.write_u64(x);
        let res = h.finish();
        let n = self.bits >> 1;
        let m = (1u64 << n) - 1;
        res & m
    }
}

#[cfg(test)]
mod tests {
    use crate::DefaultBuildHasher;

    use super::*;

    #[test]
    fn test_1a() {
        let bob = DefaultBuildHasher::new();
        let bits = 32;
        let keys = [0x1c10u64, 0x8fd6u64, 0x2d5au64, 0x7363u64, 0x5f70u64];
        let f = Feistel::new(bob, bits, &keys);
        let x = 17;
        let y = f.encrypt(x);
        let z = f.decrypt(y);
        println!("x=0x{x:0x}, y=0x{y:0x}, z=0x{z:0x}");
        assert_eq!(x, z);
    }

    #[test]
    fn test_1b() {
        let bob = DefaultBuildHasher::new();
        let bits = 32;
        let keys = [0x1c10u64, 0x8fd6u64, 0x2d5au64, 0x7363u64, 0x5f70u64];
        let f = Feistel::new(bob, bits, &keys);
        let x = 234;
        let y = f.encrypt(x);
        let z = f.decrypt(y);
        assert_eq!(x, z);
    }

    #[test]
    fn test_2() {
        let bob = DefaultBuildHasher::new();
        let bits = 56;
        let keys = [0x1c10u64, 0x8fd6u64, 0x2d5au64, 0x7363u64, 0x5f70u64];
        let f = Feistel::new(bob, bits, &keys);
        let x = 17;
        let y = f.encrypt(x);
        let z = f.decrypt(y);
        assert_eq!(x, z);
    }
}
