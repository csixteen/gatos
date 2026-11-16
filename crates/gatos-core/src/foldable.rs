use std::ops::Add;

use num::Zero;

use crate::{hkt::HKT, unordered_foldable::UnorderedFoldable};

pub trait Foldable: HKT + UnorderedFoldable {
    /// Left associative fold on `F` (`self`) using the function `f` and initial value `b`.
    fn fold_left<B, F>(self, b: B, f: F) -> B
    where
        F: Fn(B, <Self as HKT>::Inner) -> B;

    fn sum_all(self) -> <Self as HKT>::Inner
    where
        Self: Sized,
        <Self as HKT>::Inner: Zero + Add,
    {
        self.fold_left(
            <<Self as HKT>::Inner as Zero>::zero(),
            <<Self as HKT>::Inner as Add>::add,
        )
    }
}
