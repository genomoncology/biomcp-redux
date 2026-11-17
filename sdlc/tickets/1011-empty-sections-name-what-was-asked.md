# 1011 — empty-sections-name-what-was-asked

Status: COMPLETE.
Landed: 85df87989e431b447d606c1485fa3f065092366d

Milestone: 7

## Outcome

An empty section states what its source was asked, for example `Reactome lists no pathway for UniProt:P33981`, so an agent reads it as the source's answer, not a join failure.

## Why

comp-bio called TTK's empty Reactome section a join gap (bench `personas/comp-bio/runs/2026-11-16/analysis.md` 3.2); Reactome has no pathway for P33981.

## Proof

- Proof: a replayed case for TTK pathways asserting the stated identifier; fails on the base commit.

## Starts from

- Starts from: the analysis named in Why, and the BioMCP code it cites.

## Keeps

- Keeps: every other reply field and command form.

## Defers

- Defers: nothing.

## Changes

- Changes: the named command's code, help, cases and a release-notes line.

## Progress

- Measured live 2026-11-16: Reactome's `/data/mapping/UniProt/P33981/pathways?species=9606` answers HTTP 404 "No pathways found for UniProt:P33981", so TTK's empty section is Reactome's own answer. Recorded in pm job j41993 into `tests/replies/gene/pathways-ttk/` (HGNC trimmed to the TTK row; MyGene and Reactome whole).
- Shared place: every section state is the one `Section` type (`crates/biomcp-types/src/response/section.rs`), whose `Empty` state already has an optional `record` (trial, GWAS, gnomAD and CIViC empties already fill it), and the Markdown note for every empty section is one function, `no_data` in `crates/biomcp/src/view/mod.rs`. The fix adds `SourceRecord::asked(namespace, id)` to that type and makes `no_data` print `SOURCE returned no data for NAMESPACE:ID.` whenever `record` is present, so any source that fills `record` gets the wording. No schema change: `record` was already in every reply schema.
- Filled `record` with the identifier each source was asked: gene pathways (Reactome, UniProt), protein (UniProt), constraint (gnomAD) and expression (HPA) by the version-free Ensembl ID each adapter sends, GenCC (HGNC ID), OncoKB by the `hugoSymbol` it is sent; protein card features, isoforms, structures and function (UniProt), domains (InterPro) and complexes (Complex Portal) by UniProt accession; article citations (iCite) and indexing (PubMed) by PMID; cell-line ChEMBL by Cellosaurus accession.
- Proof: `tests/cases/gene/sections/pathways-ttk-empty.yaml` expects `record` `UniProt` `P33981` and the Markdown line. `view-empty/empty-section.yaml` and `table-section.yaml` now expect the HPA Ensembl and UniProt identifiers. Cheap job j41994 on 3ba307b88: schemas unchanged, `make check-cheap` and the gene, protein, view-empty, article and cell-line cases pass; with base f34d6b2bd's crates the new case fails (`record/id` missing). Recording job j41986 failed to compile (a second `no_data` caller), fixed.
- Gate j42000 on dd53de002 (origin/main f34d6b2bd, unmoved): `make --keep-going landing-gate docs-check schema-release-check` passed.
- Left as is: empties built before the source is asked (article evidence, trial document placeholders) and empties inside a card that already names its single record (NCI trial sections, drug label, variant gnomAD/hotspots/assertions/CIViC, study files). Help text unchanged: the reply now carries the identifier itself. Variant COSMIC and score empties (`variant/cosmic.rs` L60, `variant/components.rs` L74, from before this ticket) may name the card's first candidate record rather than the MyVariant ID that was sent.
- Review fix: OncoKB's empty `record` now names the `hugoSymbol` it is sent, not the NCBI Gene ID; gnomAD and HPA empties name the version-free Ensembl ID their adapters send. Cheap job j42025 on e4be190c9: formatted, schemas unchanged, `make check-cheap`, gene and view-empty cases pass.
- Gate j42032 on 207ca0921 (branch merged with origin/main 3701a52f0): `make --keep-going landing-gate docs-check schema-release-check` passed.
