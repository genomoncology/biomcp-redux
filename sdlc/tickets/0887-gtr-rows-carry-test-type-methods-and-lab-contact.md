# 0887 — gtr-rows-carry-test-type-methods-and-lab-contact

Status: COMPLETE.
Landed: d5561d2791a7f480bbaee9f8e713e1f42f77c2ce

Milestone: 7

## Outcome

A GTR test search answers "what tests exist for this gene and where can the family order one" in as few calls as NCBI itself needs: each row carries the test's gene count, method categories, lab location and order link, and the search can return all of a gene's tests (a few hundred) so the agent counts them itself.

## Why

In the rare-disease persona run of 2026-11-13 (biomed-agent-bench `personas/rare-disease/runs/2026-11-13/log.md`, analysis beside it; code-mode run in `runs/2026-11-13-raw/`), the agent paged 5×50 rows and then fetched 8 tests × 2 cards to sample single-gene vs panel vs exome, and still had no lab address, director or order link (log.md:255 on). The code-mode agent got all 215 KMT2D tests with gene count, methods, the lab's full address, director and order URL from one E-utilities search and three esummary batches (raw log.md:160-168).

## Proof

- Proof: a recorded case for KMT2D where search rows carry gene count, method categories, lab city/state/country and order or test URL, all from the recorded GTR reply; the search returns every test in one call or states the total and its pages; the counts by gene-count bucket agree with the recorded reply.

## Starts from

- Starts from: `crates/biomcp-adapters/src/gtr/`.

## Keeps

- Keeps: the test card and its sections.

## Defers

- Defers: a filter on test type.

## Changes

- Changes: `crates/biomcp-adapters/src/gtr/`, `crates/biomcp-types/`, `tests/cases/diagnostic/`, `docs/`

## Progress

- Measured live on 2026-11-13: BioMCP's GTR adapter reads NCBI's stored files `test_version.gz` (13 MB) and `test_condition_gene.txt` (47 MB), not E-utilities. KMT2D has 211 tests there, the same count E-utilities gave the persona. The version file's `genes` cell gives the gene count; it split KMT2D's tests 13 single-gene, 16 with 2-10 genes, 46 with 11-100 and 136 over 100, exactly as esummary `genecount` did (log.md:299). Its `method_categories` counts also match. The file holds the lab's state, postcode and country but no street, city, director or order link.
- Those lab contact fields exist only in E-utilities esummary and in `gtr_ftp.xml.gz` (184 MB compressed, 1.9 GB uncompressed). Adding either is a new source path, not a row change, so it is left out here.
- Changed: each GTR row (and the card header) now carries `gene_count`, `method_categories` and `laboratory.state` and `laboratory.postcode`, all optional fields. The schemas were regenerated and pass the release check.
- Reply size: a 50-row KMT2D page is 41-44 KB, so all 211 rows would be about 180 KB in one reply, over what MCP clients take comfortably. The page limit stays 50; `page.total` says 211 and the reply offers the next page, so five calls cover the gene.
- Case `tests/cases/diagnostic/gtr-kmt2d.yaml` replays the live files cut to KMT2D's 211 relation rows and their version rows. It checks the first row's new fields and walks all five pages, matching every ID and gene count against the recorded reply.
