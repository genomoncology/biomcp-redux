# 0967 — de-novo-rows-show-the-decision

Status: OPEN.

Milestone: 7

## Outcome

A de novo device row shows the decision a reader needs: BioMCP's `de-novo-granted` status is the field an agent sees first, and openFDA's "Unknown" decision value is either labeled as openFDA's or left out where BioMCP knows better.

## Why

dx-lab-director round 4 (2026-11-15): de novo rows show openFDA's "Unknown" next to BioMCP's "de-novo-granted", and the agent reported "Unknown" (calls 33, 34; log.md:29). Evidence: bench `personas/dx-lab-director/runs/2026-11-15-r4/` (job j41109, BioMCP 95a74e246, bench 0051).

## Proof

- Proof: a recorded de novo search case shows the row's decision as granted and no bare "Unknown" an agent would read as the decision.

## Starts from

- Starts from: the 510(k)/de novo row mapping in the fda-510k adapter; round 4 calls 33-34.

## Keeps

- Keeps: the source's value stays available if the schema requires it; the 1.0.0 schema rules.

## Defers

- Defers: nothing.

## Changes

- Changes: the de novo row mapping, its case.
