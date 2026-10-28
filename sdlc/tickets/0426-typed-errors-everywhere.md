# 0426 — Use typed errors everywhere

Status: OPEN.

Milestone: 5

Depends on: 0434

## Outcome

biodata declares every error code once, the stray error types fold into it, and one shared checked-ID macro replaces the local ones. No public signature returns a string or unit error. Internal enums drop `non_exhaustive` and callers match exhaustively. Error conversions keep the IO kind. Environment values, keys included, are read as UTF-8 and refused if invalid. Recording times use `Option` instead of 0 for unknown. Library decode errors on `wire_enum!` types leave the input value out of their text, and `FiniteNumberError`, `GwasValueInvalid`, `ReportIdInvalid`, `PathwayIdInvalid`, `AuthorIdInvalid`, `CellLineIdInvalid` and the `&'static str` error in `TrialCoordinates` fold into the declared codes. `License` and `ArticleMatchKind` stop reporting `source.unknown`, and a protein `Bound` with both ends missing stops reporting `coord.span.order`.

- Why: the pre-1.0 code quality review `sdlc/records/2026-10-28-code-quality-review.md`, and the BioData review of BioMCP 1.0's data model (2026-10-28), ask 20.
- Starts from: `crates/biodata/src/error.rs` and the named ID modules, `crates/biomcp/src/grammar.rs`, `config.rs`, `crates/biomcp-fetch/src`, `record.rs`, `cache.rs` and the three callers of `fetched`.
- Rules: error code strings stay stable; a test checks every biodata display code against the declaration.
- Proof: a library decode of an unknown enum value has error text without the value; the code test fails on a planted stray code; a case refuses an invalid UTF-8 key without printing it.
- Changes: the files above and cases.
- Defers: nothing.
