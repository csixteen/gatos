use crate::semigroup::Semigroup;

impl<T: Semigroup, E> Semigroup for Result<T, E>
where
    T: Semigroup,
{
    fn combine(self, right: Self) -> Self {
        match (self, right) {
            (_, b @ Err(_)) => b,
            (a @ Err(_), _) => a,
            (Ok(a), Ok(b)) => Ok(a.combine(b)),
        }
    }
}
