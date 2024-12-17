// SPDX-License-Identifier: Apache-2.0
use std::hash::BuildHasher;

use crate::{DefaultBuildHasher, Feistel};

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

    /// Transform the Permutation into an iterator over the subset `begin..end` of the permutation.
    pub fn into_range(self, begin: u64, end: u64) -> OwnedPermutationIterator<B> {
        assert!(begin <= end);
        assert!(end <= self.n);
        OwnedPermutationIterator::new(self, begin, end)
    }
}

impl<B: std::hash::BuildHasher> IntoIterator for Permutation<B> {
    type Item = u64;

    type IntoIter = OwnedPermutationIterator<B>;

    /// Transform the Permutation into an iterator.
    fn into_iter(self) -> Self::IntoIter {
        let end = self.n;
        OwnedPermutationIterator::new(self, 0, end)
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

/// An iterator over a [`Permutation`] object that owns the Permutation.
pub struct OwnedPermutationIterator<B>
where
    B: BuildHasher,
{
    source: Permutation<B>,
    curr: u64,
    end: u64,
}

impl<B> OwnedPermutationIterator<B>
where
    B: BuildHasher,
{
    fn new(source: Permutation<B>, begin: u64, end: u64) -> OwnedPermutationIterator<B> {
        OwnedPermutationIterator {
            source,
            curr: begin,
            end,
        }
    }
}

impl<B> Iterator for OwnedPermutationIterator<B>
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

    fn triangular_variance(a: f64, b: f64, c: f64) -> f64 {
        (a * a + b * b + c * c - a * b - a * c - b * c) / 18.0
    }

    fn triangular_sd(a: f64, b: f64, c: f64) -> f64 {
        triangular_variance(a, b, c).sqrt()
    }

    #[test]
    fn test_1() {
        let n = 1000;
        let nf = n as f64;
        let bob = xxh64::Xxh64Builder::new(19);
        let perm = Permutation::new(n, 19, bob);
        let mut xs = Vec::new();
        for i in 0..n {
            let x = perm.get(i);
            //println!("{i}\t{x}");
            xs.push(x);
        }

        // Check the permutation is random.
        let mut sx: f64 = 0.0;
        let mut sx2: f64 = 0.0;
        for i in 0..n as usize {
            let d = (xs[i] as f64) - (i as f64);
            sx += d;
            sx2 += d * d;
        }
        let m_bar = sx / nf;
        assert_eq!(m_bar, 0.0);
        let v_bar = sx2 / nf - m_bar * m_bar;
        let sd_bar = v_bar.sqrt();
        let sd = triangular_sd(-nf, nf, 0.0);
        let std_error = sd / nf.sqrt();
        assert!((sd_bar - sd).abs() < std_error);

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
