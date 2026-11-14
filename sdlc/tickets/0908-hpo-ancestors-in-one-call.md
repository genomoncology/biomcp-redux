# 0908 — hpo-ancestors-in-one-call

Status: OPEN.

Milestone: 7

## Outcome

A counselor's coding-consistency check, "is infantile spasms already under seizure?", takes one call. A phenotype card can return the term's full ancestor list, and that still works when Monarch is down.

## Why

In rare-disease round 2 (biomed-agent-bench `personas/rare-disease/runs/2026-11-13-r2/`, report Q4 and Q5), the agent had to make four calls to climb HP:0012469 Infantile spasms → HP:0011097 → HP:0020219 → HP:0001250, because BioMCP returns one hop per call. HPO hierarchy and text search are served by Monarch only, which returned HTTP 502 for part of that session, so those calls failed.

The agent's curl to EBI's OLS4 returned every ancestor of a term in one request each. BioMCP already uses OLS for disease search (MONDO).

## Proof

- Proof: recorded cases showing that:
  - `get phenotype HP:0012469` with an ancestors section returns the full ancestor chain to HP:0000118, with labels, in one call;
  - the same request, with Monarch replaying as unavailable, is answered from OLS and says which source served it.
  Both values must match the recorded OLS reply.

## Starts from

- Starts from:
  - the phenotype card and its hierarchy section;
  - BioMCP's existing OLS adapter for MONDO;
  - OLS4's `ancestors` endpoint for the HP ontology.

## Keeps

- Keeps: the one-hop parents and children sections; Monarch as the first source where it answers.

## Defers

- Defers: HPO text search through OLS, its own question. Round 2 showed "floppy baby" finds nothing in either OLS or Monarch.

## Changes

- Changes:
  - the phenotype query and its OLS adapter path;
  - `crates/biomcp-types/` (an added section or field);
  - `tests/cases/phenotype/`;
  - `docs/`.
