use crate::functor::Functor;

pub trait Apply: Functor {
    /// This is the equivalent of Haskell's `<*>`, whose signature is:
    ///
    /// ```text
    /// (<*>) :: f (a -> b) -> f a -> f b
    /// ```
    ///
    /// It can be thought of as function application inside a Functor.
    fn ap<A, B, F>(
        ff: <Self as Functor>::Out<F>,
        fa: <Self as Functor>::Out<A>,
    ) -> <Self as Functor>::Out<B>
    where
        F: FnOnce(A) -> B;

    /// Compose two actions, discarding any value produced by the first.
    fn product_r<B>(self, fb: <Self as Functor>::Out<B>) -> <Self as Functor>::Out<B>
    where
        Self: Sized,
    {
        let ff = self.r#as(|b| b);
        Self::ap(ff, fb)
    }

    fn product<B>(
        self,
        fb: <Self as Functor>::Out<B>,
    ) -> <Self as Functor>::Out<(<Self as Functor>::Inner, B)>
    where
        Self: Sized,
    {
        let ff = self.map(|a| move |b| (a, b));
        Self::ap(ff, fb)
    }
}
