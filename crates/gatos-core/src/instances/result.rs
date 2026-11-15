use crate::{functor::Functor, hkt::HKT, invariant::Invariant};

impl<T, E> HKT for Result<T, E> {
    type Inner = T;

    type Out<B> = Result<B, E>;
}

impl<'a, T: 'a, E> Invariant<'a> for Result<T, E> {
    fn imap<B, F, G>(self, f: F, _g: G) -> Self::Out<B>
    where
        F: Fn(Self::Inner) -> B,
        G: Fn(B) -> Self::Inner,
    {
        self.map(f)
    }
}

impl<'a, T: 'a, E> Functor<'a> for Result<T, E> {
    fn map<B, F>(self, f: F) -> <Self as HKT>::Out<B>
    where
        F: FnOnce(<Self as HKT>::Inner) -> B,
    {
        self.map(f)
    }
}
