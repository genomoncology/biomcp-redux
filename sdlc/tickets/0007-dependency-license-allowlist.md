# 0007 — Reconcile the license policy with the approved dependencies

Status: OPEN.

Milestone: 1

## Outcome

`deny.toml` names every license and scoped exception that the dependencies already approved in `planning/architecture.md` need, so adding one of them never fails the gate's license check.

- Starts from: two gate failures on 0003 and 0004 (jobs j8364 and j8386), where the MIT/Apache-only policy lacked entries for approved serde, ureq and TLS dependencies.
- Changes: `Cargo.lock`, `Cargo.toml`, `crates/biodata/Cargo.toml`, `crates/biodata/schemas/problem-code.json`, `crates/biodata/schemas/problem.json`, `crates/biodata/schemas/reason.json`, `crates/biodata/schemas/source.json`, `crates/biodata/schemas/text.json`, `crates/biodata/src/response/problem.rs`, `crates/biodata/src/source.rs`, `crates/biodata/src/text.rs`, `crates/biodata/tests/schemas.rs`, `crates/biodata/tests/schemas/compatibility.rs`, `crates/biodata/tests/schemas/compatibility_cases.rs`, `crates/biodata/tests/schemas/values.rs`, `crates/biomcp-fetch/Cargo.toml`, `crates/biomcp-fetch/clippy.toml`, `crates/biomcp-fetch/src/deadline.rs`, `crates/biomcp-fetch/src/declaration.rs`, `crates/biomcp-fetch/src/error.rs`, `crates/biomcp-fetch/src/lib.rs`, `crates/biomcp-fetch/src/request.rs`, `crates/biomcp-fetch/src/secret.rs`, `crates/biomcp-fetch/tests/transport.rs`, `crates/biomcp/Cargo.toml`, `deny.toml`.
- Keeps: `cargo deny` as the only checker. No new script, shared rule or dependency. Each exception is scoped to its crate and names its license.
- Scope: Resolve the full dependency tree of every crate the architecture approves for milestone 1 (`serde`, `serde_json`, `winnow`, `schemars`, `ureq` with its TLS stack, `rusqlite` bundled and `sha2`) at the versions resolved today, and add exactly the license entries and scoped exceptions they need. The runner's YAML crate is not chosen yet, so its stage adds its own entries. A later version that changes license metadata updates `deny.toml` in the ticket that bumps it.
- Proof: `make landing-gate` passes through pm job. A scratch check adding the remaining approved dependencies at their resolved versions to a throwaway crate passes `cargo deny check licenses` without edits, and the scratch crate is removed.
- Measure: missing-exception gate failures, 2 observed before landing, should be 0 in the week after. The PM checks the count a week after landing.
