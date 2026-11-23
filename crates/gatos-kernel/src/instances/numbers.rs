use std::ops::Neg;

use crate::{group::Group, monoid::Monoid, semigroup::Semigroup};

impl Semigroup for f32 {
    fn combine(self, right: Self) -> Self {
        self + right
    }
}

impl Semigroup for f64 {
    fn combine(self, right: Self) -> Self {
        self + right
    }
}

macro_rules! semigroup_impl {
    ($($type:ty),*) => {$(
        impl Semigroup for $type {
            fn combine(self, right: $type) -> $type {
                self.wrapping_add(right.into())
            }
        }
    )*};
}

semigroup_impl!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
);

macro_rules! monoid_impl {
    ($($type:ty),*) => {$(
        impl Monoid for $type {
            fn empty() -> Self {
                <$type>::default()
            }
        }
    )*};
}

monoid_impl!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64
);

macro_rules! group_impl {
    ($($type:ty),*) => {$(
        impl Group for $type {
            fn inverse(self) -> Self {
                self.neg()
            }
        }
    )*};
}

group_impl!(i8, i16, i32, i64, i128, isize, f32, f64);
