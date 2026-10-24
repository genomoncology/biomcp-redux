# 0105 — Align BioMCP's biodata names and grammars with the shared plan

Status: OPEN.

Milestone: 2

Depends on: 0087

## Outcome

BioMCP's biodata follows the shared plan's 2026-10-24 crate comparison settlements (`planning/biodata-variants-and-annotations.md` section 6), so a later merge is at most a move.

- Why: a comparison of both biodata crates and a review of the other consumer's design found same-name conflicts and loose grammars; the PM accepted each point listed here.
- Changes, each with its schema version raised where a wire field changes:
  - `ProteinChange` becomes the private query type `ProteinQuery`; the shared name belongs to the other crate's HGVS protein type.
  - `Strand` Rust cases become `Plus` and `Minus`, JSON stays `+` and `-`. `Build` cases become `GRCh37` and `GRCh38`, `#[non_exhaustive]`.
  - `Accession` (exact 6 or 9 RefSeq digits, required version, leading zeros refused) is the only versioned accession grammar; an unversioned `AccessionBase` serves sources that give no version; `RefSeqTranscript` and `EnsemblTranscript` fold into these. MANE and allele fields require `Accession`.
  - `ManeSelect` becomes `Mane { status: select | plus_clinical, refseq, ensembl }`, and a gene holds a list.
  - `EntrezGeneId` becomes `NcbiGeneId`. `GeneRef { hgnc_id?, ncbi_gene_id?, ensembl_gene_id?, label }` is added and used by `Variant.genes` and statement gene context in place of bare symbols.
  - `GeneSymbol` takes the stricter grammar (leading letter, then letters, digits or `-._/@`) only if every approved symbol in the HGNC bundle passes; a case proves it either way.
  - `GenomicInterval.build` is `Option<Build>`, and containment refuses when either build is absent.
  - `HgvsText` accepts an inner accession or gene symbol in parentheses, such as `NM_004333.4(BRAF):c.1799T>A`, and still refuses text with no reference.
  - Gene status wire values become snake_case (`entry_withdrawn`, `merged_split`).
  - Refusal codes follow `area.thing.problem`, listed in the plan for each shared refusal.
- Keeps: every case's facts, with schema snapshots and compatibility checks updated for the raised versions.
- Proof: round-trip tests for each changed grammar, including `.04` refused and 7-digit RefSeq refused; the HGNC symbol check; case outputs re-checked under their ceilings; MCP parity unchanged. Then make landing-gate runs through pm job after current main.
