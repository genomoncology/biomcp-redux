# Data bundles are local APIs

Decided 2026-10-23 by Ian.

- A local data bundle, such as EMA or WHO data, is a source like any remote API.
- It is queried locally through the same adapter shape: endpoints in `mod.rs`, raw rows in `raw.rs` and the mapping into biodata in `map.rs`.
- The transport downloads, revalidates and stores the bundle. `biomcp sync <bundle>` refreshes one.
- Each bundle is judged on its own merits in its entity's slice, and stays only if the slice's examples need it.
