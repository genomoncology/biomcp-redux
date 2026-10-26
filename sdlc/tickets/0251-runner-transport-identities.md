# 0251 — Replace hand-built recording identities in the new slice runners

Status: OPEN.

Milestone: 4

Depends on: none

## Outcome

The runner tests that 0221, 0234, 0238 and 0245 added build recording identities by hand, which caused four incidents and six failed jobs: three direct digest LowerHex errors and one protein method, order and escaping identity mismatch. Those tests now take identities from the transport, as architecture 5.5's recipe already requires.

- Changes: the runner test files those four tickets added under `crates/biomcp/tests/runner/`, only.
- Fix: retained replies use the existing `response_cases::copy_capture`. New synthetic replies are recorded through the existing Stub with `--record` and then have their exchange mutated. Remove the hand-built digest and identity helpers.
- Keeps: every case's assertions and measured ceilings. No public API, framework, gate or source call.
- Proof: the same cases pass with the helpers removed, and a search of the runner tree finds no hand-built recording digest. Then make landing-gate runs through pm job.
- Order: next free lane, before any further runner work on those files.
