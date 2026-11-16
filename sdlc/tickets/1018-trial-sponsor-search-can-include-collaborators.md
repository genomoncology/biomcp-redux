# 1018 — trial-sponsor-search-can-include-collaborators

Status: OPEN.

Milestone: 7

## Outcome

`search trial --sponsor` can match any sponsor role, lead or collaborator, by an option, mapped to ClinicalTrials.gov's `query.spons`.

## Why

life-sciences-ci 1.2 (bench `personas/life-sciences-ci/runs/2026-11-16/analysis.md`; `crates/biomcp-adapters/src/ctgov/filters.rs` 77-79).

## Proof

- Proof: a replayed case asserting a collaborator trial is found; fails on the base commit.

## Starts from

- Starts from: the analysis named in Why, and the BioMCP code it cites.

## Keeps

- Keeps: every other reply field and command form.

## Defers

- Defers: nothing.

## Changes

- Changes: the named command's code, help, cases and a release-notes line.
