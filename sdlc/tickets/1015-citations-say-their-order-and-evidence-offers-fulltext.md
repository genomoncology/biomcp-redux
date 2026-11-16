# 1015 — citations-say-their-order-and-evidence-offers-fulltext

Status: OPEN.

Milestone: 7

## Outcome

`article citations` says it lists the most-cited citers first and shows each citer's `cited_by`; `article citation-evidence` offers a `--fulltext` next step when Semantic Scholar has no contexts or is unavailable.

## Why

msl 3.1, 3.2, 3.4 (bench `personas/msl/runs/2026-11-16/analysis.md`; `crates/biomcp-query/src/article/order.rs` 76-116).

## Proof

- Proof: replayed cases asserting the order note, `cited_by` and the next step; fail on the base commit.

## Starts from

- Starts from: the analysis named in Why, and the BioMCP code it cites.

## Keeps

- Keeps: every other reply field and command form.

## Defers

- Defers: nothing.

## Changes

- Changes: the named command's code, help, cases and a release-notes line.
