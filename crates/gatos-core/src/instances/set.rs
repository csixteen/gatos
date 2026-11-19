use std::collections::HashSet;

use crate::{hkt::HKT, unordered_foldable::UnorderedFoldable};

pub struct HashSetK;

impl HKT for HashSetK {
    type Of<A> = HashSet<A>;
}

impl UnorderedFoldable for HashSetK {
    fn unordered_fold_map<A, B, F>(fa: Self::Of<A>, f: F) -> B
    where
        B: gatos_kernel::commutative_monoid::CommutativeMonoid,
        F: Fn(A) -> B,
    {
        fa.into_iter()
            .fold(B::empty(), |acc, a| B::combine(acc, f(a)))
    }
}
