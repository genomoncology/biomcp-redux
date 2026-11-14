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

## Progress

Design check, 2026-11-14 (no code yet; awaiting a source decision).

- `test_version.gz` (read today) already has `CLIA_number`, and the card already shows it (`clia_number 05D0981414` for 560508). It has no specimen or CAP column.
- No small GTR bulk file has specimens or CAP. `/pub/GTR/data/` holds `GTR_test_IDs.csv`, `labs_tests_by_country.txt`, `test_condition_gene.txt/.xlsx`, `test_version.gz`, `test_version_microbe.gz`, `tests_by_method_category.txt` and `gtr_ftp.xml.gz`; `xml_archive/` keeps monthly copies of the XML.
- `gtr_ftp.xml.gz` has both: `GTRLabTest/Specimen/SpecimenType` per test (exact `GTRAccession` + `Version` attributes) and `GTRLab/LabCertification` per lab (Type, Name, Number, Effective/Expiration date). For 560508 v11 it lists the same five specimens as esummary, and lab 61756 lists `CAP 7154701` (expires 2026-10-26) plus CLIA and five state licenses. It is 184 MB gzip, 1.89 GB decoded; 63,256 tests, 32,865 with specimens, 341 labs (170 with CAP, 200 with CLIA). The extracted specimen and certification values are about 4.3 MB.
- The bundle layer cannot take it as is: the whole set is capped at 128 MiB (`MAX_BUNDLE_BYTES`), parts are held and decoded whole in memory, and native TSV decoding is capped at 256 MiB. Taking the XML needs a streamed gzip-XML part format with its own ceiling in `biomcp-fetch`.
- E-utilities `esummary db=gtr` (and `efetch`, which returns the same summary) has `specimens`, `certifications` (CAP 7154701 for 560508; CLIA, NYS CLEP, CAP and ISO15189 for 575663) and `testtargetlist`. It takes the numeric ID without a version and describes the current version only. It batches several IDs per call.
- Terms: the FTP README says GTR data is public domain and asks for attribution to GTR; E-utilities falls under the same NCBI policies already in `SPEC.terms`.
- GTR000575663 (ARUP EGFR T790M ctDNA): the XML has no test-level gene either; EGFR appears only as the gene of the test's listed variant (`FoundIn`). esummary's `testtargetlist` has EGFR. The XML would fix the empty gene cell only if genes were derived from listed variants.
