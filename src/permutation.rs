use std::hash::BuildHasher;

use crate::feistel::{DefaultBuildHasher, Feistel};

pub struct Permutation<B = DefaultBuildHasher>
where
    B: BuildHasher,
{
    n: u64,
    feistel: Feistel<B>,
}

impl<B> Permutation<B>
where
    B: BuildHasher,
{
    pub fn new(n: u64, seed: u64, bob: B) -> Permutation<B> {
        let mut keys = Vec::new();
        let mut k = seed;
        for _i in 0..5 {
            k = bob.hash_one(k);
            keys.push(k);
        }
        println!("keys = {:?}", keys);

        let bits = 64 - n.leading_zeros() as usize;

        Permutation {
            n,
            feistel: Feistel::new(bob, bits, &keys),
        }
    }

    pub fn get(&self, x: u64) -> u64 {
        assert!(x < self.n);
        let mut res = self.feistel.encrypt(x);
        while res >= self.n {
            res = self.feistel.encrypt(res);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use xxhash_rust::xxh64;

    use super::*;

    #[test]
    fn test_1() {
        let n = 1000;
        let bob = xxh64::Xxh64Builder::new(19);
        let perm = Permutation::new(n, 19, bob);
        let mut xs = Vec::new();
        for i in 0..n {
            let x = perm.get(i);
            //println!("{i}\t{x}");
            xs.push(x);
        }
        let mut lt = 0;
        for i in 1..n as usize {
            if xs[i-1] < xs[i] {
                lt += 1;
            }
        }
        assert_eq!(lt, n / 2);

        xs.sort();
        for i in 0..n {
            assert_eq!(xs[i as usize], i);
        }
    }
}
