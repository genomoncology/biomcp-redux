# 0846 — several-sections-in-one-get

Status: OPEN.

Milestone: 7

## Outcome

One get returns several sections, and one drug read returns several label sections, in a single merged reply, on the command line and in typed MCP mode (a list for `section` and for label sections). Each section keeps its own paging, and the reply says which section each part came from. "Choose one section per call" is no longer an error.

## Why

`sdlc/records/2026-11-11-token-gap-root-cause.md`: in bench 0034, typed mode made 70 drug-label calls of one label section each where curl fetched six sections in one call, and BioMCP made about four calls for each curl command it replaced. Every extra call is a turn that re-reads the whole conversation. The 0834 developer found this needs merged section reads with new paging rules, too large for that ticket.

## Proof

- Proof: routine cases read a drug with three label sections and a gene with two sections in one call, in CLI and typed form, and assert one reply holding each part with its paging; an unknown section in the list is a usage error naming it and the valid ones.

## Starts from

- Starts from: drug request preparation (the "choose one section per call" refusal), `--section-limit`/`--section-offset`, `--label-section`, and the typed `section` argument after 0833 and 0834.

## Keeps

- Keeps: a single section's reply is unchanged; the typed tool list stays within 1% of main.

## Defers

- Defers: all sections by default.

## Changes

- Changes: `crates/biomcp-query/src/drug/`, `crates/biomcp/src/grammar.rs`, `crates/biomcp/typed_front_map.rs`, `tests/cases/`
