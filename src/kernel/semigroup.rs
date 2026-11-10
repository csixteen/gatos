/// A semigroup is a set with an associative operator that combines two elements from the set.
pub trait Semigroup {
    fn combine(self, x: Self) -> Self;
}
