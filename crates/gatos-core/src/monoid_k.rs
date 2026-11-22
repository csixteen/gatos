use num::Zero;

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
    fn empty<A>() -> Self::Of<A>;

    fn is_empty<A>(a: Self::Of<A>) -> bool
    where
        Self::Of<A>: PartialEq,
    {
        a == Self::empty()
    }

    fn combine_n_k<A>(a: Self::Of<A>, n: usize) -> Self::Of<A>
    where
        Self: Sized,
        Self::Of<A>: Clone,
    {
        if n.is_zero() {
            Self::empty()
        } else {
            <Self as SemigroupK>::combine_n_k(a, n.try_into().expect("should be non-zero"))
        }
    }

    fn combine_all_k<A>(xs: impl Iterator<Item = Self::Of<A>>) -> Self::Of<A> {
        match <Self as SemigroupK>::combine_all_option_k(xs) {
            Some(fa) => fa,
            None => Self::empty(),
        }
    }
}
