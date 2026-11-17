use crate::{applicative::Applicative, flat_map::FlatMap, hkt::HKT};

/// Monad allows composition of dependent effectful functions.
//
// NOTE: this trait won't be interesting until I implement Scala Cats' Eval.
pub trait Monad<'a>: HKT + Applicative<'a> + FlatMap<'a> {
    fn map<A, B, F>(fa: Self::Of<A>, f: F) -> Self::Of<B>
    where
        Self: Sized,
        F: Fn(A) -> B,
    {
        Self::flat_map(fa, |a| Self::pure(f(a)))
    }
}
