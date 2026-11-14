use crate::{functor::Functor, invariant::Invariant, semigroup_k::SemigroupK};

impl<T: 'static> Invariant for Vec<T> {
    type Inner = T;

    type Out<B: 'static> = Vec<B>;

    fn imap<B: 'static, F, G>(self, f: F, _g: G) -> Self::Out<B>
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

impl<T> Functor for Vec<T>
where
    T: 'static,
{
    type Inner = T;

    type Out<B> = Vec<B>;

    fn map<B, F>(self, f: F) -> <Self as Functor>::Out<B>
    where
        F: Fn(<Self as Functor>::Inner) -> B,
    {
        self.into_iter().map(f).collect()
    }
}
