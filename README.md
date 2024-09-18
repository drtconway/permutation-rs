# Permutation-rs

This library provides a constant space, approximately constant time method for producing permutations.

## Rationale

There are many situations where we want to process items in a randomised order. The simplest way to do
this is typically to "shuffle" a vector of items. However there are circumstances when this is undesirable -
for instance, if the vector of items is read-only. In such a case, we could make a copy of it, but if it
is very large, this may be impractical.

A special instance of when we want a permutation is sampling without replacement. We may have a very large
vector from which we want to sample just a small number of items, we can avoid generating a full permutation
by selecting random items and collecting them in a set data structure to avoid duplicates, until we have
the required number of items, however this becomes inefficient as the number of items to be sampled grows
and approaches the size of the original vector.

What we present here is an alternative approach which algorithmically generates the permutation without
actually having to store it. The intuition is straight forward.

First, let us make the observation that for the purposes of generating a permutation, we don't need the
items per se, just the indexes of the items: [0, _n_).

A block cipher uses a key to create a one-to-one mapping from plain-text blocks of _b_ bits to encrypted
blocks of _b_ bits. It has to be a one-to-one mapping or we wouldn't be able to decrypt it! So in a simple
case where _n_ = 2**_b_, we could just use a _b_ bit block cipher to encrypt indexes to generate a
permutation. This way of generating a permutation also has the neat property that you can invert the
permutation by decrypting.

Now if _n_ < 2**_b_, it is possible that when we encrypt an index _i_, the encrypted value _j_ could be
greather than or equal to _n_. However, if _j_ >= _n_, then it will never be a source index, so we can
encypt _j_ (with the same key), until we get an encrypted value less than _n_. Obviously if _n_ << 2**_b_,
we may have to do this many time. In fact, we can say the expected number of encryptions required till
we get _j_ < _n_ is 2**_b_ / _n_. So if we choose _b_ such that 2**_b_ is the smallest power of two >= _n_,
then the average number of encryptions is less than 2.

Now this technique will work with any block cipher, but since we want to be flexible on the block size _b_,
the most convenient choice is a [Feistel Cipher](https://en.wikipedia.org/wiki/Feistel_cipher). Feistel
ciphers are built on top of a hash function. For a good cypher, a high quality hash function is required,
but for our purposes any reasonable hash function will do. By default, the library will work with the
Rust standard library default hash function, but it is easy to substitute in an alternative. Our Feistel
cipher implementation is fairly simple and only works for block sizes where _b_ is an even number. As a
result, the expected number of encryption cycles is between 3 and 4, so it is slower than optimal, but
this is still O(1), so it is satisfactory for our purposes.

## Quick Start

To use this library in an existing project:

```bash
cargo add feistel-permutation-rs
```

To use it in your code:

```rust
let n = 1000000;
let seed = 29;
let perm = Permutation::new(n, seed, DefaultBuildHasher::new());
for i in 0..10 {
  println!("{} -> {}", i, perm.get(i));
}
```

Or if iterators are your thing:

```rust
let n = 1000000;
let seed = 29;
let perm = Permutation::new(n, seed, DefaultBuildHasher::new());
for j in perm.range(100, 200) {
  println!("{}", j);
}

for j in perm.iter().take(10) {
  println!("{}", j);
}
```
