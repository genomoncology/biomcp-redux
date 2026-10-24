# 0085 — Search and get NCI trials with the user's key

Status: OPEN.

Milestone: 2

Depends on: 0082
Depends on: 0077

## Outcome

`search trial --source nci` and `get trial NCT… --source nci` run only with the user's NCI key, keep NCI's own statuses, counts and fields, and take a disease filter only as an NCIt identifier, with free text through an explicit keyword option.

- Why: NCI holds cancer trial facts the registry lacks, and 0.9 silently degraded a melanoma filter to keyword search and returned a breast cancer trial first.
- Starts from: trial.md (NCI source boundary, MyDisease resolution, terms); the rulings; the keyed-sources decision; the NCI key registration from 0077.
- Risks the proof must cover: no request without the key; no fallback between NCI and the registry; a disease filter without an NCIt ID is a usage error naming the keyword option; a section is offered only after a keyed public-tier capture proves its fields; the key never enters output or recordings.
- Proof: no-key cases assert skipped with the key step and no request; one bounded keyed capture through pm job with the inherited key, which the agent never inspects, prints, copies or stores, records a melanoma NCIt search and one exact get for a replay case; keyed synthetic cases for 401, 429 and the failure set; MCP parity. Live outages never hold landing; a missing capture becomes a follow-up that gates the roadmap box. Then make landing-gate runs through pm job after current main.
- Defers: NCI biomarker fields, until a capture proves the public tier carries them.
