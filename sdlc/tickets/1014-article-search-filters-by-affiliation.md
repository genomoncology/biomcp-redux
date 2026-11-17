# 1014 — article-search-filters-by-affiliation

Status: OPEN.

Milestone: 7

## Outcome

`search article` takes `--affiliation`, sent as PubMed `[ad]` and Europe PMC `AFF:`.

## Why

msl had to tally institutions by hand from 10-affiliation indexing (bench `personas/msl/runs/2026-11-16/analysis.md` 5.1-5.3).

## Proof

- Proof: replayed cases for both sources; fails on the base commit.

## Starts from

- Starts from: the analysis named in Why, and the BioMCP code it cites.

## Keeps

- Keeps: every other reply field and command form.

## Defers

- Defers: nothing.

## Changes

- Changes: the named command's code, help, cases and a release-notes line.

## Progress

- Live syntax check, 2026-11-16 (curl): PubMed `Dana-Farber[ad]` translates to `"Dana-Farber"[Affiliation]` (45,896); quoted and unquoted forms give equal counts for plain names, but unquoted `Harvard OR Mayo[ad]` sends `harvard` to All Fields. Europe PMC `AFF:Dana-Farber Cancer Institute` binds only the first word (50,388 vs 48,136 for the quoted phrase).
- So `--affiliation` is sent as one quoted phrase: PubMed `"VALUE"[ad]` (double quotes in the value dropped), Europe PMC `AFF:` through the same quoting helper as `--author` and `--journal`. LitVar, LitSense, PubTator and Semantic Scholar refuse it with `usage.unsupported_filter` before any request. It may be the only selector, like `--journal`.
- Request schemas gain the optional `affiliation` field and its selector branch, and `ArticleFilter` gains `--affiliation`; both are widenings.
- pm job j42026 regenerated schemas and docs (fmt changed nothing) and recorded PubMed live: `search article "KRAS G12C" --affiliation "Dana-Farber Cancer Institute" --since 2023 --until 2025` gave 28 papers. Its Europe PMC call missed the deadline once; the same URL by curl answered in 0.9 s with 91 hits, so it was re-recorded in the next job.
- New case `article/search/affiliation`: the PubMed and Europe PMC searches with their exact terms and totals, and Semantic Scholar's refusal. On the base commit `--affiliation` is an unknown option, so the case fails there.
