use crate::{invariant::Invariant, semigroupal::Semigroupal};

pub trait InvariantSemigroupal<'a>: Invariant<'a> + Semigroupal {}
