use crate::core::{
    functor::Functor, invariant::Invariant, semigroup_k::SemigroupK, semigroupal::Semigroupal,
};

impl<T: 'static> Invariant for Option<T> {
    type Inner = T;

    type Out<B: 'static> = Option<B>;

    fn imap<B: 'static, F, G>(self, f: F, _g: G) -> Self::Out<B>
    where
        F: Fn(Self::Inner) -> B,
        G: Fn(B) -> Self::Inner,
    {
        // Notes for self:
        //
        // There are a few differences between Scala Cats hierarchy and Rust. In Scala Cats (and in
        // Category Theory), all Functor (covariant types) are also automatically Invariant types:
        // * If a type `F[_]` is Covariant, then we can change `F[A]` to `F[B]` using only `A => B`
        // * If a type is Contravariant, we can change `F[A]` to `F[B]` using only `B => A`.
        // * If a type is Invariant, then we need to use both transformations.
        //
        // `Option<T>` is already covariant, it only requires one of the transformations, which is
        // why we ignore `_g`.
        self.map(f)
    }
}

impl<T: 'static> Functor for Option<T> {
    type Inner = T;

    type Out<B> = Option<B>;

    fn map<B, F>(self, f: F) -> <Self as Functor>::Out<B>
    where
        F: FnOnce(<Self as Functor>::Inner) -> B,
    {
        self.map(f)
    }
}

impl<T> Semigroupal for Option<T> {
    type Inner = T;

    type Out<B> = Option<B>;

    fn product<B>(self, fb: Self::Out<B>) -> Self::Out<(Self::Inner, B)> {
        match (self, fb) {
            (Some(t), Some(b)) => Some((t, b)),
            _ => None,
        }
    }
}

impl<T> SemigroupK for Option<T> {
    type Inner = T;
    type Out<B> = Option<B>;

    fn combine_k(self, b: Self) -> Self {
        // The inner type `T` of `Option<T>` is not fully specified and can be anything,
        // therefor we don't know how to combine them, so we just use `Option::or()`.
        self.or(b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semigroupal_option() {
        let a = Some(3_i32);
        let b = Some(String::from("foo"));

        assert_eq!(None, Option::<i32>::None.product(Option::<String>::None));
        assert_eq!(None, a.product(Option::<String>::None));
        assert_eq!(None, b.clone().product(Option::<i32>::None));
        assert_eq!(Some((3_i32, String::from("foo"))), a.product(b));
    }
}
