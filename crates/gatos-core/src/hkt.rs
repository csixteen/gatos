pub trait HKT {
    /// The type produce, e.g. `Option<A>`
    type Of<A>;
}
