use crate::{commutative_semigroup::CommutativeSemigroup, reverse::Reverse, semigroup::Semigroup};

impl<T> Semigroup for Reverse<T>
where
    T: Semigroup,
{
    fn combine(self, right: Self) -> Self {
        Reverse::new(T::combine(right.into_inner(), self.into_inner()))
    }
}

impl<T> CommutativeSemigroup for Reverse<T> where T: CommutativeSemigroup {}
