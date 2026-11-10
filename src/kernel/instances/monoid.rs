use crate::kernel::{monoid::Monoid, semigroup::Semigroup};

impl<T> Monoid for T
where
    T: Semigroup + Default,
{
    fn empty() -> Self {
        T::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monoid_default() {
        assert_eq!(0, u32::empty());
        assert_eq!(Vec::<u32>::new(), Vec::<_>::empty());
        assert_eq!(String::new(), String::empty());
        assert_eq!(None, Option::<u32>::empty());
    }
}
