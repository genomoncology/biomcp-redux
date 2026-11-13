# 0885 — variant-mismatch-names-the-transcript-it-fits

Status: OPEN.

Milestone: 7

## Outcome

When a lab's HGVS names a gene and a c. change whose reference base does not match the transcript BioMCP tried, the reply says on which of the gene's RefSeq transcripts the change does fit and resolves it there, so an agent lands on the variant the patient carries, not a neighbour.

## Why

In the rare-disease persona run of 2026-11-13 (biomed-agent-bench `personas/rare-disease/runs/2026-11-13/log.md`, analysis beside it; code-mode run in `runs/2026-11-13-raw/`), the lab reported SCN1A c.4729T>C. That is valid on NM_006920.6 (p.Cys1577Arg, ClinVar VCV000068552, Pathogenic/Likely pathogenic; checked with ClinVar esearch and VariantValidator 2026-11-13). The agent tried the MANE transcript NM_001165963.4, got "Reference allele does not match … given=T, found=A" (log.md:308-311), "corrected" the base to c.4729A>G, and reported on two different variants (log.md:313-372). The code-mode agent found the right variant with one ClinVar search.

## Proof

- Proof: a recorded case where `variant normalize` given NM_001165963.4:c.4729T>C (and the gene form SCN1A c.4729T>C, if the CLI accepts a gene) names NM_006920.6 as the transcript where T matches and returns the NM_006920.6:c.4729T>C allele with its ClinGen allele ID and its MANE equivalent c.4762T>C; the mismatch reply's `next` step names that transcript instead of inviting a base change; the skill says a reference mismatch on a named gene usually means another transcript, not a wrong base.

## Starts from

- Starts from: `crates/biomcp-adapters/src/variantvalidator/`, `allele_registry/`, `mutalyzer/`, the `variant normalize` command; VariantValidator resolves gene-symbol HGVS across transcripts.

## Keeps

- Keeps: the honest mismatch message; refusing to guess when no transcript fits.

## Defers

- Defers: protein-only descriptions (p. changes) across isoforms.

## Changes

- Changes: `crates/biomcp-adapters/src/variantvalidator/`, `crates/biomcp-query/src/variant/`, `tests/cases/variant/`, skill and help text, `docs/`
