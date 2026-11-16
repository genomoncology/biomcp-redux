# 1013 — ddinter-replies-count-by-level

Status: OPEN.

Milestone: 7

## Outcome

A DDInter interactions reply gives counts by level (major, moderate, minor) over the whole set, and its `next` offers the label's `drug_interactions` section.

## Why

pt-committee reported 323 rows but not 117/200/6 (bench `personas/pt-committee/runs/2026-11-16/analysis.md` 1.3, 4.4).

## Proof

- Proof: a replayed case for apixaban asserting the level counts; fails on the base commit.

## Starts from

- Starts from: the analysis named in Why, and the BioMCP code it cites.

## Keeps

- Keeps: every other reply field and command form.

## Defers

- Defers: nothing.

## Changes

- Changes: the named command's code, help, cases and a release-notes line.
