use crate::{hkt::HKT, semigroup_k::SemigroupK};

/// MonoidK is a universal monoid which operators on kinds.
///
/// Here's how to distinguish [`Monoid`] and MonoidK:
/// * [`Monoid`] allows `A` values (or `Self`) to be combined, and also means that there is an
///   [`"empty"`] `A` value that functions as an identity.
/// * `MonoidK<F>` allows two [`F<A>`] values to be combined, for any `A`. It also means that for
///   any `A`, there is an [`"empty" F<A>`] value. The combination operation and empty value just
///   depend on the structure of `F`, but not on the structure of `A`.
///
/// [`Monoid`]: gatos_kernel::monoid::Monoid
/// [`"empty"`]: gatos_kernel::monoid::Monoid::empty
/// [`"empty" F<A>`]: MonoidK::empty
/// [`F<A>`]: HKT
pub trait MonoidK: HKT + SemigroupK {
    /// Given a type `A`, create an "empty" `F<A>` value.
    ///
    fn empty<A>() -> Self::Of<A>;

    fn is_empty<A>(a: Self::Of<A>) -> bool
    where
        Self::Of<A>: PartialEq,
    {
        a == Self::empty()
    }
}
