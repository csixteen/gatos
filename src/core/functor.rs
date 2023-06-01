pub trait Functor {
    type In;
    type Out<U>: Functor;

    fn fmap<F, U>(self, f: F) -> Self::Out<U>
    where
        F: FnMut(Self::In) -> U;
}

pub fn lift<A: Functor, B, F>(f: F) -> impl FnMut(A) -> A::Out<B>
where
    F: FnMut(A::In) -> B + Copy,
{
    move |a: A| a.fmap(f)
}

impl<T> Functor for Vec<T> {
    type In = T;
    type Out<U> = Vec<U>;

    fn fmap<F, U>(self, f: F) -> Self::Out<U>
    where
        F: FnMut(Self::In) -> U,
    {
        self.into_iter().map(f).collect()
    }
}

impl<T> Functor for Option<T> {
    type In = T;
    type Out<U> = Option<U>;

    fn fmap<F, U>(self, mut f: F) -> Self::Out<U>
    where
        F: FnMut(Self::In) -> U,
    {
        self.map(|b| f(b))
    }
}

impl<T, E> Functor for Result<T, E> {
    type In = T;
    type Out<U> = Result<U, E>;

    fn fmap<F, U>(self, mut f: F) -> Self::Out<U>
    where
        F: FnMut(Self::In) -> U,
    {
        match self {
            Ok(b) => Ok(f(b)),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lift() {
        let to_lowercase = |s: String| s.to_lowercase();
        let s = Some("HELLO".to_string());
        let mut lifted = lift(to_lowercase);
        assert_eq!(Some("hello".to_string()), lifted(s));
    }
}
