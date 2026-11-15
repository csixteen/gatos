use crate::{functor::Functor, hkt::HKT, invariant_semigroupal::InvariantSemigroupal};
use gatos_prim::function::TupledFunction2;

pub trait Apply<'a>: HKT + Functor<'a> + InvariantSemigroupal<'a> {
    /// This is the equivalent of Haskell's `<*>`, whose signature is:
    ///
    /// ```text
    /// (<*>) :: f (a -> b) -> f a -> f b
    /// ```
    ///
    /// It can be thought of as function application inside a Functor.
    fn ap<A, B, F>(ff: <Self as HKT>::Out<F>, fa: <Self as HKT>::Out<A>) -> <Self as HKT>::Out<B>
    where
        F: FnOnce(A) -> B;

    /// Compose two actions, discarding any value produced by the first.
    fn product_r<B>(self, fb: <Self as HKT>::Out<B>) -> <Self as HKT>::Out<B>
    where
        Self: Sized,
    {
        let ff = self.r#as(|b| b);
        Self::ap(ff, fb)
    }

    fn product<B>(self, fb: <Self as HKT>::Out<B>) -> <Self as HKT>::Out<(<Self as HKT>::Inner, B)>
    where
        Self: Sized,
    {
        let ff = self.map(|a| move |b| (a, b));
        Self::ap(ff, fb)
    }

    fn map2<B, F, Z>(self, fb: <Self as HKT>::Out<B>, f: F) -> <Self as HKT>::Out<Z>
    where
        Self: Sized,
        F: TupledFunction2<<Self as HKT>::Inner, B, Z>,
        <Self as HKT>::Out<(<Self as HKT>::Inner, B)>:
            Functor<'a, Inner = (<Self as HKT>::Inner, B), Out<Z> = <Self as HKT>::Out<Z>>,
    {
        <Self as Apply>::product(self, fb).map(f.tupled())
    }
}
