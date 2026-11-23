use gatos_kernel::monoid::Monoid;

use crate::{
    bifoldable::Bifoldable, flat_map::FlatMap, foldable::Foldable, hkt::HKT, monoid_k::MonoidK,
    non_empty_alternative::NonEmptyAlternative,
};

pub trait Alternative<'a>: HKT + NonEmptyAlternative<'a> + MonoidK {
    fn unite<A, G>(fga: Self::Of<G::Of<A>>) -> Self::Of<A>
    where
        Self: FlatMap<'a> + Sized,
        G: Foldable,
    {
        Self::flat_map(fga, |a| G::fold_map_k::<_, A, _, Self>(a, Self::pure))
    }

    fn separate<A, B, G>(fgab: Self::Of<G::Of<A, B>>) -> (Self::Of<A>, Self::Of<B>)
    where
        Self: FlatMap<'a>,
        Self::Of<G::Of<A, B>>: Clone,
        <Self as HKT>::Of<A>: Monoid,
        <Self as HKT>::Of<B>: Monoid,
        G: Bifoldable,
    {
        let a = Self::flat_map(fgab.clone(), |gab| {
            G::bifold_map(gab, Self::pure, |_| Self::empty())
        });
        let b = Self::flat_map(fgab, |gab| {
            G::bifold_map(gab, |_| Self::empty(), Self::pure)
        });

        (a, b)
    }
}
