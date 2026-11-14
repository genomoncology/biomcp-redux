# 0937 — skill-reads-totals-and-uses-pair-lookup

Status: OPEN.

Milestone: 7

## Outcome

The BioMCP skill tells agents to read a page's total before reporting a count or a list as complete, to use the DDInter pair lookup for a named pair, to ask for several sections in one get, and to stop after a missing-key refusal. Replies that are one page of more already say so; check the DDInter and CPIC ones do so early in the reply.

## Why

The persona reported "5 interaction records" for clopidogrel from DDInter's first page (DDInter holds 614 partners; omeprazole and esomeprazole are Major) and CPIC's first page as the whole (log.md:254-261); it retried a no-key NCI search four times (log.md:147) and read three sections as five calls (log.md:61-63). Evidence: bench `personas/dx-lab-director/runs/2026-11-14/` (j40302, BioMCP f4f726674) and its code-mode pair `runs/2026-11-14-raw/` (j40303); the analyst's source checks are listed at the end of `analysis.md`. This is the paper's W2 (narrow replies read as complete).

## Proof

- Proof: the skill's text names each step; a DDInter drug reply shows its total before its first row (measure where it sits today).

## Starts from

- Starts from: the skill in `skills/` and 0922's command map; the DDInter reply shape in `crates/biomcp-adapters/src/ddinter/` (or its query crate).

## Keeps

- Keeps: 0922's try-before-help guidance; skill size within its cap.

## Defers

- Defers: nothing.

## Changes

- Changes: the skill text; reply field order if the total sits late.
