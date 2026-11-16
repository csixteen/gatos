use std::{
    cmp::{Ordering, max_by, min_by},
    convert::identity,
    ops::Add,
};

use gatos_kernel::monoid::Monoid;
use num::Zero;

use crate::{hkt::HKT, unordered_foldable::UnorderedFoldable};

pub trait Foldable: HKT + UnorderedFoldable {
    /// Left associative fold on `F` (`self`) using the function `f` and initial value `b`.
    fn fold_left<B, F>(self, b: B, f: F) -> B
    where
        F: Fn(B, <Self as HKT>::Inner) -> B;

    fn reduce_left_to_option<B, F, G>(self, f: F, g: G) -> Option<B>
    where
        Self: Sized,
        F: Fn(<Self as HKT>::Inner) -> B,
        G: Fn(B, <Self as HKT>::Inner) -> B,
    {
        self.fold_left(None, |x, y| match (x, y) {
            (Some(b), a) => Some(g(b, a)),
            (None, a) => Some(f(a)),
        })
    }

    fn reduce_left_option<F>(self, f: F) -> Option<<Self as HKT>::Inner>
    where
        Self: Sized,
        F: Fn(<Self as HKT>::Inner, <Self as HKT>::Inner) -> <Self as HKT>::Inner,
    {
        self.reduce_left_to_option(identity, f)
    }

    fn minimum_option(self) -> Option<<Self as HKT>::Inner>
    where
        Self: Sized,
        <Self as HKT>::Inner: Ord,
    {
        self.reduce_left_option(<<Self as HKT>::Inner as Ord>::min)
    }

    fn maximum_option(self) -> Option<<Self as HKT>::Inner>
    where
        Self: Sized,
        <Self as HKT>::Inner: Ord,
    {
        self.reduce_left_option(<<Self as HKT>::Inner as Ord>::max)
    }

    fn minimum_by_option<F>(self, f: F) -> Option<<Self as HKT>::Inner>
    where
        Self: Sized,
        F: Fn(&<Self as HKT>::Inner, &<Self as HKT>::Inner) -> Ordering + Clone,
    {
        self.reduce_left_option(|x, y| min_by(x, y, f.clone()))
    }

    fn maximum_by_option<F>(self, f: F) -> Option<<Self as HKT>::Inner>
    where
        Self: Sized,
        F: Fn(&<Self as HKT>::Inner, &<Self as HKT>::Inner) -> Ordering + Clone,
    {
        self.reduce_left_option(|x, y| max_by(x, y, f.clone()))
    }

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

    fn fold(self) -> <Self as HKT>::Inner
    where
        Self: Sized,
        Self: IntoIterator<Item = <Self as HKT>::Inner>,
        <Self as HKT>::Inner: Monoid,
    {
        <<Self as HKT>::Inner as Monoid>::combine_all(self.into_iter())
    }

    fn fold_map<B, F>(self, f: F) -> B
    where
        Self: Sized,
        F: Fn(<Self as HKT>::Inner) -> B,
        B: Monoid,
    {
        self.fold_left(B::empty(), |b, a| b.combine(f(a)))
    }
}
