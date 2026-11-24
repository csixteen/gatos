use gatos_kernel::monoid::Monoid;

use crate::{
    bifoldable::Bifoldable, flat_map::FlatMap, foldable::Foldable, hkt::HKT, monoid_k::MonoidK,
    non_empty_alternative::NonEmptyAlternative,
};

pub trait Alternative<'a>: HKT + NonEmptyAlternative<'a> + MonoidK {
    /// Fold over the inner structure to combine all the values with our combine method from
    /// [`MonoidK`]. All the "interesting" values of the inner `G` are then accumulated into the
    /// result, i.e., if `G` is [`Option<T>`], we collect all the `Some` values; if `G` is
    /// [`Either`], we collect all the [`Right`] values, etc.
    ///
    /// [`Either`]: gatos_prim::either::Either
    /// [`Right`]: gatos_prim::either::Either::Right
    fn unite<A, G>(fga: Self::Of<G::Of<A>>) -> Self::Of<A>
    where
        Self: FlatMap<'a> + Sized,
        G: Foldable,
    {
        Self::flat_map(fga, |a| G::fold_map_k::<_, A, _, Self>(a, Self::pure))
    }

    /// Separate the inner foldable values into the "lefts" and "rights".
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

    fn separate_foldable<A, B, G>(fgab: Self::Of<G::Of<A, B>>) -> (Self::Of<A>, Self::Of<B>)
    where
        Self: Foldable,
        G: Bifoldable,
    {
        Self::fold_left(fgab, (Self::empty(), Self::empty()), |oaob, g| {
            G::bifold_left(
                g,
                oaob,
                |x, a| (Self::append_k(x.0, a), x.1),
                |x, b| (x.0, Self::append_k(x.1, b)),
            )
        })
    }
}
