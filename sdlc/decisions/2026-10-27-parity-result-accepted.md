# The bounded 0.9.3 parity result is accepted

Date: 2026-10-27

Decided by the PM.

The milestone 5 correctness comparison against released 0.9.3 is accepted with its named holds. Over 114 unique comparable contexts, 91 were right in both versions, 23 were wrong in 0.9.3 and right in 1.0, and none was right in 0.9.3 and wrong in 1.0. Coverage was trial 46, author 22, pathway 16, article 10, protein 5, variant 5, adverse event 4, PGx 3 (error output size bounds only), gene 2 and cell line 1. The accounting artifact is on the evidence store under 0297 with SHA-256 f0260b7702108225f27a97f45db335568dd7cf74a528d3077428c6ab92d936e6.

- No further gap filling is authorized. Diagnostic, disease, drug, GWAS and phenotype have no comparable facts, study questions are unsupported in 0.9.3, and every held native input, unsupported mapping and the broader KRAS gap stay named in the artifact.
- The outside judged comparison in 0098 covers the families with no comparable facts, since it asks both versions the same outside questions live instead of replaying harvested cases.
- 0297 stays open only for the queued matched-release size and timing measurement. Release notes may say no regression was found in the comparable set; they may not claim full parity.
