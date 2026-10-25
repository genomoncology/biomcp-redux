# 0132 — Sync the DDInter bundle and add the drug interactions section

Status: COMPLETE.
Landed: 97ed80e5e079417cd3aa4daa96396be884784f1b

Milestone: 3

Depends on: 0124
Depends on: 0126
Depends on: 0131
Depends on: 0125
Depends on: 0148

## Outcome

`biomcp sync ddinter` downloads all eight DDInter files, validates them and replaces the local set atomically with a `biomcp.sync/2` receipt. `search drug TEXT --source ddinter` returns DDInter candidates, and `get drug DDInter1744 interactions --source ddinter` returns a page of native partner rows with pair IDs, partner names, the native level, the total, the bundle revision and freshness, and DDInter's terms, paged with `--section-limit` and `--section-offset`. DDInter is labeled noncommercial and is skipped when the `noncommercial` group is disabled.

- Why: the bank's interaction questions (D13, D16) need structured partner rows that label text lacks. The 0116 recheck makes interactions a section of `get drug` with no separate verb.
- Starts from: drug.md's DDInter rows, the bundle inventory and cases I01 and B01, with recheck amendment 1; the shared `sync <bundle>` contract; 0124 and 0125's `noncommercial` declaration.
- Rules: the section is the only structured interactions spelling. The B and L duplicate of Warfarin and Capecitabine deduplicates by source pair IDs and keeps both provenances. No management prose is invented, and a missing pair never means no interaction. No DDInter ID is attached to another source's card by name. Reads never trigger a sync, and a failed validation keeps the old set. `identity_level` comes from the DDInter record type, never from a field's shape.
- Proof: outside-in cases first. Before section code, a bounded keyless pm job syncs the bundle once and captures a positive exact-ID get for DDInter1744 and DDInter1951 and a known positive for the search. Cases S04 for `ddinter`, G01 for DDInter IDs, I01 (D13 DDInter1744 with 1397 and D16 DDInter1951 with 285 at Major, the deduplicated pair, source-local total and order) and the DDInter half of B01 (synthetic partial download, malformed file and concurrent readers during one replacement) pass with size ceilings. Real DDInter rows are CC BY-NC-SA, so they stay out of the repo and replay from outside it through `--cases DIR`, while synthetic files prove sync mechanics. No licensed content enters the repo. Then make landing-gate runs through pm job.
