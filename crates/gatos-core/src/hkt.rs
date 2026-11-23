/// For Higher Kinded Types `F[_]`
pub trait HKT {
    /// The type produced, e.g. `Option<A>`
    type Of<A>;
}

/// For Higher Kinded Types `F[_, _]` (e.g. Bifoldable)
// Is this necessary? Seems to be easier than trying to force single-parameter HKT to handle two
// type arguments..
pub trait HKT2 {
    type Of<A, B>;
}
