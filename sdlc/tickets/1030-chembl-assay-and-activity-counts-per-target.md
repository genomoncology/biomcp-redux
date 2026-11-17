# 1030 — chembl-assay-and-activity-counts-per-target

Status: OPEN.

Milestone: 8

## Outcome

A gene or protein section maps the UniProt accession to its ChEMBL target and gives that target's assay and activity totals.

## Why

comp-bio 7.1 (bench `personas/comp-bio/runs/2026-11-16/analysis.md`); only `cell-line assay-count` exists today.

## Proof

- Proof: a replayed case for one of comp-bio's targets asserting both totals.

## Starts from

- Starts from: an existing adapter of the same shape, and the source's own documentation and terms.

## Keeps

- Keeps: every existing command form.

## Defers

- Defers: nothing.

## Changes

- Changes: a new adapter or section, its help, schemas, cases, recordings and a release-notes line.
