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
