# 0425 — Remove dead code and align lint settings with the standard

Status: OPEN.

Milestone: 5

## Outcome

Unused and test-only public items are removed or made crate-private, and a ratchet fails when a public item in biodata, biomcp-types or biomcp-fetch has no product reference outside its file. The eight never-built error messages leave the schemas. Lint settings match the written Rust standard: one shared clippy base checked by the ratchet, interior-mutability types denied with item-level reasons, `allow_attributes` denied, and the bool-parameter and cast rules either enforced or removed from the standard. A gate builds biodata and biomcp-types without the `schema` feature, or the feature is removed. Strings split mid-word are rejoined. Dependency duplicates move to workspace dependencies.

- Why: the pre-1.0 code quality review `sdlc/records/2026-10-28-code-quality-review.md`.
- Starts from: the items named in the record, `crates/*/clippy.toml`, `tests/ratchets`, `crates/biomcp-types/src/response/error.rs`, the Cargo manifests and the Rust standard.
- Rules: error message removal takes a schema major bump if the compatibility check calls it breaking; generated proto code keeps only referenced types if the generator allows.
- Proof: planted dead public items and planted lint drift fail the ratchet; the gate builds without the feature.
- Changes: the files above.
- Defers: nothing.
