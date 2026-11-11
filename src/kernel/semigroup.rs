/// A semigroup is a set with an associative operator that combines two elements from the set.
pub trait Semigroup: Sized {
    fn combine(self, right: Self) -> Self;
}

#[cfg(test)]
mod laws {
    use std::fmt::Debug;

    use proptest::{prelude::*, test_runner::TestCaseResult};

    use super::*;
    use crate::strategy::triple;

    fn assert_semigroup_laws<T>(a: T, b: T, c: T) -> TestCaseResult
    where
        T: Semigroup + Clone + PartialEq + Debug,
    {
        // Associativity laws:
        // (a.combine(b)).combine(c) == a.combine(b.combine(c))
        let left_assoc = a.clone().combine(b.clone()).combine(c.clone());
        let right_assoc = a.combine(b.combine(c));

        prop_assert_eq!(left_assoc, right_assoc);

        Ok(())
    }

    macro_rules! semigroup_laws {
        ( $( $test_name:ident => $strategy:expr ),+ $(,)? ) => {
            proptest! {
                $(
                    #[test]
                    fn $test_name((a, b, c) in triple($strategy)) {
                        assert_semigroup_laws(a, b, c)?;
                    }
                )+
            }
        };
    }

    semigroup_laws!(
        string_associativity => any::<String>(),
        i32_associativity => any::<i32>(),
        option_i32_associativity => any::<Option<i32>>(),
        result_i32_associativity => any::<Result<i32, ()>>()
    );
}
