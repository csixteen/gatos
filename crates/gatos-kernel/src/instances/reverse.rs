use crate::{commutative_semigroup::CommutativeSemigroup, reverse::Reverse, semigroup::Semigroup};

impl<T> Semigroup for Reverse<T>
where
    T: Semigroup,
{
    fn combine(self, right: impl Into<Self>) -> Self {
        Reverse::new(T::combine(right.into().into_inner(), self.into_inner()))
    }
}

impl<T> CommutativeSemigroup for Reverse<T> where T: CommutativeSemigroup {}
