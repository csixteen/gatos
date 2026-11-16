use crate::{commutative_semigroup::CommutativeSemigroup, monoid::Monoid, reverse::Reverse};

pub trait CommutativeMonoid: Monoid + CommutativeSemigroup {
    fn reverse(self) -> Reverse<Self> {
        Reverse::new(self)
    }
}
