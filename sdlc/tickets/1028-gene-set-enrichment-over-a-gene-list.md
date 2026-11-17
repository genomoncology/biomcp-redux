# 1028 — gene-set-enrichment-over-a-gene-list

Status: OPEN.

Milestone: 8

## Outcome

A BioMCP command takes a gene list and returns over-represented pathways and terms from a public enrichment service (g:Profiler, or Reactome's analysis service), with the service's own statistics, correction method and background stated, and the service named as the source.

## Why

Milestone 8 lists gene-set enrichment (`sdlc/planning/milestones.md`, section 8; `sdlc/planning/sources-to-add.md:53`). comp-bio Q9 needed it and BioMCP had no command that takes a gene list (bench `personas/comp-bio/runs/2026-11-16/analysis.md` 9.1).

## Proof

- Proof: a replayed case for comp-bio's five genes asserting the top Reactome terms and the stated correction; a live check recorded in Progress.

## Starts from

- Starts from: an existing adapter of the same shape, and the source's own documentation and terms.

## Keeps

- Keeps: every existing command form.

## Defers

- Defers: nothing.

## Changes

- Changes: a new adapter or section, its help, schemas, cases, recordings and a release-notes line.
