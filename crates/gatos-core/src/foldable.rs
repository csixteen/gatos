use std::{
    cmp::{Ordering, max_by, min_by},
    convert::identity,
    ops::Add,
};

use gatos_kernel::monoid::Monoid;
use gatos_prim::either::Either;
use num::Zero;

use crate::{
    applicative::Applicative, hkt::HKT, monad::Monad, monoid_k::MonoidK,
    unordered_foldable::UnorderedFoldable,
};

pub trait Foldable: HKT + UnorderedFoldable {
    // Scala Cats uses `Eval` and `Source.fromFoldable`. Trying to implement those types and
    // constructs in Rust would introduce extra overhead and unnecessary boxing. Iterators in Rust
    // are lazy and solve (part of) the problems that Eval + Source address.
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

    /// Performs a monadic left fold into monad `M`.
    fn fold_m<'a, M, A, B, F>(fa: Self::Of<A>, z: B, mut f: F) -> M::Of<B>
    where
        M: Monad<'a>,
        F: FnMut(B, A) -> M::Of<B>,
    {
        let mut iter = Self::to_iter(fa);

        M::tail_rec_m(z, |acc| match iter.next() {
            Some(a) => <M as Monad>::map(f(acc, a), Either::Left),
            None => todo!(),
        })
    }

    /// Fold implemented by mapping `A` values into `B`, in a context `G`, and then
    /// combining them using the `MonoidK<G>` instance.
    fn fold_map_k<A, B, F, G>(fa: Self::Of<A>, f: F) -> G::Of<B>
    where
        G: MonoidK,
        F: Fn(A) -> G::Of<B>,
    {
        Self::to_iter(fa).fold(G::empty(), |acc, a| G::combine_k(acc, f(a)))
    }

    fn fold_a<'a, A, G>(fga: Self::Of<G::Of<A>>) -> G::Of<A>
    where
        A: Monoid,
        G: Applicative<'a> + HKT,
    {
        Self::fold_map_a::<G::Of<A>, A, _, G>(fga, identity)
    }

    /// Fold in an [`Applicative`] context by mapping the `A` values to `G<B>`, combining the `B`
    /// values using the given [`Monoid`] instance.
    fn fold_map_a<'a, A, B, F, G>(fa: Self::Of<A>, f: F) -> G::Of<B>
    where
        B: Monoid,
        F: Fn(A) -> G::Of<B>,
        G: Applicative<'a>,
    {
        Self::to_iter(fa).fold(G::pure(B::empty()), |acc, a| G::map2(f(a), acc, B::combine))
    }
}
