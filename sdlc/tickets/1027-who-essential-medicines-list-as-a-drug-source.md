# 1027 — who-essential-medicines-list-as-a-drug-source

Status: OPEN.

Milestone: 8

## Outcome

`get drug NAME` can read the WHO Model List of Essential Medicines: whether the drug is listed, core or complementary, its section, the edition it was added, therapeutic alternatives, and the Expert Committee's text, from the WHO eEML's own published data, with its licence checked before any reply data is committed.

## Why

Milestone 8 lists "the WHO essential medicines list" (`sdlc/planning/milestones.md`, section 8; `sdlc/planning/sources-to-add.md:52`). The pt-committee run curled list.essentialmeds.org and could not tell core from complementary, missing four facts (bench `personas/pt-committee/runs/2026-11-16/analysis.md` 5.1-5.3, 10.4).

## Proof

- Proof: a replayed case for dabigatran and warfarin asserting list status, core or complementary and edition; a live check recorded in Progress.

## Starts from

- Starts from: an existing adapter of the same shape, and the source's own documentation and terms.

## Keeps

- Keeps: every existing command form.

## Defers

- Defers: nothing.

## Changes

- Changes: a new adapter or section, its help, schemas, cases, recordings and a release-notes line.
