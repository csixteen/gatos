use crate::{apply::Apply, hkt::HKT};

pub trait FlatMap<'a>: HKT + Apply<'a> {
    fn flat_map<B, F>(self, f: F) -> <Self as HKT>::Out<B>
    where
        F: Fn(<Self as HKT>::Inner) -> <Self as HKT>::Out<B>;
}
