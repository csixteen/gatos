use crate::hkt::HKT;

pub trait Semigroupal: HKT {
    fn product<A, B>(fa: Self::Of<A>, fb: Self::Of<B>) -> Self::Of<(A, B)>;
}
