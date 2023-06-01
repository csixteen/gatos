use crate::kernel::semigroup::Semigroup;

pub trait Monoid: Semigroup {
    fn empty() -> Self;
}

impl<T: Semigroup + Default> Monoid for T {
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
