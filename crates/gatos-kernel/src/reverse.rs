use crate::semigroup::Semigroup;

pub struct Reverse<T>(T);

impl<T> Reverse<T> {
    pub fn new(value: T) -> Self {
        Reverse(value)
    }

    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> From<T> for Reverse<T>
where
    T: Semigroup,
{
    fn from(value: T) -> Self {
        Reverse(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_combine() {
        let normal = ", world!".to_string().combine("Hello".to_string());
        let rev = "Hello"
            .to_string()
            .reverse()
            .combine(", world!".to_string());
        assert_eq!(normal, rev.into_inner());
    }
}
