# 0087 — Get variants by rsID, CAid, ClinVar ID or HGVS with a compact card

Status: COMPLETE.
Landed: a6e509948b15c0193f7d1bf56f0308e2304dd539

Milestone: 2

Depends on: 0078

## Outcome

`get variant INPUT` resolves an rsID, CAid, ClinVar Variation ID or reference-qualified HGVS by its published form, returns every candidate when the source gives several, and prints a compact card of identity and MyVariant's source-attributed facts, through the CLI and MCP.

- Why: identity is the first question and 0.9's weakest point: nine of its 21 search examples were usage errors, and an rsID can name three alleles.
- Starts from: variant.md sections 2 to 5 and 9; `decisions/2026-10-24-variant-slice-rulings.md`; the article reader's private protein-change parse, which stays private; architecture section 3 types, the coordinate build rule and `wire_enum!`.
- Common model: the variant types follow `planning/biodata-variants-and-annotations.md` sections 3 and 4. Before landing, the branch's allele strings become `Sequence`, its assembly text `Option<Build>`, its reference text a versioned `Accession`, its record keys and aliases `VariantId`, and its generic `VariantFact` becomes `Statement`, `PopulationFrequency` and `PredictorScore` as their sections need them. `biodata.gene` moves `build` to `Option<Build>` and `contig` to `chromosome` and rises to major version 2. This ticket adds the shared public fixture file under `tests/fixtures/variant/` with the plan's first rows, each checked against its public source, and a test that runs every row through BioMCP's types.
- Builds: biodata `VariantInput`, `VariantIdentity`, `VariantRepresentation`, `VariantResolution` and `VariantHeader`, and `biomcp.variant_card/1`; the MyVariant and ClinGen Allele Registry adapters, GET only.
- Risks the proof must cover: identity checks compare source-qualified alleles, never a gene symbol or protein label; CAR's zero-based coordinates never mix with HGVS positions; a returned coordinate carries the build its source states; a chromosome form without `--build` is a usage error; an HTTP 200 with a validation warning is rejected, not resolved; gene and protein shorthand is refused with the search step.
- Proof: cases from the design's question bank on captured replies, including rs113488022 returning three candidates with selection steps, an HGVS input resolved through CAR to its CAid, ClinVar ID and both builds, the TP53 paper spelling shown beside CAR's rejection, an identity mismatch dropped with a problem, the explicit build refusal, the five failure cases per adapter, `--resolver mutalyzer` routing, and MCP parity. Captures whose terms are unverified run from outside the repo through `--cases DIR`. Ceilings are measured plus 15 percent, capped by 0.9 for the same question. Then make landing-gate runs through pm job after current main.
- Defers: sections and search (0088 to 0096).
