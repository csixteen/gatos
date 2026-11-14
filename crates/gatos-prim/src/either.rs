#[derive(Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

use self::Either::{Left, Right};

impl<L: Clone, R: Clone> Clone for Either<L, R> {
    fn clone(&self) -> Self {
        match self {
            Left(l) => Left(l.clone()),
            Right(r) => Right(r.clone()),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        match (self, source) {
            (Left(dest), Left(source)) => dest.clone_from(source),
            (Right(dest), Right(source)) => dest.clone_from(source),
            (dest, source) => *dest = source.clone(),
        }
    }
}

impl<L, R> Either<L, R> {
    pub fn map<RR, F>(self, f: F) -> Either<L, RR>
    where
        F: FnOnce(R) -> RR,
    {
        match self {
            Left(l) => Left(l),
            Right(r) => Right(f(r)),
        }
    }

    pub fn flat_map<LL, RR, F>(self, f: F) -> Either<LL, RR>
    where
        L: Into<LL>,
        F: FnOnce(R) -> Either<LL, RR>,
    {
        match self {
            Left(left) => Left(left.into()),
            Right(right) => f(right),
        }
    }

    pub fn or_else<LL, RR, F>(self, f: F) -> Either<LL, RR>
    where
        R: Into<RR>,
        F: FnOnce() -> Either<LL, RR>,
    {
        match self {
            Left(_) => f(),
            Right(right) => Right(right.into()),
        }
    }

    pub fn map2<LL, RR, C, F>(self, that: Either<LL, RR>, f: F) -> Either<LL, C>
    where
        L: Into<LL>,
        F: FnOnce(R, RR) -> C,
    {
        self.flat_map(|a| that.map(|b| f(a, b)))
    }
}
