# 0434 — Align lint settings and guard against dead code

Status: OPEN.

Milestone: 5

Depends on: 0425

## Outcome

A ratchet fails when a public item in biodata, biomcp-types or biomcp-fetch has no product reference outside its file. The eight never-built error messages leave the schemas. Lint settings match the written Rust standard: one shared clippy base checked by the ratchet, interior-mutability types denied with item-level reasons, `allow_attributes` denied, and the bool-parameter and cast rules either enforced or removed from the standard. A gate builds biodata and biomcp-types without the `schema` feature, or the feature is removed. Strings split mid-word are rejoined. Dependency duplicates move to workspace dependencies.

- Why: the pre-1.0 code quality review `sdlc/records/2026-10-28-code-quality-review.md`. Split from 0425 so pure removals can land while this waits for the restructuring design review.
- Starts from: `crates/*/clippy.toml`, `tests/ratchets`, `crates/biomcp-types/src/response/error.rs`, the Cargo manifests and the Rust standard.
- Rules: held with the restructuring tickets until the PM releases the reviewed design. Error message removal takes a schema major bump if the compatibility check calls it breaking.
- Proof: planted dead public items and planted lint drift fail the ratchet; the gate builds without the feature.
- Changes: the files above.
- Defers: nothing.
