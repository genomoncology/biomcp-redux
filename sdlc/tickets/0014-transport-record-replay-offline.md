# 0014 — Record source exchanges and replay them without network access

Status: OPEN.

Milestone: 1

## Outcome

The shared transport records credential-free GET exchanges, replays only those recordings with virtual deadline/quota time, and answers offline requests only from checked cache data.

Depends on: 0011

- Starts from: `sdlc/plan.md` stage 4; architecture sections 4.4, 4.7, 4.9 and 4.10; the existing identity/quota/retry/cache paths and public transport test helpers.
- Changes: `Cargo.toml`, `Cargo.lock`, `deny.toml`, `crates/biomcp-fetch/Cargo.toml`, `crates/biomcp-fetch/src/**`, `crates/biomcp-fetch/tests/**`, `crates/biomcp/tests/ratchets/source.rs`.
- PM decision: 2026-10-24. Record mode bypasses cache reads and stale fallback, captures source exchanges and may write checked replies. Replay reads only recordings and fails every miss even with a warm cache. Replay may write only to its case's own temporary cache, never the user's cache. Offline reads cache only, including stale data, and never sends or syncs. No earlier stale-cache snapshot capture in this step.
- Scope: Extend the existing mode/request path with Record, Replay and Offline. Record human-readable request metadata and ordered reply sequences under dir/source/key.json using the existing credential-free identity. Capture status, Retry-After, delay and JSON/text body so retries, malformed content and 200-with-error-body are ordinary recordings. Replay feeds the same body/status/check/retry path without opening network connections. Virtual time advances by recorded delay, quota spacing and retry waits, with per-leg budgets; no real sleeping in replay. Missing/exhausted or invalid recordings are typed local errors, never cache fallback. Offline returns fresh/stale checked cache entries and a typed source offline miss. Preserve required-key refusal before any recording/cache read.
- Cache ownership: Replay ignores the user's ordinary configured cache location. Default replay uses no persistent writes; explicit replay cache writes require a case-owned temporary allocation whose lifetime/ownership is clear. Choose the smallest API that enforces that distinction; do not accept a normal user cache path merely because replay was selected. Keep record and live cache writes under existing checked-body rules.
- Keeps: One transport/retry/decoder/check implementation and one request identity. No per-source URL overrides or product test hooks. Never write credentials in recording metadata, paths, errors or diagnostics. Provider reply bytes remain checked provider content under the established credential boundary. GET only; XML/POST/batches arrive separately. No adapters or live provider requests.
- Proof: Public API/local-server tests record then replay a successful GET with zero replay connections, a retry sequence, malformed body, rejected checked body, deadline delay and recording miss/exhaustion. Warm ordinary cache cannot hide a replay miss or receive replay writes. A replay-checked reply can populate only its isolated case cache for a later offline request; offline fresh/stale/miss never connect. Parallel recorded calls sharing one quota retain virtual spacing without asserting which thread wins. Verify long recorded waits advance virtual time without real waiting, via event/clock outcomes instead of tight wall-clock ceilings. Run make landing-gate through pm job after current main.
- Defers: XML, POST bodies and scoped batch calls follow before adapters consume the complete transport. CLI flags and runner replay/offline steps arrive when real entity commands consume transport. No full transport or record/replay roadmap tick until the public mode contract and its real consumers satisfy the entire item.
