/// A semigroup is a set with an associative operator that combines two elements from the set.
pub trait Semigroup: Sized {
    fn combine(left: Self, right: Self) -> Self;
}
