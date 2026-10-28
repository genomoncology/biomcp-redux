# 0440 — Make every schema pattern match its parser

Status: OPEN.

Milestone: 5

## Outcome

Every text value's schema pattern accepts exactly what its parser accepts: `CaId` refuses `CA000000`, `VcvId` refuses `VCV000000000`, `HgvsText::PATTERN` refuses `(123)` references and `.0` versions, and `Doi` refuses C1 control characters, with any other mismatch the test finds.

- Why: the BioData review of BioMCP 1.0's data model (2026-10-28), ask 4. Ian ruled that incomplete or incorrect 1.0 data model types are fixed before the tag, because after it the release gate freezes every pattern, type set and schema ID.
- Starts from: `crates/biodata/src/variant/id.rs:37`, `HgvsText::PATTERN`, `Doi` and every type built with `text_value!`.
- Rules: fix the pattern or the parser, whichever matches the published format.
- Proof: a table in `tests/schemas/values.rs` that asserts `validator.is_valid(s) == s.parse::<T>().is_ok()` on near-miss strings for every `text_value!` type.
- Changes: the patterns, the table and the baseline.
- Defers: nothing.
