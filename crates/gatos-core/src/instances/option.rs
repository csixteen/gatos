use crate::{
    applicative::Applicative, apply::Apply, flat_map::FlatMap, foldable::Foldable,
    functor::Functor, hkt::HKT, invariant::Invariant, invariant_monoidal::InvariantMonoidal,
    semigroup_k::SemigroupK, semigroupal::Semigroupal, unordered_foldable::UnorderedFoldable,
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

    fn tail_rec_m<A, B, F>(a: A, mut f: F) -> Self::Of<B>
    where
        F: FnMut(A) -> Self::Of<gatos_prim::either::Either<A, B>>,
    {
        use gatos_prim::either::Either::*;

        let mut curr = a;

        loop {
            match f(curr) {
                Some(Left(next)) => curr = next,
                Some(Right(b)) => return Some(b),
                None => return None,
            }
        }
    }
}

impl UnorderedFoldable for OptionK {
    fn unordered_fold_map<A, B, F>(fa: Self::Of<A>, f: F) -> B
    where
        B: gatos_kernel::commutative_monoid::CommutativeMonoid,
        F: Fn(A) -> B,
    {
        fa.into_iter()
            .fold(B::empty(), |acc, a| B::combine(acc, f(a)))
    }
}

impl Foldable for OptionK {
    type Iter<A> = ::std::option::IntoIter<A>;

    fn to_iter<A>(fa: Self::Of<A>) -> Self::Iter<A> {
        fa.into_iter()
    }
}
