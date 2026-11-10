use crate::core::invariant::Invariant;

pub trait Functor: Invariant {
    type Inner;
    type Out<B>;

    fn map<B, F>(self, f: F) -> <Self as Functor>::Out<B>
    where
        F: FnOnce(<Self as Functor>::Inner) -> B;

    /// Alias for [`Functor::map()`]
    fn fmap<B, F>(self, f: F) -> <Self as Functor>::Out<B>
    where
        Self: Sized,
        F: FnOnce(<Self as Functor>::Inner) -> B,
    {
        self.map(f)
    }

    /// Lift a function `f` to operator on Functors.
    fn lift<B, F>(f: F) -> impl FnOnce(Self) -> <Self as Functor>::Out<B>
    where
        Self: Sized,
        F: FnOnce(<Self as Functor>::Inner) -> B,
    {
        move |fa: Self| fa.fmap(f)
    }

    /// Replaces the `A` value in `F[A]` with the supplied value.
    fn r#as<B>(self, b: B) -> <Self as Functor>::Out<B>
    where
        Self: Sized,
    {
        self.map(|_| b)
    }

    fn void(self) -> <Self as Functor>::Out<()>
    where
        Self: Sized,
    {
        self.r#as(())
    }
}
