use gatos_prim::function::TupledFunction2;

use crate::{functor::Functor, hkt::HKT, invariant_semigroupal::InvariantSemigroupal};

pub trait Apply<'a>: HKT + Functor<'a> + InvariantSemigroupal<'a> {
    /// This is the equivalent of Haskell's `<*>`, whose signature is:
    ///
    /// ```text
    /// (<*>) :: f (a -> b) -> f a -> f b
    /// ```
    ///
    /// It can be thought of as function application inside a Functor.
    fn ap<A, B, F>(ff: Self::Of<F>, fa: Self::Of<A>) -> Self::Of<B>
    where
        F: FnOnce(A) -> B;

    /// Compose two actions, discarding any value produced by the first.
    fn product_r<A, B>(fa: Self::Of<A>, fb: Self::Of<B>) -> Self::Of<B>
    where
        Self: Sized,
    {
        let ff = Self::r#as(fa, |b| b);
        Self::ap(ff, fb)
    }

    fn product<A, B>(fa: Self::Of<A>, fb: Self::Of<B>) -> Self::Of<(A, B)>
    where
        Self: Sized,
    {
        let ff = Self::map(fa, |a| move |b| (a, b));
        Self::ap(ff, fb)
    }

    fn map2<A, B, F, Z>(fa: Self::Of<A>, fb: Self::Of<B>, f: F) -> Self::Of<Z>
    where
        Self: Sized,
        F: TupledFunction2<A, B, Z>,
    {
        let p = <Self as Apply>::product(fa, fb);
        Self::map(p, f.tupled())
    }
}
