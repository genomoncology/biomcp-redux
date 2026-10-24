# 0007 — Reconcile the license policy with the approved dependencies

Status: OPEN.

Milestone: 1

## Outcome

`deny.toml` names every license and scoped exception that the dependencies already approved in `planning/architecture.md` need, so adding one of them never fails the gate's license check.

- Starts from: two gate failures on 0003 and 0004 (jobs j8364 and j8386), where the MIT/Apache-only policy lacked entries for approved serde, ureq and TLS dependencies.
- Changes: `deny.toml` only.
- Keeps: `cargo deny` as the only checker. No new script, shared rule or dependency. Each exception is scoped to its crate and names its license.
- Scope: Resolve the full dependency tree of every crate the architecture approves for milestone 1 (`serde`, `serde_json`, `winnow`, `schemars`, `ureq` with its TLS stack, `rusqlite` bundled, `sha2`, and the runner's test dependencies), and add exactly the license entries and scoped exceptions they need.
- Proof: `make landing-gate` passes through pm job. A scratch check adding the remaining approved dependencies to a throwaway crate passes `cargo deny check licenses` without edits, and the scratch crate is removed.
- Measure: missing-exception gate failures, 2 observed before landing, should be 0 in the week after. The PM checks the count a week after landing.
