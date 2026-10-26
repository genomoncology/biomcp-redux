# 0280 — Correct docs that disagree with what ships and normalize the roadmap

Status: OPEN.

Milestone: 4

## Outcome

Architecture, method and roadmap text match shipped behavior, and every unticked roadmap box has an owning ticket or a recorded ruling.

- Why: the milestone 4 review found stale and conflicting statements that would mislead the next slice or the milestone 5 docs rewrite.
- Starts from: the review findings in this ticket, ROADMAP.md, README.md, architecture.md, slices.md and biodata-variants-and-annotations.md.
- Changes:
  - README.md line 18 says where HTTP checks run, matching release.md.
  - Bundle sync: architecture.md lines 861 to 862 and 1916 and README state the two shipped rules exactly: the legacy HGNC, ClinGen and EMA bundles still auto-sync on read under `auto_sync_max_bytes`, and the explicitly synced bundles, such as GTR, WHO and cBioPortal studies, never sync on read. No behavior change.
  - architecture.md line 1570's MCP description size and slices.md line 58's help cap rule follow the current caps and the measured-caps decision.
  - biodata-variants-and-annotations.md line 26 no longer says variant sections are open tickets.
  - ROADMAP.md marks the UMLS key line dropped with the disease ruling, ticks the MyDisease.info line citing 0137's trial condition ID resolution, explains or ticks GWAS Slice complete, and gives pharmacogenomics, diagnostic and adverse event the same checklist rows as the other slices, and ticks their clinical review and Slice complete rows citing 0169, 0177 and 0162 and their recorded clinical reviews; PM ruling 2026-10-26 (m8126) affirms those slices are accepted.
  - The comparator is named once: the parity ledger uses released 0.9.3, and the agent comparison uses the latest released 0.9.
  - architecture.md section 2.1 lists the dependencies the ratchet table allows, including rustls, rustls-platform-verifier, httpdate, tempfile, csv, flate2 and thiserror, or the ratchet drops them.
  - upgrade.md lists every legacy key variable, including `NCI_API_KEY` and `ORCID_ACCESS_TOKEN`.
- Proof: `make docs-check` passes, and a reviewer checks each listed line against the code or ruling it cites.
- Defers: the full docs rewrite, which is milestone 5 work.
