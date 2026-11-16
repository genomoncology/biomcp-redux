# 1017 — label-replies-name-application-and-recent-changes

Status: OPEN.

Milestone: 7

## Outcome

A `get drug … label` reply names the label's FDA application number(s) and, when present, shows its recent major changes.

## Why

pt-committee couldn't tell which of 11 ELIQUIS labels it read (1.2, 7.1); msl took KRAZATI's current label for a stale one (4.2) (bench `personas/pt-committee/runs/2026-11-16/analysis.md`, msl's). Follows 1005.

## Proof

- Proof: a replayed case asserting both; fails on the base commit.

## Starts from

- Starts from: the analysis named in Why, and the BioMCP code it cites.

## Keeps

- Keeps: every other reply field and command form.

## Defers

- Defers: nothing.

## Changes

- Changes: the named command's code, help, cases and a release-notes line.
