pub trait Semigroupal {
    type Inner;
    type Out<B>;

    fn product<B>(self, fb: Self::Out<B>) -> Self::Out<(Self::Inner, B)>;
}
