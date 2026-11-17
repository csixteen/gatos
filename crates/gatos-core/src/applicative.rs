use crate::{apply::Apply, hkt::HKT, invariant_monoidal::InvariantMonoidal};

pub trait Applicative<'a>: HKT + Apply<'a> + InvariantMonoidal<'a> {
    /// `pure` lifts any value into the Applicative Functor.
    fn pure<A>(a: A) -> Self::Of<A>;

    fn unit() -> Self::Of<()> {
        Self::pure(())
    }
}
