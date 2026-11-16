use std::num::NonZeroUsize;

use crate::{intercalate::Intercalate, reverse::Reverse, semigroup::Semigroup};

pub trait CommutativeSemigroup: Semigroup {
    fn reverse(self) -> Reverse<Self> {
        Reverse::new(self)
    }

    fn intercalate(self, middle: Self) -> Intercalate<Self> {
        Intercalate::new(self, middle)
    }

    fn combine_n(self, n: NonZeroUsize) -> Self
    where
        Self: Clone;
}
