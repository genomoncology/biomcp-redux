# 0951 — sync-time-limit-is-configurable

Status: OPEN.

Milestone: 7

## Outcome

A user can set the bulk sync time limit (today a fixed 120 seconds) through BioMCP's configuration, with 120 s kept as the default, and GTR's after-sync read uses the same setting instead of its own fixed 120 s. A sync that runs out of time says which limit it hit and how to raise it.

## Why

0931's review (2026-11-14): `FetchConfig::sync_budget` (`crates/biomcp-fetch/src/lib.rs:172`, from 87282f9ad) is a config field with a default but nothing in `crates/biomcp/src` lets a user set it, and `crates/biomcp-adapters/src/gtr/bundle.rs:54` has its own fixed `Duration::from_secs(120)` (6d0dec4fa). 0931 grows the GTR sync from about 57 MB to 245 MB; it took 43.7 s on the M5, but below about 2.5 MB/s it will time out where it used to succeed. Workspace rule: make configuration values configurable and choose a default.

## Proof

- Proof: a case or test sets a short limit and gets the named timeout message; the default stays 120 s; GTR's after-sync read follows the setting.

## Starts from

- Starts from: the two lines above; how other FetchConfig values reach users (environment or config file).

## Keeps

- Keeps: the 120 s default; other limits.

## Defers

- Defers: nothing.

## Changes

- Changes: config plumbing, the GTR read, help/docs, a test.

## Progress

Build, 2026-11-14.

- Route: other FetchConfig values (`auto_sync_max_bytes`, `source_data_max_age_days`) reach users only through `config.json`, with no flag or environment variable. The new `sync_timeout_seconds` (positive whole seconds) follows the same route into `FetchConfig::sync_budget`; omitted, the 120 s default holds.
- GTR's after-sync count now runs on the sync limit. Before, its code asked for a fixed 120 s, but `Fetch::leg` capped that at GTR's 15 s source deadline, so in practice it had 15 s.
- A sync that fails with `deadline-exceeded` adds a `request-guidance` notice first: "This sync stopped at its N-second time limit. Set sync_timeout_seconds in config.json to a larger number of seconds to give a slow download more time." No reply schema change.
- `biomcp help` sync line and `docs/configuration.md` name the setting and its default; a new feature line `bundle-sync-time-limit`.
- Cases: `drug/sync-time-limit` (limit 1 s) and `drug/sync-time-limit-raised` (limit 3 s) replay the existing `drug/markets` WHO PQ recording, whose transfer took 2,419 ms: the first fails with the named notice, the second completes with 29 rows (job j40953).
- Live on the M5 (j40953), `sync gtr` from NCBI with `sync_timeout_seconds` 5 failed with `deadline-exceeded`, exit 4, and the notice naming the 5-second limit.

Review fixes, 2026-11-14 (PM review of b75a43bff).

- New `Fetch::sync_leg(spec)` starts a leg on the sync limit without the source's request cap, as `sync_set` already did. `sync`, `sync_catalog` and `sync_set` use it, and so do the after-sync reads of GTR (was 15 s in practice) and WikiPathways (was its 15 s source deadline). Every leg a sync opens now runs on the sync limit, so the time-limit notice names the limit that actually ran out.
- The after-sync read starts its own limit when the download ends, so GTR and WikiPathways syncs can take up to about twice the setting; `docs/configuration.md` says so.
- `crates/biomcp-fetch/tests/bundle.rs` `sync_leg_follows_the_configured_limit`: on a replay (simulated) clock with a 15 s source deadline, `sync_leg` has exactly 600 s with a 600 s setting and 1 s with 1 s, while an ordinary leg stays capped at 15 s; the default stays 120 s.
