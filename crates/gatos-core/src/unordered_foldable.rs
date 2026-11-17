use std::convert::identity;

use gatos_kernel::commutative_monoid::CommutativeMonoid;

use crate::hkt::HKT;

pub trait UnorderedFoldable: HKT {
    fn unordered_fold_map<A, B, F>(fa: Self::Of<A>, f: F) -> B
    where
        B: CommutativeMonoid,
        F: Fn(A) -> B;

    fn unordered_fold<A>(fa: Self::Of<A>) -> A
    where
        Self: Sized,
        A: CommutativeMonoid,
    {
        Self::unordered_fold_map(fa, identity)
    }
}
