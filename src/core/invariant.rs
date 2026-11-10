pub trait Invariant {
    type Inner;
    type Out<B>;

    fn imap<B, F, G>(self, f: F, g: G) -> Self::Out<B>
    where
        F: Fn(Self::Inner) -> B,
        G: Fn(B) -> Self::Inner;
}

#[cfg(test)]
// We don't actually need to run the tests
#[allow(unused)]
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
}
