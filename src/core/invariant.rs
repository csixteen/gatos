/// "Type class" for functors that define `imap`, which transforms an `F[A]` into an `F[B]`,
/// given two transformations, one from `A` to `B` and another from `B` to `A`.
pub trait Invariant {
    type Inner;
    type Out<B>;

    fn imap<B, F, G>(self, f: F, g: G) -> Self::Out<B>
    where
        F: Fn(Self::Inner) -> B,
        G: Fn(B) -> Self::Inner;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestContainer<T>(T);

    impl<A> Invariant for TestContainer<A> {
        type Inner = A;

        type Out<B> = TestContainer<B>;

        fn imap<B, F, G>(self, f: F, _g: G) -> Self::Out<B>
        where
            F: Fn(Self::Inner) -> B,
            G: Fn(B) -> Self::Inner,
        {
            TestContainer(f(self.0))
        }
    }

    #[test]
    fn test_container_invariant() {
        let original = TestContainer(5_i32);
        let transformed =
            original.imap(|x| x.to_string(), |y| y.parse::<i32>().unwrap_or_default());
        assert_eq!(transformed.0, "5");
    }
}
