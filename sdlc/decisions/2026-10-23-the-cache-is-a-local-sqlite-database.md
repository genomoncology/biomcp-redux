# The cache is a local SQLite database

Decided 2026-10-23 by Ian.

- The cache is one local SQLite database file.
- The transport owns the raw reply table. The store owns entities, aliases, links and query pools.
- A query pool is a stored snapshot, so paging is stable and a warm page makes no request.
- Later, the same database backs a browsable wiki in web server mode: one page per stored entity, links between them, and a way to pull in more. The wiki waits until after the entity slices.
- Postgres waits until a shared deployment needs concurrent writers.
