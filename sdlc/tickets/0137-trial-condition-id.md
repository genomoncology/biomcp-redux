# 0137 — Resolve an explicit disease identifier for trial search

Status: COMPLETE.
Landed: 2ad9bd619996f0913d9bc9c9fe6de2c8e14437e1

Milestone: 3

Depends on: 0141

Changes: trial search requests, disease identity resolution, trial cases and generated help

## Outcome

- `search trial --condition-id` resolves an exact input CURIE through the named MyDisease resolver. ClinicalTrials.gov receives the returned concept's primary label unchanged in `query.cond`, never a synonym or another spelling. The response echoes the input identifier, resolver, resolved concept, primary label and exact source request.
- `--condition` keeps its literal behavior. Conflicting condition selectors produce a usage error before any source request. Resolution never chooses a candidate, rewrites a label or falls back to a name search.
- NCI accepts an explicit NCIt identifier or one unique source-provided NCIt mapping. Ambiguity returns the source's candidates. Ambiguity, a missing mapping and resolver failure stop before any trial request. A missing NCI key skips before the resolver call. Preserve source disable policy, provenance, cache validation and existing trial filters.
- Proof: outside-in CLI and MCP cases cover a positive exact resolution, exact ClinicalTrials.gov text, explicit NCI input, unique NCIt mapping, ambiguous resolution, absent mapping, resolver failure and conflicting selectors. Failure cases assert zero trial requests. Known positives come from captured direct queries: MONDO:0005105 resolves to `query.cond=melanoma` using the retained MyDisease capture and the ClinicalTrials.gov melanoma recordings; DOID:3910 resolves to MONDO:0005061; NCIt C3224 runs against the existing NCI capture. The disease design's D05, D08, D13 and D21 supply the other cases. Synthetic replies cover only failures. Generated help and trial accounting name the option. Run the relevant cases and routine gate through pm job.

## Design authority

The PM's 0117 review accepts explicit condition identifiers and requires this behavior in a separate trial-owned ticket. The disease design names this ticket and leaves trial implementation here. Implementation dependencies on the disease resolver belong in the PM's ordered build queue.
- PM ruling 2026-10-25: ClinicalTrials.gov and NCI trial pages gain an optional `condition_resolution` record: the input CURIE, the resolver when one was used, the returned concept and its unchanged primary label, source-provenanced NCIt choices, and the selected NCIt only when unique. An explicit NCIt input names no MyDisease resolver. Ambiguous or missing mappings show the resolution and make zero trial requests. The native request field is unchanged, and schema versions follow the compatibility check.
