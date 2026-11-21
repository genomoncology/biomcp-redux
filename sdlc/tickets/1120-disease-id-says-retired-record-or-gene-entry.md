# 1120 — disease-id-says-retired-record-or-gene-entry

Status: COMPLETE.
Landed: 2686e30a559f3de84f6a2fdc738acdc8b5b36e3e

Milestone: 7

## Outcome

`biomcp get disease ID` given an Orphanet, OMIM or DOID number never silently answers with a different kind of record. When the number resolves to a retired record, the reply says it is retired and names its replacement where the source gives one. When an OMIM number is a gene entry, not a disease, the reply says so and offers the gene lookup.

## Why

Raised by the review of ticket 1062 (2026-11-19), which made `get disease` resolve these numbers to a MONDO record. OMIM numbers genes and diseases in one series, and MONDO keeps retired records. A caller who gets a record back without either fact stated can report the wrong disease. PM decision, 2026-11-20, with Ian's agreement ("I'm good with whatever recommendations you have around OMIM and genes"): state the fact; do not guess a substitute.

## Proof

- Proof: a replayed case for an identifier that resolves to a retired MONDO record: the reply marks it retired and gives the replacement the source names; fails on the base commit.
- Proof: a replayed case for an OMIM gene entry given to `get disease`: the reply says it is a gene entry and its next step is the gene lookup; fails on the base commit.

## Starts from

- Starts from: ticket 1062's change in the disease get path. The developer first finds one real identifier of each kind and shows what main returns for it; if main already states the fact, say so and close that half.
- Ticket 1100 covers the third case, a number that maps to a broader or narrower record.

## Keeps

- Keeps: an exact, current mapping answers as it does today.

## Defers

- Defers: nothing.

## Changes
