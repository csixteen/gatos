use std::convert::identity;

use gatos_prim::either::Either;

use crate::{apply::Apply, hkt::HKT};

pub trait FlatMap<'a>: HKT + Apply<'a> {
    fn flat_map<A, B, F>(fa: Self::Of<A>, f: F) -> Self::Of<B>
    where
        F: FnMut(A) -> Self::Of<B>;

    /// "flatten" a nested `F` of `F` structure into a single-layer `F` structure.
    fn flatten<A>(ffa: Self::Of<Self::Of<A>>) -> Self::Of<A> {
        Self::flat_map(ffa, identity)
    }

    // Sequentially compose two actions, discarding any value produced by the first.
    //fn product_r_eval<A, B>(fa: Self::Of<A>, fb: Eval<Self::Of<B>>) -> Self::Of<B>
    //where
    //    Self::Of<B>: Clone + 'static,
    //{
    //    Self::flat_map(fa, |_| fb.value())
    //}

    //fn product_l_eval<A, B>(fa: Self::Of<A>, fb: Eval<Self::Of<B>>) -> Self::Of<A>
    //where
    //    A: Clone,
    //    Self::Of<B>: Clone + 'static,
    //    Self: Sized,
    //{
    //    Self::flat_map(fa, |a| Self::r#as(fb.value(), a))
    //}

    /// `if` lifted into monad.
    fn if_m<B, F, E>(fa: Self::Of<bool>, if_true: F, if_false: E) -> Self::Of<B>
    where
        F: Fn() -> Self::Of<B>,
        E: Fn() -> Self::Of<B>,
    {
        Self::flat_map(fa, |a| if a { if_true() } else { if_false() })
    }

    fn tail_rec_m<A, B, F>(a: A, f: F) -> Self::Of<B>
    where
        F: FnMut(A) -> Self::Of<Either<A, B>>;
}
