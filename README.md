# Hashable Map

This crate provides simple wrappers around `std::collections::HashMap` and `std::collections::HashSet` without any additional dependencies that implement `Hash`.
The `Hash` implementation respects the required property 

```
k1 == k2 -> hash(k1) == hash(k2)
```

In other words, if two keys are equal, their hashes must also be equal.

The implementation is generic over all `BuildHasher` implementations, which means you can use this crate with the default `RandomState` as well as other hash builder implementations such as `FxBuildHasher`.
The only requirement is that the hasher built by the hash builder implements `Default`, and that the default implementation results in the same hashes over different instances.