# 0843 — drug-names-answer-without-waiting-on-chembl

Status: OPEN.

Milestone: 7

## Outcome

A trial or article search naming a real drug gets its widened names without waiting on live ChEMBL requests, so the name step costs well under a second whether or not ChEMBL is slow.

## Why

The drug-name lookup ticket (0832) made an unfinished lookup harmless to the reply's status and skipped ChEMBL when MyChem has no row. It left one wait: a real drug name MyChem cannot settle (imatinib, pembrolizumab) still waits up to 6.4 seconds on three sequential ChEMBL requests, and a timed-out ChEMBL request is not cached, so a repeat pays again. Its review confirmed both.

## Proof

- Proof: first measure, on live sources, how often a real drug name reaches ChEMBL and what it waits; put the numbers here before choosing.
- Proof: a routine case with ChEMBL stalled shows a named-drug search widened and complete in under a second; it fails on the base commit.

## Starts from

- Starts from: `crates/biomcp-query/src/drug/search_names.rs`, the cause written in ticket 0832, and its review.
- PM decision: 2026-11-11. Candidates, cheapest first: cache a timed-out or negative ChEMBL answer for a bounded time; make the three ChEMBL requests one or run them together; hold ChEMBL's drug names locally as a data asset refreshed on a schedule. A local copy is a new data asset with its own `sdlc/data/` entry and a licence read (ChEMBL is CC BY-SA 3.0, which hosted use must attribute); take that route only if the cheaper ones do not remove the wait.

## Keeps

- Keeps: no result presented as widened when it was not; names frozen across pages; a real failure of the searched source still reports partial or failed.

## Defers

- Defers: whether a name source that was asked and timed out should read `failed` rather than `skipped` while the reply stays `complete`. The review raised it; it changes the shared status rule and needs a design review first. Until then `skipped` with its reason stands.

## Changes

- Changes: `crates/biomcp-query/src/drug/`, `crates/biomcp-fetch/src/`, `tests/cases/search/drug-names/`
