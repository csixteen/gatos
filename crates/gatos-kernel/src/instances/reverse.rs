use crate::{commutative_semigroup::CommutativeSemigroup, reverse::Reverse, semigroup::Semigroup};

impl<T> Semigroup for Reverse<T>
where
    T: Semigroup,
{
    fn combine(self, right: impl Into<Self>) -> Self {
        Reverse::new(T::combine(right.into().into_inner(), self.into_inner()))
    }
}

impl<T> CommutativeSemigroup for Reverse<T>
where
    T: CommutativeSemigroup,
{
    fn combine_n(self, n: std::num::NonZeroUsize) -> Self
    where
        Self: Clone,
    {
        // TODO: confirm if this is correct
        <Self as Semigroup>::combine_n(self, n)
    }
}
