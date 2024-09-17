// SPDX-License-Identifier: Apache-2.0
#![warn(missing_docs)]

//! Constant-space permutations over integers.
//! 
//! This crate provides constant-space, constant-time random access
//! permutations over dense integer ranges. It is built on top of a
//! simple Feistel Network cipher.

mod feistel;
mod permutation;
mod utils;

pub use feistel::*;
pub use permutation::*;
pub use utils::*;