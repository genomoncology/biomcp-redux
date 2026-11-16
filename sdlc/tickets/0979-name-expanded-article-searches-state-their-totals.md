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
- Gate `make --keep-going landing-gate docs-check schema-release-check` passed on the branch merged with origin/main (94395bcad) in pm job j41476 at c6e1b687b.

### Review fix, 2026-11-15

- With Europe PMC (or PubMed) plus Semantic Scholar on a drug name, the merged per-name Semantic Scholar row has no rows-read number, so 0873's filter in `source_total` did not count it as searched; the total became Europe PMC's own count (1,178) while the new sentence said it counts the articles read. `source_total` now takes the searched sources from the query's own source list (`Cursor::sources`), so name and title lookups stay out and a merged per-name row stays in; the total falls back to the articles held.
- Live check, pm job j41491: `search article tebentafusp --source europepmc --source semantic-scholar --limit 3` gave total 260 (the pool) with Europe PMC's 1,178 on its own row. j41491 also passed `make check-cheap` and the `article/` cases.
- Case `article/search/expanded-name-window.yaml` replays `tests/replies/article/search/expanded-name-window`: MyChem, Europe PMC and PubMed copied from the live `europepmc-window` set, and four synthetic Semantic Scholar replies at the per-name URLs j41491 sent, each stating an invented total and no papers (README has `Synthetic: semantic-scholar`). It asserts total 100 and the sentence; before the fix the total was 1,178.
- Review-fix gate passed on the branch merged with origin/main (94395bcad) in pm job j41494 at 1090be54c.
