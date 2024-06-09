# Hashable Map

This is a simple wrapper around `std::collections::HashMap` without any additional dependencies that implements `Hash`.
The `Hash` implementation respects the property 

```
k1 == k2 -> hash(k1) == hash(k2)
```

In other words, if two keys are equal, their hashes must also be equal.