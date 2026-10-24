# 0089 — Show ClinVar and ClinGen evidence repository assertions for a variant

Status: COMPLETE.
Landed: 5c55f2efe84c6bc0112962658d730891af3c4f9e

Milestone: 2

Depends on: 0087

## Outcome

The `clinvar` section reads ClinVar by checked Variation ID and returns VCV, RCV and SCV assertions apart, with framework, condition, review status and dates. The `erepo` section reads the ClinGen evidence repository by CAid. `variant assertions ID --source clinvar|erepo` pages the assertions and selects one by ID and version.

- Why: classification evidence is what clinicians check first, and 0.9's 108 KB ClinVar reply buried it.
- Starts from: variant.md (ClinVar and ERepo rows, section budgets); the rulings; NCBI EFetch with the optional key; 0087's identity.
- Risks the proof must cover: a ClinVar text-search hit never becomes identity; germline, somatic clinical-impact and oncogenicity assertions stay in their own domains; no assertion for a CAid is distinct from an unknown allele; no vote or merged label.
- ClinVar classifications outside the model's propositions stay native records with `statement_unmapped: true` and full provenance, never forced into pathogenicity, as the common model section 3.6 says; a case asserts one drug response row this way.
- Proof: the shared fixture's ClinVar germline and ClinGen expert panel Statement rows run through BioMCP's `Statement`; TP53 CA000434 returns PCER000001171 version 1.1.0 with its expert panel; ClinVar sections for the design's questions keep assertions apart; "No Records Found" asserts empty; paging and selection; the failure cases; MCP parity; ceilings from the design's section table plus envelope. Then make landing-gate runs through pm job after current main.
