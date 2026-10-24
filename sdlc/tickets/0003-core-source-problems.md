# 0003 — Add the source and problem types the transport needs

Status: OPEN.

Milestone: 1

## Outcome

The biodata public API validates text once and round-trips typed source identities, failure reasons and field problems through JSON without storing input text in errors.

Depends on: 0001
Depends on: 0002

- Starts from: `sdlc/plan.md` stage 3's minimal types, `sdlc/planning/architecture.md` sections 3.2, 3.6 and 3.8, and `sdlc/planning/rust-standards.md` biodata rules. The workspace and ratchets arrive in the prerequisite batch.
- Changes: `Cargo.toml`, `Cargo.lock`, `crates/biodata/Cargo.toml`, `crates/biodata/README.md`, `crates/biodata/src/lib.rs`, `crates/biodata/src/text.rs`, `crates/biodata/src/source.rs`, `crates/biodata/src/response/mod.rs`, `crates/biodata/src/response/problem.rs`, `crates/biodata/tests/core.rs`.
- Scope: Implement the approved Source and Reason enums and Problem shape, plus the checked Text and closed ProblemCode values those fields need. Use the design's canonical names and stable codes. Add only dependencies this API uses. Types have serde in both directions; checked constructors and deserialization share validation. Error messages carry stable codes and never rejected text. Immutable owned text uses Box<str>. Export no entity types or transport behavior.
- Keeps: biodata does no I/O and names no other workspace crate. Source identity remains an enum. Problem's record is optional checked text and its field remains the design's field-name text. Product text uses BioMCP or BioMCP 1.0. No provider requests, credentials, clinical interpretation or roadmap ticks.
- Proof: Public API table tests first fail without the types, then pass for canonical serde values, Text trimming and rejection of empty or whitespace-only text, stable error codes that omit input, and Problem JSON round trips with absent optional fields omitted. Reject unknown closed-set names. Run `make landing-gate` through pm job after merging main. Keep the table in the documented biodata public-test exception; add no product test hooks.
- Defers: Identifier, date and coordinate conventions, schemas, envelope and section types arrive in the rest of stage 3. Transport starts after this minimal API lands. ROADMAP updates belong to the lander when their entire item is satisfied.
