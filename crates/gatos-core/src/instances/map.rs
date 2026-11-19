use std::{collections::HashMap, hash::Hash, marker::PhantomData};

use crate::{
    apply::Apply, flat_map::FlatMap, functor::Functor, hkt::HKT, invariant::Invariant,
    semigroupal::Semigroupal, unordered_foldable::UnorderedFoldable,
};

pub struct HashMapK<K>(PhantomData<K>);

impl<K> HKT for HashMapK<K> {
    type Of<A> = HashMap<K, A>;
}

impl<'a, K> Invariant<'a> for HashMapK<K>
where
    K: Eq + Hash,
{
    fn imap<A, B, F, G>(fa: Self::Of<A>, f: F, _g: G) -> Self::Of<B>
    where
        A: 'a,
        F: Fn(A) -> B + 'a,
        G: Fn(B) -> A + 'a,
    {
        fa.into_iter().map(|(k, v)| (k, f(v))).collect()
    }
}

impl<'a, K> Functor<'a> for HashMapK<K>
where
    K: Eq + Hash,
{
    fn map<A, B, F>(fa: Self::Of<A>, mut f: F) -> Self::Of<B>
    where
        F: FnMut(A) -> B,
    {
        fa.into_iter().map(|(k, v)| (k, f(v))).collect()
    }
}

impl<K: Eq + Hash> Semigroupal for HashMapK<K> {
    fn product<A, B>(fa: Self::Of<A>, mut fb: Self::Of<B>) -> Self::Of<(A, B)> {
        fa.into_iter()
            .filter_map(|(k, a)| fb.remove(&k).map(|b| (k, (a, b))))
            .collect()
    }
}

impl<'a, K> Apply<'a> for HashMapK<K>
where
    K: Eq + Hash,
{
    fn ap<A, B, F>(mut ff: Self::Of<F>, fa: Self::Of<A>) -> Self::Of<B>
    where
        F: FnOnce(A) -> B,
    {
        // This is a "lossy" operation if `fa::keys()` is not a subset of `ff:keys()`.
        fa.into_iter()
            .filter_map(|(k, v)| ff.remove(&k).map(|fff| (k, fff(v))))
            .collect()
    }
}

impl<'a, K> FlatMap<'a> for HashMapK<K>
where
    K: Eq + Hash,
{
    fn flat_map<A, B, F>(fa: Self::Of<A>, f: F) -> Self::Of<B>
    where
        F: FnMut(A) -> Self::Of<B>,
    {
        fa.into_values().flat_map(f).collect()
    }

    fn tail_rec_m<A, B, F>(a: A, mut f: F) -> Self::Of<B>
    where
        F: FnMut(A) -> Self::Of<gatos_prim::either::Either<A, B>>,
    {
        use gatos_prim::either::Either::*;

        let mut res = HashMap::new();
        let mut stack: Vec<(K, A)> = Vec::new();

        for (k, step) in f(a) {
            match step {
                Left(a) => stack.push((k, a)),
                Right(b) => {
                    res.insert(k, b);
                }
            }
        }

        // ensure that only matching keys `k` from the inner map are in the result
        while let Some((k, curr)) = stack.pop() {
            if let Some(step) = f(curr).remove(&k) {
                match step {
                    Left(a) => stack.push((k, a)),
                    Right(b) => {
                        res.insert(k, b);
                    }
                }
            }
        }

        res
    }
}

impl<K> UnorderedFoldable for HashMapK<K> {
    fn unordered_fold_map<A, B, F>(fa: Self::Of<A>, f: F) -> B
    where
        B: gatos_kernel::commutative_monoid::CommutativeMonoid,
        F: Fn(A) -> B,
    {
        fa.into_values()
            .fold(B::empty(), |acc, a| B::combine(acc, f(a)))
    }
}
