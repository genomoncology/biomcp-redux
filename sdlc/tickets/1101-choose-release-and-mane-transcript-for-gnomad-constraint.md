# 1101 — choose-release-and-mane-transcript-for-gnomad-constraint

Status: OPEN.

Milestone: 7

## Outcome

Read constraint for several genes by an explicit gnomAD release and MANE Select transcript, with selectable columns and a clear reply when the release or transcript is unavailable.

## Why

Found by a hand solve of research tasks, 2026-11-20. `biomcp get gene ATM ATR KAT5 constraint` returned `"version":"v4"`. The requested v4.1 values were taken from the versioned table outside BioMCP. Ticket 1078 owns general constraint field selection.

New source: gnomAD release constraint tables, https://gnomad.broadinstitute.org/downloads, gnomAD Consortium at the Broad Institute

## Proof

- Proof: A saved release has different values for a MANE Select and a canonical transcript. Assert the requested release, transcript identifier and metric in a compact multi-gene reply and check the missing-MANE case.

## Starts from

- Starts from: `crates/biomcp-query/src/gene/sections/constraint.rs`, `crates/biomcp-adapters/src/gnomad/` and ticket 1078.

## Keeps

- Keeps: The existing unqualified constraint read retains its default selection.

## Defers

- Defers: Other transcript selection modes.

## Changes

- Changes: The gene constraint command and gnomAD adapter, help and cases.
