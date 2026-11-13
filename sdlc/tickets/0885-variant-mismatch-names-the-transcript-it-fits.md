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

## Progress

- 2026-11-13 live check (jobs j39132, j39136): VariantValidator refuses a bare gene symbol (`SCN1A:c.4729T>C`, InvalidReferenceError) but validates it when given the gene's transcripts as `select_transcripts` (`NM_…|NM_…`): it returns a record for each transcript where the base fits (NM_006920.6, NM_001353949.2, NM_001353950.2 …, all NC_000002.12:g.165994236A>G) and a ReferenceMismatchError for the rest. Its `tools/gene2transcripts_v2/NM_001165963.4/all/refseq/GRCh38` gives the gene symbol and the transcript list in one call. Mutalyzer refuses a gene symbol (EGENEASREFERENCEID) and lists the gene's transcripts as options without checking them. The ClinGen Allele Registry refuses a gene symbol (500 "Unknown reference: SCN1A"; 400 parse error with a space). So VariantValidator is the only source that answers "which transcripts fit"; BioMCP uses its answer.
- 2026-11-13 change: when any normalize service reports a reference mismatch (CAR IncorrectReferenceAllele, Mutalyzer ESEQUENCEMISMATCH, VariantValidator ReferenceMismatchError) on an NM_ c. description, BioMCP asks VariantValidator for the gene's transcripts and which of them accept the change, lists them in the rejected result's new optional `fits`, resolves each distinct genomic allele (up to 3) with the same service in an added result whose new optional `hgvs` names the transcript, and offers a `disambiguate` next step naming that transcript. The rejection and its message are unchanged; no fit means no added result and no step. The CLI does not accept a gene-symbol HGVS (biodata's HgvsText needs an accession), so the gene form is not tried. Skill, use case 16 and the `--hgvs` help say a mismatch usually means another transcript.
