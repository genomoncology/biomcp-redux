# 0020 — Run bounded source batches with ordered results

Status: OPEN.

Milestone: 1

## Outcome

Leg::batch runs bounded concurrent chunk callbacks under the source quota and absolute deadline, returns normal results in input order, retains stale reports and advances replay without reusing occupied virtual quota slots.

Depends on: 0018

- Starts from: architecture 4.2/4.5; Leg/LegReport/budget/finish in lib.rs; deadline.rs Deadline/VirtualClock; quota.rs shared source admission; request.rs per-attempt execution; existing public contract stub and record/replay tests.
- Changes: `crates/biomcp-fetch/src/lib.rs`, `crates/biomcp-fetch/src/batch.rs`, `crates/biomcp-fetch/src/deadline.rs`, `crates/biomcp-fetch/src/quota.rs`, `crates/biomcp-fetch/src/request.rs`, `crates/biomcp-fetch/tests/batch.rs`, `crates/biomcp/tests/ratchets/source.rs`, `sdlc/planning/architecture.md`.
- PM decision: 2026-10-24, m4292. Conservative FIFO replay retains virtual completion times; thread arrival may affect ordering/delay, no deterministic scheduler. MAX_BATCH_WORKERS=8 bounded by chunk count, separate from provider admission; nested callback-leg batches execute inline. Bound is per outer invocation; independent calls share provider quota. Use per-chunk RequestError results; callback panics propagate after scoped cleanup. Empty input returns empty; otherwise no-key, foreign-clock, expired-budget refusals in that order return per chunk without callback execution. Deadlines bound transport only.
- Scope: Add batch<C: Sync,T: Send>(&self, chunks: &[C], call: impl Fn(&Leg<'_>, &C) -> Result<T, RequestError> + Sync) -> Vec<Result<T, RequestError>>. Use bounded scoped workers/handles and assign each result to its input position. Callback legs share existing request, decoder, cache, quota and recording paths; keep one owner for stale flags and original fallback failures. Real callbacks share parent absolute deadline. Replay workers begin at parent current instant, retain progress between assigned chunks and parent joins greatest completion. Extend existing quota permits to retain virtual completion time, preventing slot reuse before completion while preserving token spacing and FIFO admission. No ideal-event simulator, new dependency or recording format.
- Keeps: Existing mode ordering, local errors and checked-cache behavior stay. Source failures preserve other normal results. No async runtime, detached workers, global pool, provider calls, product hooks or simulated callback execution. Transport deadlines cannot preempt arbitrary callback computation/blocking.
- Proof: Event-driven public API local-peer contracts prove overlap, configured worker and provider in-flight bounds, input order and mixed failures. Replay captures prove independent worker progress, parent maximum joining, subsequent batches and deadline exhaustion; a sole-slot case proves a second request cannot start before the first recorded completion. Do not assert exact interleaving/timing that thread arrival can change. Cover stale reports, shared cursors, nested inline execution, callback panic cleanup and empty/refused callbacks. Use barriers only where admission allows every participant. Reuse mode evidence; standard landing gate through pm job after main.
- Defers: Provider-specific chunk sizes and typed mappings belong to their adapters. Real delayed command MCP scheduling proof waits for the first gene/provider command; no fake slow command here. Bundle endpoints begin with the first HGNC consumer.
