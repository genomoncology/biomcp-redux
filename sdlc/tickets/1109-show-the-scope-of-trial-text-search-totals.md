# 1109 — show-the-scope-of-trial-text-search-totals

Status: OPEN.

Milestone: 7

## Outcome

A trial text search names the registry search mode and text used and explains that its total counts those matches, with guidance for structured acronym and title discovery when available.

## Why

Found by a hand solve of research tasks, 2026-11-20. `biomcp search trial SURMOUNT --limit 50` returned `"total":21`. The first row is a different trial and five registry records found by checking acronym and official title are missing. The reported count is for the registry’s text search.

## Proof

- Proof: Replay a text query and a structured source comparison with different match sets. Assert the text reply states its scope and leaves the registry total unchanged.

## Starts from

- Starts from: `crates/biomcp-adapters/src/ctgov/mod.rs` emits `query.term` and takes the registry total.

## Keeps

- Keeps: The registry’s text matches, ranks and counts remain intact.

## Defers

- Defers: Adding structured selectors belongs to the acronym and official title ticket.

## Changes

- Changes: Trial search reply, help and cases.
