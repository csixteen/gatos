use crate::{monoid::Monoid, semigroup::Semigroup};

macro_rules! impl_tuples {
    () => {};
    (($HeadT:ident $head_a:ident $head_b:ident) $(($TailT:ident $tail_a:ident $tail_b:ident))*) => {
        impl<$HeadT: Semigroup, $($TailT: Semigroup),*> Semigroup for ($HeadT, $($TailT,)*) {
            fn combine(self, other: Self) -> Self {
                let ($head_a, $($tail_a,)*) = self;
                let ($head_b, $($tail_b,)*) = other;
                ($head_a.combine($head_b), $($tail_a.combine($tail_b),)*)
            }
        }

        impl<$HeadT: Monoid, $($TailT: Monoid),*> Monoid for ($HeadT, $($TailT,)*) {
            fn empty() -> Self {
                ($HeadT::empty(), $($TailT::empty(),)*)
            }
        }
    }
}

// I was looking into how Rust's standard library implements traits for tuples of multiple arities,
// but they use some nightly features, namely the `.index()`. RFC 3086 introduces Macro Metavariable
// Expressions under `#![feature(macro_metavar_expr)]`, which includes:
// * `${index()}`, which evaluates to the current repetition index (`0`, `1`, etc) of the innermost
//   `$()*` or `$()+` block.
// * `${len()}`, which evaluates to the total number of elements in the repetition.
//
// These metavariables address the pain in the ass of destructuring `self` into variables named
// after the type parameters...which is exactly what I'm doing here, because I'm using stable Rust.
// Another option would be to also pass the indices manually:
// ```
// impl_tuples!(A 0, B 1, C 2, ...)
// ```
impl_tuples!((A a1 a2) (B b1 b2) (C c1 c2) (D d1 d2) (E e1 e2) (F f1 f2) (G g1 g2));

// 0-tuple, or unit type

impl Semigroup for () {
    fn combine(self, _: Self) -> Self {}
}

impl Monoid for () {
    fn empty() -> Self {}
}
