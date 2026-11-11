use crate::core::semigroup_k::SemigroupK;

impl<T> SemigroupK for Vec<T> {
    type Inner = T;
    type Out<B> = Vec<B>;

    fn combine_k(mut self, mut b: Self) -> Self {
        self.append(&mut b);
        self
    }
}
