# 0314 — Freeze the 1.0 schema baseline from the final candidate source

Status: OPEN.

Milestone: 5

## Outcome

The 0290 schema baseline holds the current 275-file schema set exported from product source commit 20929da9e748bece4fef6d1d6ae78e3691dbd00c, with a manifest that names that source commit and proves exact inventory and content equality with the candidate's generated schemas. The commit that lands this refresh becomes the final candidate for 0293.

- Why: schema-first job j16553 passed compatibility and drift checks but found the provisional 0290 baseline from 831719b9 differs in 7 of 275 files, all from the approved 0312 change that removes the ClinVar default of 3. 0293 requires the published baseline to match the final candidate. PM ruling 2026-10-27 (m8126 ask 55).
- Starts from: the 0290 gate and provisional snapshot, the j16553 evidence on the evidence store under 0293, and the 0293 rules.
- Rules: the manifest names the real product source commit and never its own landing commit. No product schema change and no version bump.
- Proof: the gate passes against the refreshed baseline, and a check shows the 275 baseline files equal the files generated from the candidate, byte for byte.
- Changes: the baseline snapshot and its manifest.
- Defers: publication, which needs Ian.
