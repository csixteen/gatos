use crate::{
    applicative::Applicative, apply::Apply, flat_map::FlatMap, functor::Functor, hkt::HKT,
    invariant::Invariant, invariant_monoidal::InvariantMonoidal, semigroup_k::SemigroupK,
    semigroupal::Semigroupal,
};

impl<T, E> HKT for Result<T, E> {
    type Inner = T;

    type Out<B> = Result<B, E>;
}

impl<'a, T: 'a, E> Invariant<'a> for Result<T, E> {
    fn imap<B, F, G>(self, f: F, _g: G) -> Self::Out<B>
    where
        F: Fn(Self::Inner) -> B,
        G: Fn(B) -> Self::Inner,
    {
        self.map(f)
    }
}

impl<'a, T: 'a, E> Functor<'a> for Result<T, E> {
    fn map<B, F>(self, f: F) -> <Self as HKT>::Out<B>
    where
        F: FnOnce(<Self as HKT>::Inner) -> B,
    {
        self.map(f)
    }
}

impl<T, E> Semigroupal for Result<T, E> {
    fn product<B>(
        self,
        fb: <Self as HKT>::Out<B>,
    ) -> <Self as HKT>::Out<(<Self as HKT>::Inner, B)> {
        Ok((self?, fb?))
    }
}

impl<'a, T: 'a, E> InvariantMonoidal<'a> for Result<T, E> {
    fn unit() -> <Self as HKT>::Out<()> {
        Ok(())
    }
}

impl<T, E> SemigroupK for Result<T, E> {
    fn combine_k(self, b: Self) -> Self {
        // same reason as `Option::combine_k()`
        self.or(b)
    }
}

impl<'a, T: 'a, E> Apply<'a> for Result<T, E> {
    fn ap<A, B, F>(ff: <Self as HKT>::Out<F>, fa: <Self as HKT>::Out<A>) -> <Self as HKT>::Out<B>
    where
        F: FnOnce(A) -> B,
    {
        Ok(ff?(fa?))
    }
}

impl<'a, T: 'a, E> Applicative<'a> for Result<T, E> {
    fn pure<B>(x: B) -> <Self as HKT>::Out<B> {
        Ok(x)
    }
}

impl<'a, T: 'a, E> FlatMap<'a> for Result<T, E> {
    fn flat_map<B, F>(self, f: F) -> <Self as HKT>::Out<B>
    where
        F: Fn(<Self as HKT>::Inner) -> <Self as HKT>::Out<B>,
    {
        self.and_then(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_result_apply() {
        assert_eq!(
            Ok(2_i32),
            Result::<i32, ()>::ap(Ok(|x: i32| x + 1), Ok(1_i32))
        );
        assert_eq!(Err(()), Result::<i32, ()>::ap(Ok(|x: i32| x + 1), Err(())));
    }
}
