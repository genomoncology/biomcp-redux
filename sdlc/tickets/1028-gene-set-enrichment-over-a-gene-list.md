# 1028 — gene-set-enrichment-over-a-gene-list

Status: OPEN.

Milestone: 8

## Outcome

A BioMCP command takes a gene list and returns over-represented pathways and terms from a public enrichment service (g:Profiler, or Reactome's analysis service), with the service's own statistics, correction method and background stated, and the service named as the source.

## Why

Milestone 8 lists gene-set enrichment (`sdlc/planning/milestones.md`, section 8; `sdlc/planning/sources-to-add.md:53`). comp-bio Q9 needed it and BioMCP had no command that takes a gene list (bench `personas/comp-bio/runs/2026-11-16/analysis.md` 9.1).

## Proof

- Proof: a replayed case for comp-bio's five genes asserting the top Reactome terms and the stated correction; a live check recorded in Progress.

## Starts from

- Starts from: an existing adapter of the same shape, and the source's own documentation and terms.

## Keeps

- Keeps: every existing command form.

## Defers

- Defers: nothing.

## Changes

- Changes: a new adapter or section, its help, schemas, cases, recordings and a release-notes line.

## Progress

- Compared live 2026-11-16 with comp-bio's five genes (POLQ WRN CHEK1 TTK PARP1). Both services agree on the top: Homology Directed Repair, then DNA Double-Strand Break Repair, DNA Repair and HDR through MMEJ. Both find 4 of 5 genes; TTK is in no Reactome pathway (Reactome's own mapping for UniProt P33981 answers 404), so this is the data, not a join gap.
  - Reactome Analysis Service (`POST /AnalysisService/identifiers/projection`): rows carry `pValue`, `fdr`, `found` and `total`, but the reply does not say which test, correction or background it used; the user guide says a hypergeometric test with Benjamini-Hochberg FDR over all Reactome molecules, and the background size is only derivable from `ratio`. Unmatched inputs need a second call (`/token/{token}/notFound`). It accepts only `text/plain` bodies (JSON and form answer 415), which BioMCP's transport does not send.
  - g:Profiler g:GOSt (`POST /gprofiler/api/gost/profile/`): every reply states the correction method (`significance_threshold_method`), the threshold, the background (`domain_scope` and each term's `effective_domain_size`), the data release (`version`) and the unmapped and ambiguous inputs. Its documentation names the test (Fisher's one-tailed, cumulative hypergeometric) and offers g:SCS (default), Bonferroni and Benjamini-Hochberg. It takes a JSON body, which the existing transport already sends. Its terms (biit.cs.ut.ee/gprofiler/page/terms-of-service) allow free use including commercial, welcome API tools, ask for citation and say each data source keeps its own terms.
  - Chose g:Profiler: it is the one that states its statistics and background in the reply, it fits the existing JSON POST path with no transport change, and it serves Reactome terms (`REAC`) plus GO and WikiPathways from one call. BioMCP asks it only for openly licensed term sources (Reactome, GO BP/MF/CC, WikiPathways) and never for KEGG, TRANSFAC, CORUM, HPA, HP or miRTarBase, whose terms differ.
- Command shape: `biomcp gene enrichment GENE... [--terms reactome,go-bp,go-mf,go-cc,wikipathways] [--correction g-scs|fdr|bonferroni] [--limit N] [--offset N]`. It follows the facet form `<entity> <facet> ID...` (`docs/reference/commands.md`; `article recommendations` already takes a list), named for the input entity with a noun facet, as the CLI design review (`sdlc/records/2026-11-07-cli-design-review.md`) keeps verb-first rows and facet nouns. The MCP `gene` and `pathway` tools both carry it as action `enrichment`. Defaults: all five term sources, g:SCS (g:Profiler's own default), threshold 0.05, the annotated-genes background, ten terms per page.
- Reply (`biomcp.gene-enrichment/1`): release, test, correction, threshold, background, unmapped and ambiguous inputs, then terms in g:Profiler's order with adjusted p value, overlap, query size, term size, background size and the input genes in each term. A 300-gene list measured 2.5 MB from g:Profiler, inside the 8 MB transport bound; the input cap is 500 genes.
- Live check 2026-11-17, pm job j42159 (recorded into `tests/replies/gene/enrichment/`): `gene enrichment POLQ WRN CHEK1 TTK PARP1 --terms reactome --correction fdr` answered complete from g:Profiler release e114_eg62_p19_9dc92ac8 with 61 significant Reactome terms. First: Homology Directed Repair (REAC:R-HSA-5693538, adjusted p 1.8e-06, overlap 4 of 4, term size 140, background 11,056; POLQ, WRN, CHEK1, PARP1), then DNA Double-Strand Break Repair, DNA Repair, HDR through MMEJ. With the defaults (all five term sources, g:SCS) and NOTAGENE1 added, 58 terms, Reactome first, GO terms next, and `unmapped: ["NOTAGENE1"]`.
- Changed: new source `gprofiler` (adapter `crates/biomcp-adapters/src/gprofiler/`, quota 1/s, Attribution terms, health probe `/api/util/data_versions/`); query and reply types `GeneEnrichmentQuery` and `GeneEnrichment` (`biomcp.gene-enrichment-query/1`, `biomcp.gene-enrichment/1`); runner `crates/biomcp-query/src/gene/enrichment.rs` pages g:Profiler's whole significant list locally and offers the next page; MCP `gene` and `pathway` tools gain action `enrichment`. The retired 0.9 `enrich` word and `get pathway ID enrichment` now answer with a new message naming `gene enrichment` (the old message stays in the enum; the 1.0.0 error-message review in `release/schema-reviews.json` is extended); the parity row for `enrich` moves to ported. Commas also separate genes, as 0.9's `enrich` took them. Docs: caution section, release-notes line, feature line, nightly live check row.
- Cases: `tests/cases/gene/enrichment.yaml` (top Reactome terms, stated correction, background size, unmapped input, paging), `tests/cases/local/refusals-gene-enrichment.yaml` (help and refusals). `mcp/parity.yaml`'s source-schema step goes from 4,096 to 4,730 bytes: the Source enum grew to 4,113 bytes, plus 15 percent under the measured-caps decision. `typed_followups.rs` expects `enrichment` among the gene tool's actions.
- Note for the PM: `sdlc/decisions/2026-10-25-entity-boundary-rulings.md` says g:Profiler and 0.9's `enrich` are dropped and an agent can call g:Profiler itself. Milestone 8 (2026-11-11) and this ticket bring it back; I followed the newer plan and left the ruling file unchanged.
- Jobs: j42152, j42154, j42156 (builds; failures fixed: missing source coverage, a retired-word rule that swallowed `gene enrichment`, a changed fixed message the release contract refused), j42159 (live recording, check-cheap), j42161 and j42164 (check-cheap and touched cases pass), j42166 (gate on 725402b6f: cases failed on the typed-followups action list and schema-release-check on the new error message; both fixed).
- Gate j42170 passed on 5a89dd242 (branch merged with origin/main 1d1817556): `make --keep-going landing-gate docs-check schema-release-check`.
