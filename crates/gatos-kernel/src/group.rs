use crate::monoid::Monoid;

/// A Group is a [`Monoid`] where each element has an `inverse`.
///
/// `combine(self, inverse(self))` = `combine(inverse(self), self)` = `empty`
pub trait Group: Monoid {
    fn inverse(self) -> Self;

    /// Removes `right` from `self`. Equivalent to `combine(self, inverse(right))`.
    fn remove(self, right: Self) -> Self
    where
        Self: Sized,
    {
        self.combine(right.inverse())
    }
}

#[cfg(test)]
mod laws {
    use std::fmt::Debug;

    use proptest::{prelude::*, test_runner::TestCaseResult};

    use super::*;

    fn assert_group_laws<T>(a: T) -> TestCaseResult
    where
        T: Group + Clone + PartialEq + Debug,
    {
        // Left inverse: `empty == combine(inverse(a), a)`
        let left_inv = a.clone().inverse().combine(a.clone());

        // Right inverse: `empty == combine(a, inverse(a))`
        let right_inv = a.clone().combine(a.inverse());

        prop_assert_eq!(T::empty(), left_inv);
        prop_assert_eq!(T::empty(), right_inv);

        Ok(())
    }

    proptest! {
        #[test]
        fn i32_inverse(a in any::<i32>()) {
            assert_group_laws(a)?;
        }
    }
}
