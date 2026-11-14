use std::ops::Neg;

use crate::{group::Group, monoid::Monoid};

impl<T> Group for T
where
    T: Monoid + Neg<Output = Self>,
{
    fn inverse(self) -> Self {
        self.neg()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_negative_ints() {
        assert_eq!(-5_i32, 5_i32.inverse());
    }

    #[test]
    fn test_remove_ints() {
        assert_eq!(3_i32, 5_i32.remove(2_i32));
    }
}
