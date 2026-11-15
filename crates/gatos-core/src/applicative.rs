use crate::{apply::Apply, hkt::HKT, invariant_monoidal::InvariantMonoidal};

pub trait Applicative<'a>: HKT + Apply<'a> + InvariantMonoidal<'a> {
    /// `pure` lifts any value into the Applicative Functor.
    fn pure<B>(x: B) -> <Self as HKT>::Out<B>;

    fn unit() -> <Self as HKT>::Out<()> {
        Self::pure(())
    }
}
