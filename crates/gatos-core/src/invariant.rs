use crate::hkt::HKT;

/// "Type class" for functors that define `imap`, which transforms an `F[A]` into an `F[B]`,
/// given two transformations, one from `A` to `B` and another from `B` to `A`.
pub trait Invariant<'a>: HKT {
    /// Transforms `F<A>` into `F<B>` using an isomorphism, i.e., `f: A -> B and g: B -> A`
    fn imap<A, B, F, G>(fa: Self::Of<A>, f: F, g: G) -> Self::Of<B>
    where
        A: 'a,
        F: Fn(A) -> B + 'a,
        G: Fn(B) -> A + 'a;
}

#[cfg(test)]
mod laws {
    use proptest::{prelude::*, test_runner::TestCaseResult};

    use super::{
        tests::{Codec, CodecK},
        *,
    };

    fn assert_invariant_identity<'a, A: 'a, F, EQ>(fa: F::Of<A>, eq: EQ) -> TestCaseResult
    where
        F: Invariant<'a>,
        F::Of<A>: Clone,
        EQ: Fn(&F::Of<A>, &F::Of<A>) -> bool,
    {
        let mapped = F::imap(fa.clone(), |a| a, |a| a);
        prop_assert!(eq(&mapped, &fa));

        Ok(())
    }

    macro_rules! identity_laws {
        ( $( $test_name:ident => $strategy:expr ),+ $(,)* ) => {
            proptest! {
                $(
                    #[test]
                    fn $test_name((values, inputs) in $strategy) {
                        let a = Codec::<i32>::new();
                        assert_invariant_identity::<'_, i32, CodecK<'_>, _>(a, |left, right| {
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
    use std::{marker::PhantomData, str::FromStr, sync::Arc};

    use super::*;

    struct TestContainerK;

    impl HKT for TestContainerK {
        type Of<A> = TestContainer<A>;
    }

    struct TestContainer<T>(T);

    impl<'a> Invariant<'a> for TestContainerK {
        fn imap<A, B, F, G>(fa: Self::Of<A>, f: F, _g: G) -> Self::Of<B>
        where
            F: Fn(A) -> B,
            G: Fn(B) -> A,
        {
            TestContainer(f(fa.0))
        }
    }

    #[test]
    fn test_container_invariant() {
        let original = TestContainer(5_i32);
        let transformed = TestContainerK::imap(
            original,
            |x| x.to_string(),
            |y| y.parse::<i32>().unwrap_or_default(),
        );
        assert_eq!(transformed.0, "5");
    }

    // Trying to emulate the implementation of Invariant on types that are actually invariant,
    // i.e., consume and produce `A`. This is the case for codecs, bidirectional parsers,
    // endomorphisms (A -> A), etc.
    #[derive(Clone)]
    pub(super) struct Codec<'a, A> {
        encode: Arc<dyn Fn(A) -> String + 'a>,
        #[allow(clippy::type_complexity)]
        decode: Arc<dyn Fn(&str) -> Option<A> + 'a>,
    }

    impl<'a, A> Codec<'a, A>
    where
        A: ToString + FromStr + Copy + PartialEq,
    {
        pub fn new() -> Codec<'a, A> {
            Codec {
                encode: Arc::new(|n: A| n.to_string()),
                decode: Arc::new(|s| s.parse::<A>().ok()),
            }
        }

        pub(super) fn eq(
            left: &Codec<'a, A>,
            right: &Codec<'a, A>,
            values: &[A],
            inputs: &[String],
        ) -> bool {
            values
                .iter()
                .all(|v| (left.encode)(*v) == (right.encode)(*v))
                && inputs.iter().all(|i| (left.decode)(i) == (right.decode)(i))
        }
    }

    pub(super) struct CodecK<'a>(PhantomData<&'a ()>);

    impl<'a> HKT for CodecK<'a> {
        type Of<A> = Codec<'a, A>;
    }

    impl<'a> Invariant<'a> for CodecK<'a> {
        fn imap<A, B, F, G>(fa: Self::Of<A>, f: F, g: G) -> Self::Of<B>
        where
            A: 'a,
            F: Fn(A) -> B + 'a,
            G: Fn(B) -> A + 'a,
        {
            let encode = fa.encode;
            let decode = fa.decode;

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
        let bool_codec: Codec<'_, bool> =
            CodecK::imap(int_codec, |n| n != 0, |b| if b { 1 } else { 0 });

        assert_eq!((bool_codec.encode)(true), "1");
        assert_eq!((bool_codec.decode)("1"), Some(true));
        assert_eq!((bool_codec.decode)("5"), Some(true));
        assert_eq!((bool_codec.decode)("invalid"), None);
    }
}
