# 0460 — Parse every public CIViC and ClinVar record into 1.0 values

Status: OPEN.

Milestone: 5

Depends on: 0435, 0436, 0440, 0447

## Outcome

A sweep reads the CIViC monthly TSV release, the ClinVar variant summary and the ClinVar-GKS release, and the CC0 CIViC and ClinGen evidence sets, and parses every variant, molecular profile, disease, drug, citation, identifier and HGVS string into its 1.0 value. Each record parses or is refused with a named reason, and the report counts both per type. Per-gene CIViC counts match the dump, and every variant ID BioMCP prints for a ClinVar record matches ClinVar's own spelling.

- Why: Ian's rule that BioMCP's first users are the public datasets we try to replicate (m13050, 2026-10-28). The 1.0 data model freezes at the tag, so values the real data needs must be found now.
- Starts from: the CIViC monthly TSV (CC0), ClinVar and ClinVar-GKS (public domain), the evidence sets in the research benchmarks folder, and the biodata value types.
- Rules: the sweep is a test tool, not a product command; BioMCP itself still never downloads bulk data for a user. Dumps stay outside the repository, pinned by release date and checksum, and run as a job; only a small sampled fixture of refused and edge rows enters the repository. A refusal with a reason the source format allows becomes a value fix in the owning BioData ticket or a new ticket.
- Proof: the report shows zero unexplained refusals; each explained refusal names the source rule that makes the record invalid; the sampled fixture fails a build that reverts any fix the sweep caused.
- Changes: a sweep tool under the test tooling, the sampled fixture, and fixes it finds.
- Defers: GENIE, MSK-CHORD and other registered or restricted datasets until Ian approves.
