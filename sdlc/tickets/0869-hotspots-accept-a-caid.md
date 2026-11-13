# 0869 — hotspots-accept-a-caid

Status: COMPLETE.
Landed: 9927c3f3326832406aeac29c1b5a648e3f7d1035

Milestone: 7

## Outcome

`get variant CA123643 ... hotspots` returns the hotspots section for a ClinGen CAid, as the section's own message says it accepts, or the message stops claiming it.

## Why

Bench 0040's rerun (failure #7 in `sdlc/records/2026-11-12-persona-rerun-failures.md`, tumor-board log :303-306): the section is `unavailable`, `no-join-id`, with a problem saying it requires "one exact source-qualified allele (CAid or reference-qualified genomic HGVS)" while the input is a CAid.

## Proof

- Proof: a routine case with recorded replies gets BRAF V600E by CAid with the hotspots section and returns the same hotspot rows as the genomic HGVS input.

## Starts from

- Starts from: the hotspots section's join in `crates/biomcp-query/src/variant/`; the CAid resolution other sections use.

## Keeps

- Keeps: genomic HGVS input unchanged.

## Defers

- Defers: nothing.

## Changes

- Changes: `crates/biomcp-query/src/variant/`, `tests/cases/variant/`
