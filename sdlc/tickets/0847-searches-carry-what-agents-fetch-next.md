# 0847 — searches-carry-what-agents-fetch-next

Status: COMPLETE.
Landed: 1f20bac05c47eaa2576456e60efa99228dd09904

Milestone: 7

## Outcome

Search rows for device premarket approvals, WHO prequalified diagnostics and trials carry the few detail fields agents fetched one record at a time in bench 0034, so a question about a set of records needs one search instead of a search plus a get per row. Each added field is named in the search's help.

## Why

`sdlc/records/2026-11-11-token-gap-root-cause.md`: over two typed repeats agents made 81 PMA gets, 52 WHO diagnostic gets and 28 trial-results gets, loop after loop, each a turn. Fewer turns is the lever that let the command line beat curl by 33% on the GPT model.

## Proof

- Proof: before building, list from the 0034 records which fields agents read from those gets; routine cases show each search row carrying them, and a size table shows the per-row growth.

## Starts from

- Starts from: the 0034 run records on the build machine (`biomed-agent-bench`), the PMA, WHO diagnostic and trial search row builders.

## Keeps

- Keeps: rows stay short: add only fields agents fetched in loops, and note per-row bytes before and after.

## Defers

- Defers: other entities until the records show the same loop.

## Changes

- Changes: `crates/biomcp-adapters/src/`, `crates/biomcp-query/src/`, `tests/cases/`
