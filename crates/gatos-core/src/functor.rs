use crate::{hkt::HKT, invariant::Invariant};

pub trait Functor<'a>: HKT + Invariant<'a> {
    fn map<A, B, F>(fa: Self::Of<A>, f: F) -> Self::Of<B>
    where
        F: FnMut(A) -> B;

    fn imap<A, B, F, G>(fa: Self::Of<A>, f: F, _g: G) -> Self::Of<B>
    where
        Self: Sized,
        F: FnMut(A) -> B,
    {
        Self::map(fa, f)
    }

    /// Alias for [`Functor::map()`]
    fn fmap<A, B, F>(fa: Self::Of<A>, f: F) -> Self::Of<B>
    where
        Self: Sized,
        F: FnMut(A) -> B,
    {
        Self::map(fa, f)
    }

    /// Lift a function `f` to operator on Functors.
    fn lift<A, B, F>(f: F) -> impl FnOnce(Self::Of<A>) -> Self::Of<B>
    where
        Self: Sized,
        F: Fn(A) -> B,
    {
        move |fa| Self::fmap(fa, f)
    }

    /// Replaces the `A` value in `F[A]` with the supplied value.
    fn r#as<A, B>(fa: Self::Of<A>, b: B) -> Self::Of<B>
    where
        B: Clone,
        Self: Sized,
    {
        Self::map(fa, |_| b.clone())
    }

    fn void<A>(fa: Self::Of<A>) -> Self::Of<()>
    where
        Self: Sized,
    {
        Self::r#as(fa, ())
    }
}
