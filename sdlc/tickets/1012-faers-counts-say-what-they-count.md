# 1012 — faers-counts-say-what-they-count

Status: OPEN.

Milestone: 7

## Outcome

FAERS count replies say, in a notice, that all names go in one query so a report naming two spellings counts once, and, with `--serious death` and `--reaction`, that these are reports flagged death that list the reaction, not deaths caused by it.

## Why

pt-committee 8.4 and life-sciences-ci 5.2 (bench `personas/pt-committee/runs/2026-11-16/analysis.md`); code mode's own FAERS sums were its worst errors in 0059.

## Proof

- Proof: a replayed case asserting both notices; fails on the base commit.

## Starts from

- Starts from: the analysis named in Why, and the BioMCP code it cites.

## Keeps

- Keeps: every other reply field and command form.

## Defers

- Defers: nothing.

## Changes

- Changes: the named command's code, help, cases and a release-notes line.

## Progress

- FAERS's drug-name notice now reads "one FAERS query, so a report naming several counts once" when other names were searched (`adverse_event/names.rs`).
- A FAERS search with `--serious death` and `--reaction` adds a request-guidance notice on FAERS: "Counts reports flagged death that list this reaction, not deaths caused by it." (`adverse_event.rs`). No new reply field; help already says the flag is never a reaction outcome.
- Full gate (landing-gate, docs-check, schema-release-check) passed at 8e1e2faa7 in job j41976.
- Pre-gate job j41969 (fmt, docs and schema regenerate, check-cheap, adverse-event cases) passed; only formatting changed.
- Cases: `faers-widened-names` asserts the new drug-name wording; `faers-names-by-default` asserts the death notice on the widened, typed-only and two-name pembrolizumab myocarditis death searches,, on the replies recorded in job j38466. Release-notes line added.
