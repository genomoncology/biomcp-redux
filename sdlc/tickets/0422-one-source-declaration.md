# 0422 — Describe each source once

Status: OPEN.

Milestone: 5

Depends on: 0421

## Outcome

Each source's display name, health probe, bundle state, key and help come from its adapter declaration, and the bundle list, key help and probe tables derive from them with no `_` fallback, so a missing row is a compile error. The registry `expect` becomes an exhaustive match. The transport no longer names HGNC; HGNC declares its index backfill as a bundle hook.

- Why: the pre-1.0 code quality review `sdlc/records/2026-10-28-code-quality-review.md`. Fallbacks hide gaps: a new bundle could sync HGNC, a new keyed source could show OncoKB help, and an unlisted source reports rejected in health.
- Starts from: `crates/biomcp-types/src/source.rs`, `crates/biomcp-adapters/src/registry*.rs`, `guidance.rs`, `quotas.rs`, `crates/biomcp-fetch/src/secret.rs`, `bundle/transfer.rs`, `cache_hgnc.rs`, `crates/biomcp/src/grammar.rs`, `command.rs`, `command/sync.rs`.
- Rules: no behavior change; no new framework.
- Proof: planted missing rows fail the build; existing cases pass.
- Changes: the files above and cases.
- Defers: nothing.
