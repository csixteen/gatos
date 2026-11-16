use crate::{applicative::Applicative, flat_map::FlatMap, hkt::HKT};

/// Monad allows composition of dependent effectful functions.
//
// NOTE: this trait won't be interesting until I implement Scala Cats' Eval.
pub trait Monad<'a>: HKT + Applicative<'a> + FlatMap<'a> {
    fn map<B, F>(self, f: F) -> <Self as HKT>::Out<B>
    where
        Self: Sized,
        F: Fn(<Self as HKT>::Inner) -> B,
    {
        self.flat_map(|a| <Self as Applicative>::pure(f(a)))
    }
}
