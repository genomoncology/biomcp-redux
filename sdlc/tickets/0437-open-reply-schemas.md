# 0437 — Publish open reply schemas and make the gate protect readers

Status: OPEN.

Milestone: 5

## Outcome

Reply and value schemas are generated without `additionalProperties: false`, and reply types are read without `deny_unknown_fields`. Query schemas stay closed. The `biomcp schema` root description and the docs state that readers ignore unknown fields and unknown enum values within a major, and growable enums such as `Source`, `Reason`, `Proposition` and `Predicate` carry an `x-` annotation saying so. The compatibility gate marks each contract as query or reply: for replies, removing a name from the required set takes a major, a reviewed pattern widening recorded beside the baseline is allowed, and a declared successor schema ID is allowed at a new major.

- Why: the BioData review of BioMCP 1.0's data model (2026-10-28), ask 3 and 10. Ian ruled that incomplete or incorrect 1.0 data model types are fixed before the tag, because after it the release gate freezes every pattern, type set and schema ID.
- Starts from: `crates/biomcp-types/schemas`, the schema generator, the reply type derives and `crates/biomcp-types/tests/schemas/compatibility.rs:35`, `:54`, `:67` and `:68`.
- Rules: `architecture.md` near line 1057 already says a removed required reply field takes a major; the gate now enforces it.
- Proof: a recorded reply with an added property at `/result` and at one nested value and an unknown enum value validates against the baseline schema with a `jsonschema` dev-dependency (approved) and reads into its Rust type; a planted reply change that removes one required name fails the gate.
- Changes: the generator, derives, gate, docs and baseline.
- Defers: nothing.
