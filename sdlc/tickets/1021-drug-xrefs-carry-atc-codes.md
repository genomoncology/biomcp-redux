# 1021 — drug-xrefs-carry-atc-codes

Status: OPEN.

Milestone: 7

## Outcome

`get drug X xrefs` gives the drug's ATC codes.

## Why

pt-committee 9.3 (bench `personas/pt-committee/runs/2026-11-16/analysis.md`); MyChem's card fields omit `chembl.atc_classifications`.

## Proof

- Proof: a replayed case for apixaban asserting B01AF02; fails on the base commit.

## Starts from

- Starts from: the analysis named in Why, and the BioMCP code it cites.

## Keeps

- Keeps: every other reply field and command form.

## Defers

- Defers: nothing.

## Changes

- Changes: the named command's code, help, cases and a release-notes line.
