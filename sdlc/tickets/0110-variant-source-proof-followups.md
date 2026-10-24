# 0110 — Capture positive CIViC, Cancer Hotspots and gnomAD release proof

Status: OPEN.

Milestone: 2

Depends on: 0090
Depends on: 0091

## Outcome

Three bounded captures from landed main close the variant source-proof gaps that gate roadmap boxes, never code landing:

- CIViC: a positive exact variant membership join, such as BRAF V600E's own molecular profile evidence, recorded and replayed. The landed capture proves only a native zero.
- Cancer Hotspots: an exact join between the variant's residue and alternate and a hotspot row, recorded and replayed. Today hotspot rows stay candidates.
- gnomAD: release evidence captured with the same request as its counts, so `PopulationFrequency.dataset_version` can be typed. `gnomad_r4` is a moving selector, not a release, so landed counts stay native without a typed frequency.

- Proof: one pm job per source from landed main, keyless, with existing retries only; each success becomes a replay case (from outside the repo where terms bar storage) and ticks its roadmap box; each failure is recorded through pm progress with the box left unchecked. No refresh loops.
