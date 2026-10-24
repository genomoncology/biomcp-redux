# 0019 — Validate every case result against its checked-in schema

Status: OPEN.

Milestone: 1

## Outcome

Every CLI and MCP application result in a YAML case is checked against the repository schema it names, and malformed or unknown schema results fail independently of case assertions.

Depends on: 0015

- Starts from: architecture 3.9 and 10.1–10.2; runner mod.rs run_step; assertions::check; mcp::call; binary schema.rs and build.rs generated artifact identities; checked-in biodata schemas; current typed usage envelope and raw schema command exception.
- Changes: `Cargo.lock`, `crates/biomcp/Cargo.toml`, `crates/biomcp/tests/runner/mod.rs`, `crates/biomcp/tests/runner/schema.rs`, `crates/biomcp/tests/runner/schema_proof.rs`, `crates/biomcp/tests/cases.rs`, `tests/cases/schema/**`.
- Scope: Use existing workspace jsonschema with default features off only in the runner. Include the build-generated schema table from OUT_DIR; no runtime scan or second identity list. Compile validators once per owning suite and reuse them. Resolve local $defs and registered artifact references in memory; unregistered references fail with no filesystem or network retrieval. Preserve relative artifact $id values. Both parsed CLI and MCP results pass one mandatory validation boundary before assertions, including no-expect and paired cases. A normal result requires string schema naming a registered artifact and must validate. Raw schema JSON is allowed only for actual successful schema <known id>; require equality with the requested checked-in artifact, including $id, regardless of formatting. Arbitrary $id, another artifact or mutation cannot bypass validation. Bound identity/pointer/value diagnostics; never dump an entire instance or error list. Keep parity, status, ceilings and assertions independent; no production dependency, public API, YAML override or new registry format.
- Keeps: Existing CLI bytes/exits, MCP framing/status and assertion semantics remain unchanged. Protocol errors are outside application-result validation. No normal response envelopes, query grammar, schema override YAML key or new registry format. Existing drift/compatibility tests retain their independent role.
- Proof: Real runner accepts raw artifacts and typed usage over CLI/MCP, including no-expect cases. A small runner-only proof uses the same parsed-result boundary with empty assertions: unknown/missing/nonstring identity, invalid nested biomcp.error/1 code and missing required field fail schema validation independently of exit/parity/assertions. Reject a modified raw artifact and known artifact for another schema request. Valid/invalid nested instances exercise local $defs; a small in-memory registry fixture proves registered cross-artifact constraints work. Reject unregistered HTTP/file references with retrieval unavailable. Keep proof in existing executable with no product fake command or alternate grammar. Preserve parity/ceiling proof; run standard landing gate through pm job, using coverage when available.
- Ownership: Coverage 0016 owns launch.rs/proof.rs; this ticket uses separate schema_proof.rs and may run in parallel without editing them. POST owns transport, while pending XML waits for POST and this ticket root dependency ownership. Correct narrow claims as needed.
- Defers: Normal generic envelopes and their cases arrive with the first gene command under the existing PM ruling. The schema runner must then pick up their checked-in artifacts without a new list.
