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
