use crate::{
    applicative::Applicative, apply::Apply, flat_map::FlatMap, foldable::Foldable,
    functor::Functor, hkt::HKT, invariant::Invariant, invariant_monoidal::InvariantMonoidal,
    semigroup_k::SemigroupK, semigroupal::Semigroupal, unordered_foldable::UnorderedFoldable,
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

impl<'a> FlatMap<'a> for VecK {
    fn flat_map<A, B, F>(fa: Self::Of<A>, f: F) -> Self::Of<B>
    where
        F: FnMut(A) -> Self::Of<B>,
    {
        fa.into_iter().flat_map(f).collect()
    }

    fn tail_rec_m<A, B, F>(a: A, f: F) -> Self::Of<B>
    where
        F: Fn(A) -> Self::Of<gatos_prim::either::Either<A, B>>,
    {
        todo!()
    }
}

impl UnorderedFoldable for VecK {
    fn unordered_fold_map<A, B, F>(fa: Self::Of<A>, f: F) -> B
    where
        B: gatos_kernel::commutative_monoid::CommutativeMonoid,
        F: Fn(A) -> B,
    {
        todo!()
    }
}

impl Foldable for VecK {
    type Iter<A> = ::std::vec::IntoIter<A>;

    fn to_iter<A>(fa: Self::Of<A>) -> Self::Iter<A> {
        fa.into_iter()
    }
}
