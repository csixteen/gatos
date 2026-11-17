use std::marker::PhantomData;

use crate::{
    applicative::Applicative, apply::Apply, flat_map::FlatMap, functor::Functor, hkt::HKT,
    invariant::Invariant, invariant_monoidal::InvariantMonoidal, semigroup_k::SemigroupK,
    semigroupal::Semigroupal,
};

pub struct ResultK<E>(PhantomData<E>);

impl<E> HKT for ResultK<E> {
    type Of<A> = Result<A, E>;
}

impl<'a, E> Invariant<'a> for ResultK<E> {
    fn imap<A, B, F, G>(fa: Self::Of<A>, f: F, _g: G) -> Self::Of<B>
    where
        A: 'a,
        F: Fn(A) -> B + 'a,
        G: Fn(B) -> A + 'a,
    {
        fa.map(f)
    }
}

impl<E> Semigroupal for ResultK<E> {
    fn product<A, B>(fa: Self::Of<A>, fb: Self::Of<B>) -> Self::Of<(A, B)> {
        Ok((fa?, fb?))
    }
}

impl<'a, E> InvariantMonoidal<'a> for ResultK<E> {
    fn unit() -> Self::Of<()> {
        Ok(())
    }
}

impl<E> SemigroupK for ResultK<E> {
    fn combine_k<A>(a: Self::Of<A>, b: Self::Of<A>) -> Self::Of<A> {
        a.or(b)
    }
}

impl<'a, E> Functor<'a> for ResultK<E> {
    fn map<A, B, F>(fa: Self::Of<A>, f: F) -> Self::Of<B>
    where
        F: FnMut(A) -> B,
    {
        fa.map(f)
    }
}

impl<'a, E> Apply<'a> for ResultK<E> {
    fn ap<A, B, F>(ff: Self::Of<F>, fa: Self::Of<A>) -> Self::Of<B>
    where
        F: FnOnce(A) -> B,
    {
        Ok(ff?(fa?))
    }
}

impl<'a, E> Applicative<'a> for ResultK<E> {
    fn pure<A>(a: A) -> Self::Of<A> {
        Ok(a)
    }
}

impl<'a, E> FlatMap<'a> for ResultK<E> {
    fn flat_map<A, B, F>(fa: Self::Of<A>, f: F) -> Self::Of<B>
    where
        F: FnOnce(A) -> Self::Of<B>,
    {
        fa.and_then(f)
    }
}
