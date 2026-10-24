# 0021 — Add the gene identity types and schemas to biodata

Status: OPEN.

Milestone: 1

## Outcome

`biodata` holds the gene slice's identifiers, closed sets, `Gene`, `GeneHeader`, `GeneInput` and `GeneResolution` exactly as `planning/slices/gene.md` section 5 names them, with checked-in versioned schemas, so the gene adapters and commands build on settled types.

- Starts from: `planning/slices/gene.md` sections 5 and 9, `planning/architecture.md` sections 3.1 to 3.4, 3.9 and 3.10, `planning/rust-standards.md`, and the core conventions already on main (ids, dates, coordinates, text, schema generation).
- Changes: `crates/biodata/src/id/` (gene identifiers), `crates/biodata/src/gene.rs` or a `gene/` module, `crates/biodata/src/lib.rs`, `crates/biodata/schemas/` (new gene schemas), `crates/biodata/tests/` (identifier tables, round trips, schema drift).
- Keeps: Entities know nothing about sections, cards or envelopes (rust-standards rule 15, architecture 3.10). `Option` means the source has no value. No HTTP, adapters, commands or bundle code. No new dependency. Fields are declared in priority order: identifiers, closed values, lists, prose. Header lists are capped by type.
- Scope: `HgncId`, `EntrezGeneId`, `EnsemblGeneId` with its version kept apart, `GeneSymbol` compared and hashed without case, `UniProtAcc`, `OmimId`, `RefSeqTranscript`, `EnsemblTranscript`, `Cytoband`; `GeneStatus`, `LocusGroup` (non-exhaustive), `MatchKind`; `GeneInput` parsing into an HGNC ID, Ensembl ID, Entrez ID or symbol-like text, with alias text kept as text; `Gene` with `mane_select`, `spans` per build and a whole dated `summary`; `GeneHeader`; `GeneResolution` with resolved, ambiguous, withdrawn and not found.
- Proof: biodata integration tables through the public API for every identifier's valid and invalid forms, Display and FromStr round trips, case-insensitive symbol equality and hashing, GeneInput classification of HGNC:1097, 1097 as Entrez, ENSG00000157764.17, BRAF, HER-2/neu and MLN 19; JSON round trips for a resolved, ambiguous and withdrawn value drawn from the gene slice's examples; schema drift and compatibility passing; `make landing-gate` through pm job.
- Defers: `GeneCard`, `GeneSections`, section payload types and the envelope arrive with the first gene command, as the order change on that ticket says. HGNC and MyGene.info adapters wait for bundle sync and the full shared transport.
