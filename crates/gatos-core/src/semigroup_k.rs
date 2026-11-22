use std::num::NonZeroUsize;

use crate::hkt::HKT;

/// Similar to [`Semigroup`], with the difference that it operates on kinds,
/// i.e., on type constructors. It only operates on type constructors with one parameter
/// (the equivalent of Scala's `F[_]`). Because of that, we can find [`Semigroup`] implementation
/// for concret types, such as `i32`, but can only find [`SemigroupK`] implementation for
/// type constructors, such as `Option<T>`.
///
/// [`Semigroup`]: crate::kernel::semigroup::Semigroup
pub trait SemigroupK: HKT {
    /// Combines two `F[_]`.
    fn combine_k<A>(a: Self::Of<A>, b: Self::Of<A>) -> Self::Of<A>;

    fn combine_n_k<A>(a: Self::Of<A>, n: NonZeroUsize) -> Self::Of<A>
    where
        Self: Sized,
        Self::Of<A>: Clone,
    {
        Self::repeated_combine_n_k(a, n)
    }

    fn repeated_combine_n_k<A>(a: Self::Of<A>, n: NonZeroUsize) -> Self::Of<A>
    where
        Self: Sized,
        Self::Of<A>: Clone,
    {
        if usize::from(n) == 1_usize {
            a
        } else {
            _repeated::<A, Self>(a.clone(), usize::from(n).saturating_sub(1), a)
        }
    }
}

#[inline]
fn _repeated<A, T>(a: T::Of<A>, n: usize, acc: T::Of<A>) -> T::Of<A>
where
    T::Of<A>: Clone,
    T: SemigroupK + HKT,
{
    if n == 1 {
        T::combine_k(a, acc)
    } else {
        let x = if (n & 1) == 1 {
            T::combine_k(a.clone(), acc)
        } else {
            acc
        };
        _repeated::<A, T>(T::combine_k(a.clone(), a), n >> 1, x)
    }
}

#[cfg(test)]
mod laws {
    use std::fmt::Debug;

    use gatos_strategy::triple;
    use proptest::{prelude::*, test_runner::TestCaseResult};

    use super::*;
    use crate::instances::{option::OptionK, vec::VecK};

    fn assert_semigroup_k_laws<A, F>(a: F::Of<A>, b: F::Of<A>, c: F::Of<A>) -> TestCaseResult
    where
        F: SemigroupK,
        F::Of<A>: Clone + PartialEq + Debug,
    {
        // Associativity laws:
        // (a.combine_k(b)).combine_k(c) == a.combine_k(b.combine_k(c))
        let left_assoc = F::combine_k(F::combine_k(a.clone(), b.clone()), c.clone());
        let right_assoc = F::combine_k(a, F::combine_k(b, c));

        prop_assert_eq!(left_assoc, right_assoc);

        Ok(())
    }

    macro_rules! semigroup_k_kaws {
        ( $( $test_name:ident => $aa:ty => $hkt:ty ),+ $(,)* ) => {
            proptest! {
                $(
                    #[test]
                    fn $test_name((a, b, c) in triple(any::<$aa>())) {
                        assert_semigroup_k_laws::<_, $hkt>(a, b, c)?;
                    }
                )+
            }
        };
    }

    semigroup_k_kaws!(
        option_associativity => Option<i32> => OptionK,
        vec_associativity => Vec<i32> => VecK
    );
}
