use crate::kernel::semigroup::Semigroup;

/// Similar to [`Semigroup`], with the difference that it operates on kinds,
/// i.e., on type constructors. It only operates on type constructors with one parameter
/// (the equivalent of Scala's `F[_]`). Because of that, we can find [`Semigroup`] implementation
/// for concret types, such as `i32`, but can only find [`SemigroupK`] implementation for
/// type constructors, such as `Option<T>`.
///
/// [`Semigroup`]: crate::kernel::semigroup::Semigroup
pub trait SemigroupK: Sized {
    type Inner;
    type Out<B>;

    /// Combines two `F[_]`.
    fn combine_k(self, b: Self) -> Self;

    // Notes for self:
    //
    // In Scala Cats, `SemigroupK[F].algebra[A]` converts `F[A]` into a standard `Semigroup[F[A]]`.
    // In Rust, a blanket implementation of `impl<F: SemigroupK> Semigroup for F {}` would conflict
    // with concret `Semigroup` implementations, e.g., `Option<T>` where `T: Semigroup`. As a
    // workaround, we can use `Algebra<T>`, which wraps `Self`, and implements `Semigroup` by
    // delegating `combine` to `combine_k`.
    fn algebra(self) -> Algebra<Self> {
        Algebra(self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Algebra<T>(pub T);

impl<T: SemigroupK> Semigroup for Algebra<T> {
    fn combine(self, right: Self) -> Self {
        Algebra(self.0.combine_k(right.0))
    }
}

#[cfg(test)]
mod laws {
    use std::fmt::Debug;

    use proptest::{prelude::*, test_runner::TestCaseResult};

    use super::*;
    use crate::strategy::triple;

    fn assert_semigroup_k_laws<T>(a: T, b: T, c: T) -> TestCaseResult
    where
        T: SemigroupK + Clone + PartialEq + Debug,
    {
        // Associativity laws:
        // (a.combine_k(b)).combine_k(c) == a.combine_k(b.combine_k(c))
        let left_assoc = a.clone().combine_k(b.clone()).combine_k(c.clone());
        let right_assoc = a.combine_k(b.combine_k(c));

        prop_assert_eq!(left_assoc, right_assoc);

        Ok(())
    }

    macro_rules! semigroup_k_kaws {
        ( $( $test_name:ident => $strategy:expr ),+ $(,)* ) => {
            proptest! {
                $(
                    #[test]
                    fn $test_name((a, b, c) in triple($strategy)) {
                        assert_semigroup_k_laws(a, b, c)?;
                    }
                )+
            }
        };
    }

    semigroup_k_kaws!(
      option_associativity => any::<Option<i32>>(),
      vec_associativity => any::<Vec<i32>>()
    );
}
