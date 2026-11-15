use crate::hkt::HKT;

pub trait Semigroupal: HKT {
    fn product<B>(self, fb: <Self as HKT>::Out<B>)
    -> <Self as HKT>::Out<(<Self as HKT>::Inner, B)>;
}
