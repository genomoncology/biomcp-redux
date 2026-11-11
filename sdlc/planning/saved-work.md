# Saved work: design

2026-11-11. The design for milestone 9, after one design review. The owner-facing note is `notes/biomcp-1.1/2 Saved work.md` in the workspace; this file is what tickets build from. PM decisions here are dated 2026-11-11.

## Records

Six `biomcp.saved-*` and `biomcp.change-report` schemas in `crates/biomcp-types`. Times are Unix seconds. IDs are 16 random bytes in hex, as pool IDs are.

- **`biomcp.saved-ref/1`**: `entity` (the registry's entity word), `id` (the exact ID `get <entity>` accepts), `source` (optional; drug and variant IDs are source-specific), `as_of`, `source_date` (optional `SourceTimestamp`).
- **`biomcp.saved-search/1`**: `id`, `name` (80 characters, unique per project, case-insensitive), `project` (default `default`), `args` (the CLI words as given; this is what re-runs), `schema` (the request schema ID at save time), `request_key`, `created`, `checked` (optional).
- **`biomcp.saved-snapshot/1`**: `id`, `search`, `taken`, `total` (provider count), `complete` (holds every matching ID), `row_schema` (the reply schema the fingerprints were computed under), `sources` (`source`, `source_date`, `count`), `rows` (`ref`, 32-byte `fingerprint`).
- **`biomcp.saved-list/1`**: `id`, `name`, `project`, `created`, `updated`, `items` (`ref`, optional `note`, `added`).
- **`biomcp.change-report/1`**: a reply body derived from two snapshots, never stored. `search`, `from` and `to` (`id`, `taken`), `state` (`first`, `unchanged`, `changed`, `not_due`, `rebaselined`, `incomplete`), `new`, `gone`, `changed` (lists of refs), `next_due`.
- **`biomcp.saved-note/1`**: `id` (hash of target and text, so a repeated write changes nothing), `target` (project, list, search or item), `text` (2,000 characters), `created`.

## Store

A new file `saved.sqlite` in the data folder, never the cache folder, with its own resolver beside the cache's in `crates/biomcp/src/paths.rs`. Rollback journal, not WAL, so the one file is the whole state. Tables: `saved_search`, `saved_snapshot`, `saved_snapshot_row`, `saved_list`, `saved_list_item`, `saved_note`, `meta`. Project is a column. Keep the two newest snapshots per search. The pool tables are a pattern to follow, not tables to reuse: they hold article payloads.

## Commands

- `--save-as NAME` on any `search` command.
- `biomcp saved show [NAME]`, `saved changed [NAME]`, `saved list add NAME --entity E [--source S] [--note T] ID...`, `saved list remove NAME ID...`, `saved note TARGET TEXT`, `saved export`, `saved delete NAME`, `saved delete --all`.
- MCP single mode needs nothing new. Typed mode gets one tool, `saved`, with actions `show`, `save_search`, `changed`, `list_add`, `list_remove`, `note`, `export`, `delete`; `save_search` takes the canonical call shape follow-ups already use.
- A `save` next step on a first page with results, runnable as written.
- PM decision: these names stand.

## Change detection

- A snapshot holds the full matching ID set up to a cap, not one page. Above the cap `complete` is false and only totals compare, reported as `incomplete`.
- When `row_schema` differs between two snapshots, the report is `rebaselined`, never a list of every row as changed.
- A re-check must read the source, not the reply cache. Hosted mode refuses `--fresh` today, so `saved changed` needs its own reviewed path that bypasses the cache within the once-an-hour floor.
- `checked` lives in the file. Inside the floor, `changed` answers `not_due` and runs nothing.

## Hosted

The core never learns who the user is, has no AWS client and shares no disk with the wrapper. So the wrapper owns S3 and the core owns the file's contents.

1. A task volume shared by both containers.
2. For a `saved` call the wrapper takes a per-user lock, conditionally fetches `accounts/<user id>/saved.sqlite` into the volume, and sets an account header from the verified user ID, stripping any client copy.
3. The core, started with a saved-work folder, derives the path from the header and accepts no path. Hosted mode without the header refuses.
4. After the reply, if the file changed, the wrapper writes it back conditionally and only then returns the reply.
5. On a conflict it fetches again and replays the call, up to three times, then answers unavailable.

A task killed mid-write loses only a write nobody was told succeeded. A retry is safe because adds are set inserts and note IDs are content hashes. The bucket is private, versioned and retained, with old versions expiring; versions double as the upgrade backup. Wrapper cap 5 MB per file. The session-start line about unchecked searches is local only for now.

## Compatibility

PM decision: this replaces "no compatibility, exact pin" for `biomcp.saved-*` only.

1. Each release checks in a `saved.sqlite` it wrote. A test opens every one with the current build and compares `saved export` with a checked-in expected export.
2. Migrations may only create a table, create an index, or add a column with a default. A test rejects any other statement, so any 1.x build reads any 1.x file, which a rolling deploy needs.
3. The schema release check gains an additive-only mode for `biomcp.saved-*`.
4. A file from a newer version is refused and left untouched. The cache's drop-and-rebuild path must be unreachable for this file.
5. A saved search re-runs from its stored words through the grammar. A retired filter returns the usage error with the original words and never runs something else.

## Privacy

Stored: search words, names, notes, IDs, fingerprints, times. Never stored: keys, tokens, addresses, source content. The wrapper keeps logging the tool name only. Delete everything removes the object and its versions and both tasks' copies. Notes come back labelled as user data, not instructions. Export is typed JSON of the six formats, returned inline with a size cap on hosted.

## Evidence that it works

- Two recorded days of one trial search with hand-counted adds, removals and updates; the report equals the hand count.
- The same replay under two builds shows no row as changed.
- Eight parallel list adds for one account across two wrappers: all eight present.
- From a running task, 200 each of conditional fetch, full fetch and conditional write at 5 KB, 500 KB and 5 MB. If the 95th percentile adds more than about 300 ms to a saved call, take the DynamoDB fallback.
- A task stopped mid-call leaves an object that passes an integrity check.
- Two accounts: one never sees the other's names, and a client-sent account header is ignored.
- Tools-list and reply tokens before and after, on the bench.

## Order

1. Core: the saved file, lists, saved searches, export and delete in the CLI.
2. Hosting: the S3 latency measurement, which needs no core work.
3. Core: snapshots and `saved changed`.
4. Core: notes and limits.
5. Core: the typed `saved` tool, the `save` step, the saved-work folder and the account header.
6. Hosting: bucket, role, shared volume, wrapper fetch, write-back and retry.
7. Hosting: delete and export on the account page, the privacy text for Ian, and counters.
