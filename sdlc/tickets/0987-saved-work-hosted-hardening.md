# 0987 — saved-work-hosted-hardening

Status: OPEN.

Milestone: 7

## Outcome

Hosted saved work stays correct when a front end follows only `docs/configuration.md`: the docs state the whole front-end contract, a hosted re-check runs under the same policy as any hosted call, and parallel `changed` calls for one account keep the hourly floor.

## Why

The 0984 review (2026-11-16, accepted at 76ef8d4e8) found four gaps. None blocked landing, because the planned deployment (ECS on Linux, a per-user lock in the wrapper) avoids them:

1. `docs/configuration.md:153` omits the per-user lock and conditional write from `sdlc/planning/saved-work.md` (Hosted 2, 4, 5). It also omits deleting a stale `saved.sqlite-journal` before placing a fetched file, which SQLite would otherwise replay onto the new file. And it omits that the core creates `DIR/<account>/` with mode 0700 (`saved.rs:295`).
2. `saved_changed.rs:39,66` checks the hourly floor before the source read, outside the write transaction, so parallel calls all re-read the source and the earlier snapshot is dropped.
3. On a case-insensitive filesystem, accounts `Alice` and `alice` share `DIR/Alice/` (`saved_mcp.rs:33,92`).
4. `command/recheck.rs:27-32` runs stored words through `run_request` without `mcp_policy::refusal`, and accepts any `Run` action, not only searches. It can't be exploited today because the core writes those words only from a search that already passed policy, but hosted MCP now reaches this code.

## Proof

- Proof: a hosted re-check of a stored non-search, or of a search that hosted policy refuses, is refused (a test that writes such a row directly into the store); it fails on the base commit.
- Proof: two `changed` calls for one search inside the same hour store one snapshot, and the second answers not due (a store-level test); it fails on the base commit.
- Proof: the docs paragraph names the per-account lock, the journal delete, the folder owner, and that DIR must be case-sensitive (or the core accepts only lowercase accounts; the developer picks and records why).

## Starts from

- Starts from: the 0984 review findings above; `crates/.../saved_changed.rs` `record_check`; `command/recheck.rs`; `docs/configuration.md:153`.

## Keeps

- Keeps: the typed MCP 2,400-character budget; saved-store COMPATIBILITY 1 and additive-only migrations; the header format agreed in 0984 unless the lowercase option is chosen.

## Defers

- Defers: nothing.

## Changes

- Changes: saved store check recording, recheck policy, configuration docs, tests.
