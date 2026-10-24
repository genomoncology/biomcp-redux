# 0115 — Capture a positive Cancer Hotspots candidate row

Status: OPEN.

Milestone: 2

## Outcome

One bounded keyless capture from landed main shows the hotspots section returning BRAF's V600 residue row as a candidate, with its source transcript and residue and alternate counts, and replays it as a case. The case claims no transcript match, as 0113's review found no source that ties a variant to the hotspot transcript.

- Why: 0110's Hotspots attempt used CA123643, whose ClinGen Allele Registry identity carried no gene, so the section stopped with `no_join_id` before any Hotspots request. The preflight BRAF gene and protein input gives the section the gene it needs.
- Proof: one pm job from landed main with the preflight BRAF gene and protein input and existing retries only. A success becomes a replay case, from outside the repo if Cancer Hotspots' terms bar storage, and ticks the Cancer Hotspots roadmap box. A failure is recorded through pm progress with the box left unchecked. No refresh loops.
