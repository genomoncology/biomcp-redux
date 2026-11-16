# 1007 — nci-code-examples-name-their-disease

Status: OPEN.

Milestone: 7

## Outcome

Every help, schema and error text that gives an NCIt code as an example names its disease, such as `C3224 (melanoma)`.

## Why

The 2026-11-12 trial-ops run took C3224 from `query/trial.rs:51`'s example as pancreatic cancer; C3224 is melanoma. The bench's fact copied the error (bench `personas/trial-ops/runs/2026-11-16/analysis.md` 9.1).

## Proof

- Proof: a grep test that every `C[0-9]{4,}` example in help text has a disease name beside it; `make docs-check` passes.

## Starts from

- Starts from: the analysis named in Why, and the BioMCP code it cites.

## Keeps

- Keeps: every other reply field and command form.

## Defers

- Defers: nothing.

## Changes

- Changes: the named command's code, help, cases and a release-notes line.
