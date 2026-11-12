use std::num::NonZeroUsize;

/// A semigroup is a set with an associative operator that combines two elements from the set.
pub trait Semigroup: Sized {
    fn combine(self, right: Self) -> Self;

    fn combine_n(self, n: NonZeroUsize) -> Self
    where
        Self: Clone,
    {
        Self::repeated_combine_n(self, n)
    }

    fn combine_all_option(xs: impl Iterator<Item = Self>) -> Option<Self> {
        xs.reduce(Self::combine)
    }

    /// Between each pair of elements insert middle.
    // In Scala Cats, the method signature is as follows:
    // `def intercalate(middle: A): Semigroup[A]`
    //
    // In my case, a type `T` can only have one canonical implementation of `Semigroup`
    // (`impl Semigroup for T`), which means that returning a modified `Semigroup` requires
    // wrapping `T` in a newtype struct, for example. I decided to return `Intercalate`, which
    // implements `Semigroup` (Rust uses similar patterns, such as `Rev`).
    fn intercalate(self, middle: Self) -> Intercalate<Self> {
        Intercalate {
            value: self,
            middle,
        }
    }

    fn repeated_combine_n(self, n: NonZeroUsize) -> Self
    where
        Self: Clone,
    {
        #[inline]
        fn _repeated<T: Semigroup + Clone>(y: T, k: usize, acc: T) -> T {
            if k == 1 {
                acc
            } else {
                _repeated(y.clone(), k - 1, acc.combine(y))
            }
        }

        _repeated(self.clone(), n.into(), self)
    }
}

pub struct Intercalate<T> {
    pub value: T,
    pub middle: T,
}

impl<T> Intercalate<T> {
    pub fn new(value: T, middle: T) -> Self {
        Intercalate { value, middle }
    }

    pub fn into_inner(self) -> T {
        self.value
    }
}

impl<T> Semigroup for Intercalate<T>
where
    T: Semigroup + Clone,
{
    fn combine(self, right: Self) -> Self {
        let combined = self.value.combine(self.middle.clone().combine(right.value));
        Intercalate {
            value: combined,
            ..self
        }
    }
}

#[cfg(test)]
mod laws {
    use std::fmt::Debug;

    use proptest::{prelude::*, test_runner::TestCaseResult};

    use super::*;
    use crate::strategy::triple;

    fn assert_semigroup_laws<T>(a: T, b: T, c: T) -> TestCaseResult
    where
        T: Semigroup + Clone + PartialEq + Debug,
    {
        // Associativity laws:
        // (a.combine(b)).combine(c) == a.combine(b.combine(c))
        let left_assoc = a.clone().combine(b.clone()).combine(c.clone());
        let right_assoc = a.combine(b.combine(c));

        prop_assert_eq!(left_assoc, right_assoc);

        Ok(())
    }

    macro_rules! semigroup_laws {
        ( $( $test_name:ident => $strategy:expr ),+ $(,)? ) => {
            proptest! {
                $(
                    #[test]
                    fn $test_name((a, b, c) in triple($strategy)) {
                        assert_semigroup_laws(a, b, c)?;
                    }
                )+
            }
        };
    }

    semigroup_laws!(
        string_associativity => any::<String>(),
        i32_associativity => any::<i32>(),
        option_i32_associativity => any::<Option<i32>>(),
        result_i32_associativity => any::<Result<i32, ()>>()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_intercalate() {
        let words = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let combined = Semigroup::combine_all_option(
            words
                .into_iter()
                .map(|x| Intercalate::new(x, ", ".to_string())),
        )
        .map(Intercalate::into_inner);

        assert_eq!(combined, Some("a, b, c".to_string()));
    }
}
