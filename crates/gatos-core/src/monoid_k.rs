use crate::semigroup_k::SemigroupK;

pub trait MonoidK: SemigroupK {
    fn empty<B>() -> Self;
}
