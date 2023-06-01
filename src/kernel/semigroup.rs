use std::collections::HashSet;
use std::hash::Hash;

pub trait Semigroup {
    fn combine(self, x: Self) -> Self;
}

impl Semigroup for () {
    fn combine(self, _: Self) -> Self {}
}

impl Semigroup for String {
    fn combine(mut self, x: Self) -> Self {
        self.push_str(&x);
        self
    }
}

impl<T> Semigroup for Vec<T> {
    fn combine(mut self, mut x: Vec<T>) -> Vec<T> {
        self.append(&mut x);
        self
    }
}

impl<T: Eq + Hash> Semigroup for HashSet<T> {
    fn combine(mut self, x: Self) -> Self {
        self.extend(x);
        self
    }
}

impl<T: Semigroup> Semigroup for Option<T> {
    fn combine(self, x: Self) -> Self {
        match (self, x) {
            (None, None) => None,
            (a @ Some(_), None) => a,
            (None, b @ Some(_)) => b,
            (Some(a), Some(b)) => Some(a.combine(b)),
        }
    }
}

impl<T: Semigroup, E> Semigroup for Result<T, E> {
    fn combine(self, x: Self) -> Self {
        match (self, x) {
            (_, b @ Err(_)) => b,
            (a @ Err(_), _) => a,
            (Ok(a), Ok(b)) => Ok(a.combine(b)),
        }
    }
}

impl Semigroup for f32 {
    fn combine(self, x: Self) -> Self {
        self + x
    }
}

impl Semigroup for f64 {
    fn combine(self, x: Self) -> Self {
        self + x
    }
}

macro_rules! semigroup_impl {
    ($($type:ty),*) => {$(
        impl Semigroup for $type {
            fn combine(self, x: $type) -> $type {
                self.wrapping_add(x)
            }
        }
    )*};
}

semigroup_impl!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
