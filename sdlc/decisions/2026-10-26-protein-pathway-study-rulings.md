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
