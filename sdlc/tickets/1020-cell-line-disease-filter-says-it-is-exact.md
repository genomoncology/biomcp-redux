# 1020 — cell-line-disease-filter-says-it-is-exact

Status: OPEN.

Milestone: 7

## Outcome

`search cell-line --disease` says in a notice that Cellosaurus matches the exact disease name, so narrower terms are separate searches.

## Why

life-sciences-ci 10.3 (bench `personas/life-sciences-ci/runs/2026-11-16/analysis.md`).

## Proof

- Proof: a replayed case asserting the notice; fails on the base commit.

## Starts from

- Starts from: the analysis named in Why, and the BioMCP code it cites.

## Keeps

- Keeps: every other reply field and command form.

## Defers

- Defers: nothing.

## Changes

- Changes: the named command's code, help, cases and a release-notes line.
