# 0931 — gtr-specimens-and-certifications

Status: OPEN.

Milestone: 7

## Outcome

A GTR test card, and a search row where cheap, carries the test's specimen types and its lab's certifications (CLIA and CAP numbers) as NCBI publishes them. The card says where they came from.

## Why

The persona needed specimen and accreditation columns for its test menu and validation packet; BioMCP's GTR card had neither, so it went to E-utilities (log.md:58, 77-80), and code mode filled specimen on 21 of 21 rows from esummary (raw log.md:79-82, 320-322). Evidence: bench `personas/dx-lab-director/runs/2026-11-14/` (j40302, BioMCP f4f726674) and its code-mode pair `runs/2026-11-14-raw/` (j40303); the analyst's source checks are listed at the end of `analysis.md`. NCBI's esummary for test 560508 lists five specimens and CAP 7154701; BioMCP reads `test_version.gz`, which has no specimen column. The same check found that the bulk file leaves the gene cell empty for one current EGFR test (GTR000575663, ARUP's ctDNA T790M test), which E-utilities indexes; note whether the chosen source fixes that too.

## Proof

- Proof: a recorded NCBI reply for test 560508 gives its five specimens and CAP 7154701 on the card.

## Starts from

- Starts from: `crates/biomcp-adapters/src/gtr/`; NCBI E-utilities esummary for GTR (`db=gtr`) and GTR's other bulk files. Check live which source holds specimens and certifications and its terms; prefer a bulk file over per-test calls if one has them.

## Keeps

- Keeps: the 1.0.0 schema rules (optional fields only); GTR search counts and paging; no network reads on search if the current search is offline.

## Defers

- Defers: CPT codes (GTR's field is empty upstream: esummary `cptcode ""`).

## Changes

- Changes: the GTR adapter, its card and row types, cases.
