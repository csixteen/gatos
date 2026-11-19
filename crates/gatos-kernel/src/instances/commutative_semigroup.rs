use num::Num;

use crate::{commutative_semigroup::CommutativeSemigroup, semigroup::Semigroup};

impl<T> CommutativeSemigroup for T where T: Semigroup + Num {}
