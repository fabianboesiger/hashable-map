use std::{collections::HashMap, hash::{Hash, Hasher, RandomState}, ops::{Deref, DerefMut}};

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

impl<K, V, S> Hash for HashableMap<K, V, S>
where
    K: Hash,
    V: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        let hash = self.iter().map(|(k, v)| {
            let mut hasher = std::hash::DefaultHasher::new();
            k.hash(&mut hasher);
            v.hash(&mut hasher);
            hasher.finish()
        }).fold(0, u64::wrapping_add);

        state.write_u64(hash);
    }
}

#[cfg(test)]
mod tests {
    use std::hash::DefaultHasher;

    use super::*;

    #[test]
    fn hash() {
        let mut map_a = HashableMap::<_, _, RandomState>::default();
        map_a.insert(1, "1");
        map_a.insert(2, "2");
        let mut map_b = HashableMap::<_, _, RandomState>::default();
        map_b.insert(1, "1");
        map_b.insert(2, "2");
        let mut map_c = HashableMap::<_, _, RandomState>::default();
        map_c.insert(1, "1");
        map_c.insert(3, "3");

        let mut hasher_a = DefaultHasher::new();
        map_a.hash(&mut hasher_a);
        let mut hasher_b = DefaultHasher::new();
        map_b.hash(&mut hasher_b);
        let mut hasher_c = DefaultHasher::new();
        map_c.hash(&mut hasher_c);

        assert_eq!(hasher_a.finish(), hasher_b.finish());
        assert_ne!(hasher_a.finish(), hasher_c.finish());
    }
}
