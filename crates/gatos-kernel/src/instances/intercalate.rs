use crate::{intercalate::Intercalate, semigroup::Semigroup};

impl<T> Semigroup for Intercalate<T>
where
    T: Semigroup + Clone,
{
    fn combine(self, right: Self) -> Self {
        let (value, middle) = self.into_tuple();
        let combined = value.combine(middle.clone().combine(right.into_inner()));
        Intercalate::new(combined, middle)
    }
}
