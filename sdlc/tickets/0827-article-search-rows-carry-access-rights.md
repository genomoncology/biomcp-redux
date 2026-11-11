# 0827 — article-search-rows-carry-access-rights

Status: OPEN.

Milestone: 8

## Outcome

Each `search article` row says whether the paper is open access and, where the source states one, its licence, so a reader can tell what may be shared without opening each paper.

## Why

The liaison persona's sharing-rights question was answered from another service's batch call; BioMCP knows the licence only inside `get article ... pdf` and full text.

## Proof

- Proof: a routine case under `tests/cases/article/` asserts the two fields on a recorded page holding one open and one closed paper; a row whose source gives no licence shows none.

## Starts from

- Starts from: `ArticleHeader` in `crates/biomcp-types`, `crates/biomcp-query/src/article/sections/pdf.rs` (`PdfLink` with status and licence) and `crates/biomcp-adapters/src/article_rights/`. Europe PMC search rows carry `isOpenAccess` and `license`.

## Keeps

- Keeps: every existing field, schema ID rule (a changed reply schema takes a new major), source attribution and page behavior; no new source, key or dependency. No extra request per row: use what the search response already holds, and say nothing where it holds nothing.

## Defers

- Defers: licence lookup for rows whose source omits it waits for OpenAlex.

## Changes

- Changes: `crates/biomcp-adapters/src/europepmc/`, `crates/biomcp-types/src/article/`, `crates/biomcp-query/src/article/`, `tests/cases/article/`

## Amendment, 2026-11-11

- Amendment (developer, 2026-11-11): rows carry `open_access` only. Europe PMC's `lite` search page, which BioMCP sends, has `isOpenAccess` on every row and `license` on none (100-row BRAF V600E page: 0 licences). Only `resultType=core` carries `license` (73 of 100 rows), but it also carries whole abstracts: about 8 times the bytes (770 KB against 87 KB a page) and, in eight cold samples, 1.2 to 32 seconds against about 1 second for `lite`, two of them past the 15-second Europe PMC deadline. Switching would break the Keeps on page behavior and would need an abstract-rights filter on search pages, so row licences wait with the OpenAlex deferral above.
- Amendment (reviewer, 2026-11-11): the three article reply schemas gain an optional `open_access` property and keep their major. The schema check treats an optional addition as compatible within a major and requires a new major only for removals, renames, retyping or narrowing (`crates/biomcp-types/tests/schemas/compatibility.rs` `check_reviewed`, case "optional addition" in `compatibility_cases.rs`), so the Keeps phrase "a changed reply schema takes a new major" covers breaking changes only.
