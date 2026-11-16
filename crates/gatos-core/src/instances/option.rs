use crate::{
    applicative::Applicative, apply::Apply, flat_map::FlatMap, functor::Functor, hkt::HKT,
    invariant::Invariant, invariant_monoidal::InvariantMonoidal, semigroup_k::SemigroupK,
    semigroupal::Semigroupal,
};

impl<T> HKT for Option<T> {
    type Inner = T;

    type Out<B> = Option<B>;
}

impl<'a, T: 'a> Invariant<'a> for Option<T> {
    fn imap<B, F, G>(self, f: F, _g: G) -> Self::Out<B>
    where
        F: Fn(Self::Inner) -> B,
        G: Fn(B) -> Self::Inner,
    {
        // Notes for self:
        //
        // There are a few differences between Scala Cats hierarchy and Rust. In Scala Cats (and in
        // Category Theory), all Functor (covariant types) are also automatically Invariant types:
        // * If a type `F[_]` is Covariant, then we can change `F[A]` to `F[B]` using only `A => B`
        // * If a type is Contravariant, we can change `F[A]` to `F[B]` using only `B => A`.
        // * If a type is Invariant, then we need to use both transformations.
        //
        // `Option<T>` is already covariant, it only requires one of the transformations, which is
        // why we ignore `_g`.
        self.map(f)
    }
}

impl<'a, T: 'a> Functor<'a> for Option<T> {
    fn map<B, F>(self, f: F) -> <Self as HKT>::Out<B>
    where
        F: FnOnce(<Self as HKT>::Inner) -> B,
    {
        self.map(f)
    }
}

impl<T> Semigroupal for Option<T> {
    fn product<B>(self, fb: Self::Out<B>) -> Self::Out<(Self::Inner, B)> {
        Some((self?, fb?))
    }
}

impl<'a, T: 'a> InvariantMonoidal<'a> for Option<T> {
    fn unit() -> <Self as HKT>::Out<()> {
        Some(())
    }
}

impl<T> SemigroupK for Option<T> {
    fn combine_k(self, b: Self) -> Self {
        // The inner type `T` of `Option<T>` is not fully specified and can be anything,
        // therefor we don't know how to combine them, so we just use `Option::or()`.
        self.or(b)
    }
}

impl<'a, T: 'a> Apply<'a> for Option<T> {
    fn ap<A, B, F>(ff: <Self as HKT>::Out<F>, fa: <Self as HKT>::Out<A>) -> <Self as HKT>::Out<B>
    where
        F: FnOnce(A) -> B,
    {
        Some(ff?(fa?))
    }
}

impl<'a, T: 'a> Applicative<'a> for Option<T> {
    fn pure<B>(x: B) -> <Self as HKT>::Out<B> {
        Some(x)
    }
}

impl<'a, T: 'a> FlatMap<'a> for Option<T> {
    fn flat_map<B, F>(self, f: F) -> <Self as HKT>::Out<B>
    where
        F: Fn(<Self as HKT>::Inner) -> <Self as HKT>::Out<B>,
    {
        self.and_then(f)
    }
}
