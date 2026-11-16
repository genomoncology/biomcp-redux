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
