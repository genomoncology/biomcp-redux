# 0137 — Resolve an explicit disease identifier for trial search

Status: OPEN.

Milestone: 2

Depends on: 0117

Changes: trial search requests, disease identity resolution, trial cases and generated help

## Outcome

- `search trial --condition-id` resolves an exact input CURIE through the named MyDisease resolver. ClinicalTrials.gov receives the returned concept's primary label unchanged in `query.cond`. The response echoes the input identifier, resolver, resolved concept, primary label and exact source request.
- `--condition` keeps its literal behavior. Conflicting condition selectors produce a usage error before any source request. Resolution never chooses a candidate, rewrites a label or falls back to a name search.
- NCI accepts an explicit NCIt identifier or one unique source-provided NCIt mapping. Ambiguity, a missing mapping and resolver failure stop before trial I/O. Preserve source disable policy, provenance, cache validation and existing trial filters.
- Proof: outside-in CLI and MCP cases cover a positive exact resolution, exact ClinicalTrials.gov text, explicit NCI input, unique NCIt mapping, ambiguous resolution, absent mapping, resolver failure and conflicting selectors. Failure cases assert zero trial requests. Use existing recordings or synthetic local replies; no provider call is required. Generated help and trial accounting name the option. Run the relevant cases and routine gate through pm job.

## Design authority

The PM's 0117 review accepts explicit condition identifiers and requires this behavior in a separate trial-owned ticket. The disease design names this ticket and leaves trial implementation here. Implementation dependencies on the disease resolver belong in the PM's ordered build queue.
