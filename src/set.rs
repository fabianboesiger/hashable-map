use std::{collections::HashSet, hash::{BuildHasher, Hash, Hasher, RandomState}, ops::{Deref, DerefMut}};

#[derive(Clone, Debug, Default)]
pub struct HashableSet<T, S = RandomState>(HashSet<T, S>);

impl<T> HashableSet<T> {
    pub fn new() -> Self {
        Self(HashSet::new())
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self(HashSet::with_capacity(capacity))
    }
}

impl<T, S> HashableSet<T, S> {
    pub fn with_hasher(hash_builder: S) -> Self {
        Self(HashSet::with_hasher(hash_builder))
    }

    pub fn with_capacity_and_hasher(capacity: usize, hash_builder: S) -> Self {
        Self(HashSet::with_capacity_and_hasher(capacity, hash_builder))
    }
}

impl<T, S> Deref for HashableSet<T, S> {
    type Target = HashSet<T, S>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, S> DerefMut for HashableSet<T, S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T, S> From<HashSet<T, S>> for HashableSet<T, S> {
    fn from(value: HashSet<T, S>) -> Self {
        Self(value)
    }
}

impl<T, S> Into<HashSet<T, S>> for HashableSet<T, S> {
    fn into(self) -> HashSet<T, S> {
        self.0
    }
}

impl<T, S, D> Hash for HashableSet<T, S>
where
    T: Hash,
    S: BuildHasher<Hasher = D>,
    D: Hasher + Default
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        let hash = self.iter().map(|t| {
            let mut hasher = D::default();
            t.hash(&mut hasher);
            hasher.finish()
        }).fold(0, u64::wrapping_add);

        state.write_u64(hash);
    }
}

#[cfg(test)]
mod tests {
    use std::hash::DefaultHasher;
    use fxhash::FxBuildHasher;

    use super::*;

    #[test]
    fn random_state() {
        let mut map_a = HashableSet::<_, RandomState>::default();
        map_a.insert(1);
        map_a.insert(2);
        let mut map_b = HashableSet::<_, RandomState>::default();
        map_b.insert(1);
        map_b.insert(2);
        let mut map_c = HashableSet::<_, RandomState>::default();
        map_c.insert(1);
        map_c.insert(3);

        let mut hasher_a = DefaultHasher::new();
        map_a.hash(&mut hasher_a);
        let mut hasher_b = DefaultHasher::new();
        map_b.hash(&mut hasher_b);
        let mut hasher_c = DefaultHasher::new();
        map_c.hash(&mut hasher_c);

        assert_eq!(hasher_a.finish(), hasher_b.finish());
        assert_ne!(hasher_a.finish(), hasher_c.finish());
    }

    #[test]
    fn fx_build_hasher() {
        let mut map_a = HashableSet::<_, FxBuildHasher>::default();
        map_a.insert(1);
        map_a.insert(2);
        let mut map_b = HashableSet::<_, FxBuildHasher>::default();
        map_b.insert(1);
        map_b.insert(2);
        let mut map_c = HashableSet::<_, FxBuildHasher>::default();
        map_c.insert(1);
        map_c.insert(3);

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
