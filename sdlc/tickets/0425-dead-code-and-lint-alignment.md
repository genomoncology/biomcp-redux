# 0425 — Remove dead code

Status: OPEN.

Milestone: 5

## Outcome

Unused public and crate items named in the code quality review are deleted, and test-only public items become crate-private or move under `#[cfg(test)]`. Behavior, schemas, help, lint settings and ratchets stay unchanged, so every existing case and the schema compatibility check pass with no new allowance.

- Why: the pre-1.0 code quality review `sdlc/records/2026-10-28-code-quality-review.md`. Ian held the restructuring for a design review on 2026-10-28 but allowed pure dead-code removal to proceed.
- Starts from: the dead and test-only items named in the record.
- Rules: a removal that changes a schema, an error message, a lint setting, a ratchet or any output belongs in 0434 or the restructuring tickets, so leave it. Generated proto code keeps only referenced types if the generator allows without a generator change.
- Proof: the full gate passes with unchanged schemas and an unchanged case list; the diff deletes code or narrows visibility only.
- Changes: the files that hold the named items.
- Defers: the dead-item ratchet, the eight never-built error messages, lint alignment, the `schema` feature gate, rejoined strings and dependency duplicates, which move to 0434.
