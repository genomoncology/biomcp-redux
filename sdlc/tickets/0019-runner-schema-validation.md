# 0019 — Validate every case result against its checked-in schema

Status: OPEN.

Milestone: 1

## Outcome

Every CLI and MCP application result in a YAML case is checked against the repository schema it names, and malformed or unknown schema results fail independently of case assertions.

Depends on: 0015

- Starts from: architecture 3.9 and 10.1–10.2; runner mod.rs run_step; assertions::check; mcp::call; binary schema.rs and build.rs generated artifact identities; checked-in biodata schemas; current typed usage envelope and raw schema command exception.
- Changes: `Cargo.toml`, `Cargo.lock`, `crates/biomcp/Cargo.toml`, `crates/biomcp/tests/runner/mod.rs`, `crates/biomcp/tests/runner/schema.rs`, `crates/biomcp/tests/runner/negative_cases.rs`, `crates/biomcp/tests/runner/proof.rs`, `crates/biomcp/tests/cases.rs`, `tests/cases/schema/**`, `crates/biomcp/tests/ratchets/source.rs`, `deny.toml`.
- Scope: Use the architecture-approved jsonschema test dependency with default features off and local-only schema resolution. Derive the test registry from existing checked-in artifact discovery, without another hand-maintained list. Validate each parsed CLI result and MCP structured application result even when expect is absent; parity and size ceilings remain separate checks. Unknown schema IDs, missing schema identity and invalid instances fail with useful pointer diagnostics. Raw schema-command artifacts remain raw schema JSON and are checked as the exact known artifact identity/content, rather than wrapped in an envelope. Never fetch schema references or access user files. Compile/reuse validators within the owning suite instead of per assertion. Verify actual dependency features/license metadata, keep exceptions exact package/version only if approved policy permits, and add no production dependency or public API.
- Keeps: Existing CLI bytes/exits, MCP framing/status and assertion semantics remain unchanged. Protocol errors are outside application-result validation. No normal response envelopes, query grammar, schema override YAML key or new registry format. Existing drift/compatibility tests retain their independent role.
- Proof: The real case runner accepts current schema artifacts and typed usage results over CLI/MCP. Focused runner proof demonstrates unknown/missing identities and invalid known-schema instances fail even without expect; schema resolution remains local and unsupported references fail without network. Preserve parsed parity and byte/token checks. Run make landing-gate through pm job after current main, reusing the coverage gate when landed. Use a test-only runner boundary for malformed-instance proof, never a product fake command.
- Ownership: Coverage 0016 owns launch.rs/proof.rs until it lands; keep preparation read-only and start development after its cleanup or narrow the actual claim to avoid that overlap. POST/XML own transport; this ticket owns the test-only validator dependency files when admitted.
- Defers: Normal generic envelopes and their cases arrive with the first gene command under the existing PM ruling. The schema runner must then pick up their checked-in artifacts without a new list.
