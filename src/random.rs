use crate::F;

/// In range [0, 1]
pub fn random_float() -> F {
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hasher};

    RandomState::new().build_hasher().finish() as F / u64::MAX as F
}

/// In [min, max]
pub fn random_float_in(min: F, max: F) -> F {
    min + (max-min)*random_float()
}
