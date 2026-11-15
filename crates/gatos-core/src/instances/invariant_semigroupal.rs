use crate::{
    invariant::Invariant, invariant_semigroupal::InvariantSemigroupal, semigroupal::Semigroupal,
};

impl<'a, T> InvariantSemigroupal<'a> for T where T: Invariant<'a> + Semigroupal {}
