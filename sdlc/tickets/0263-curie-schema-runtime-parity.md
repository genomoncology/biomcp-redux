# 0263 — Align the Curie schema pattern with its runtime check

Status: OPEN.

Milestone: 1

## Outcome

The published `Curie` schema pattern rejects every string that `Curie` runtime validation rejects, including DEL (U+007F) and the C1 control characters U+0080 to U+009F that `char::is_control` refuses. A checked-ID contract case proves that the schema and runtime agree on the same accepted and rejected values.

- Why: the schema accepts `"DepMap:ACH\u007f1"` while runtime rejects it, so a client that trusts the published schema can send or expect a value BioMCP refuses.
- Starts from: `crates/biodata/src/variant/statement/reference.rs`, where the runtime check at line 16 rejects whitespace and control characters and the `text_value!` pattern at line 28 excludes only `\s` and `\x00-\x1f`; the 0257 cell line ID schemas as the accepted parity pattern; the coordinator report in m7153.
- Keeps: every currently valid Curie, the prefix character rule, the existing global schema names and every other schema. No source requests and no new test framework.
- Changes: the `Curie` pattern, the generated schemas it feeds, and one contract case in the existing biodata or biomcp-types contract tests.
- Proof: the contract case runs a fixed table through both runtime parsing and the published schema pattern, with DEL, a C1 control, a tab, a non-breaking space and valid IDs such as `DepMap:ACH-000362` and `HGNC:3765`, and asserts both reject or both accept each value. The case fails before the pattern change.
- PM ruling 2026-10-26: keep existing schema majors. Runtime never accepted these values, so no accepted value shrinks, and 1.0 is unreleased. The 41 generated artifacts may be corrected mechanically in the Curie pattern string only, after which the unchanged generator must reproduce them byte for byte and the parity table must pass. Runtime, schema identities and compatibility code stay unchanged. Any other difference the generator reports stops the work and returns to the PM.
- Defers: nothing.
