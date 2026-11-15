use gatos_kernel::semigroup::Semigroup;

use crate::{hkt::HKT, semigroup_k::SemigroupK};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Algebra<T>(pub T);

impl<T> HKT for Algebra<T> {
    type Inner = T;

    type Out<B> = Algebra<B>;
}

impl<T: SemigroupK> Semigroup for Algebra<T> {
    fn combine(self, right: Self) -> Self {
        Algebra(self.0.combine_k(right.0))
    }
}
