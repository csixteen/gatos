pub trait HKT {
    /// The inner type `A` in `F<A>`
    type Inner;

    /// `F<B>`
    type Out<B>: HKT<Inner = B>;
}
