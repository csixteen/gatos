pub trait TupledFunction2<A, B, C>
where
    Self: Fn(A, B) -> C + Sized,
{
    fn tupled(self) -> impl Fn((A, B)) -> C {
        move |(a, b)| self(a, b)
    }
}

impl<A, B, C, F> TupledFunction2<A, B, C> for F where F: Fn(A, B) -> C {}
