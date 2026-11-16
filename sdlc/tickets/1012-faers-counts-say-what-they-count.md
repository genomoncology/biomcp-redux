# 1012 — faers-counts-say-what-they-count

Status: OPEN.

Milestone: 7

## Outcome

FAERS count replies say, in a notice, that all names go in one query so a report naming two spellings counts once, and, with `--serious death` and `--reaction`, that these are reports flagged death that list the reaction, not deaths caused by it.

## Why

pt-committee 8.4 and life-sciences-ci 5.2 (bench `personas/pt-committee/runs/2026-11-16/analysis.md`); code mode's own FAERS sums were its worst errors in 0059.

## Proof

- Proof: a replayed case asserting both notices; fails on the base commit.

## Starts from

- Starts from: the analysis named in Why, and the BioMCP code it cites.

## Keeps

- Keeps: every other reply field and command form.

## Defers

- Defers: nothing.

## Changes

- Changes: the named command's code, help, cases and a release-notes line.
