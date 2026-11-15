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

## Progress

- Decided (within the outcome): `saved ...`, `help saved` and `--save-as` are read before the shared command tree (`crates/biomcp/src/saved.rs`, called from `main.rs`), not added to `crates/biomcp-types/src/query/commands.rs`. The shared tree feeds MCP single and typed tools, hosted discovery, help and tool output schemas, so leaving it untouched keeps MCP and hosted behaviour unchanged; over MCP `saved` stays an unknown command (`tests/cases/saved/lists.yaml` checks it). `docs/reference/commands.md` renders the `saved` grammar beside the shared tree.
- Decided: every saved command replies with one body, `biomcp.saved/1` (`searches`, `lists`). The record formats `biomcp.saved-ref/1`, `biomcp.saved-search/1` and `biomcp.saved-list/1` are registered schema contracts. A list item keeps an optional `--note` (2,000 characters) as the design's `saved-list` record has; `saved note` stays deferred.
- Decided: one name per project across searches and lists, compared without ASCII case, so `saved show NAME` and `saved delete NAME` are never ambiguous. A search is saved only when it exits 0; the same words under the same name again change nothing, other words are refused before the search runs. Stored words drop run options (`--config`, `--replay`, `--record`, `--case-cache`, `--observe`, `--offline`, `--fresh`, `--format`, `--source_pacing`). `request_key` is the SHA-256 of the checked request.
- Decided: `saved list add` checks each ID by parsing `get ENTITY ID [--source S]`, with no source request. Reads of a missing file create nothing.
- Store: `saved.sqlite` in `$XDG_DATA_HOME/biomcp` or `~/.local/share/biomcp` (Windows `%APPDATA%\biomcp`), never the cache folder; cache cleanup only matches `cache-N-M.sqlite` names. Rollback journal (never set to WAL). The file's version is the count of statements in `MIGRATIONS` it has run, kept in `meta`; a larger count is refused untouched. `migrations_only_add` rejects any statement but create table, create index or add column with a default.
- Decided: the store lives in `crates/biomcp-query/src/store/saved/`, not `src/saved/`, because the structural ratchet keeps every SQLite reference under `src/store/` (`crates/biomcp/tests/ratchets/source.rs`). Its tests are listed in `placement.rs` and architecture 10.4.
- Fixture: `tests/fixtures/saved/1.0.0/saved.sqlite` and `export.json` were written by `saved-work/checked-in-files-export` with `BIOMCP_WRITE_SAVED_FIXTURE` in job j41461 (one saved trial search from the recorded `trial/condition` replies, two lists); the same trial reads them back with this build.
