use std::{
    cmp::{Ordering, max_by, min_by},
    convert::identity,
    ops::Add,
};

use gatos_kernel::monoid::Monoid;
use num::Zero;

use crate::{hkt::HKT, unordered_foldable::UnorderedFoldable};

pub trait Foldable: HKT + UnorderedFoldable {
    type Iter<A>: Iterator<Item = A>;

    fn to_iter<A>(fa: Self::Of<A>) -> Self::Iter<A>;

    /// Left associative fold on `F` (`self`) using the function `f` and initial value `b`.
    fn fold_left<A, B, F>(fa: Self::Of<A>, b: B, f: F) -> B
    where
        F: FnMut(B, A) -> B,
    {
        Self::to_iter(fa).fold(b, f)
    }

    fn fold_right<A, B, F>(fa: Self::Of<A>, b: B, mut f: F) -> B
    where
        Self::Iter<A>: DoubleEndedIterator,
        F: FnMut(A, B) -> B,
    {
        Self::to_iter(fa).rfold(b, |acc, a| f(a, acc))
    }

    fn reduce_left_to_option<A, B, F, G>(fa: Self::Of<A>, f: F, g: G) -> Option<B>
    where
        Self: Sized,
        F: Fn(A) -> B,
        G: Fn(B, A) -> B,
    {
        Self::fold_left(fa, None, |x, y| match (x, y) {
            (Some(b), a) => Some(g(b, a)),
            (None, a) => Some(f(a)),
        })
    }

    fn reduce_left_option<A, F>(fa: Self::Of<A>, f: F) -> Option<A>
    where
        Self: Sized,
        F: Fn(A, A) -> A,
    {
        Self::reduce_left_to_option(fa, identity, f)
    }

    fn minimum_option<A>(fa: Self::Of<A>) -> Option<A>
    where
        Self: Sized,
        A: Ord,
    {
        Self::reduce_left_option(fa, A::min)
    }

    fn maximum_option<A>(fa: Self::Of<A>) -> Option<A>
    where
        Self: Sized,
        A: Ord,
    {
        Self::reduce_left_option(fa, A::max)
    }

    fn minimum_by_option<A, F>(fa: Self::Of<A>, f: F) -> Option<A>
    where
        Self: Sized,
        F: Fn(&A, &A) -> Ordering + Clone,
    {
        Self::reduce_left_option(fa, |x, y| min_by(x, y, f.clone()))
    }

    fn maximum_by_option<A, F>(fa: Self::Of<A>, f: F) -> Option<A>
    where
        Self: Sized,
        F: Fn(&A, &A) -> Ordering + Clone,
    {
        Self::reduce_left_option(fa, |x, y| max_by(x, y, f.clone()))
    }

    fn sum_all<A>(fa: Self::Of<A>) -> A
    where
        Self: Sized,
        A: Zero + Add,
    {
        Self::fold_left(fa, A::zero(), A::add)
    }

    fn fold<A>(fa: Self::Of<A>) -> A
    where
        Self: Sized,
        Self::Of<A>: IntoIterator<Item = A>,
        A: Monoid,
    {
        A::combine_all(fa.into_iter())
    }

    fn fold_map<A, B, F>(fa: Self::Of<A>, f: F) -> B
    where
        Self: Sized,
        F: Fn(A) -> B,
        B: Monoid,
    {
        Self::fold_left(fa, B::empty(), |b, a| b.combine(f(a)))
    }
}
