use crate::{hkt::HKT, invariant::Invariant};

pub trait Functor<'a>: HKT + Invariant<'a> {
    fn map<B, F>(self, f: F) -> <Self as HKT>::Out<B>
    where
        F: FnMut(<Self as HKT>::Inner) -> B;

    fn imap<B, F, G>(self, f: F, _g: G) -> <Self as HKT>::Out<B>
    where
        Self: Sized,
        F: FnMut(<Self as HKT>::Inner) -> B,
    {
        self.map(f)
    }

    /// Alias for [`Functor::map()`]
    fn fmap<B, F>(self, f: F) -> <Self as HKT>::Out<B>
    where
        Self: Sized,
        F: FnMut(<Self as HKT>::Inner) -> B,
    {
        self.map(f)
    }

    /// Lift a function `f` to operator on Functors.
    fn lift<B, F>(f: F) -> impl FnOnce(Self) -> <Self as HKT>::Out<B>
    where
        Self: Sized,
        F: Fn(<Self as HKT>::Inner) -> B,
    {
        move |fa: Self| fa.fmap(f)
    }

    /// Replaces the `A` value in `F[A]` with the supplied value.
    fn r#as<B>(self, b: B) -> <Self as HKT>::Out<B>
    where
        B: Clone,
        Self: Sized,
    {
        self.map(|_| b.clone())
    }

    fn void(self) -> <Self as HKT>::Out<()>
    where
        Self: Sized,
    {
        self.r#as(())
    }
}
