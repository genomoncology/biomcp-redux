# 0013 — Return schema-command errors in the shared JSON envelope

Status: OPEN.

Milestone: 1

## Outcome

The existing schema command returns a compact, versioned JSON error envelope for usage failures, with typed status/codes, fixed messages and the same documented exit codes.

Depends on: 0010
Depends on: 0009

- Starts from: `sdlc/plan.md` stages 3 and 6; architecture sections 3.8, 3.9, 8.4 and 8.5; current biodata values/schemas, binary argument/output boundaries and real schema-command cases.
- Changes: `crates/biodata/src/lib.rs`, `crates/biodata/src/response/**`, `crates/biodata/src/macros.rs`, `crates/biodata/src/source.rs`, `crates/biodata/schemas/**`, `crates/biodata/tests/**`, `crates/biomcp/src/main.rs`, `crates/biomcp/src/config.rs`, `crates/biomcp/src/output.rs`, `crates/biomcp/src/exit.rs`, `tests/cases/schema/**`.
- Scope: Add only the shared error-response types the existing command consumes, following the architecture's schema/status/error shape and checked serde rules. Error code/status sets remain typed and fixed messages retain no rejected input. Generate a checked-in biomcp.error/1 schema through the existing registry. Map existing schema-command usage errors through one JSON output boundary and one exit mapping. Successful schema output remains plain embedded schema JSON. A failed write cannot emit an envelope on the broken stdout; preserve fixed local-output stderr diagnostics and exit 5 for that path.
- Keeps: No fake search/get body, unconsumed generic envelope, request grammar, service layer or provider command. Normal response envelopes and section/next-step types arrive with their real consumers. No credentials, argument text, raw serde diagnostics or provider bodies in error messages. Existing exit 0/2/5 behavior remains. Reuse current byte/token rules and actual runner proofs.
- Ownership: Runner assertion lane 0012 owns runner helpers and its existing case edits. Claim separate error case YAML files, coordinate any shared case migration before writing it, and change no runner internals. Cache 0011 owns root manifests/lock; this step adds no dependencies.
- Proof: Real CLI cases cover missing/extra arguments, unknown schema ID and successful schema printing. Validate error outputs against the generated error schema, exercise serde round trips through the public error API and prove a distinctive invalid input never appears in stdout/stderr. Use the existing actual broken-output proof for exit 5. Run make landing-gate through pm job after current main and case changes integrate.
- Defers: Full envelope bodies, query echoes, statuses for live data, requested sections and typed next steps wait for their first real command consumer. MCP and full surface/coverage checks stay with their stages. No entire-output-contract roadmap tick from this step alone.
