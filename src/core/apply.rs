/// Apply is a weaker version of an Applicative.
/// It has apply but doesn't have pure.
use crate::core::functor::Functor;

pub trait Apply: Functor {
    /// Given a function in the Apply context, it applies the
    /// function to the value (self).
    fn ap<B, F>(self, ff: Self::Out<F>) -> Self::Out<B>
    where
        F: FnMut(Self::In) -> B;
}

impl<T> Apply for Vec<T> {
    fn ap<B, F>(self, mut ff: Self::Out<F>) -> Self::Out<B>
    where
        F: FnMut(Self::In) -> B,
    {
        self.into_iter()
            .zip(ff.iter_mut())
            .map(|(a, f)| f(a))
            .collect()
    }
}

impl<T> Apply for Option<T> {
    fn ap<B, F>(self, ff: Self::Out<F>) -> Self::Out<B>
    where
        F: FnMut(Self::In) -> B,
    {
        self.fmap(ff?)
    }
}

impl<T, E> Apply for Result<T, E> {
    fn ap<B, F>(self, ff: Self::Out<F>) -> Self::Out<B>
    where
        F: FnMut(Self::In) -> B,
    {
        self.fmap(ff?)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_option_apply_some() {
        let some_f = Some(|x: i32| (x + 1) as i64);
        let some_i32 = Some(3_i32);
        assert_eq!(some_i32.ap(some_f), Some(4_i64));
    }

    #[test]
    fn test_option_apply_none() {
        let none_f: Option<fn(i32) -> i64> = None;
        let some_i32 = Some(3_i32);
        assert_eq!(some_i32.ap(none_f), None);
    }
}
