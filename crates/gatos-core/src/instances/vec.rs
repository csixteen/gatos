use crate::{
    applicative::Applicative, apply::Apply, functor::Functor, hkt::HKT, invariant::Invariant,
    invariant_monoidal::InvariantMonoidal, semigroup_k::SemigroupK, semigroupal::Semigroupal,
};

pub struct VecK;

impl HKT for VecK {
    type Of<A> = Vec<A>;
}

impl<'a> Invariant<'a> for VecK {
    fn imap<A, B, F, G>(fa: Self::Of<A>, f: F, _g: G) -> Self::Of<B>
    where
        A: 'a,
        F: Fn(A) -> B + 'a,
        G: Fn(B) -> A + 'a,
    {
        fa.into_iter().map(f).collect()
    }
}

impl SemigroupK for VecK {
    fn combine_k<A>(mut a: Self::Of<A>, mut b: Self::Of<A>) -> Self::Of<A> {
        a.append(&mut b);
        a
    }
}

impl Semigroupal for VecK {
    fn product<A, B>(fa: Self::Of<A>, fb: Self::Of<B>) -> Self::Of<(A, B)> {
        fa.into_iter().zip(fb).collect()
    }
}

impl<'a> InvariantMonoidal<'a> for VecK {
    fn unit() -> Self::Of<()> {
        Vec::new()
    }
}

impl<'a> Functor<'a> for VecK {
    fn map<A, B, F>(fa: Self::Of<A>, f: F) -> Self::Of<B>
    where
        F: FnMut(A) -> B,
    {
        fa.into_iter().map(f).collect()
    }
}

impl<'a> Apply<'a> for VecK {
    fn ap<A, B, F>(ff: Self::Of<F>, fa: Self::Of<A>) -> Self::Of<B>
    where
        F: FnOnce(A) -> B,
    {
        fa.into_iter().zip(ff).map(|(a, f)| f(a)).collect()
    }
}

impl<'a> Applicative<'a> for VecK {
    fn pure<A>(a: A) -> Self::Of<A> {
        vec![a]
    }
}
