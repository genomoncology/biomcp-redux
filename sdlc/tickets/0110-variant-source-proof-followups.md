# 0110 — Capture positive CIViC, Cancer Hotspots and gnomAD release proof

Status: OPEN.

Milestone: 2

Depends on: 0090
Depends on: 0091
Depends on: 0089
Depends on: 0111
Depends on: 0112
Depends on: 0113

## Outcome

Three bounded captures from landed main close the variant source-proof gaps that gate roadmap boxes, never code landing:

- CIViC: a positive exact variant membership join, such as native Variant 12 (BRAF V600E, CA123643) and its molecular profile evidence, recorded and replayed after 0111. The landed capture proves only a native zero.
- Cancer Hotspots: an exact join between the variant's residue and alternate and a hotspot row, recorded and replayed after 0113 adds a transcript-qualified match. If 0113 closes without code, the capture proves a positive residue-level candidate row instead.
- gnomAD: release evidence captured with the same request as its counts, so `PopulationFrequency.dataset_version` can be typed, captured for 7-140753336-A-T after 0112. `gnomad_r4` is a moving selector, not a release, so landed counts stay native without a typed frequency.

- ClinVar: a real SCV whose classification is drug response, such as Variation 666267 with SCV000965609.1 (drug response to dabrafenib), after 0089 lands. The version in the future reply controls. It is recorded and replayed to show the label kept native with `statement_unmapped: true`. 0089 lands on a synthetic row.
- Proof: one pm job per source from landed main, keyless, with existing retries only; each success becomes a replay case (from outside the repo where terms bar storage) and ticks its roadmap box; each failure is recorded through pm progress with the box left unchecked. No refresh loops.
