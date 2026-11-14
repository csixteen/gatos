use gatos_kernel::semigroup::Semigroup;

use crate::semigroup_k::SemigroupK;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Algebra<T>(pub T);

impl<T: SemigroupK> Semigroup for Algebra<T> {
    fn combine(self, right: Self) -> Self {
        Algebra(self.0.combine_k(right.0))
    }
}
