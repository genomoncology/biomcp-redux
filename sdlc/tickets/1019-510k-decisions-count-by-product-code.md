# 1019 — 510k-decisions-count-by-product-code

Status: OPEN.

Milestone: 7

## Outcome

`search diagnostic --source fda-510k` can count decisions by product code over every code, and says when the sum matches the total.

## Why

life-sciences-ci 6.4: openFDA's facet stops at 100 codes (bench `personas/life-sciences-ci/runs/2026-11-16/analysis.md`).

## Proof

- Proof: a replayed case asserting a count row and the total check; fails on the base commit.

## Starts from

- Starts from: the analysis named in Why, and the BioMCP code it cites.

## Keeps

- Keeps: every other reply field and command form.

## Defers

- Defers: nothing.

## Changes

- Changes: the named command's code, help, cases and a release-notes line.

## Progress

- 2026-11-16, measured openFDA live (no key): `device/510k.json?count=product_code` returns 100 terms when no `limit` is sent (the analyst's curl); `limit=1000` returns 1,000 terms without a key on the device endpoint; `limit=1001` is refused ("Limit cannot exceed 1000 results for count requests"); `skip` with `count` is refused. `count=product_code.exact` says "Nothing to count", so the field is already exact. Decision dates 2024 (the persona's Q6 window): 792 codes summing to 3,129, the search total, with GEX 82 first, as the analyst found. Wider windows pass 1,000 codes: 1990-1999 gives 1,000 codes summing to 40,321 of 44,082; the whole endpoint 151,394 of 176,625.
- Design: as FAERS `--aggregate`, `search diagnostic --source fda-510k --aggregate product-code` sends the search's own expression with `count=product_code&limit=1000` beside the first page. The reply keeps the page and adds `product_code_counts`: `items` (product code and count, most first), `sum`, `sum_matches_total` (the sum equals the page total) and `limit_reached` (openFDA's 1,000 codes came back; narrow the decision dates). Refused for other sources (with a message) and with `--offset`; the page's next step drops the count. Splitting a wide window into date slices to pass 1,000 codes is left out to keep it to one request.
- Recorded in j42077 (no key; `limit=1000` accepted there too): 2024 gives 792 terms, one an empty product code with 1 record. j42079 showed that empty term failed the reply as `bad-reply`, so empty terms are now counted in `without_product_code`, which `sum` includes. Case `tests/cases/diagnostic/fda-510k-product-code-counts.yaml` replays `tests/replies/diagnostic/510k-product-codes/`.
