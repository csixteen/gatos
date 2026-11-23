use crate::{monoid::Monoid, semigroup::Semigroup};

impl<T> Semigroup for Option<T>
where
    T: Semigroup,
{
    fn combine(self, right: Self) -> Self {
        match (self, right) {
            (None, None) => None,
            (a @ Some(_), None) => a,
            (None, b @ Some(_)) => b,
            (Some(a), Some(b)) => Some(a.combine(b)),
        }
    }
}

impl<T> Monoid for Option<T>
where
    T: Semigroup,
{
    fn empty() -> Self {
        None
    }
}
