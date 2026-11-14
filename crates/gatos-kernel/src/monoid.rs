use std::num::NonZeroUsize;

use crate::semigroup::Semigroup;

/// A monoid extends a [`Semigroup`] with `empty`, which is the identity element for the associative
/// operator in the semigroup.
pub trait Monoid: Semigroup {
    fn empty() -> Self;

    fn combine_n(self, n: NonZeroUsize) -> Self
    where
        Self: Clone,
    {
        Self::repeated_combine_n(self, n)
    }

    fn combine_all(xs: impl Iterator<Item = Self>) -> Self {
        xs.fold(Self::empty(), Self::combine)
    }

    fn combine_all_option(xs: impl Iterator<Item = Self>) -> Option<Self> {
        // The implementation in Scala Cats checks whether the `IterableOnce` is empty. I can just
        // use `Iterator::reduce`, which does the same.
        xs.reduce(Self::combine)
    }
}

#[cfg(test)]
mod laws {
    use std::fmt::Debug;

    use proptest::{prelude::*, test_runner::TestCaseResult};

    use super::*;

    fn assert_monoid_laws<T>(a: T) -> TestCaseResult
    where
        T: Monoid + Clone + PartialEq + Debug,
    {
        // Left identity: `combine(a, empty) == a`
        let left_id = a.clone().combine(T::empty());

        // Right identity: `combine(empty, a) == a`
        let right_id = T::empty().combine(a.clone());

        prop_assert_eq!(left_id.clone(), right_id);
        prop_assert_eq!(left_id, a);

        Ok(())
    }

    macro_rules! monoid_laws {
        ( $( $test_name:ident => $strategy:expr ),+ $(,)* ) => {
            proptest! {
                $(
                    #[test]
                    fn $test_name(a in $strategy) {
                        assert_monoid_laws(a)?;
                    }
                )+
            }
        };
    }

    monoid_laws!(
        string_identity => any::<String>(),
        i32_identity => any::<i32>(),
        option_i32_identity => any::<Option<i32>>(),
        vec_i32_identity => any::<Vec<i32>>(),
    );
}
