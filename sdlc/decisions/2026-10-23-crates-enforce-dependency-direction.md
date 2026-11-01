# Crates enforce dependency direction

Decided 2026-10-23 by the PM, applying Ian's layered design and his ratchet ruling. Updated 2026-10-31 to describe the implemented boundaries after the approved restructure.

`crates/biomcp/tests/ratchets/dependencies.rs` owns the allowed dependency table. Cargo manifests declare actual dependencies. `sdlc/planning/architecture.md` describes each layer without copying that table.

Cargo, per-crate Clippy rules and `crates/biomcp/tests/ratchets/dependencies.rs` enforce these boundaries. `crates/biomcp/tests/ratchets/source.rs` checks placement and reasoned exceptions. The allowed table includes the fetch crate's bounded streaming runtime dependencies.
