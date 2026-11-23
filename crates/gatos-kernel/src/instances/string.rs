use crate::{monoid::Monoid, semigroup::Semigroup};

impl Semigroup for String {
    fn combine(mut self, right: Self) -> Self {
        self.push_str(&right);
        self
    }
}

impl Monoid for String {
    fn empty() -> Self {
        String::new()
    }
}
