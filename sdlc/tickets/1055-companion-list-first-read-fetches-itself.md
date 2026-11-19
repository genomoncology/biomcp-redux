# 1055 — companion-list-first-read-fetches-itself

Status: OPEN.

Milestone: 7

## Outcome

On a machine that has never fetched FDA's companion diagnostic list, `biomcp search diagnostic --source fda-cdx ...` answers with rows on the first call. The caller never has to run `biomcp sync fda-cdx` first.

## Why

Ian, 2026-11-19: "the agent shouldn't have to sync anything with regards to BioMCP, right? It should just auto-sync, right, or auto-do whatever it needs to do." An agent's first call to the list was refused, and an agent that does not retry goes around BioMCP for good.

## Proof

- Proof: with an empty cache, `biomcp search diagnostic --source fda-cdx --biomarker KRAS` returns `status: complete` and rows in one call; it fails on the base commit, which returns `status: failed` with the source's state `syncing` and a `next` step of `sync fda-cdx`.

## Starts from

- Starts from: Seen on main be96d8d40 (1.0.0), Linux, 2026-11-19, while agents used the command line build on real tasks. The first call replied `{"status":"failed",...,"sources":[{"source":"fda-cdx","state":"syncing","reason":"syncing"}],"next":[{"kind":"sync","args":["sync","fda-cdx"]}]}`. `biomcp sync fda-cdx` then reported 244 rows and 181,000 bytes, and the same search answered.
- HGNC and ClinGen already fetch on read under `auto_sync_max_bytes`; the stored tables do not (ticket 0280 records the two rules). This list is 181 KB.
- `get disease MONDO:0007947 prevalence --source orphadata` fails the same way, with a `next` step of `sync orphadata`.
- PM, 2026-11-19: the developer says in the ticket which other stored sources are small enough to fetch on first read the same way, and does those too or says why not.

## Keeps

- Keeps: `--offline` and replay never fetch. A source over the size limit still asks for an explicit sync, and says its size.

## Defers

- Defers: refreshing a list that is already stored but old; that keeps its present rule.

## Changes

- Changes: the stored-source read path, its cases, and the docs lines that state the sync rules.
