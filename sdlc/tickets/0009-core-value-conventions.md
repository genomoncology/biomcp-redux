# 0009 — Add checked identifier, date and coordinate conventions

Status: OPEN.

Milestone: 1

## Outcome

The biodata public API supplies the identifier pattern, partial calendar dates and typed coordinate bounds that the first entity slice can reuse, with checked JSON construction and schemas.

Depends on: 0005

- Starts from: `sdlc/plan.md` stage 3; architecture sections 3.1, 3.2 and 3.9; the current canonical-value macro, error types, schema registry and public-library test conventions.
- Changes: `crates/biodata/src/lib.rs`, `crates/biodata/src/macros.rs`, `crates/biodata/src/error.rs`, `crates/biodata/src/date.rs`, `crates/biodata/src/coord.rs`, `crates/biodata/src/id/**`, `crates/biodata/schemas/**`, `crates/biodata/tests/**`, `crates/biodata/README.md`.
- Scope: Establish the private-field identifier pattern with HgncId as the first real identifier: checked positive numeric construction, canonical HGNC text, FromStr and Display, Hash with Eq, JSON through the same parser and a string schema with its pattern. Share textual serde/schema boilerplate without copying canonical facts. Add PartialDate at year/month/day precision with real calendar checks, and Position1 plus Span0 with the counting and bounds rules from architecture 3.2. Error enums use stable codes and retain no rejected text. Keep schemas optional and checked in through the existing registry.
- Keeps: No gene entity, provider grammar, gene symbol parser or other entity identifiers before their slice needs them. No default genome assembly, coordinate conversion or clinical interpretation. Values are immutable and constructors/deserialization enforce the same invariant. biodata stays free of I/O and dependency expansion. No generic binding-hostile public types.
- Proof: Public API boundary tables first fail without the types, then prove construction/serde parity, canonical text round trips, zero/overflow/malformed identifier rejection, partial-date precision, leap and invalid calendar dates, and coordinate boundaries. Errors omit distinctive bad input. Validate representative serialized values against generated schemas and run existing drift and bounded compatibility checks. Run make landing-gate through pm job after merging current main before review.
- Defers: GeneSymbol, other identifiers, Build, Contig and GenomicInterval arrive with the gene slice against its source formats. Query/envelope/section types and their schemas follow separately. ROADMAP ticks belong only to the landing candidate when its entire item is satisfied.
