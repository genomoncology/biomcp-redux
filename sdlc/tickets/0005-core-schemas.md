# 0005 — Generate and check the minimal core schemas

Status: COMPLETE.
Landed: 55d3cd871fa60a5ad54eaaed76de492b61eb422d

Milestone: 1

## Outcome

The existing biodata core types expose an optional schema feature with checked-in, versioned JSON Schemas, routine drift checks and compatibility checks against the checked-in contract before regeneration.

Depends on: 0003

- Starts from: `sdlc/plan.md` stage 3; `sdlc/planning/architecture.md` sections 3.9 and 3.10; `sdlc/planning/rust-standards.md`; and the public types in 0003. Generate only schemas for types that exist.
- Changes: `Cargo.lock`, `Cargo.toml`, `crates/biodata/Cargo.toml`, `crates/biodata/schemas/problem-code.json`, `crates/biodata/schemas/problem.json`, `crates/biodata/schemas/reason.json`, `crates/biodata/schemas/source.json`, `crates/biodata/schemas/text.json`, `crates/biodata/src/response/problem.rs`, `crates/biodata/src/source.rs`, `crates/biodata/src/text.rs`, `crates/biodata/tests/schemas.rs`, `crates/biodata/tests/schemas/compatibility.rs`, `crates/biodata/tests/schemas/compatibility_cases.rs`, `crates/biodata/tests/schemas/values.rs`, `crates/biomcp-fetch/Cargo.toml`, `crates/biomcp-fetch/clippy.toml`, `crates/biomcp-fetch/src/deadline.rs`, `crates/biomcp-fetch/src/declaration.rs`, `crates/biomcp-fetch/src/error.rs`, `crates/biomcp-fetch/src/lib.rs`, `crates/biomcp-fetch/src/request.rs`, `crates/biomcp-fetch/src/secret.rs`, `crates/biomcp-fetch/tests/transport.rs`, `crates/biomcp/Cargo.toml`, `deny.toml`.
- Scope: Derive JSON Schema 2020-12 for existing serializable public Text, Source, Reason, ProblemCode and Problem. Keep schemars optional behind schema; enable it in the binary's biodata dependency. One small test registry names each type, schema ID and major version. Generate checked-in schemas from that registry. Match canonical enum spellings, string newtypes, optional fields and rejected unknown fields. Reuse the canonical enum macro's spelling literals. Add no query types or general schema framework.
- Compatibility: Compare generated schemas with checked-in contracts before updating. Resolve local refs and compare property paths and types for forms these core types generate. Report unsupported forms explicitly. UPDATE_SCHEMAS=1 checks compatibility before writing and never runs in ordinary checks. Small schema tables cover optional additions, removal, rename, retyping, unresolved refs and a major increase.
- PM decision: 2026-10-24. Before any public release exists, compatibility runs before regeneration and reviewers check every snapshot change. An ordinary drift check cannot recover an older contract after coordinated edits. Independent comparison with last released schemas belongs to milestone 5 and must land before the first public release.
- Ownership: Transport ticket 0004 owns root workspace declarations and Cargo.lock in the parallel batch. Request dependency declarations and lock updates through the coordinator. This ticket owns biodata files and the binary's biodata dependency feature. The transport developer assembles the combined candidate after both branches are ready; a combined gate precedes review.
- Keeps: The Rust types remain the source for schema generation. biodata product code does no I/O and never names serde_json. Schema tooling lives in the documented public-library test exception. No query or entity types, CLI schema command, grammar or transport behavior. No synthetic API types just to exercise a future schema subset check.
- Proof: Run make landing-gate through pm job against current main. The gate detects schema drift and validates representative serialized public values without remote resolution. Enum schemas accept every canonical spelling and reject unknown ones. Compatibility tables reject same-major removal, rename and retyping, accept optional additions and permit a documented major-version break. A changed schema file fails ordinary drift checking.
- Defers: Query-subset and disclosure-layer checks arrive with the query and entity types they constrain. Identifier/date/coordinate schemas arrive with those types. CLI schema printing and envelope validation belong to stage 6. No roadmap schema tick until the whole roadmap item is satisfied.
