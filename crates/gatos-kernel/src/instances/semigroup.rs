use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use crate::semigroup::Semigroup;

impl Semigroup for () {
    fn combine(self, _: impl Into<Self>) -> Self {}
}

impl Semigroup for String {
    fn combine(mut self, right: impl Into<Self>) -> Self {
        self.push_str(&right.into());
        self
    }
}

impl<T> Semigroup for Vec<T> {
    fn combine(mut self, right: impl Into<Vec<T>>) -> Vec<T> {
        self.append(&mut right.into());
        self
    }
}

impl<T> Semigroup for HashSet<T>
where
    T: Eq + Hash,
{
    fn combine(mut self, right: impl Into<Self>) -> Self {
        self.extend(right.into());
        self
    }
}

impl<K, V> Semigroup for HashMap<K, V>
where
    K: Eq + Hash,
{
    fn combine(mut self, right: impl Into<Self>) -> Self {
        self.extend(right.into());
        self
    }
}

impl<T: Semigroup> Semigroup for Option<T>
where
    T: Semigroup,
{
    fn combine(self, right: impl Into<Self>) -> Self {
        match (self, right.into()) {
            (None, None) => None,
            (a @ Some(_), None) => a,
            (None, b @ Some(_)) => b,
            (Some(a), Some(b)) => Some(a.combine(b)),
        }
    }
}

impl<T: Semigroup, E> Semigroup for Result<T, E>
where
    T: Semigroup,
{
    fn combine(self, right: impl Into<Self>) -> Self {
        match (self, right.into()) {
            (_, b @ Err(_)) => b,
            (a @ Err(_), _) => a,
            (Ok(a), Ok(b)) => Ok(a.combine(b)),
        }
    }
}

impl Semigroup for f32 {
    fn combine(self, right: impl Into<Self>) -> Self {
        self + right.into()
    }
}

impl Semigroup for f64 {
    fn combine(self, right: impl Into<Self>) -> Self {
        self + right.into()
    }
}

macro_rules! semigroup_impl {
    ($($type:ty),*) => {$(
        impl Semigroup for $type {
            fn combine(self, right: impl Into<$type>) -> $type {
                self.wrapping_add(right.into())
            }
        }
    )*};
}

semigroup_impl!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
);
