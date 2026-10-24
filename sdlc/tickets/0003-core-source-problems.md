# 0003 — Add the source and problem types the transport needs

Status: OPEN.

Milestone: 1

## Outcome

The biodata public API validates text once and round-trips typed source identities, failure reasons and field problems through JSON without storing input text in errors.

Depends on: 0001
Depends on: 0002

- Starts from: `sdlc/plan.md` stage 3's minimal types, `sdlc/planning/architecture.md` sections 3.2, 3.6 and 3.8, and `sdlc/planning/rust-standards.md` biodata rules. The workspace and ratchets arrive in the prerequisite batch.
- Changes: `Cargo.toml`, `Cargo.lock`, `deny.toml`, `crates/biodata/Cargo.toml`, `crates/biodata/README.md`, `crates/biodata/src/lib.rs`, `crates/biodata/src/text.rs`, `crates/biodata/src/source.rs`, `crates/biodata/src/response/mod.rs`, `crates/biodata/src/response/problem.rs`, `crates/biodata/tests/core.rs`.
- Scope: Implement Source and Reason from the named architecture sections, Problem with its documented fields, checked Text and the minimal ProblemCode set below. Implement FromStr and Display for types with text forms, with parsing of each printed value returning an equal value. Construction and deserialization share Text validation. Add only dependencies this API uses. Immutable owned text uses Box<str>. Export no entity types or transport behavior.
- Reviewed API detail: Source initially has the seven variants in architecture section 3.6. Reason has the eleven variants in section 3.8. Both use canonical JSON names and remain non_exhaustive. ProblemCode initially contains ValueUnknown with spelling value.unknown; identifier and date codes arrive with their validators.
- Reviewed API detail: One public biodata error enum covers Text construction and canonical-name parsing, with stable codes text.empty, source.unknown, reason.unknown and problem_code.unknown. Errors retain no rejected input. Problem rejects unknown JSON fields and omits an absent record.
- Keeps: biodata does no I/O and names no other workspace crate. Source identity remains an enum. Problem's record is optional checked text and its field remains the design's field-name text. Product text uses BioMCP or BioMCP 1.0. No provider requests, credentials, clinical interpretation or roadmap ticks.
- Proof: Add routine tables in crates/biodata/tests/core.rs through the public library API. Exercise every Source, Reason and ProblemCode variant through FromStr, Display and JSON round trips. Exercise Text construction and JSON deserialization with surrounding whitespace, empty input and whitespace-only input; both paths produce equal checked values or the same stable error code. Unknown canonical names fail through parsing and JSON. Errors omit a distinctive rejected input. Round-trip Problem with and without record, assert omission and reject an unknown field. Observe failure before implementation, then run make landing-gate through pm job after merging main. Use the documented biodata public-test exception and add no product test hooks.
- Defers: Identifier, date and coordinate conventions, schemas, envelope and section types arrive in the rest of stage 3. Transport starts after this minimal API lands. ROADMAP updates belong to the lander when their entire item is satisfied.
