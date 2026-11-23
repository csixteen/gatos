use std::marker::PhantomData;

use crate::{
    applicative::Applicative,
    apply::Apply,
    bifoldable::Bifoldable,
    flat_map::FlatMap,
    foldable::Foldable,
    functor::Functor,
    hkt::{HKT, HKT2},
    invariant::Invariant,
    invariant_monoidal::InvariantMonoidal,
    semigroup_k::SemigroupK,
    semigroupal::Semigroupal,
    unordered_foldable::UnorderedFoldable,
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

    fn tail_rec_m<A, B, F>(a: A, mut f: F) -> Self::Of<B>
    where
        F: FnMut(A) -> Self::Of<gatos_prim::either::Either<A, B>>,
    {
        use gatos_prim::either::Either::*;

        let mut curr = a;

        loop {
            match f(curr) {
                Ok(Left(aa)) => curr = aa,
                Ok(Right(b)) => return Ok(b),
                Err(e) => return Err(e),
            }
        }
    }
}

impl<E> UnorderedFoldable for ResultK<E> {
    fn unordered_fold_map<A, B, F>(fa: Self::Of<A>, f: F) -> B
    where
        B: gatos_kernel::commutative_monoid::CommutativeMonoid,
        F: Fn(A) -> B,
    {
        fa.into_iter()
            .fold(B::empty(), |acc, a| B::combine(acc, f(a)))
    }
}

impl<E> Foldable for ResultK<E> {
    type Iter<A> = ::std::result::IntoIter<A>;

    fn to_iter<A>(fa: Self::Of<A>) -> Self::Iter<A> {
        fa.into_iter()
    }
}

pub struct ResultK2;

impl HKT2 for ResultK2 {
    type Of<A, B> = Result<A, B>;
}

impl Bifoldable for ResultK2 {
    fn bifold_left<A, B, C, F, G>(fab: Self::Of<A, B>, c: C, f: F, g: G) -> C
    where
        F: Fn(C, A) -> C,
        G: Fn(C, B) -> C,
    {
        match fab {
            Ok(a) => f(c, a),
            Err(b) => g(c, b),
        }
    }

    fn bifold_right<A, B, C, F, G>(fab: Self::Of<A, B>, c: C, f: F, g: G) -> C
    where
        F: Fn(A, C) -> C,
        G: Fn(B, C) -> C,
    {
        match fab {
            Ok(a) => f(a, c),
            Err(b) => g(b, c),
        }
    }
}
