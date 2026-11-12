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
mod tests {
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
    struct Codec<A> {
        encode: Box<dyn Fn(A) -> String>,
        #[allow(clippy::type_complexity)]
        decode: Box<dyn Fn(&str) -> Option<A>>,
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
                encode: Box::new(move |b| encode(g(b))),
                decode: Box::new(move |s| decode(s).map(&f)),
            }
        }
    }

    #[test]
    fn test_invariant_on_invariant_types() {
        let int_codec = Codec {
            encode: Box::new(|n: i32| n.to_string()),
            decode: Box::new(|s| s.parse::<i32>().ok()),
        };
        let bool_codec: Codec<bool> = int_codec.imap(|n| n != 0, |b| if b { 1 } else { 0 });

        assert_eq!((bool_codec.encode)(true), "1");
        assert_eq!((bool_codec.decode)("1"), Some(true));
        assert_eq!((bool_codec.decode)("invalid"), None);
    }
}
