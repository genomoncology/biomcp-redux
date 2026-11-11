# 0831 — saved-lists-and-searches-in-the-cli

Status: OPEN.

Milestone: 9

## Outcome

On the command line a user saves a search with `--save-as NAME`, adds IDs to a named list, sees both with `saved show`, exports everything as typed JSON with `saved export`, and removes one or all with `saved delete`. The records live in `saved.sqlite` in the data folder and survive a cache clean and an upgrade.

## Why

It is the first piece of milestone 9 a person can use, and every later piece (what changed, notes, MCP, hosted) writes to this file.

## Proof

- Proof: routine cases under a new `tests/cases/saved/` run save, add, show, export and delete against a temporary data folder; `store clean` leaves the saved file; a checked-in `saved.sqlite` from this build opens and exports to the checked-in expected JSON; a test rejects any migration statement other than create table, create index or add column with a default.

## Starts from

- Starts from: `sdlc/planning/saved-work.md` (Records, Store, Commands, Compatibility), which is the design; the cache store in `crates/biomcp-query/src/store/` as the pattern for schema ownership and ID making; `crates/biomcp/src/paths.rs`; the command registry in `crates/biomcp-types/src/query/commands.rs`; how `--fresh` is parsed as a global option.
- PM decision: 2026-11-11. Only `biomcp.saved-ref`, `biomcp.saved-search` and `biomcp.saved-list` and their tables in this ticket. Snapshots, change reports, notes, the MCP tool and the `save` next step are later tickets in the design's Order.

## Keeps

- Keeps: the cache file and its drop-and-rebuild upgrade path are untouched and cannot reach the saved file; no source content is stored; MCP and hosted behavior are unchanged; no new dependency beyond the SQLite already linked.

## Defers

- Defers: everything after step 1 in the design's Order, each to its own ticket filed when this lands.

## Changes

- Changes: `crates/biomcp-query/src/saved/`, `crates/biomcp-types/src/saved/`, `crates/biomcp/src/paths.rs`, `crates/biomcp-types/src/query/commands.rs`, `tests/cases/saved/`
