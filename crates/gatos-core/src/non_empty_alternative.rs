use crate::{applicative::Applicative, hkt::HKT, semigroup_k::SemigroupK};

pub trait NonEmptyAlternative<'a>: HKT + Applicative<'a> + SemigroupK {
    /// Lift `a` into `F<_>` and prepend it to `fa`.
    fn prepend_k<A>(a: A, fa: Self::Of<A>) -> Self::Of<A> {
        Self::combine_k(Self::pure(a), fa)
    }

    /// Lift `a` into `F<_>` and append it to `fa`.
    fn append_k<A>(fa: Self::Of<A>, a: A) -> Self::Of<A> {
        Self::combine_k(fa, Self::pure(a))
    }
}
