// SPDX-License-Identifier: Apache-2.0
use std::hash::BuildHasher;

use crate::{Feistel, DefaultBuildHasher};

/// An object of constructing random access permutations.
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
    /// Construct a new permutation over the range `0..n`.
    pub fn new(n: u64, seed: u64, bob: B) -> Permutation<B> {
        let mut keys = Vec::new();
        let mut k = seed;
        for _i in 0..5 {
            k = bob.hash_one(k);
            keys.push(k);
        }
        //println!("keys = {:?}", keys);

        // Code assumes an even number of bits. Rounding up
        // increases the constant factor in [`get`] but doesn't
        // alter the big-O complexity.
        let z = 64 - n.leading_zeros() as usize;
        let bits = z + (z & 1);

        Permutation {
            n,
            feistel: Feistel::new(bob, bits, &keys),
        }
    }

    /// Get the xth element of the permutation.
    pub fn get(&self, x: u64) -> u64 {
        assert!(x < self.n);
        let mut res = self.feistel.encrypt(x);
        while res >= self.n {
            res = self.feistel.encrypt(res);
        }
        res
    }

    /// Construct an iterator over the entire permutation.
    pub fn iter(&self) -> PermutationIterator<'_, B> {
        PermutationIterator::new(self, 0, self.n)
    }

    /// Construct an iterator over the subset `begin..end` of the permutation.
    pub fn range(&self, begin: u64, end: u64) -> PermutationIterator<'_, B> {
        assert!(begin <= end);
        assert!(end <= self.n);
        PermutationIterator::new(self, begin, end)
    }
}

/// An iterator over a [`Permutation`] object.
pub struct PermutationIterator<'a, B>
where
    B: BuildHasher,
{
    source: &'a Permutation<B>,
    curr: u64,
    end: u64,
}

impl<'a, B> PermutationIterator<'a, B>
where
    B: BuildHasher,
{
    fn new(source: &'a Permutation<B>, begin: u64, end: u64) -> PermutationIterator<'a, B> {
        PermutationIterator {
            source,
            curr: begin,
            end,
        }
    }
}

impl<'a, B> Iterator for PermutationIterator<'a, B>
where
    B: BuildHasher,
{
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr == self.end {
            None
        } else {
            let res = self.source.get(self.curr);
            self.curr += 1;
            Some(res)
        }
    }
}

#[cfg(test)]
mod tests {
    use xxhash_rust::xxh64;

    use super::*;

    #[test]
    fn test_1() {
        let n = 1002;
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
            if xs[i - 1] < xs[i] {
                lt += 1;
            }
        }
        assert_eq!(lt, n / 2);

        xs.sort();
        for i in 0..n {
            assert_eq!(xs[i as usize], i);
        }
    }

    #[test]
    fn test_2() {
        let n = 1000000;
        let seed = 29;
        let perm = Permutation::new(n, seed, DefaultBuildHasher::new());
        for j in perm.range(100, 200) {
            println!("{}", j);
        }
        for j in perm.iter().take(10) {
            println!("{}", j);
        }
    }
}
