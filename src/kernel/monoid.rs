use crate::kernel::semigroup::Semigroup;

/// A monoid extends a [`Semigroup`] with `empty`, which is the identity element for the associative
/// operator in the semigroup.
pub trait Monoid: Semigroup {
    fn empty() -> Self;
}
