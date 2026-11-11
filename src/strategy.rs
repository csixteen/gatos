use std::fmt::Debug;

use proptest::strategy::Strategy;

pub fn triple<T, S>(strategy: S) -> impl Strategy<Value = (T, T, T)>
where
    T: Debug,
    S: Strategy<Value = T> + Clone,
{
    (strategy.clone(), strategy.clone(), strategy)
}
