# 0969 — variant-hgvs-refusal-suggests-gene-coding

Status: OPEN.

Milestone: 7

## Outcome

When `search variant --variant` gets a full HGVS name it does not take, the refusal shows the runnable `--gene ... --coding ...` form built from that name, not only a pointer to help.

## Why

dx-lab-director round 4 (2026-11-15): the refusal sent the agent to help and cost 2 extra calls (calls 73-75). Evidence: bench `personas/dx-lab-director/runs/2026-11-15-r4/` (job j41109, BioMCP 95a74e246, bench 0051).

## Proof

- Proof: a case: `search variant --variant` with the persona's HGVS name refuses with the `--gene`/`--coding` command an agent can run as is.

## Starts from

- Starts from: the variant search argument checks and refusal wording; 0956 and 0957 for the refusal-shows-the-form pattern.

## Keeps

- Keeps: what `--variant` accepts; the refusal schema.

## Defers

- Defers: nothing.

## Changes

- Changes: variant search refusal, its case.
