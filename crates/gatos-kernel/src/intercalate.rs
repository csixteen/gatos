use crate::semigroup::Semigroup;

pub struct Intercalate<T> {
    value: T,
    middle: T,
}

impl<T> Intercalate<T> {
    pub fn new(value: T, middle: T) -> Self {
        Intercalate { value, middle }
    }

    pub fn into_inner(self) -> T {
        self.value
    }

    pub fn into_tuple(self) -> (T, T) {
        (self.value, self.middle)
    }
}

impl<T: Clone> Intercalate<T> {
    pub fn value(&self) -> T {
        self.value.clone()
    }

    pub fn middle(&self) -> T {
        self.middle.clone()
    }
}

impl<T> From<(T, T)> for Intercalate<T>
where
    T: Semigroup,
{
    fn from(value: (T, T)) -> Self {
        Intercalate {
            value: value.0,
            middle: value.1,
        }
    }
}
