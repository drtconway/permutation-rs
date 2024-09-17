// SPDX-License-Identifier: Apache-2.0
use std::hash::BuildHasher;

/// A BuildHasher for the standard DefaultHasher.
pub struct DefaultBuildHasher {}

impl DefaultBuildHasher {
    /// Build a new BuildHasher for constructing [`std::collections::hash_map::DefaultHasher`] objects.
    pub fn new() -> DefaultBuildHasher {
        DefaultBuildHasher {}
    }
}

impl BuildHasher for DefaultBuildHasher {
    type Hasher = std::collections::hash_map::DefaultHasher;
    fn build_hasher(&self) -> Self::Hasher {
        std::collections::hash_map::DefaultHasher::new()
    }
}