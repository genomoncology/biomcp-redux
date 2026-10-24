# 0020 — Run bounded source batches with ordered results

Status: OPEN.

Milestone: 1

## Outcome

Leg::batch runs chunk requests concurrently under the existing source quota and absolute budget, returns every result in input order and preserves correct replay time and leg reports.

Depends on: 0018

- Starts from: architecture 4.2/4.5; Leg/LegReport/budget/finish in lib.rs; deadline.rs Deadline/VirtualClock; quota.rs shared source admission; request.rs per-attempt execution; existing public contract stub and record/replay tests.
- Changes: `crates/biomcp-fetch/src/lib.rs`, `crates/biomcp-fetch/src/batch.rs`, `crates/biomcp-fetch/src/deadline.rs`, `crates/biomcp-fetch/tests/batch.rs`, `crates/biomcp/tests/ratchets/source.rs`.
- Scope: Add the architecture batch shape using scoped standard threads and existing RequestError results. Keep chunk/result order independent of completion order and avoid a thread or retained handle per unbounded chunk. Use a few named worker/admission limits with no new user configuration. Callback requests reuse normal Leg methods, quotas and all transport modes; no nested HTTP client or retry policy. Real branches share one absolute deadline. Replay branches start at the current parent instant, preserve shared quota spacing and consume only their own request delays; join parent progress at latest completion rather than summing overlapping branch delays. Later work/batches start from joined time. Merge stale flags and original typed fallback failures into the owning leg without duplicating report truth. Missing required keys and wrong clocks preserve existing refusal before storage/network; do not invoke callbacks for a skipped leg. Callback panic behavior and exact worker/clock policy need a recorded decision if architecture does not settle them.
- Keeps: No async runtime, detached worker, global state, worker-pool framework, provider calls or product hook. Existing request/cache/record ordering and finite limits stay. Pure empty input returns empty results. Body/SQL/record errors remain typed local results; source failures do not discard other chunks. No caller-facing timeout success criterion in tests.
- Proof: Public API cases use event/barrier-driven local peers and distinct results to prove overlap, quota/in-flight bounds, ordered mixed success/failure and complete results without fixed sleeps. A replay capture with different delays and quota spacing proves branch overlap, joined remaining budget, later-batch progress and deadline exhaustion, without wall-clock limits. Verify stale report retention and replay cursor ownership through batch calls; verify empty/skipped/foreign-clock paths cause no calls. Existing request and mode contracts plus make landing-gate pass via pm job after current main.
- Defers: Provider-specific chunk sizes and typed mappings belong to their adapters. Real delayed command MCP scheduling proof waits for the first gene/provider command; no fake slow command here. Bundle endpoints begin with the first HGNC consumer.
