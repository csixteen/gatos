use crate::core::{invariant::Invariant, semigroup_k::SemigroupK};

impl<T> Invariant for Vec<T> {
    type Inner = T;

    type Out<B> = Vec<B>;

    fn imap<B, F, G>(self, f: F, _g: G) -> Self::Out<B>
    where
        F: Fn(Self::Inner) -> B,
        G: Fn(B) -> Self::Inner,
    {
        // Just like `Option<T>`, `Vec<T>` is a covariant functor. As such, `g` is ignored.
        self.into_iter().map(f).collect()
    }
}

impl<T> SemigroupK for Vec<T> {
    type Inner = T;
    type Out<B> = Vec<B>;

    fn combine_k(mut self, mut b: Self) -> Self {
        self.append(&mut b);
        self
    }
}
