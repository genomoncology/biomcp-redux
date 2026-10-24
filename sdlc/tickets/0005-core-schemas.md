# 0005 — Generate and check the minimal core schemas

Status: OPEN.

Milestone: 1

## Outcome

The existing biodata core types expose an optional schema feature, and the routine gate rejects drift or incompatible changes to their checked-in, versioned JSON Schemas.

Depends on: 0003

- Starts from: `sdlc/plan.md` stage 3; `sdlc/planning/architecture.md` sections 3.9 and 3.10; `sdlc/planning/rust-standards.md`; and the public types in 0003. Generate only schemas for types that exist.
- Changes: `Cargo.toml`, `Cargo.lock`, `crates/biodata/Cargo.toml`, `crates/biodata/src/lib.rs`, `crates/biodata/src/source.rs`, `crates/biodata/src/text.rs`, `crates/biodata/src/response/problem.rs`, `crates/biodata/schemas/**`, `crates/biodata/tests/schemas.rs`, `crates/biodata/tests/schemas/**`, `crates/biomcp/Cargo.toml`.
- Scope: Derive schema from existing public serializable core types with schemars behind feature schema. The binary enables that feature, so normal gate builds use the shipped feature set. Check in named, major-versioned schemas under biodata/schemas. One schema registry feeds generation, drift and compatibility checks. UPDATE_SCHEMAS=1 regenerates intentionally. Compatibility resolves local refs and compares existing paths and types as architecture 3.9 specifies; optional additions keep the major while removals, renames and retyping require a major increase. Keep any unsupported comparison case explicit rather than silently passing it.
- Keeps: The Rust types remain the source for schema generation. biodata product code does no I/O and never names serde_json. Schema tooling lives in the documented public-library test exception. No query or entity types, CLI schema command, grammar or transport behavior. No synthetic API types just to exercise a future schema subset check.
- Proof: Public schema tests fail on a changed checked-in schema, a removed or retyped existing path without a major increase, or an unresolved local ref. Isolated schema tables demonstrate that optional additions pass and major-version changes permit the documented break. Validate representative canonical core JSON against its generated schema without remote resolution. Run make landing-gate through pm job after merging current main. Keep schema update mode from running in ordinary checks.
- Defers: Query-subset and disclosure-layer checks arrive with the query and entity types they constrain. Identifier/date/coordinate schemas arrive with those types. CLI schema printing and envelope validation belong to stage 6. No roadmap schema tick until the whole roadmap item is satisfied.
