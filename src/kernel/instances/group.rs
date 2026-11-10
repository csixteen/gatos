use std::ops::Neg;

use crate::kernel::{group::Group, monoid::Monoid};

impl<T> Group for T
where
    T: Monoid + Neg<Output = Self>,
{
    fn inverse(value: Self) -> Self {
        value.neg()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_negative_ints() {
        assert_eq!(-5_i32, Group::inverse(5_i32));
    }

    #[test]
    fn test_remove_ints() {
        assert_eq!(3_i32, Group::remove(5_i32, 2_i32));
    }
}
