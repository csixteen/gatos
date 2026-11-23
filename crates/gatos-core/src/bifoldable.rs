use gatos_kernel::monoid::Monoid;

use crate::hkt::HKT2;

/// A type class abstraction over types that give rise to two independent [`Foldable`]s.
///
/// [`Foldable`]: super::foldable::Foldable
pub trait Bifoldable: HKT2 {
    /// Collapse the structure with a left-associative function.
    fn bifold_left<A, B, C, F, G>(fab: Self::Of<A, B>, c: C, f: F, g: G) -> C
    where
        F: Fn(C, A) -> C,
        G: Fn(C, B) -> C;

    /// Collapse the structure with a right-associative function.
    fn bifold_right<A, B, C, F, G>(fab: Self::Of<A, B>, c: C, f: F, g: G) -> C
    where
        F: Fn(A, C) -> C,
        G: Fn(B, C) -> C;

    /// Collapse the structure by mapping each element to an element of a type that has a
    /// [`Monoid`].
    ///
    /// [`Monoid`]: gatos_kernel::monoid::Monoid
    fn bifold_map<A, B, C, F, G>(fab: Self::Of<A, B>, f: F, g: G) -> C
    where
        C: Monoid,
        F: Fn(A) -> C,
        G: Fn(B) -> C,
    {
        Self::bifold_left(
            fab,
            C::empty(),
            |c, a| C::combine(c, f(a)),
            |c, b| C::combine(c, g(b)),
        )
    }

    fn bifold<A, B>(fab: Self::Of<A, B>) -> (A, B)
    where
        A: Monoid,
        B: Monoid,
        (A, B): Monoid,
    {
        Self::bifold_map(fab, |a| (a, B::empty()), |b| (A::empty(), b))
    }
}
