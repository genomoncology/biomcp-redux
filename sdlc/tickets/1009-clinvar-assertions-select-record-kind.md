# 1009 — clinvar-assertions-select-record-kind

Status: OPEN.

Milestone: 7

## Outcome

`variant assertions --source clinvar` and the ClinVar section take a record-kind filter (vcv, rcv or scv), with its own total, so per-submitter rows arrive alone.

## Why

tumor-board paged eight times for BRCA1 R1699Q's 58 SCVs and lost a page to a row with no record id (bench `personas/tumor-board/runs/2026-11-16/analysis.md` 9.1, calls.md 1694, 1822); both graders named it a gap in both arms.

## Proof

- Proof: a replayed case for clinvar:37636 asking scv rows only, asserting the submitter total; fails on the base commit.

## Starts from

- Starts from: the analysis named in Why, and the BioMCP code it cites.

## Keeps

- Keeps: every other reply field and command form.

## Defers

- Defers: nothing.

## Changes

- Changes: the named command's code, help, cases and a release-notes line.
