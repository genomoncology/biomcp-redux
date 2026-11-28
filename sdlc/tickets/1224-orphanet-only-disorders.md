# 1224 — orphanet-only-disorders

Status: COMPLETE.
Landed: 1f3e7845a5a06e2b28e5d2e307a4367b68b0293b

Milestone: 10

## Outcome

`get disease ORPHA:<code>` returns a disease card from Orphadata for a disorder that has no MONDO mapping, and its `prevalence` section works from the Orphanet code alone.

## Why

1,715 of Orphadata's 11,645 disorders (14.7%) have no MONDO mapping and fail today.

## Evidence

Checked 2026-11-27 against the live source and the 1.0.0 build at 4efd1dcb2426. Full tables: [source reach census](../records/2026-11-27-source-reach-census.md).

- Failing case: `biomcp get disease ORPHA:699029` returns `status: failed`, MyDisease `not-found`. Orphadata holds the disorder.
- Ticket 1062 made Orphanet IDs work when a mapping exists.
- Related: `biomcp get disease MONDO:0020668 phenotypes --source monarch` (an obsolete MONDO ID) returns `complete` with 0 rows though the card names its replacement, MONDO:0010047, which has 51 phenotypes.

## Proof

- Proof: outside-in cases on recorded Orphadata files: ORPHA:699029 returns its name, classification and prevalence; the obsolete MONDO ID's sections follow the replacement or say the ID is obsolete.
- Proof: the Orphadata census, a scheduled job, runs every disorder code of a pinned release against the mapping and holds the share with a card as a floor.

## Starts from

- Starts from: `crates/biomcp-query/src/disease/orphadata.rs`, `crates/biomcp-adapters/src/orphadata/`, `crates/biomcp-adapters/src/mydisease/`.

## Keeps

- Keeps: disorders with a MONDO mapping keep their merged card.

## Defers

- Defers: Orphadata natural history and functional consequences products, which have no route.

## Changes

- Changes: `crates/biomcp-query/src/disease/`, `crates/biomcp-adapters/src/orphadata/`, disease cases.
