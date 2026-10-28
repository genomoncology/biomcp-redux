# 0360 — Record every bad source value as a problem through shared helpers

Status: COMPLETE.
Landed: 33113b465d2310286a99ba83cce78f8163d62f3a

Milestone: 5

Depends on: 0387

## Outcome

One shared helper module records a source value an adapter cannot map: the field is left out, a problem names it, and the reply still returns. It replaces the 17 copies of `fn problem(` and 18 copies of `fn text(`. HGNC, ClinVar, EMA and Reactome named sites use it, including the ClinVar `detail.rs` optional condition qualified-ID `parse().ok()` site, and their problems reach the reply. The architecture states the rule. The remaining adapter families move in 0371 to 0375.

- Why: the docs lead's product findings report (2026-10-27), items D1 and D2. `Cytoband::new(...).ok()` in the HGNC map drops a bad value with no problem, empty text fails a whole ClinVar reply but is silently absent in EMA. Ian's ruling that 1.0 ships every known fix (2026-10-27).
- Starts from: the adapter map modules, `hgnc/map.rs`, `clinvar/map.rs`, `ema/raw.rs`, `reactome/map.rs` and `reactome/relations.rs`, and the architecture's adapter section.
- Rules: PM ruling 2026-10-27 (m10687): 0360 covers the shared helper, the 35 named copies and the HGNC, ClinVar, EMA and Reactome fixes with their problem propagation; the remaining adapter sites move in 0371. A required identity field that cannot map still fails the item, as the architecture's `Mapped { value, problems }` shape says. No text heuristics.
- Proof: cases for a bad cytoband, empty ClinVar text, a bad ClinVar condition qualified ID and empty EMA text each return the reply with one problem naming the field; a ratchet fails on a local `fn problem(` or `fn text(` copy in an adapter.
- Changes: adapters, the shared helper module, the architecture section, the ratchet and cases.
- Defers: nothing.
