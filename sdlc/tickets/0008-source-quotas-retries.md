# 0008 — Enforce shared source quotas and deadline-bound retries

Status: OPEN.

Milestone: 1

## Outcome

Every request acquires its declared source quota, shares provider limits across source legs, and retries only the approved failures within its leg deadline.

Depends on: 0004
Depends on: 0007

- Starts from: `sdlc/plan.md` stage 4; architecture sections 4.2, 4.5, 4.6 and 4.7; the existing biomcp-fetch request path and public transport tests. Follow the approved staged API by adding quota fields only when enforced.
- Changes: `Cargo.toml`, `Cargo.lock`, `deny.toml`, `crates/biomcp-fetch/src/**`, `crates/biomcp-fetch/tests/**`, `crates/biomcp-fetch/Cargo.toml`, `crates/biomcp/tests/ratchets/dependencies.rs`, `crates/biomcp/tests/ratchets/source.rs`.
- Scope: Add SourceKey, Rate and Quota with validated construction and SourceSpec's declared quota. One Fetch owns limiter state keyed only by SourceKey. Sources sharing a quota share FIFO rate and in-flight limits. Required-key refusal remains before any permit acquisition or request. Apply the declared keyed rate only when a credential exists. Release permits on every success/failure path and make deadline cancellation leave no stuck queue entry. Extend the existing single request path with idempotency-aware retries, deterministic request-based jitter and Retry-After seconds or HTTP-date parsing as section 4.6 specifies. A wait beyond the remaining deadline fails immediately with typed reason and stated wait.
- Keeps: No limit derived from URL, host or endpoint. No per-adapter scheduler, worker pool, async runtime or parallel retry implementation. No provider calls or adapters yet. No cache, replay, bundle or XML placeholders. Keep source deadline caps, body/reply checks and credential secrecy. Add only dependencies whose saved parsing/code cost is recorded and update exact license metadata in this ticket if needed.
- Proof: Public API tables and local-server cases prove shared-key spacing and in-flight caps across two source specs, independent keys, keyed and unkeyed rates, FIFO progress after cancellation, and permit release after failures. Exercise the retry status table, idempotency refusal, exhaustion, both Retry-After forms and wait beyond deadline. Use events and observed request starts rather than fixed startup sleeps or tight wall-clock success ceilings. Keep slow/exhaustive concurrency outside routine checks. Run make landing-gate through pm job on current main before review.
- Defers: The next transport step supplies SQLite reply cache, record/replay/offline and virtual quota time. Adapter quota statics arrive with adapters; test fixtures exercise the public declarations under the existing library-test exception. No transport roadmap tick until the whole stage is complete.
