use std::{collections::HashSet, hash::Hash};

use crate::{monoid::Monoid, semigroup::Semigroup};

impl<T> Semigroup for HashSet<T>
where
    T: Eq + Hash,
{
    fn combine(mut self, right: Self) -> Self {
        self.extend(right);
        self
    }
}

impl<T> Monoid for HashSet<T>
where
    T: Eq + Hash,
{
    fn empty() -> Self {
        HashSet::new()
    }
}
