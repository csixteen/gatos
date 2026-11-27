use gatos_prim::either::Either::{Left, Right};

use crate::{
    alternative::Alternative, applicative::Applicative, flat_map::FlatMap, functor::Functor,
    hkt::HKT,
};

/// Monad allows composition of dependent effectful functions.
pub trait Monad<'a>: HKT + Applicative<'a> + FlatMap<'a> {
    fn map<A, B, F>(fa: Self::Of<A>, f: F) -> Self::Of<B>
    where
        Self: Sized,
        F: Fn(A) -> B,
    {
        Self::flat_map(fa, |a| Self::pure(f(a)))
    }

    /// Executes an action repeatedly as long as the given `bool` expression returns `true`.
    /// Collects the results into an arbitrary `Alternative` value, such as a `Vec`.
    fn while_m<A, F, G>(p: Self::Of<bool>, body: F) -> Self::Of<G::Of<A>>
    where
        Self::Of<bool>: Clone, // does this make sense? Need to try it out
        F: Fn() -> Self::Of<A>,
        G: Alternative<'a>,
        Self::Of<A>: Clone,
        G::Of<A>: Clone,
    {
        Self::tail_rec_m(G::empty(), |xs: G::Of<A>| {
            Self::if_m(
                p.clone(),
                || <Self as Functor>::map(body(), |bv| Left(G::append_k(xs.clone(), bv))),
                || Self::pure(Right(xs.clone())),
            )
        })
    }

    /// Executes an action repeatedly as long as the given `bool` expression returns `true`.
    /// Discards results.
    fn while_m_<A, F>(p: Self::Of<bool>, body: F) -> Self::Of<()>
    where
        Self: Sized,
        Self::Of<bool>: Clone,
        F: Fn() -> Self::Of<A>,
    {
        Self::tail_rec_m((), |_| {
            Self::if_m(
                p.clone(),
                || Self::r#as(body(), Left(())),
                || Self::pure(Right(())),
            )
        })
    }
}
