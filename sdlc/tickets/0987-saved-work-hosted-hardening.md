# 0987 — saved-work-hosted-hardening

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

## Progress

- Floor (finding 2): `record_check` now reads `checked` inside its write transaction and, inside the floor, keeps nothing and returns the `not_due` report pointing at the kept snapshot. Parallel calls that both saw the search due may each still read the source once, but only the first snapshot is kept and the earlier one is never dropped; the documented per-account lock in the front end stops the parallel reads in hosted mode. Store test `a_second_check_inside_the_floor_keeps_nothing`; existing store tests make the search due again between checks with a `due` helper.
- Re-check policy (finding 4): `recheck` applies `mcp_policy::refusal` to the stored words on any MCP call (hosted included) before setting its own `fresh`, and runs only a first search page, as `--save-as` accepts. Proof in the hosted runner (`saved_hosted.rs`, `stored_rechecks`): a row written straight into the file with `--fresh` answers `usage.mcp_unavailable`, a stored `get trial` answers a usage error, and neither keeps a snapshot.
- Case folding (finding 3): chose the docs line, "DIR must be on a case-sensitive filesystem". It keeps the header format agreed in 0984 and the planned deployment (ECS on Linux) already meets it; lowercase-only would change the header contract for every front end.
- Docs (finding 1): `docs/configuration.md` hosted saved paragraph now names the per-account lock, the conditional fetch and write-back with replay up to three times, the `saved.sqlite-journal` delete, the 0700 account folder, and the case-sensitive DIR.
- Cheap job j41577 (aa069b1d5): fmt, docs and schema regeneration, `make check-cheap`, `biomcp-query` saved store tests (13 passed, new floor test included) and the `saved` cases (9 passed, `saved-work/hosted-account-folder` included). Regeneration changed nothing; fmt reflowed the new runner test, committed. Base-commit failure of both proofs is by construction, not run: on 2a705db95 `record_check` keeps a second snapshot, and `recheck` runs the stored `--fresh` search and keeps a snapshot.
- Full gate j41578 passed on f83f06f2f (`make --keep-going landing-gate docs-check schema-release-check`; origin/main was still 2a705db95, so the branch already contained it).
