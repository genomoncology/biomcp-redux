# 0013 — Return schema-command errors in the shared JSON envelope

Status: COMPLETE.
Landed: 9641c3a50b79456ba41ccc245c1ada71c06ad266

Milestone: 1

## Outcome

The existing schema command returns a compact, versioned JSON error envelope for usage failures, with typed status/codes, fixed messages and the same documented exit codes.

Depends on: 0010
Depends on: 0009
Depends on: 0012

- Starts from: `sdlc/plan.md` stages 3 and 6; architecture sections 3.8, 3.9, 8.4 and 8.5; current biodata values/schemas, binary argument/output boundaries and real schema-command cases.
- Changes: `crates/biodata/src/lib.rs`, `crates/biodata/src/response/mod.rs`, `crates/biodata/src/response/error.rs`, `crates/biodata/schemas/error.json`, `crates/biodata/tests/schemas.rs`, `crates/biodata/tests/schemas/values.rs`, `crates/biomcp/Cargo.toml`, `crates/biomcp/src/main.rs`, `crates/biomcp/src/output.rs`, `crates/biomcp/src/exit.rs`, `crates/biomcp/tests/runner/mod.rs`, `tests/cases/schema/arguments.yaml`, `tests/cases/schema/errors.yaml`.
- PM decision: 2026-10-24. Land only the error envelope now. Normal generic envelopes, query echoes, Section, Step and next steps arrive with the first gene command, following architecture 3.5, 3.8 and 3.10 exactly. The first gene ticket records this move and proves the types through real output.
- Reviewed API detail: Export ErrorEnvelope, ErrorStatus and ErrorCode from biodata. ErrorEnvelope::schema_usage() constructs the only supported error. Usage status serializes as usage; SchemaUsage code as usage.schema; the fixed message is expected schema <known artifact id>. All current usage failures use that error; add no unconsumed variants. The exact object is schema/status/error, with code/message inside error. Schema field and artifact ID are biomcp.error/1. Fields are private and accept no input. Serde rejects missing/unknown fields, unsupported literals and changed messages. Derive schema from the same checked representation, without a general SchemaId type.
- Scope: Registry entries carry complete canonical IDs so current biodata IDs stay unchanged and the error artifact uses biomcp.error/1. Promote existing serde_json into normal binary dependencies; no new package or biodata dependency. Serialize the typed error at the existing output boundary; use exit.rs for consumed success/usage/output-failure mapping and return ExitCode. Success remains plain schema JSON. A failed stdout write retains fixed stderr diagnostic and exit 5; do not try a second stdout response.
- Ownership: Wait for 0012 to land before changing runner/mod.rs, avoiding competing writers. Change only its old empty-stdout usage assumption so JSON from nonzero commands reaches existing assertions. Do not change operators or semantics. This ticket owns arguments.yaml and errors.yaml after that dependency clears. Cache 0011 owns root manifests/lock, and this ticket needs no lock change.
- Keeps: No fake search/get body, unconsumed generic envelope, request grammar, service layer or provider command. Normal response envelopes and section/next-step types arrive with their real consumers. No credentials, argument text, raw serde diagnostics or provider bodies in error messages. Existing exit 0/2/5 behavior remains. Reuse current byte/token rules and actual runner proofs.
- Ownership: Runner assertion lane 0012 owns runner helpers and its existing case edits. Claim separate error case YAML files, coordinate any shared case migration before writing it, and change no runner internals. Cache 0011 owns root manifests/lock; this step adds no dependencies.
- Proof: Built CLI cases cover missing/extra arguments, unknown commands/schema IDs and success, with complete fixed error envelope at exit 2 and empty stderr. A distinctive bad argument appears in neither stream. Print biomcp.error/1 and check exact artifact ID. Existing schema-contract tests validate serialized errors, round trips and rejection of changed literals/messages/unknown fields; actual CLI cases assert that same payload. Preserve the real broken-output proof for exit 5. Run make landing-gate through pm job after current main and the coordinated runner migration.
- Defers: Full envelope bodies, query echoes, statuses for live data, requested sections and typed next steps wait for their first real command consumer. MCP and full surface/coverage checks stay with their stages. No entire-output-contract roadmap tick from this step alone.
