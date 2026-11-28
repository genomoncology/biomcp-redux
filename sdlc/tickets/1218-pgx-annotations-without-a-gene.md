# 1218 — pgx-annotations-without-a-gene

Status: COMPLETE.
Landed: 0c98747cbd2ada43b33cb048300d0f856e2e59bb

Ruling: Ian, 2026-11-27, on the recommendation that variant-first and drug-first pharmacogenomic routes come back because some gene-less annotations are level 1A: "Yes on your rec." This supersedes the route drops in tickets 0166 and 0168. Build option (a): `search pgx --variant <rsID>` and `--drug <name>`.

Milestone: 10

## Outcome

An agent can read ClinPGx clinical annotations starting from a variant (rsID) or a drug, so annotations that name no gene are reachable, and can narrow a gene's annotations by evidence level and drug.

## Why

254 of ClinPGx's 5,192 clinical annotations (4.9%) name no gene, including level 1A ones, and every PGx route today starts from a gene.

## Evidence

Checked 2026-11-27 against the live source and the 1.0.0 build at 4efd1dcb2426. Full tables: [source reach census](../records/2026-11-27-source-reach-census.md).

- Failing case: `biomcp get pgx annotations --gene rs12777823 --source clinpgx` returns `failed`, `not-found`. ClinPGx holds level 1A clinical annotation PA166134655 for rs12777823 and warfarin; it is reachable only by an accession the agent has no way to find.
- `biomcp search pgx --drug warfarin --source cpic` returns `usage.retired`; CPIC lists seven genes for warfarin. Tickets 0166 and 0168 dropped the drug-first route on purpose.
- Gene routes are faithful: CYP2D6 122 of 122, CYP2C9 74, DPYD labels 12. There is no selector for level, drug or agency, so a caller pages 50 rows per gene.
- Options: (a) `search pgx --variant <rsID>` and `--drug <name>` returning annotation and pair rows; (b) variant and drug sections on `get variant` and `get drug`. Census recommendation: (a), because it reuses the PGx reply shapes. The PM decides, since 0166 ruled the other way.

## Proof

- Proof: outside-in cases on recorded replies: the rs12777823 annotation is returned from a variant-first call; a drug-first call for warfarin returns its CPIC genes and ClinPGx annotations; `--level 1A` narrows a gene's rows.

## Starts from

- Starts from: `crates/biomcp-adapters/src/clinpgx/read.rs`, `crates/biomcp-adapters/src/clinpgx/annotation_raw.rs`, `crates/biomcp-adapters/src/cpic/read.rs`, `crates/biomcp-query/src/pgx/`.

## Keeps

- Keeps: gene-first replies and totals are unchanged.

## Defers

- Defers: ClinPGx variant annotations, pathways and haplotype records, which have no route.

## Changes

- Changes: `crates/biomcp-adapters/src/clinpgx/`, `crates/biomcp-adapters/src/cpic/`, `crates/biomcp-query/src/pgx/`, grammar and help, PGx cases.
