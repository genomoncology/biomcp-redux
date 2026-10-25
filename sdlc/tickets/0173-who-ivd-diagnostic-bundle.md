# 0173 — Sync the WHO in vitro diagnostics bundle and get the exact product card

Status: OPEN.

Milestone: 4

Depends on: 0170
Depends on: 0171
Depends on: 0172

## Outcome

`biomcp sync who-ivd` downloads WHO's prequalified in vitro diagnostics CSV export, validates its columns, keeps every row unfiltered and replaces the local copy atomically with a `biomcp.sync/2` receipt. `get diagnostic 0466-046-00 --source who-ivd` returns the exact product card with the whole product name and every native row for that WHO Product ID in source order, each keeping its product code, assay format, manufacturer, marker, regulatory version text and prequalification year.

- Why: Q23 and Q24 ask about WHO HPV diagnostics for cervical screening, and the 0154 rulings give WHO a card only, answered only for a known WHO Product ID.
- Starts from: diagnostic.md's WHO rows, card sketch, next-step table and case map, with the 0154 PM rulings and recheck amendment 5; the j12632 WHO capture; the shared `sync <bundle>` contract and stored-only bundle read API; 0172's dispatch registration; 0170's types and 0171's registry.
- Rules: rows group only by the native WHO Product ID, and neither a row count nor a code count is a product count. No WHO search, marker, manufacturer or type selector, and no section. A prequalification year is not a date, and regulatory version text never yields an FDA application. No gene, drug or FDA step; the card's steps are sync and fresh only. Reads never trigger a sync, and a failed validation keeps the old usable copy. WHO is skipped when the `noncommercial` group is disabled.
- Proof: outside-in cases first, watched to fail. Before adapter code, one bounded keyless pm job syncs the bundle once and records the receipt. Q23 cases assert the cards for `0455-180-00` (two rows), `0085-028-00` (one row), `0466-046-00` (nine rows) and `0633-078-00` (three rows); a bundle case asserts 435 rows across 125 products kept unfiltered on the 2026-10-25 capture; an unknown product ID asserts a not-found status; a missing bundle names `biomcp sync who-ivd`; a section or search with `--source who-ivd` fails before I/O. Sync cases cover a synthetic partial download, a malformed file, a new or missing column and two concurrent readers during one replacement, with zero requests on reads. Each case has a size ceiling from its measured CLI and MCP output, starting from the design's provisional 3,241 bytes for the nine-row card. Real WHO rows fall under WHO's terms, which reserve reproduction, so they replay from outside the repo through `--cases DIR`, and synthetic files prove sync mechanics. No licensed content enters the repo. Then make landing-gate runs through pm job.
