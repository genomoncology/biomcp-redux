# Least code, easy upkeep, fast

Decided by Ian for all his projects: "I want the cleanest best approach, least code, easiest to maintain and fast." Then: "That's what I want for everything." Recorded here 2026-10-23 by the PM.

- Every design choice and every review is judged by three tests: the least code we own, the easiest to maintain, and fast. Fast weighs as much as the other two.
- A dependency is not free.
- Depend on a mature crate, vendor MIT or Apache code, or write our own, case by case. `deny.toml` holds the allowed licenses.
