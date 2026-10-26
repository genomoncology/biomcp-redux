# 0247 — Make the source registry ratchet name the entry it expects

Status: OPEN.

Milestone: 1

Depends on: none

## Outcome

The source registry ratchet in `crates/biomcp/tests/ratchets/source.rs` infers a source's module from the first path segment under `crates/biomcp-adapters/src/` and reports "implemented source must appear in source registry" when `&crate::{module}::SPEC` is missing. For a single-file module such as `interpro.rs` the inferred name keeps its `.rs` suffix, so the ratchet fails although the registry entry exists, and the message sends the developer looking for a missing declaration. Three registry-series jobs failed this way.

- Changes: `crates/biomcp/tests/ratchets/source.rs` only.
- Fix: derive the module name from both directory modules and single-file modules by dropping a trailing `.rs`, and make the failure message print the exact registry text it looked for, such as `&crate::interpro::SPEC`.
- Keeps: the rule that every implemented source appears in the registry.
- Proof: a ratchet unit check covers a directory module and a single-file module that are both registered and pass, and an unregistered single-file module whose failure names `&crate::NAME::SPEC`. Then make landing-gate runs through pm job.
