use crate::{
    alternative::Alternative, applicative::Applicative, apply::Apply, flat_map::FlatMap,
    foldable::Foldable, functor::Functor, hkt::HKT, invariant::Invariant,
    invariant_monoidal::InvariantMonoidal, monoid_k::MonoidK,
    non_empty_alternative::NonEmptyAlternative, semigroup_k::SemigroupK, semigroupal::Semigroupal,
    unordered_foldable::UnorderedFoldable,
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

impl MonoidK for VecK {
    fn empty<A>() -> Self::Of<A> {
        Vec::new()
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

    fn tail_rec_m<A, B, F>(a: A, mut f: F) -> Self::Of<B>
    where
        F: FnMut(A) -> Self::Of<gatos_prim::either::Either<A, B>>,
    {
        use gatos_prim::either::Either::*;

        // Unlike OptionK, a single step `f(a)` on VecK can produce zero or more branches
        // (`Vec<Either<A, B>>`).
        let mut res = Vec::new();
        let mut stack = vec![a];

        while let Some(curr) = stack.pop() {
            // this is so we preserve Scala Cats' depth-first ordering matching `flatMap` semantics:
            // we reverse the step results before pushing them onto the stack so that the first
            // element is poppped and processed next.
            for item in f(curr).into_iter().rev() {
                match item {
                    Left(next) => stack.push(next),
                    Right(b) => res.push(b),
                }
            }
        }

        res
    }
}

impl UnorderedFoldable for VecK {
    fn unordered_fold_map<A, B, F>(fa: Self::Of<A>, f: F) -> B
    where
        B: gatos_kernel::commutative_monoid::CommutativeMonoid,
        F: Fn(A) -> B,
    {
        // for VecK, `unordered_fold_map` maps each element `A` to a `CommutativeMonoid`, and
        // combines the results with `B::combine()` using `B::empty()` as initial value.
        fa.into_iter()
            .fold(B::empty(), |acc, a| B::combine(acc, f(a)))
    }
}

impl Foldable for VecK {
    type Iter<A> = ::std::vec::IntoIter<A>;

    fn to_iter<A>(fa: Self::Of<A>) -> Self::Iter<A> {
        fa.into_iter()
    }
}

impl<'a> Alternative<'a> for VecK {}

impl<'a> NonEmptyAlternative<'a> for VecK {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instances::option::OptionK;

    #[test]
    fn test_vec_fold_a() {
        let values = vec![
            Some("hello".to_string()),
            Some(", ".to_string()),
            Some("world!".to_string()),
        ];
        let result = VecK::fold_a::<_, OptionK>(values);
        assert_eq!(result, Some("hello, world!".to_string()));
    }

    #[test]
    fn test_vec_unite() {
        let values = vec![vec![1, 2], vec![3, 4]];
        let result = VecK::unite::<_, VecK>(values);
        assert_eq!(result, vec![1, 2, 3, 4]);
    }
}
