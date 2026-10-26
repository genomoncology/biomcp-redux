# Protein, pathway and study slice rulings

Date: 2026-10-26. Decided by the PM on the 0190, 0191 and 0194 investigation proposals. No question goes to Ian.

## Protein

- Keep UniProt identity, features, isoforms, sequence and structure references, InterPro location matches and Complex Portal exact-member access.
- Defer STRING. Its mapping returns one STRING ID for the two distinct CDKN2A protein products, so it cannot prove protein-specific identity. A later research question that needs STRING reopens it.
- Keep `protein complex-members ACCESSION --complex CPX-ID` with bounded pages and an anchor that must be an actual participant.
- Accept the proposed ceilings as starting targets. Each product ticket sets its final ceiling from measured output plus 15%.
- Defer `--disease` and `--existence`. Drop `--all-species`.

## Pathway

- Reactome is the default search source.
- Add `sync wikipathways` for the two metadata catalogs. WikiPathways genes through exact per-pathway GPML is approved as its own later ticket after the Reactome tickets, with its own parser and bounds.
- Retire `pathway drugs`, `pathway articles` and `pathway trials`. Each one fans out or matches titles; the retirement message names the explicit gene, article or trial command.
- The product ticket measures and fixes the Reactome search window and proves the real paging and filter parameter spellings before relying on them.

## Study

- Keep native study counts and gene aggregates as source-specific metadata. The shared `CohortObservation` subject stays `VariantRef`; no shared-model change.
- Keep the source-wide `restricted` use for cBioPortal. Per-study LICENSE links appear as notices; per-study use labels wait until a need appears.
- Accept `search study`, `get study` with `profiles`, `sample-lists`, `counts` and `files`, and `sync cbioportal-studies` for the metadata catalog.
- The gene-count section must first prove a gene-filtered aggregate request. The unfiltered BRCA reply is 4.2 MB for one study; if no filtered request exists, the ticket brings the cost back to the PM before building.
- 0.9 calculation leaves and chart options retire with fixed messages, as the entity boundary ruling says.

## All three

0.9 timings and output sizes stay marked unmeasured; the milestone 5 comparison measures them. The proposals land as slice designs, and the PM files product tickets from them.

## Phenotype

Added 2026-10-26 on the 0192 proposal.

- Keep Monarch term search restricted to the HP namespace, exact HPO and Monarch term cards, `synonyms`, `xrefs`, Monarch `hierarchy`, and `diseases` and `genes` association sections that reuse the disease slice's association model. Add source taxon to that shared model only if it is missing.
- Defer Monarch similarity. The disease slice already retired symptom-to-diagnosis ranking, and one research question does not outweigh that ruling. The 0.9 `search phenotype` HP-ID-set form gets a fixed retirement message that names term search and the association sections.
- Accept the defaults: five term headers, five-row text and link sections, one complete association row, and whole definitions and comments. The hierarchy section returns parent and child links, not whole native nodes, and its ceiling comes from that measured output.
- Defer translations and HPO or Monarch bundles.

## GWAS

Added 2026-10-26 on the 0193 proposal.

- Keep the GWAS Catalog v2 API only, through the existing variant adapter. Defer every bundle; no `sync gwas`.
- Accept the command tree: `search gwas` by `--variant`, `--trait`, `--trait-id`, `--gene`, `--study` and `--pubmed`, with `--include-child-traits` and `--extended-geneset` off by default; `get gwas` for an association or a study, with `loci`, `associations` and `ancestries` sections. The 0.9 positional query and the `-g` and `-l` aliases keep working.
- Default page of three associations, ordered by native p value ascending, as measured.
- `--p-value` is exact, not page-scoped. It requires the default p-value-ascending order. Rows arrive in that order, so the command returns rows until the first row above the threshold and stops; it continues to the next source page only while every row so far qualifies. It reports a qualifying total only when it reached the end. Any other sort with `--p-value` is a usage error.
- Combined selectors use the source's own conjunction, as the FGFR2 plus trait capture proved. The 0.9 local intersection of two 50-row prefixes retires.
- Keep the restricted source category and the existing adapter deadline.

## Author

Added 2026-10-26 on the 0195 proposal.

- Accept the metadata-only boundary. Types, output ceilings and question answers wait for captures on a source route Ian rules eligible; no Semantic Scholar or ORCID API capture runs before that ruling.
- Rich paper metadata is the named `details` section of `author papers`. The 0.9 `--full` flag keeps working as its alias, as with the 0.9 key names.
- The canonical source spelling is `semantic-scholar`; `semanticscholar:` IDs and `--source semanticscholar` keep working as aliases.
- Defer topic-wide author ranking and cross-provider merging.

## Cell line

Added 2026-10-26 on the 0196 proposal.

- Cellosaurus and ChEMBL through their APIs only; no bundles. PharmacoDB is deferred until it publishes an affirmative data reuse grant, with its 0.9 drug-response routes. The HPA gene-to-cell expression capability stays deferred until an exact cell identity join exists.
- Accept native-order search with `start` and `rows`, five headers, a next offset only when the window fills, and no total claim. Name normalization, the second hyphenless search and human-first reranking retire.
- Accept the sections `names`, `identity`, `xrefs`, `variants` and `chembl`, and the separate `cell-line assay-count CHEMBL_CELL_ID` command. Defaults: five names, five xrefs, three variation rows.
- Keep `all`, unlike the proposal. Protein and pathway keep `all`, and the entities stay consistent: `all` returns each named section's bounded first page, including the one `chembl` request, and never the assay count.
- Accept the payload targets as starting points; each product ticket sets final ceilings from measured output plus 15%.

## Ticket filing rulings

Added 2026-10-26 while filing the slice tickets.

- Phenotype: `get phenotype HP:ID` defaults to `hpo` for the card, `synonyms` and `xrefs`. `--source monarch` selects Monarch's. `hierarchy`, `diseases` and `genes` use Monarch without a flag, and `--source hpo` with them fails before I/O. A search header's get step uses the default source. HPO's `attribution` is its use label; no new source group.
- GWAS: `search gwas` has no sort option, so threshold searches always use native p-value-ascending order. A p value equal to the threshold qualifies. Further threshold pages are continuation steps. `--include-child-traits` without `--trait-id` and `--extended-geneset` without `--gene` are usage errors. A selector whose capture fails stays deferred and returns to the PM.
- Study: `get study` sections page with `--section-limit` and `--section-offset`, as other entities do. The patient total stays in `counts`. The DataHub `study_list.json` catalog is deferred. DataHub file reads sit under the `cbioportal` declaration. Empty search text with no `--cancer-type` fails before I/O. `all` never sends the gene aggregate request.
- Pathway: `pathway drugs` retirement uses the `gene drugs` guidance, because reverse gene-to-drug search is unsupported.
- Protein: `function` is a get section only. Source spellings are `interpro` and `complex-portal`. `protein features --type` takes UniProt's native feature type label exactly, such as `Domain` or `DNA binding`, with no alias mapping. `ProteinSite` is not adopted until a command uses it. `ProteinHeader` shares the gene slice's `Protein` record fields without a second name for one concept.
