use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use crate::kernel::semigroup::Semigroup;

impl Semigroup for () {
    fn combine(_: Self, _: Self) -> Self {}
}

impl Semigroup for String {
    fn combine(mut left: Self, right: Self) -> Self {
        left.push_str(&right);
        left
    }
}

impl<T> Semigroup for Vec<T> {
    fn combine(mut left: Self, mut right: Vec<T>) -> Vec<T> {
        left.append(&mut right);
        left
    }
}

impl<T> Semigroup for HashSet<T>
where
    T: Eq + Hash,
{
    fn combine(mut left: Self, right: Self) -> Self {
        left.extend(right);
        left
    }
}

impl<K, V> Semigroup for HashMap<K, V>
where
    K: Eq + Hash,
{
    fn combine(mut left: Self, right: Self) -> Self {
        left.extend(right);
        left
    }
}

impl<T: Semigroup> Semigroup for Option<T> {
    fn combine(left: Self, right: Self) -> Self {
        match (left, right) {
            (None, None) => None,
            (a @ Some(_), None) => a,
            (None, b @ Some(_)) => b,
            (Some(a), Some(b)) => Some(Semigroup::combine(a, b)),
        }
    }
}

impl<T: Semigroup, E> Semigroup for Result<T, E> {
    fn combine(left: Self, right: Self) -> Self {
        match (left, right) {
            (_, b @ Err(_)) => b,
            (a @ Err(_), _) => a,
            (Ok(a), Ok(b)) => Ok(Semigroup::combine(a, b)),
        }
    }
}

impl Semigroup for f32 {
    fn combine(left: Self, right: Self) -> Self {
        left + right
    }
}

impl Semigroup for f64 {
    fn combine(left: Self, right: Self) -> Self {
        left + right
    }
}

macro_rules! semigroup_impl {
    ($($type:ty),*) => {$(
        impl Semigroup for $type {
            fn combine(left: $type, right: $type) -> $type {
                left.wrapping_add(right)
            }
        }
    )*};
}

semigroup_impl!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
