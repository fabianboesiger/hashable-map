use std::{
    collections::HashMap,
    hash::{BuildHasher, Hash, Hasher, RandomState},
    ops::{Deref, DerefMut},
};

#[derive(Clone, Debug, Default)]
pub struct HashableMap<K, V, S = RandomState>(HashMap<K, V, S>);

impl<K, V> HashableMap<K, V> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self(HashMap::with_capacity(capacity))
    }
}

impl<K, V, S> HashableMap<K, V, S> {
    pub fn with_hasher(hash_builder: S) -> Self {
        Self(HashMap::with_hasher(hash_builder))
    }

    pub fn with_capacity_and_hasher(capacity: usize, hash_builder: S) -> Self {
        Self(HashMap::with_capacity_and_hasher(capacity, hash_builder))
    }
}

impl<K, V, S> Deref for HashableMap<K, V, S> {
    type Target = HashMap<K, V, S>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K, V, S> DerefMut for HashableMap<K, V, S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<K, V, S> From<HashMap<K, V, S>> for HashableMap<K, V, S> {
    fn from(value: HashMap<K, V, S>) -> Self {
        Self(value)
    }
}

impl<T, S> From<HashableMap<T, S>> for HashMap<T, S> {
    fn from(value: HashableMap<T, S>) -> HashMap<T, S> {
        value.0
    }
}

impl<K, V, S, D> Hash for HashableMap<K, V, S>
where
    K: Hash,
    V: Hash,
    S: BuildHasher<Hasher = D>,
    D: Hasher + Default,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        let hash = self
            .iter()
            .map(|(k, v)| {
                let mut hasher = D::default();
                k.hash(&mut hasher);
                v.hash(&mut hasher);
                hasher.finish()
            })
            .fold(0, u64::wrapping_add);

        state.write_u64(hash);
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use ahash::AHasher;
    use fnv::FnvBuildHasher;
    use fxhash::FxBuildHasher;
    use gxhash::GxBuildHasher;
    use rand::prelude::SliceRandom;
    use rand::{
        distributions::{Distribution, Standard},
        thread_rng,
    };
    use std::hash::{BuildHasherDefault, DefaultHasher};

    use super::*;

    #[test]
    fn insertion_order_random_state() {
        insertion_order::<RandomState, _>()
    }

    #[test]
    fn insertion_order_fx_build_hasher() {
        insertion_order::<FxBuildHasher, _>()
    }

    #[test]
    fn insertion_order_gx_build_hasher() {
        insertion_order::<GxBuildHasher, _>()
    }

    #[test]
    fn insertion_order_fnv_build_hasher() {
        insertion_order::<FnvBuildHasher, _>()
    }

    #[test]
    fn insertion_order_ahash_build_hasher() {
        insertion_order::<BuildHasherDefault<AHasher>, _>()
    }

    fn insertion_order<B: BuildHasher<Hasher = H> + Default, H: Hasher + Default>() {
        let values = generate_random_values::<i32, 128>();
        let values_shuffled = shuffle(&values);
        let values_other = generate_random_values::<i32, 128>();

        assert_ne!(values, values_shuffled);
        assert_ne!(values, values_other);
        assert_ne!(values_shuffled, values_other);

        let mut a = HashableMap::<_, _, B>::default();
        a.extend(values.iter().copied().map(|k| (k, k)));
        let mut b = HashableMap::<_, _, B>::default();
        b.extend(values_shuffled.iter().copied().map(|k| (k, k)));
        let mut c = HashableMap::<_, _, B>::default();
        c.extend(values_other.iter().copied().map(|k| (k, k)));

        assert_hash_eq(&a, &b);
        assert_hash_ne(&a, &c);
        assert_hash_ne(&b, &c)
    }

    #[test]
    fn same_keys_different_values_gx_build_hasher() {
        same_keys_different_values::<RandomState, _>()
    }

    fn same_keys_different_values<B: BuildHasher<Hasher = H> + Default, H: Hasher + Default>() {
        let keys = generate_random_values::<i32, 128>();
        let values1 = generate_random_values::<i32, 128>();
        let values2 = generate_random_values::<i32, 128>();

        assert_ne!(values1, values2);

        let mut a = HashableMap::<_, _, B>::default();
        a.extend(keys.iter().copied().zip(values1.iter().copied()));
        let mut b = HashableMap::<_, _, B>::default();
        b.extend(keys.iter().copied().zip(values2.iter().copied()));

        assert_hash_ne(&a, &b)
    }

    #[test]
    fn different_keys_same_values_gx_build_hasher() {
        different_keys_same_values::<RandomState, _>()
    }

    fn different_keys_same_values<B: BuildHasher<Hasher = H> + Default, H: Hasher + Default>() {
        let keys1 = generate_random_values::<i32, 128>();
        let keys2 = generate_random_values::<i32, 128>();
        let values = generate_random_values::<i32, 128>();

        assert_ne!(keys1, keys2);

        let mut a = HashableMap::<_, _, B>::default();
        a.extend(keys1.iter().copied().zip(values.iter().copied()));
        let mut b = HashableMap::<_, _, B>::default();
        b.extend(keys2.iter().copied().zip(values.iter().copied()));

        assert_hash_ne(&a, &b)
    }

    pub(crate) fn generate_random_values<T, const N: usize>() -> [T; N]
    where
        Standard: Distribution<[T; N]>,
    {
        rand::random()
    }

    pub(crate) fn shuffle<T, const N: usize>(values: &[T; N]) -> [T; N]
    where
        T: Clone,
    {
        let mut values = values.clone();
        values.shuffle(&mut thread_rng());
        values
    }

    pub(crate) fn assert_hash_eq<H: Hash>(a: &H, b: &H) {
        let mut hasher_a = DefaultHasher::new();
        a.hash(&mut hasher_a);
        let mut hasher_b = DefaultHasher::new();
        b.hash(&mut hasher_b);

        assert_eq!(hasher_a.finish(), hasher_b.finish());
    }

    pub(crate) fn assert_hash_ne<H: Hash>(x: &H, y: &H) {
        let mut hasher_x = DefaultHasher::new();
        x.hash(&mut hasher_x);
        let mut hasher_y = DefaultHasher::new();
        y.hash(&mut hasher_y);

        assert_ne!(hasher_x.finish(), hasher_y.finish());
    }
}
