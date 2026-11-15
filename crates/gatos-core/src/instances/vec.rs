use crate::{functor::Functor, hkt::HKT, invariant::Invariant, semigroup_k::SemigroupK};

impl<T> HKT for Vec<T> {
    type Inner = T;

    type Out<B> = Vec<B>;
}

impl<'a, T: 'a> Invariant<'a> for Vec<T> {
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
    fn combine_k(mut self, mut b: Self) -> Self {
        self.append(&mut b);
        self
    }
}

impl<'a, T: 'a> Functor<'a> for Vec<T> {
    fn map<B, F>(self, f: F) -> <Self as HKT>::Out<B>
    where
        F: FnMut(<Self as HKT>::Inner) -> B,
    {
        self.into_iter().map(f).collect()
    }
}
