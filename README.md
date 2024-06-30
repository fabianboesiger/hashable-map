# Hashable Map

This crate provides simple wrappers `HashableMap` and `HashableSet` that implement `Hash` around `std::collections::HashMap` and `std::collections::HashSet` without any additional dependencies.

## Implementation Details

The `Hash` implementation of `HashableMap` and `HashableSet` respects the required property 

```
k1 == k2 -> hash(k1) == hash(k2)
```

In other words, if two keys are equal, their hashes must also be equal.


The implementation is generic over all `BuildHasher` implementations, which means you can use this crate with the default `RandomState` as well as other hash builder implementations such as `FxBuildHasher`.
The only requirement is that the hasher built by the hash builder implements `Default`, and that the default implementation results in the same hashes over different instances.
This requirement is due to the fact that to compute the hash of the entries, this crate uses instances of the defaut hasher built by the hash builder.
More formally, to construct the hashes for entries, we use an instance of `D::default()`, where `S: BuildHasher<Hasher = D>, D: Hasher + Default`.
The hashes of the entries are then added by using the commutative operator `wrapping_add` such that different orders of entries still result in the same hash, which is required for satisfying the `Hash` property stated above.

## Serde Support

This crate has the option to enable support for serialization and deserialization by enabling the `serde` feature.