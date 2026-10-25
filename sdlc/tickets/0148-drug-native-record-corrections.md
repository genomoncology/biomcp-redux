# 0148 — Correct the shared drug records to keep native source facts

Status: COMPLETE.
Landed: 8364fe50b98025dbce54c077ccc220ab19d9baea

Milestone: 3

Depends on: 0124

## Outcome

The shared drug records keep what the sources send before ChEMBL and DDInter map onto them. A target keeps its native ChEMBL target reference with an optional gene, and at least one of the two is present. An indication keeps its native EFO and MeSH IDs, each with its own label. A trial stage keeps the source's field and value unchanged, and native references stay typed. An interaction keeps its primary attribution and adds further attributions without duplicates.

- Why: a contract review of the captures found the current shapes lose facts. ChEMBL captures 015 and 017 give a target ID with no gene label. Captures 014 and 020 give separate EFO and MeSH labels, a phase and references. The DDInter pair 1951 and 285 occurs in both the B3379 and L16759 files.
- Starts from: the 0124 types, the ChEMBL and DDInter captures, and the shared model's Attribution.
- Proof: synthetic public contract tests for each shape, the drug schemas that change take a version change, and the gene and variant schemas stay unchanged. No provider call. Then make landing-gate runs through pm job. 0127 and 0132 do their final mapping after this lands.
