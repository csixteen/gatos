use crate::{invariant::Invariant, semigroupal::Semigroupal};

pub trait InvariantSemigroupal: Invariant + Semigroupal {}
