# 0061 — Capture a successful public Semantic Scholar search

Status: OPEN.

Milestone: 1

Depends on: 0042

## Outcome

Capture a successful keyed Semantic Scholar search from landed main and verify its source-native identifiers and order. In the same bounded job, capture the positive TLDR for PMID 17611497, a successful search with the openAccessPdf filter from 0053, and the closed-access 17611497 and HTML 33377972 paper-link answers from 0052. This follow-up gates the Semantic Scholar roadmap tick; it never blocks code landing.

- Starts from: the recorded-proof landing decision and 0042. Job j9693 recorded three HTTP 429 attempts at the shared keyless interval. The failure envelope and successful synthetic response remain routine replay proof.
- Keeps: the shared quota, credential isolation, canonical request identity and offline landing gate.
- Changes: `tests/replies/article/semantic-scholar/**`, `tests/cases/article/semantic-scholar/**`, `ROADMAP.md`.
- Key: 2026-10-25 PM ruling. Keyless jobs j9693 and j10121 hit 429 on every retry, and keyless traffic can never prove the source. The machine environment holds the user's key as the 0.9 name `S2_API_KEY`, and 1.0 reads only `SEMANTIC_SCHOLAR_API_KEY`, so no earlier job sent a key. The job maps the inherited value by name, as `SEMANTIC_SCHOLAR_API_KEY="$S2_API_KEY"` in the job command, and the agent never inspects, prints, copies, records or stores it. The product keeps its single documented variable, and the upgrade guide still tells 0.9 users to rename `S2_API_KEY`. The recording must show no key in any request key, recording, trace, error or envelope.
- Proof: a build delegate runs one bounded actual-command `--record` search through `pm job` from landed main, with the mapped key and only existing retries. On success, preserve raw metadata and source order, run its replay through CLI and MCP, record measured ceilings in the case and get review before landing. If the source still rate limits, record the job outcome through pm progress and leave the roadmap unchecked for a later bounded attempt.
- Defers: keyless success, source ranking and changed product behavior.
