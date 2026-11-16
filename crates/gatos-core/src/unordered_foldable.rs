use std::convert::identity;

use gatos_kernel::commutative_monoid::CommutativeMonoid;

use crate::hkt::HKT;

pub trait UnorderedFoldable: HKT {
    fn unordered_fold_map<B, F>(self, f: F) -> B
    where
        B: CommutativeMonoid,
        F: Fn(<Self as HKT>::Inner) -> B;

    fn unordered_fold(self) -> <Self as HKT>::Inner
    where
        Self: Sized,
        <Self as HKT>::Inner: CommutativeMonoid,
    {
        self.unordered_fold_map(identity)
    }
}
