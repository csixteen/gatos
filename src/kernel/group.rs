use crate::kernel::monoid::Monoid;

/// A Group is a [`Monoid`] where each element has an `inverse`.
///
/// `combine(self, inverse(self))` = `combine(inverse(self), self)` = `empty`
pub trait Group: Monoid {
    fn inverse(value: Self) -> Self;

    /// Removes `right` from `left`. Equivalent to `combine(left, inverse(right))`.
    fn remove(left: Self, right: Self) -> Self
    where
        Self: Sized,
    {
        Self::combine(left, Self::inverse(right))
    }
}
