use crate::{monoid::Monoid, semigroup::Semigroup};

impl<T> Semigroup for Vec<T> {
    fn combine(mut self, mut right: Self) -> Vec<T> {
        self.append(&mut right);
        self
    }
}

impl<T> Monoid for Vec<T> {
    fn empty() -> Self {
        Vec::new()
    }
}
