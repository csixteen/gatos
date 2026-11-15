use crate::{
    applicative::Applicative, apply::Apply, functor::Functor, hkt::HKT, invariant::Invariant,
    invariant_monoidal::InvariantMonoidal, semigroup_k::SemigroupK, semigroupal::Semigroupal,
};

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

impl<T> Semigroupal for Vec<T> {
    fn product<B>(
        self,
        fb: <Self as HKT>::Out<B>,
    ) -> <Self as HKT>::Out<(<Self as HKT>::Inner, B)> {
        self.into_iter().zip(fb).collect()
    }
}

impl<'a, T: 'a> InvariantMonoidal<'a> for Vec<T> {
    fn unit() -> <Self as HKT>::Out<()> {
        Vec::new()
    }
}

impl<'a, T: 'a> Apply<'a> for Vec<T> {
    fn ap<A, B, F>(ff: <Self as HKT>::Out<F>, fa: <Self as HKT>::Out<A>) -> <Self as HKT>::Out<B>
    where
        F: FnOnce(A) -> B,
    {
        fa.into_iter().zip(ff).map(|(a, f)| f(a)).collect()
    }
}

impl<'a, T: 'a> Applicative<'a> for Vec<T> {
    fn pure<B>(x: B) -> <Self as HKT>::Out<B> {
        vec![x]
    }
}
