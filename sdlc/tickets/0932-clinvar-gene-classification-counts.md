# 0932 — clinvar-gene-classification-counts

Status: OPEN.

Milestone: 7

## Outcome

One call gives a gene's ClinVar variant counts by germline classification and by review status, each variant counted once, with the ClinVar release it came from.

## Why

A truth-set table for a hereditary-panel validation needs "BRCA1: N pathogenic or likely pathogenic, M at expert panel". The BioMCP agent fell back to overlapping E-utilities word counts (log.md:205-212, 223-230); code mode parsed ClinVar's 458 MB file and double-counted every variant across genome builds (31,753 records, 8,403 P/LP; raw log.md:237-244). ClinVar's `variant_summary.txt.gz` gives 4,327 BRCA1 P/LP on GRCh38, 2,286 at expert panel. Evidence: bench `personas/dx-lab-director/runs/2026-11-14/` (j40302, BioMCP f4f726674) and its code-mode pair `runs/2026-11-14-raw/` (j40303); the analyst's source checks are listed at the end of `analysis.md`.

## Proof

- Proof: a case over a recorded ClinVar reply (or a small slice of the summary file) for one gene asserts counts by classification and review status, each variant once, with the release date.

## Starts from

- Starts from: `crates/biomcp-adapters/src/clinvar/`; ClinVar's E-utilities property filters (`"clinsig pathogenic"[prop]` gave 4,752 for BRCA1, including multi-gene records) and `variant_summary.txt.gz`. Decide live which is honest and cheap; say in the reply what is counted (single-gene records or all).

## Keeps

- Keeps: the 1.0.0 schema rules (an added section or optional fields); existing variant cards.

## Defers

- Defers: somatic classifications and per-condition counts.

## Changes

- Changes: the ClinVar adapter, a gene section or command, help, cases.

## Progress

- 2026-11-14 design, measured live: E-utilities `[prop]` classification filters follow each record's aggregate germline classification. Against all 16,172 BRCA1 esummary records (Build261108-1750.3), `"clinsig pathogenic"[prop]` 4,398 = P 4,170 + P/LP 228 and `"clinsig likely pathogenic"[prop]` 582 = LP 354 + P/LP 228, so the overlapping filter counts split exactly into disjoint buckets; the benign side behaves the same way. `[rvst]` also indexes somatic clinical impact and oncogenicity review statuses: BRCA1 single-submitter P/LP gives 1,617 against 1,612 true, and 207 of TP53's 1,241 P/LP records match two statuses. So review buckets come from Variation ID lists, and only records listed twice are summarized (6 for BRCA1). `"single gene"[prop]` drops 1,818 BRCA1 small variants that also list LOC126862571, an overlapping regulatory element, so every record listing the gene counts. `1:999999999[cpos]` keeps GRCh38-placed records: 4,342 of BRCA1's 4,752 P/LP (summary file: 4,327 GRCh38 rows); the rest are mostly GRCh37-only exon-level deletions and duplications, plus 22 protein-only records.
- Built `get gene GENE clinvar` (PM approved the design): `build` and `build_updated` from einfo, a `scope` sentence, the search `term`, `records`, disjoint `germline` buckets, `pathogenic_or_likely` beside `pathogenic_or_likely_grch38`, and `pathogenic_or_likely_review` (practice guideline, expert panel, multiple submitters, single submitter, no assertion criteria, other). The section's requests run one at a time. Sixteen searches plus at most a few esummaries; TP53 fresh took 4 s under the default budget.
- Throttling: parallel requests at the default NCBI pacing failed one request per gene as `bad-reply` (j40584, j40617, j40637). Observations showed it was an HTTP 429. The M5 sends an NCBI key, and NCBI's 429 body names the key (`{"error":"API rate limit exceeded","api-key":...}`), so the reflected-credential guard refused it as a bad reply without retry. The transport now retries a 429 whose body names the key, after dropping that body; other reflections are still refused (`crates/biomcp-fetch/tests/transport/credentials.rs` `throttle_replies_naming_the_key_retry_without_disclosure`).
- Recorded live in j40643: `tests/cases/gene/sections/clinvar.yaml` over `tests/replies/gene/clinvar/` asserts BRCA1's counts above, 2,287 at expert panel (summary file: 2,286), the build and the scope.
- Gates: j40646, j40654, j40665 and j40681 each failed on a different rule: schemas generated before the `scope` field was added; the adapter source rules (300-line adapter file, 160-character line, a raw `serde(default)`, a helper named `bad`); the ceiling of 170 parent modules with logic, after the first split made `clinvar/gene.rs` a parent; and the rule that every `updated` property is a PartialDate. ClinVar's build date is now `build_updated` (ClinVar's own spelling, with a time). The count code lives in sibling files `clinvar/gene.rs`, `gene_reply.rs` and `gene_review.rs`. Schema regeneration refused to rename a field already in the checked-in `gene-cards.json`, so that schema was reset to main and regenerated (j40720).
- Gate j40726 on 917e1339 passed `make --keep-going landing-gate docs-check schema-release-check`.
