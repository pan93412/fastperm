fastperm
===

A dead-simple, extreme fast permission flag system for Rust with no dependencies.

Usage
---

```rs
use fastperm::{add, rm, check};

// Add a flag to a permission digit.
assert_eq!(add(0, 1), 2);
// Remove a flag from a permission digit.
assert_eq!(rm(2, 1), 0);
// Check if a flag is in a permission digit.
assert!(check(2, 1));
```

Authors
---

- pan93412 \<pan93412@gmail.com\>
