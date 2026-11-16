# 1010 — section-counts-use-one-field-name

Status: OPEN.

Milestone: 7

## Outcome

Every section that counts its whole set carries the count under one field name, the one the skill tells agents to read.

## Why

comp-bio read `.data.total` on the complexes section, got nothing, and reported 5 shown rows for WRN's 14 complexes; complexes and domains carry `native_total`, features `total`, and the skill says read `total` (bench `personas/comp-bio/runs/2026-11-16/analysis.md` 10.1; `crates/biomcp-types/src/protein/response.rs` 72-79).

## Proof

- Proof: a schema ratchet that no section has both names; the protein cases updated; release-notes line for the renamed field.

## Starts from

- Starts from: the analysis named in Why, and the BioMCP code it cites.

## Keeps

- Keeps: every other reply field and command form.

## Defers

- Defers: nothing.

## Changes

- Changes: the named command's code, help, cases and a release-notes line.
