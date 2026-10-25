# 0133 — Offer typed next steps from drug responses to other entities

Status: COMPLETE.
Landed: 9b112cfd6379e88abb2e8b3aeb0d22191671ae08

Milestone: 3

Depends on: 0126

Depends on: 0127

Depends on: 0128

Depends on: 0129

Depends on: 0130

Depends on: 0131

Depends on: 0132

## Outcome

Drug responses name typed next commands into the entities that own them. A CIViC evidence row whose molecular profile states a variant, gene and tumor offers the keyed OncoKB variant step with that explicit context, and a response without that context names what is missing. Each next command uses its owning entity's selectors and rules.

- Why: drug.md keeps OncoKB therapy levels with the variant slice's keyed source, reached by next steps, and keeps cross-entity next commands in their owning entities.
- Starts from: drug.md section 2's OncoKB paragraph and case N01; the variant OncoKB command and its keyed-source rules; decision `2026-10-24-keyed-sources-run-only-with-the-user-s-key.md`; the drug adapters from 0126 to 0132.
- Rules: no therapy level is copied onto a drug response, no variant is inferred from a drug name, and a missing key sends no OncoKB request. A next step exists only where a source states the context it needs. `identity_level` comes from the endpoint or record type, never from a field's shape.
- Proof: outside-in cases first. Case N01 over D05, D17 and D19 contexts asserts the OncoKB step only with explicit variant, build and tumor context, the named missing context otherwise, and zero OncoKB requests without a key, from recorded replies. A keyed run reuses the existing stub-backed OncoKB pattern with the key never recorded. Each case keeps its size ceiling. No licensed content enters the repo, and replies whose terms bar storage replay from outside the repo. Then make landing-gate runs through pm job.
- PM ruling 2026-10-25 on N01: the scoped lookup satisfies it. The step looks up one source-listed genomic variant in the stated tumor context, and appears only when one native profile member gives its gene, genomic HGVS and build, exactly one distinct compatible expression exists, and the same claim names the tumor. The step shows which member and HGVS it came from. It never implies that the allele stands for every profile member or carries the treatment evidence. Otherwise the output keeps its missing-context form and offers no variant step.
