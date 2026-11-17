use crate::{
    applicative::Applicative, apply::Apply, flat_map::FlatMap, functor::Functor, hkt::HKT,
    invariant::Invariant, invariant_monoidal::InvariantMonoidal, semigroup_k::SemigroupK,
    semigroupal::Semigroupal,
};

pub struct OptionK;

impl HKT for OptionK {
    type Of<A> = Option<A>;
}

impl<'a> Invariant<'a> for OptionK {
    fn imap<A, B, F, G>(fa: Self::Of<A>, f: F, _g: G) -> Self::Of<B>
    where
        A: 'a,
        F: Fn(A) -> B + 'a,
        G: Fn(B) -> A + 'a,
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
        fa.map(f)
    }
}

impl Semigroupal for OptionK {
    fn product<A, B>(fa: Self::Of<A>, fb: Self::Of<B>) -> Self::Of<(A, B)> {
        Some((fa?, fb?))
    }
}

impl<'a> InvariantMonoidal<'a> for OptionK {
    fn unit() -> Self::Of<()> {
        Some(())
    }
}

impl SemigroupK for OptionK {
    fn combine_k<A>(a: Self::Of<A>, b: Self::Of<A>) -> Self::Of<A> {
        a.or(b)
    }
}

impl<'a> Functor<'a> for OptionK {
    fn map<A, B, F>(fa: Self::Of<A>, f: F) -> Self::Of<B>
    where
        F: FnMut(A) -> B,
    {
        fa.map(f)
    }
}

impl<'a> Apply<'a> for OptionK {
    fn ap<A, B, F>(ff: Self::Of<F>, fa: Self::Of<A>) -> Self::Of<B>
    where
        F: FnOnce(A) -> B,
    {
        Some(ff?(fa?))
    }
}

impl<'a> Applicative<'a> for OptionK {
    fn pure<A>(a: A) -> Self::Of<A> {
        Some(a)
    }
}

impl<'a> FlatMap<'a> for OptionK {
    fn flat_map<A, B, F>(fa: Self::Of<A>, f: F) -> Self::Of<B>
    where
        F: FnOnce(A) -> Self::Of<B>,
    {
        fa.and_then(f)
    }
}
