# 0366 — Report bundle and store sizes

Status: OPEN.

Milestone: 5

## Outcome

`biomcp health` reports each local bundle's stored size and the total store size on disk, and `sync` reports both the source bytes downloaded and the stored size it produced.

- Why: the docs lead's second-pass product findings (2026-10-27), item G7. About 103 MB of source files became an 855 MB store, and no command shows it. Ian's ruling that 1.0 ships every known fix (2026-10-27).
- Starts from: the store's bundle tables, the health reply and the sync reply.
- Rules: sizes are measured from the files, never estimated; no path is printed.
- Proof: a fixture store with known sizes shows them in health and in a sync reply.
- Changes: health, sync, the store size reader, schemas and cases.
- Defers: nothing.
