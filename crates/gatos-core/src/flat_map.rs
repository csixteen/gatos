use crate::{apply::Apply, hkt::HKT};

pub trait FlatMap<'a>: HKT + Apply<'a> {
    fn flat_map<A, B, F>(fa: Self::Of<A>, f: F) -> Self::Of<B>
    where
        F: FnOnce(A) -> Self::Of<B>;
}
