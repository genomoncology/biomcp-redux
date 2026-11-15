# 0979 — name-expanded-article-searches-state-their-totals

Status: OPEN.

Milestone: 7

## Outcome

An article search that queries one source once per drug name (Semantic Scholar, LitSense) says in plain words that the per-name counts overlap and are not added, instead of showing rows read as an unlabelled total.

## Why

0873 review (2026-11-15): when a drug name expands into one request per name, the source appears several times, its count is dropped, and the page total falls back to rows read with nothing labelling it until the end of the window. The item 0873 listed (one source named twice) is now refused as a usage error, so this path was left.

## Proof

- Proof: a routine case with a drug name that expands to several names, on recorded replies (Semantic Scholar replies synthetic, per 0920), asserts the sentence on the first page; it fails on the base commit.

## Starts from

- Starts from: 0873's changes to `crates/biomcp-query/src/article/page.rs` and `article/sources.rs`.

## Keeps

- Keeps: never sum or invent totals; the reply schema.

## Defers

- Defers: nothing.

## Changes

- Changes: article search paging notices, a case.

## Progress

### 2026-11-15

- When drug names make a source take one request per name (Semantic Scholar, LitSense), that source's row now carries a notice on the first page: "Semantic Scholar was searched once per drug name. Its counts per name ("Canonical therapy" 1, "EX-66" 2, "Example brand" 2) overlap, so BioMCP does not add them; the page total counts the distinct articles read." Each name's count is quoted with the term sent; nothing is summed, and the source's `count` stays empty as before. The reply schema is unchanged (an existing `request-guidance` notice).
- Case `tests/cases/article/search/expanded-name-totals.yaml` replays the existing synthetic `search/drug-names` set (its README carries `Synthetic: semantic-scholar`, per 0920): `search article EX-66 --source semantic-scholar` gives page total 3 and the sentence on Semantic Scholar's row. On main the row has no notice, so the case fails there.
- Checks: pm job j41471 ran fmt, `make check-cheap` (passed) and the `article/` and `search/` cases; only the new case's source index needed fixing (Semantic Scholar is row 8, after the name resolvers).
