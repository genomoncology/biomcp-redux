# 0048 — Build the --gene PubMed term without previous symbols

Status: OPEN.

Milestone: 1

## Outcome

`search article --gene INPUT` sends PubMed the group `(APPROVED_SYMBOL OR INPUT_AS_TYPED)`. When the two names are equal without case, the group is the bare symbol. The rest of the term follows the landed order: the group, then TEXT, `--variant`, `--drug` and `--disease`, joined by single spaces, with ` AND ` after the group when anything follows. Previous symbols and aliases that the agent did not type are never added. Resolution, disambiguation and absent answers stay as 0035 landed them.

- Starts from: `decisions/2026-10-24-no-text-heuristics.md`; the term rule in article.md section 5 as amended; 0035's term builder in `crates/biomcp-query/src/article/**` and its cases under `tests/cases/article/search/`.
- Why: bare previous symbols are not field-tagged, so PubMed matches them as words. ABCC8's previous symbol SUR grew the ABCC8 group from 2,216 to 171,977 records by matching the French word "sur", and ERBB2's NGL added no record. The typed input reaches PubMed either way.
- PM decision: 2026-10-24. The group holds only the approved symbol and the input as typed. An HGNC ID input such as `HGNC:6407` contributes only the approved symbol, because an ID is not a word PubMed indexes.
- Keeps: the pool, page, ceiling and failure behavior of 0035; every case whose term has no previous symbol; the query echo of the exact term.
- Proof: re-record the affected replies once with `--record`. Case 6 (`--gene HER2 low trastuzumab deruxtecan`) asserts the term `(ERBB2 OR HER2) AND low trastuzumab deruxtecan`, 35665782 within the first 10, and its walk equals the newly recorded floor. Case 7 (`--gene SUR1 R1437Q`) asserts the term `(ABCC8 OR SUR1) AND R1437Q` with whatever total and notice the recording shows. `--gene KRAS --variant G12C --drug sotorasib --disease lung` asserts `KRAS AND G12C sotorasib lung`. `--gene AKT1 --variant E17K` keeps `AKT1 AND E17K` and total 192. A case with `--gene HGNC:6407 G12C` asserts `KRAS AND G12C`. Search every other landed case and ticket proof for a term that pins a previous symbol, and update each one. Page ceilings follow the m4341 rule. Then make landing-gate runs through pm job.
- Defers: nothing.
