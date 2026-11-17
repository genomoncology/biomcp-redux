# 1031 — study-copy-number-counts-per-gene

Status: OPEN.

Milestone: 8

## Outcome

`get study ID` has a section that, for given genes, counts amplifications and deep deletions from the study's discrete copy-number (GISTIC) profile.

## Why

comp-bio 8.3 (bench `personas/comp-bio/runs/2026-11-16/analysis.md`); the GISTIC profile is listed but can't be queried.

## Proof

- Proof: a replayed case for brca_tcga asserting counts for one amplified gene.

## Starts from

- Starts from: an existing adapter of the same shape, and the source's own documentation and terms.

## Keeps

- Keeps: every existing command form.

## Defers

- Defers: nothing.

## Changes

- Changes: a new adapter or section, its help, schemas, cases, recordings and a release-notes line.
