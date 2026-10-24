# 0011 — Cache checked source replies in SQLite

Status: OPEN.

Milestone: 1

## Outcome

The shared transport reuses fresh checked replies, serves stale replies only under the documented fallback rules, and never stores failed or unchecked provider bodies.

Depends on: 0008

- Starts from: `sdlc/plan.md` stage 4; architecture sections 4.4, 4.8, 7.1–7.3 and 7.5; the existing request identity, quota/retry path, public transport tests and approved dependency metadata.
- Changes: `Cargo.toml`, `Cargo.lock`, `deny.toml`, `crates/biomcp-fetch/Cargo.toml`, `crates/biomcp-fetch/src/lib.rs`, `crates/biomcp-fetch/src/declaration.rs`, `crates/biomcp-fetch/src/request.rs`, `crates/biomcp-fetch/src/error.rs`, `crates/biomcp-fetch/src/cache.rs`, `crates/biomcp-fetch/tests/cache.rs`, `crates/biomcp-fetch/tests/support/**`, `crates/biomcp/tests/ratchets/source.rs`.
- Scope: Add explicit cache-path configuration, exported fetch schema version and the owner/version meta row. Open SQLite in WAL mode with busy handling and a transport-owned, reasoned Mutex around its connection. Add endpoint Freshness and one reply table keyed by the existing credential-free method/URL/body identity. Revalidate decode and Reply::check on every read. Write only validated 2xx bodies and the documented short-lived 404 miss; store no other status or rejected body. A fresh hit makes no request or quota wait. Never freshness bypasses storage. A fresh-request mode bypasses the cache and rewrites only checked replies. Serve stale cache after documented source failures with an explicit stale/failure outcome. Refuse owner-version mismatch without destructive migration.
- Keeps: One request/retry path, no separate cache HTTP client. Required-key refusal precedes all cache access. No credential in keys, URLs, database bodies authored by the transport, diagnostics or Debug. Caller-owned SQLite files and old versions are never dropped or silently migrated. SQL stays in the fetch cache module. No bundle or pool tables, cache cleanup command, XML, recording or replay placeholders.
- Proof: Public local-server cases prove fresh hit with zero second requests, fresh bypass, Never bypass, checked-body rejection with no stored hit, short-lived negative caching, stale fallback with its reason, and revalidation failure after raw checks change. Exercise owner-version refusal and two Fetch processes sharing one database without corrupting replies. Seed expiry through the documented SQLite file contract rather than real minute-long sleeps or a product test hook. Run make landing-gate through pm job after merging current main.
- Defers: Offline and recording/replay modes with virtual quota time follow as the next transport step. The binary builds versioned cache filenames when it owns config. Bundle and pool tables wait for their slices. The transport roadmap item remains unticked until its full stage is complete.
