# 0048 — Build the --gene PubMed term without previous symbols

Status: OPEN.

Milestone: 1

## Outcome

`search article --gene INPUT` sends PubMed the group `(APPROVED_SYMBOL OR INPUT_AS_TYPED)`. When the two names are equal without case, the group is the bare symbol. The rest of the term follows the landed order: the group, then TEXT, `--variant`, `--drug` and `--disease`, joined by single spaces, with ` AND ` after the group when anything follows. Previous symbols and aliases that the agent did not type are never added. Resolution, disambiguation and absent answers stay as 0035 landed them.

- Changes: `crates/biomcp-query/src/article/term.rs`, `tests/cases/article/search/gene-terms.yaml`, `tests/cases/article/search/her2-floor.json`, `tests/replies/article/search/live/pubmed/**`, `tests/replies/article/search/synthetic-gene-deadline/pubmed/**`, `tests/replies/article/search/measurements.json`, `sdlc/tickets/0035-article-search-pubmed.md`. Replace automatic previous-symbol expansion and refresh its recorded public proof.
- Starts from: `decisions/2026-10-24-no-text-heuristics.md`; the term rule in article.md section 5 as amended; 0035's term builder in `crates/biomcp-query/src/article/**` and its cases under `tests/cases/article/search/`.
- Why: bare previous symbols are not field-tagged, so PubMed matches them as words. ABCC8's previous symbol SUR grew the ABCC8 group from 2,216 to 171,977 records by matching the French word "sur", and ERBB2's NGL added no record. The typed input reaches PubMed either way.
- PM decision: 2026-10-24. The group holds only the approved symbol and the input as typed. An HGNC ID input such as `HGNC:6407` contributes only the approved symbol, because an ID is not a word PubMed indexes.
- Keeps: the pool, page, ceiling and failure behavior of 0035; every case whose term has no previous symbol; the query echo of the exact term.
- Proof: re-record the affected replies once with `--record`. Case 6 (`--gene HER2 low trastuzumab deruxtecan`) asserts the term `(ERBB2 OR HER2) AND low trastuzumab deruxtecan`, 35665782 within the first 10, and its walk equals the newly recorded floor. Case 7 (`--gene SUR1 R1437Q`) asserts the term `(ABCC8 OR SUR1) AND R1437Q` with whatever total and notice the recording shows. `--gene KRAS --variant G12C --drug sotorasib --disease lung` asserts `KRAS AND G12C sotorasib lung`. `--gene AKT1 --variant E17K` keeps `AKT1 AND E17K` and total 192. A case with `--gene HGNC:6407 G12C` asserts `KRAS AND G12C`. Search every other landed case and ticket proof for a term that pins a previous symbol, and update each one. Page ceilings follow the m4341 rule. Then make landing-gate runs through pm job.
- Defers: nothing.

- Reviewed boundaries, 2026-10-24: j9381 proved the owning command still included NGL before the fix. j9391 captured changed PubMed canonical requests with checked final terms passed as TEXT, using the saved HER2 ESearch and its explicit pool after the first capture exceeded the command deadline. Record mode forces bundle refreshes, so metadata capture omits --gene; owning replay cases use existing HGNC rows and prove generated term identities match. No source behavior changed. HER2 now records 547 unique PMIDs over 55 pages in exact ESearch order, with 35665782 first. SUR1 records count 0 and only R1437Q in not_found_words. KRAS records 390; HGNC:6407 G12C records 2,068. AKT1 replies and its 192-PMID floor stay unchanged.
- Measured owning replay pages: HER2 first 5,052 bytes and largest 5,604, giving ceilings 5,810 and 6,445; SUR1 526 gives 605; KRAS 4,511 gives 5,188, below its matching 1,305-token baseline; HGNC-ID 4,150 gives 4,773. No exact old baseline exists for expanded HER2, SUR1 or the HGNC-ID question; no replacement was inferred. Existing AKT1 ceiling stays unchanged. Historical checks in 0035 remain labeled historical, while its case proof now names the current terms.
