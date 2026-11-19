use std::num::NonZeroUsize;

use crate::semigroup::Semigroup;

/// Bands are semigroups whose `combine` operation is also idempotent.
pub trait Band: Semigroup {
    fn repeated_combine_n(self, _n: NonZeroUsize) -> Self {
        self
    }
}
