use std::{collections::HashMap, hash::Hash};

use crate::{monoid::Monoid, semigroup::Semigroup};

impl<K, V> Semigroup for HashMap<K, V>
where
    K: Eq + Hash,
{
    fn combine(mut self, right: Self) -> Self {
        self.extend(right);
        self
    }
}

impl<K, V> Monoid for HashMap<K, V>
where
    K: Eq + Hash,
{
    fn empty() -> Self {
        HashMap::new()
    }
}
