# 1011 — empty-sections-name-what-was-asked

Status: OPEN.

Milestone: 7

## Outcome

An empty section states what its source was asked, for example `Reactome lists no pathway for UniProt:P33981`, so an agent reads it as the source's answer, not a join failure.

## Why

comp-bio called TTK's empty Reactome section a join gap (bench `personas/comp-bio/runs/2026-11-16/analysis.md` 3.2); Reactome has no pathway for P33981.

## Proof

- Proof: a replayed case for TTK pathways asserting the stated identifier; fails on the base commit.

## Starts from

- Starts from: the analysis named in Why, and the BioMCP code it cites.

## Keeps

- Keeps: every other reply field and command form.

## Defers

- Defers: nothing.

## Changes

- Changes: the named command's code, help, cases and a release-notes line.
