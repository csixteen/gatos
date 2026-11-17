use crate::invariant_semigroupal::InvariantSemigroupal;

pub trait InvariantMonoidal<'a>: InvariantSemigroupal<'a> {
    fn unit() -> Self::Of<()>;

    fn point<A>(a: A) -> Self::Of<A>
    where
        A: Clone + 'static,
    {
        Self::imap(Self::unit(), move |_| a.clone(), |_| ())
    }
}
