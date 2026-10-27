# 0360 — Record every bad source value as a problem through shared helpers

Status: OPEN.

Milestone: 5

## Outcome

Every adapter handles a source value it cannot map the same way: the field is left out and a problem names the field, and the reply still returns. No adapter drops a bad value silently, and no single bad text field fails a whole reply. The architecture states this rule. One shared helper module replaces the 17 copies of `fn problem(` and 18 copies of `fn text(`.

- Why: the docs lead's product findings report (2026-10-27), items D1 and D2. `Cytoband::new(...).ok()` in the HGNC map drops a bad value with no problem, empty text fails a whole ClinVar reply but is silently absent in EMA. Ian's ruling that 1.0 ships every known fix (2026-10-27).
- Starts from: the adapter map modules, `hgnc/map.rs`, `clinvar/map.rs`, `ema/raw.rs`, `reactome/map.rs` and `reactome/relations.rs`, and the architecture's adapter section.
- Rules: PM ruling 2026-10-27 (m10687): 0360 covers the shared helper, the 35 named copies and the HGNC, ClinVar, EMA and Reactome fixes with their problem propagation; the remaining adapter sites move in 0371. A required identity field that cannot map still fails the item, as the architecture's `Mapped { value, problems }` shape says. No text heuristics.
- Proof: cases for a bad cytoband, empty ClinVar text and empty EMA text each return the reply with one problem naming the field; a ratchet fails on a local `fn problem(` or `fn text(` copy in an adapter.
- Changes: adapters, the shared helper module, the architecture section, the ratchet and cases.
- Defers: nothing.
