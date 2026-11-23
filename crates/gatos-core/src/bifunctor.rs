use std::convert::identity;

use crate::{applicative::Applicative, hkt::HKT2};

/// A type class of types which give rise to two independent, covariant [`Functor`]s.
///
/// [`Functor`]: super::functor::Functor
pub trait Bifunctor: HKT2 {
    /// The quintessential method of the Bifunctor trait, it applies a function to each "side" of
    /// the bifunctor.
    fn bimap<A, B, C, D, F, G>(fab: Self::Of<A, B>, f: F, g: G) -> Self::Of<C, D>
    where
        F: Fn(A) -> C,
        G: Fn(B) -> D;

    /// Applies `f` to the "left" functor.
    fn left_map<A, B, C, F>(fab: Self::Of<A, B>, f: F) -> Self::Of<C, B>
    where
        F: Fn(A) -> C,
    {
        Self::bimap(fab, f, identity)
    }

    /// Lift left into `F` using [`Applicative`].
    ///
    /// [`Applicative`]: super::applicative::Applicative
    fn left_lift_to<'a, A, B, C>(fab: Self::Of<A, B>) -> Self::Of<C::Of<A>, B>
    where
        C: Applicative<'a>,
    {
        Self::left_map(fab, C::pure)
    }
}
