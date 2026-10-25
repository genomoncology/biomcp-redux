# 0153 — Investigate the pharmacogenomics slice and propose its design

Status: OPEN.

Milestone: 4

## Outcome

`sdlc/planning/slices/pharmacogenomics.md` holds the pharmacogenomics slice investigation and proposed design, through steps 1 to 4 of `planning/slices.md`, ready for PM review. No product code changes.

- Why: milestone 4 delivers the remaining entities, and drug-gene dosing guidance bears directly on cancer therapy choices, such as fluoropyrimidines with DPYD and thiopurines with TPMT and NUDT15.
- Starts from: `planning/slices.md` with its type-before-capture rule; the drug, variant and disease designs; the shared variant model's Statement and Subject rules; the pharmacogenomics sources in ROADMAP.md; 0.9's pharmacogenomics commands; decisions on text heuristics, keyed sources and open-by-default terms.
- Examples: 20 to 40 real clinical pharmacogenomic questions from guidelines, papers and public issues, covering a guideline for a gene and drug pair, a phenotype from a diplotype, a variant's clinical annotation level, a cancer drug, an empty result and a failing source, each with its source and accepted answer. Clinical judgments go to a clinical reviewer.
- Sources: ask CPIC and PharmGKB directly for each example and record counts, speed, requests, identifiers, the fields each public tier carries, and terms, with use labels and groups.
- Identity: record how each source names genes, alleles, diplotypes, drugs and guidelines, how they join to the gene, variant and drug slices without name matching, and where star alleles fit or do not fit the shared variant model.
- 0.9: measure 0.9's commands only behind verified network containment, otherwise mark them unmeasured; account for every command, option and section.
- Design: the delight questions, types and schema, command tree with headers, drill-downs and next steps, disclosure layers and measured token budget, sources kept and dropped, a case list per command and option, and an "Ian's questions" section.
- Proof: the PM reviews the design on its branch. Live captures run through pm job with raw evidence on the evidence store. No licensed content enters the repo.
- PM review 2026-10-25 of 924a5b03, returned with fixes that use the evidence already captured. PM rulings: ship the gene-keyed `recommendations` section from `recommendation_view` proved by C09 and C15, with the drug shown only as its native name, and capture `drugid` and `order` in the adapter ticket with no local drug filter; ClinPGx annotations page from the `view=min` collection with its total labeled as observed, a single-annotation base get behind a disambiguate step after one capture, one collection per kind, defaulting to clinical; CPIC and ClinPGx are both labeled `noncommercial` under the shared ClinPGx data use policy, captured in the type ticket; named selectors stay, the diplotype lookup is `get pgx --gene TPMT diplotype --diplotype '*1/*3A' --source cpic`, sections use `--section-limit` and `--section-offset`, and there is no positional search; star alleles and diplotypes stay native records with no Statement and no `statement_unmapped` flag, and no ClinPGx level maps to a shared proposition; the source is named `clinpgx`, old `pharmgkb` spellings get a retirement message, and CPIC stays its own source. Fixes:
  1. Give each question a status (answered, partial or unanswered) with its source and section, including the irinotecan gap.
  2. Replace `pgx diplotype` with the diplotype get section above.
  3. Mark `--drug-id`, the pair card, `search --drug TEXT`, `--population`, `--lookup-key` and `--pgx-testing` unproven or deferred until a positive capture; drop `--drug TEXT` unless a question needs it.
  4. State that a ClinPGx `PA` accession is not the shared `PaId`.
  5. Add JSON shapes for the header, card and each page, per-command next steps under architecture 6.10, and remove frequency from next steps.
  6. Project page bytes offline from retained captures, mark the rest unmeasured, and name the contained 0.9 run as a later ticket.
  7. Move the PM choices out of "Ian's questions", which says None.
- PM recheck 2026-10-25 of ea085c5d: all fixes closed; accepted with these amendments, applied on the branch before it lands:
  1. `--source cpic` is required on the search, card, recommendations, guidelines and diplotype commands; `annotations` requires `--source clinpgx`; any other source and section pairing fails before I/O, including `get pgx --gene X --source clinpgx` with no section.
  2. Next steps: pair search gives page, then the gene card, then the `recommendations` section, with no disambiguate step while pair get is deferred. The gene card gives `recommendations`, `guidelines` and `annotations --source clinpgx` section steps. Help and diplotype lookup are not steps.
  3. `biomcp.pgx_annotation/1` is deferred with its command.
  4. `--pgx-testing` is deferred until a native positive capture.
  5. CYP2D6 has 1,316 recommendation rows and no drug filter yet, so tamoxifen questions Q18 to Q23 need paging until the adapter ticket proves the native `drugid` filter; that capture is required in the adapter ticket.
- Defers: tickets, which the PM files from the reviewed design.
