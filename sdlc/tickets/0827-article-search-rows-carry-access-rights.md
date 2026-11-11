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
