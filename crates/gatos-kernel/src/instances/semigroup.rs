use crate::semigroup::Semigroup;

impl Semigroup for () {
    fn combine(self, _: Self) -> Self {}
}
