use crate::{hkt::HKT, invariant::Invariant, invariant_semigroupal::InvariantSemigroupal};

pub trait InvariantMonoidal<'a>: InvariantSemigroupal<'a> {
    fn unit() -> <Self as HKT>::Out<()>;

    fn point<B>(value: B) -> <Self as HKT>::Out<B>
    where
        B: Clone + 'static,
        <Self as HKT>::Out<()>: Invariant<'a, Inner = (), Out<B> = <Self as HKT>::Out<B>>,
    {
        let unit: <Self as HKT>::Out<()> = <Self as InvariantMonoidal>::unit();
        <<Self as HKT>::Out<()> as Invariant>::imap(unit, move |()| value.clone(), |_: B| ())
    }
}
