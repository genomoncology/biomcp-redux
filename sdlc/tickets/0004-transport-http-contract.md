# 0004 — Add the shared transport contract and checked HTTP replies

Status: OPEN.

Milestone: 1

## Outcome

Callers use one blocking transport to send declared source requests under deadlines, enforce required keys, decode checked replies and return typed source failures without exposing credentials.

Depends on: 0003

- Starts from: `sdlc/plan.md` stage 4; `sdlc/planning/architecture.md` sections 4.1–4.4, 4.7, 4.10, 4.12 and 10.4; and the minimal biodata types from 0003. Read the complete transport design before choosing this step's internal boundaries.
- Changes: `Cargo.toml`, `Cargo.lock`, `crates/biomcp-fetch/**`, `crates/biomcp/tests/ratchets/policy.rs`, `crates/biomcp/tests/ratchets/dependencies.rs`, `crates/biomcp/tests/ratchets/source.rs`.
- Scope: Create biomcp-fetch when it first has behavior. Implement SourceSpec, endpoint declarations, checked JSON replies, SourceError, Fetch construction, Leg, explicit deadlines, bounded bodies and redacted credential handling from the design. Use blocking ureq with the approved TLS setup and config-only proxies. Required keys are checked before sending. Decode and Reply::check failures are typed bad_reply outcomes. Introduce only the public methods that have behavior in this step; no placeholder cache, retry, XML, bundle or replay methods. Source-key declarations are typed from the start, with rate scheduling in the following transport step.
- Keeps: The public API never reads environment settings. Secrets never enter Debug, errors or any recorded request identity. No provider calls or live keys. KEGG remains uncalled. No adapters or clinical output. Existing workspace tests and lint limits remain on. Add the fetch clippy policy and narrowly extend the ratchet's existing allowlist for public transport tests.
- PM decision: Public transport tests are a permanent contract through crates/biomcp-fetch/tests against a local stub server with no external network, as architecture 10.4 now records. Delete a duplicate public-library test only when a later binary case proves its behavior.
- Proof: Public transport tests first fail without the crate, then exercise checked success, explicit 404/4xx/5xx outcomes, broken JSON, a 200 error body rejected by Reply::check, content-type rejection, body limits, deadline expiry and required-key refusal with zero server requests. Use distinctive placeholder credentials and assert their omission from errors and Debug. Tests isolate config and state, wait for server events and use no short wall-clock performance limit. Run make landing-gate through pm job before review.
- Defers: Source-key quota scheduling, retry and Retry-After; SQLite reply cache and stale fallback; record/replay/offline; XML and batches each follow as transport steps. Bundle storage waits for the gene slice. No transport roadmap item is ticked until the full stage is complete.
