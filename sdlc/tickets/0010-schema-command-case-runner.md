# 0010 — Run outside-in YAML cases through the schema command

Status: OPEN.

Milestone: 1

## Outcome

The built BioMCP binary prints a named checked-in schema, and one YAML integration target runs real schema-command cases with JSON-pointer equality assertions and output size ceilings.

Depends on: 0005

- Starts from: `sdlc/plan.md` stages 5 and 6; architecture sections 3.9, 8.5 and 10.1; the existing checked-in core schemas and binary entry point.
- Changes: `crates/biomcp/Cargo.toml`, `crates/biomcp/build.rs`, `crates/biomcp/src/main.rs`, `crates/biomcp/src/config.rs`, `crates/biomcp/src/schema.rs`, `crates/biomcp/src/output.rs`, `crates/biomcp/tests/cases.rs`, `crates/biomcp/tests/runner/**`, `tests/cases/schema/**`, `crates/biomcp/tests/ratchets/source.rs`.
- PM decision: 2026-10-24. Land biomcp schema <name> with the first runner ticket, so cases exercise a real JSON-producing command. Schema output is plain schema JSON; envelope assertions arrive with the first envelope-producing command.
- Scope: Embed the checked-in schemas in the binary and print the requested schema by its canonical name. Derive discovery from existing schema artifacts rather than maintaining another schema-name list. Return explicit exit codes for success, bad arguments/unknown schema and local output failure. Build one harness=false integration target using libtest-mimic to name each YAML case and run Cargo's built binary. Initial case support is story metadata, run, expected exit, JSON-pointer equality, and max_bytes/max_tokens at the step or pointer; tokens use the architecture's compact-byte rule. Parse YAML with a maintained serde implementation and record why it was chosen. Isolate home/config/cache per case. Accept --cases DIR and Cargo-style filtering through the real test entry point; report a readable pointer/expected/actual failure.
- Ownership: Ticket 0008 owns root Cargo.toml/Cargo.lock until it reports ready. Request needed workspace dependency declarations and lock updates through the coordinator. This ticket owns binary runtime/test manifest entries. If schemas from 0009 are not ready, generate binary discovery from whichever schemas exist without adding stubs.
- Keeps: Schema printing reads embedded artifacts and calls no provider. Parse command arguments without inventing a general CLI grammar before request schemas exist. Add only required reasoned lint exceptions for argument/config and output boundaries. No fake product command, runner-only product hook, remote schema resolution, or new public dependency except the selected runner tooling.
- Proof: Cases through the built schema command verify a real schema's canonical identity and fields, success and bad arguments. Invoke the runner against isolated bad case files to prove wrong pointer values, missing pointers, wrong exits and breached whole-output/pointer ceilings each fail with the named case and readable diff. Assert case state is isolated. The runner's actual command must execute these proofs; tests of assertion helpers alone do not establish it. Run make landing-gate through pm job after integrating dependency changes and current main.
- Defers: MCP, replay/offline flags, next-step following and paging arrive with their surfaces. Remaining assertion kinds, surface coverage, envelope schema validation and the coverage floor follow as runner stages. No runner roadmap tick until the whole item is complete.
