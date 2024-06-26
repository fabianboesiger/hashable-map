use std::{
    hash::{BuildHasher, Hash, Hasher, RandomState},
    ops::{Deref, DerefMut},
};

macro_rules! make_hashable_set {
    ($hash_set_impl:ident) => {
        #[derive(Clone, Debug, Default)]
        pub struct HashableSet<T, S = RandomState>($hash_set_impl<T, S>);

        impl<T> HashableSet<T> {
            pub fn new() -> Self {
                Self($hash_set_impl::new())
            }

            pub fn with_capacity(capacity: usize) -> Self {
                Self($hash_set_impl::with_capacity(capacity))
            }
        }

        impl<T, S> HashableSet<T, S> {
            pub fn with_hasher(hash_builder: S) -> Self {
                Self($hash_set_impl::with_hasher(hash_builder))
            }

            pub fn with_capacity_and_hasher(capacity: usize, hash_builder: S) -> Self {
                Self($hash_set_impl::with_capacity_and_hasher(
                    capacity,
                    hash_builder,
                ))
            }
        }

        impl<T, S> Deref for HashableSet<T, S> {
            type Target = $hash_set_impl<T, S>;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<T, S> DerefMut for HashableSet<T, S> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl<T, S> From<$hash_set_impl<T, S>> for HashableSet<T, S> {
            fn from(value: $hash_set_impl<T, S>) -> Self {
                Self(value)
            }
        }

        impl<T, S> From<HashableSet<T, S>> for $hash_set_impl<T, S> {
            fn from(value: HashableSet<T, S>) -> $hash_set_impl<T, S> {
                value.0
            }
        }

        impl<T, S, D> Hash for HashableSet<T, S>
        where
            T: Hash,
            S: BuildHasher<Hasher = D>,
            D: Hasher + Default,
        {
            fn hash<H: Hasher>(&self, state: &mut H) {
                state.write_usize(self.len());

                let hash = self
                    .iter()
                    .map(|t| {
                        let mut hasher = D::default();
                        t.hash(&mut hasher);
                        hasher.finish()
                    })
                    .fold(0, u64::wrapping_add);

                state.write_u64(hash);
            }
        }

        impl<T, S> PartialEq for HashableSet<T, S>
        where
            T: Eq + Hash,
            S: BuildHasher,
        {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }

        impl<T, S> Eq for HashableSet<T, S>
        where
            T: Eq + Hash,
            S: BuildHasher,
        {
        }

        #[cfg(feature = "serde")]
        impl<T, S> serde::Serialize for HashableSet<T, S>
        where
            $hash_set_impl<T, S>: serde::Serialize,
        {
            fn serialize<Ser>(&self, serializer: Ser) -> Result<Ser::Ok, Ser::Error>
            where
                Ser: serde::Serializer,
            {
                self.0.serialize(serializer)
            }
        }

        #[cfg(feature = "serde")]
        impl<'de, T, S> serde::Deserialize<'de> for HashableSet<T, S>
        where
            $hash_set_impl<T, S>: serde::Deserialize<'de>,
        {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                Ok(Self($hash_set_impl::deserialize(deserializer)?))
            }
        }
    };
}

use std::collections::HashSet;
make_hashable_set!(HashSet);

#[cfg(test)]
pub(crate) mod tests {
    use std::hash::BuildHasherDefault;

    use super::*;

    use crate::map::tests::*;

    #[test]
    fn insertion_order_random_state() {
        insertion_order::<RandomState, _>()
    }

    #[test]
    fn insertion_order_fx_build_hasher() {
        insertion_order::<fxhash::FxBuildHasher, _>()
    }

    #[test]
    fn insertion_order_fnv_build_hasher() {
        insertion_order::<fnv::FnvBuildHasher, _>()
    }

    #[test]
    fn insertion_order_ahash_build_hasher() {
        insertion_order::<BuildHasherDefault<ahash::AHasher>, _>()
    }

    fn insertion_order<B: BuildHasher<Hasher = H> + Default, H: Hasher + Default>() {
        let values = generate_random_values::<i32, 128>();
        let values_shuffled = shuffle(&values);
        let values_other = generate_random_values::<i32, 128>();

        assert_ne!(values, values_shuffled);
        assert_ne!(values, values_other);
        assert_ne!(values_shuffled, values_other);

        let mut a = HashableSet::<_, B>::default();
        a.extend(values.iter().copied());
        let mut b = HashableSet::<_, B>::default();
        b.extend(values_shuffled.iter().copied());
        let mut c = HashableSet::<_, B>::default();
        c.extend(values_other.iter().copied());

        assert_hash_eq(&a, &b);
        assert_hash_ne(&a, &c);
        assert_hash_ne(&b, &c)
    }
}
