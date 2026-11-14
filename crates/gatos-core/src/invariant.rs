/// "Type class" for functors that define `imap`, which transforms an `F[A]` into an `F[B]`,
/// given two transformations, one from `A` to `B` and another from `B` to `A`.
pub trait Invariant {
    // the inner type `A` inside `F<A>`
    type Inner: 'static;
    // `F<B>` for any type `B`
    type Out<B: 'static>: Invariant<Inner = B>;

    /// Transforms `F<A>` into `F<B>` using an isomorphism, i.e., `f: A -> B and g: B -> A`
    fn imap<B, F, G>(self, f: F, g: G) -> Self::Out<B>
    where
        B: 'static,
        F: Fn(Self::Inner) -> B + 'static,
        G: Fn(B) -> Self::Inner + 'static;
}

#[cfg(test)]
mod laws {
    use std::convert::identity;

    use proptest::{prelude::*, test_runner::TestCaseResult};

    use super::{tests::Codec, *};

    fn assert_invariant_identity<T, A, F>(a: T, eq: F) -> TestCaseResult
    where
        A: 'static,
        T: Invariant<Inner = A, Out<A> = T> + Clone,
        F: Fn(&T, &T) -> bool,
    {
        let left = a.clone().imap(identity, identity);
        let right = a;

        prop_assert!(eq(&left, &right));

        Ok(())
    }

    macro_rules! identity_laws {
        ( $( $test_name:ident => $strategy:expr ),+ $(,)* ) => {
            proptest! {
                $(
                    #[test]
                    fn $test_name((values, inputs) in $strategy) {
                        let a = Codec::<i32>::new();
                        assert_invariant_identity(a, |left, right| {
                            Codec::eq(left, right, &values, &inputs)
                        })?;
                    }
                )+
            }
        };
    }

    identity_laws!(
        codec_i32_identity => (
            prop::collection::vec(-10_000i32..10_000, 0..30),
            prop::collection::vec(any::<String>(), 0..30)
        )
    );
}

#[cfg(test)]
mod tests {
    use std::{str::FromStr, sync::Arc};

    use super::*;

    struct TestContainer<T>(T);

    impl<A: 'static> Invariant for TestContainer<A> {
        type Inner = A;

        type Out<B: 'static> = TestContainer<B>;

        fn imap<B, F, G>(self, f: F, _g: G) -> Self::Out<B>
        where
            B: 'static,
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

    // Trying to emulate the implementation of Invariant on types that are actually invariant,
    // i.e., consume and produce `A`. This is the case for codecs, bidirectional parsers,
    // endomorphisms (A -> A), etc.
    #[derive(Clone)]
    pub(super) struct Codec<A> {
        encode: Arc<dyn Fn(A) -> String>,
        #[allow(clippy::type_complexity)]
        decode: Arc<dyn Fn(&str) -> Option<A>>,
    }

    impl<A> Codec<A>
    where
        A: ToString + FromStr + Copy + PartialEq,
    {
        pub fn new() -> Codec<A> {
            Codec {
                encode: Arc::new(|n: A| n.to_string()),
                decode: Arc::new(|s| s.parse::<A>().ok()),
            }
        }

        pub(super) fn eq(
            left: &Codec<A>,
            right: &Codec<A>,
            values: &[A],
            inputs: &[String],
        ) -> bool {
            values
                .iter()
                .all(|v| (left.encode)(*v) == (right.encode)(*v))
                && inputs.iter().all(|i| (left.decode)(i) == (right.decode)(i))
        }
    }

    impl<A: 'static> Invariant for Codec<A> {
        type Inner = A;

        type Out<B: 'static> = Codec<B>;

        fn imap<B, F, G>(self, f: F, g: G) -> Self::Out<B>
        where
            B: 'static,
            F: Fn(Self::Inner) -> B + 'static,
            G: Fn(B) -> Self::Inner + 'static,
        {
            let encode = self.encode;
            let decode = self.decode;

            Codec {
                encode: Arc::new(move |b| encode(g(b))),
                decode: Arc::new(move |s| decode(s).map(&f)),
            }
        }
    }

    #[test]
    fn test_invariant_on_invariant_types() {
        // * decoding (`&str -> Option<A>`) is Covariant, i.e., it produces an `A`. In other words,
        // to get a `bool` out of a `String`, we first decode `i32` and then convert to `bool` using
        // `f`.
        // * encoding (`A -> String`) is Contravariant, i.e., it consumes `A`. In other words, in
        //   order to encode a `bool` into a `String`, we cannot pass the `bool` directly into
        //   `int_codec.encode` (it expects an `i32`). First, we need to convert `bool` into `i32`
        //   using `g`.
        let int_codec = Codec::<i32>::new();
        let bool_codec: Codec<bool> = int_codec.imap(|n| n != 0, |b| if b { 1 } else { 0 });

        assert_eq!((bool_codec.encode)(true), "1");
        assert_eq!((bool_codec.decode)("1"), Some(true));
        assert_eq!((bool_codec.decode)("5"), Some(true));
        assert_eq!((bool_codec.decode)("invalid"), None);
    }
}
