use crate::kernel::monoid::Monoid;

/// A Group is a [`Monoid`] where each element has an `inverse`.
///
/// `combine(self, inverse(self))` = `combine(inverse(self), self)` = `empty`
pub trait Group: Monoid {
    fn inverse(self) -> Self;

    /// Removes `b` from `self`. Equivalent to `combine(self, inverse(b))`.
    fn remove(self, b: Self) -> Self
    where
        Self: Sized,
    {
        self.combine(b.inverse())
    }
}
