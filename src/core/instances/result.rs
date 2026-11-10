use crate::core::{functor::Functor, invariant::Invariant};

impl<T, E> Invariant for Result<T, E> {
    type Inner = T;

    type Out<B> = Result<B, E>;

    fn imap<B, F, G>(self, f: F, _g: G) -> Self::Out<B>
    where
        F: Fn(Self::Inner) -> B,
        G: Fn(B) -> Self::Inner,
    {
        self.map(f)
    }
}

impl<T, E> Functor for Result<T, E> {
    type Inner = T;

    type Out<B> = Result<B, E>;

    fn map<B, F>(self, f: F) -> <Self as Functor>::Out<B>
    where
        F: FnOnce(<Self as Functor>::Inner) -> B,
    {
        self.map(f)
    }
}
