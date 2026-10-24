# 0061 — Capture a successful public Semantic Scholar search

Status: OPEN.

Milestone: 1

Depends on: 0042

## Outcome

Capture a successful keyless Semantic Scholar search from landed main and verify its source-native identifiers and order. This follow-up gates the Semantic Scholar roadmap tick; it never blocks code landing.

- Starts from: the recorded-proof landing decision and 0042. Job j9693 recorded three HTTP 429 attempts at the shared keyless interval. The failure envelope and successful synthetic response remain routine replay proof.
- Keeps: the shared quota, credential isolation, canonical request identity and offline landing gate.
- Changes: `tests/replies/article/semantic-scholar/**`, `tests/cases/article/semantic-scholar/**`, `ROADMAP.md`.
- Proof: a build delegate runs one bounded actual-command `--record` search through `pm job` from landed main, with no key and only existing retries. On success, preserve raw metadata and source order, run its replay through CLI and MCP, record measured ceilings in the case and get review before landing. If the source still rate limits, record the job outcome through pm progress and leave the roadmap unchecked for a later bounded attempt.
- Defers: keyed traffic, source ranking and changed product behavior.
