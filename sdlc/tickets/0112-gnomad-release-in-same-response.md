# 0112 — Read the gnomAD release from the same response as its counts

Status: COMPLETE.
Landed: 19f954da18eef770364751d3a050fa1651895b64

Milestone: 2

## Outcome

The gnomAD variant request also asks for `va.exome.derivedFrom.version` and `va.genome.derivedFrom.version`. Each `PopulationFrequency` carries that version as `dataset_version` only when the same response supplies it. Counts without a version in their response stay native without a typed frequency.

- Why: `gnomad_r4` is a moving selector, not a release. The gnomAD browser schema (`graphql-api/src/graphql/types/va.graphql` and `resolvers/va.ts`) returns the dataset version beside the counts. A resolver constant or a version filled in from an older saved body would label counts with a release that did not produce them.
- Starts from: the gnomAD adapter, the shared `PopulationFrequency` type, and the landed gnomAD cases.
- Proof: a case built from a published response shape asserts the request asks for both versions, the frequency carries the version from that response, and a response without a version yields no typed frequency. A case also asserts that the VA `focusAlleleCount` and `locusAlleleCount` equal the exome and genome AC and AN in the same response, and records a problem when they differ. No case fills a version into a retained body. Then make landing-gate runs through pm job. The live capture for 7-140753336-A-T stays in 0110.
