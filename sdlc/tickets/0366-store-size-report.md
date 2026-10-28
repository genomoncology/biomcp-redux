# 0366 — Report bundle and store sizes

Status: OPEN.

Milestone: 5

Depends on: 0387

## Outcome

`biomcp health` reports the measured total bytes of the files the store owns, counting each database, WAL and SHM file once and showing retained generations and partial downloads separately. Per-bundle physical size is reported as unavailable because bundles share one database. `sync` reports the complete source body bytes separately from the bytes downloaded in that invocation.

- Why: the docs lead's second-pass product findings (2026-10-27), item G7. About 103 MB of source files became an 855 MB store, and no command shows it. Ian's ruling that 1.0 ships every known fix (2026-10-27).
- Starts from: the store's bundle tables, the health reply and the sync reply.
- Rules: sizes are measured from the files, never estimated or attributed; no path is printed. Storage layout is unchanged (PM ruling m10780).
- Proof: a fixture store with known file sizes shows its total in health, with generations and partial downloads separate; a resumed and a 304 sync each report download bytes distinct from source body bytes.
- Changes: health, sync, the store size reader, schemas and cases.
- Defers: nothing.
