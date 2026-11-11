use crate::kernel::semigroup::Semigroup;

/// A monoid extends a [`Semigroup`] with `empty`, which is the identity element for the associative
/// operator in the semigroup.
pub trait Monoid: Semigroup {
    fn empty() -> Self;
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
