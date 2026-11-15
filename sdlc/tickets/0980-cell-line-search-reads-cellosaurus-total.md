# 0980 — cell-line-search-reads-cellosaurus-total

Status: OPEN.

Milestone: 7

## Outcome

Cell-line search reports Cellosaurus's own match count as its total, instead of a note that the reply gives none.

## Why

0873's live check (2026-11-15, pm job j41438): Cellosaurus's JSON search reply has no count, but its txt format carries `numFound` in the header (2,331 for HeLa). 0873 added a "no total" note; the 0873 Outcome prefers the source's own count.

## Proof

- Proof: first check live whether the JSON reply can carry the count (a parameter or header) without a second request; if not, measure the cost of one txt request with `rows=0`.
- Proof: `tests/cases/cell-line/search-total.yaml` asserts the source's total on a recorded reply; it fails on the base commit.

## Starts from

- Starts from: 0873's `crates/biomcp-query/src/cell_line/mod.rs` note and the Cellosaurus adapter.

## Keeps

- Keeps: one request per search unless the measured cost of a second is small; the reply schema.

## Defers

- Defers: nothing.

## Changes

- Changes: Cellosaurus search adapter, cell-line search total, its case.

## Progress

### Live check, 2026-11-15 (pm job j41467, main 94395bcad)

| Request (`q=idsy:"HeLa"`) | Count in reply? | Bytes | Time |
| --- | --- | --- | --- |
| `format=json`, rows=3 | No: only `cell-line-list`; no count header | 857 | 0.41 s |
| `format=json`, rows=0 | No: empty `cell-line-list` | 51 | 0.36 s |
| `format=json`, `fields=id,ac,numFound` | No: the field is ignored | 324 | 0.38 s |
| `format=xml`, rows=1 | No | 291 | 0.35 s |
| `format=tsv`, rows=1 | Yes, header `response.numFound: 2355`, then flat rows | 400 | 0.35 s |
| `format=txt`, rows=0 | Yes, `response.numFound: 2355`, and `query.q` echoes the query | 373 | 0.35 s |

Five back-to-back pairs: the 10-row JSON search page (14,618 bytes) took 0.453-0.460 s; the txt `rows=0` count 0.348-0.358 s. A filtered query (`var:"BRAF" AND ox:"NCBI_TaxID=9606"`) also states its count (1,714).

The JSON reply can't carry the count. TSV carries count and rows in one reply, but its rows are flat columns, so reading search rows from it would replace the JSON row parser and its checks. Decision under the Keeps: a second request is small (373 bytes, about 0.35 s, same host and quota, cached a day like the search), and it runs only when the page is full; a short page already ends the source's matches.

### Changes

- `cellosaurus::count` (new `crates/biomcp-adapters/src/cellosaurus/count.rs`) sends the search's own query with `format=txt&fields=ac&rows=0` and reads `response.numFound`, only when the header's `query.q` echoes the query sent; anything else is a bad reply.
- Cell-line search: a full page reads the count; `page.total` and Cellosaurus's `sources[].count` are that count, and the page step follows only while rows remain (offset 2352 of 2355 with limit 3 has no step). A short page's total is the rows read to its end, with no extra request. If the count request fails, the source says BioMCP could not read Cellosaurus's match count and the page step still follows. 0873's "gives no total" notice is gone.
- `docs/cautions.md` cell-line paragraph says where the total comes from.
- Cases: `cell-line/search-total.yaml` asserts total 2,355 and source count 2,355 on replies recorded in pm job j41475. `cache-admission/cellosaurus-candidates.yaml` (MOLM13, limit 1) now gets its live count reply (1) from j41475, so it asserts total 1 instead of a page step.
- Checks: j41469 (first cut) found a too-long function, new `.ok()` drops and the adapter file over 300 lines; j41475 passed `make lint-workspace` and the ratchets after the count moved to its own file and kept its errors.
- Gate j41480 failed only the 12 cache-admission cases: their fixture loader reads every file in `tests/replies/cache-admission/` as JSON, so the count reply's `.bin` sidecar broke it. The MOLM13 count reply is now stored inline as a `text` body (the same recorded bytes).
