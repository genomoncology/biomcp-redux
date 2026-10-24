# 0082 — Get trials by NCT ID with the card, eligibility, outcomes and arms

Status: COMPLETE.
Landed: 4cc1d5d38acd7cc9734076231aeb55a49469b0be

Milestone: 2

Depends on: 0081

## Outcome

`get trial NCT…` returns one card per input with identity, title, overall status, phases, study type, conditions, intervention names, sponsor, dates and the whole brief summary. The sections `eligibility`, `outcomes` and `arms` return the registry's text and records intact, paging by character offset or whole record with continuations, through the CLI and MCP.

- Why: the eight exact records answer most of the 24 questions; eligibility and outcomes are where 0.9 was largest and where truncation would mislead.
- Starts from: trial.md (field table, budgets, the agent's next questions); the rulings; 0081's types and adapter.
- Risks the proof must cover: returned identity equals the requested NCT ID; a well-formed absent ID (NCT01811710, 404) is `absent`, distinct from an invalid ID and from failure; a missing maximum age is no limit, never a number; an outcome keeps its time frame; an arm keeps its interventions; eligibility pages rejoin to the exact source text.
- Proof: cases T01 to T08, T10 to T17, T19, T20 and T22 on the captured records, each asserting the facts the question turns on, such as FLAURA's phase and primary outcome; VISION's 20,836-byte outcomes page by whole records and the walk recovers every outcome; multi-input get keeps per-input state; the five failure cases; MCP parity; measured ceilings. Then make landing-gate runs through pm job after current main.
- Defers: locations, contacts, references and documents (0083, 0084).
