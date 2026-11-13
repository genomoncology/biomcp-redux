# 0883 — cdx-drug-join-includes-group-labeling

Status: OPEN.

Milestone: 7

## Outcome

A companion-diagnostic search by drug application returns every row of FDA's list that names that application, including the group-labeling table, or says which rows it could not join.

## Why

From the bench 0041/0042 reg-affairs runs (2026-11-13): for osimertinib (TAGRISSO, NDA 208065) BioMCP's `search diagnostic --source fda-cdx --drug-application NDA208065` returns 10 rows, all `individual` (checked on build b87cb3b3e). The raw-tools agent parsed FDA's own list page and found 15 rows naming TAGRISSO, including ONCO/Reveal P200011, MI Cancer Seek P240010 and cobas S031 group labeling (`personas/reg-affairs/runs/2026-11-13-raw/log.md:36`). If the extra rows are group-labeling rows that BioMCP holds but does not join to the application, the reply is silently incomplete for exactly the join reg-affairs called its best demo.

## Proof

- Proof: first compare, live, FDA's list rows naming NDA 208065 and BLA 125514 with BioMCP's rows, and record which are missing and why; then a routine case from a recorded page showing the group rows joined, or a stated problem for any row whose application cannot be read.

## Starts from

- Starts from: `crates/biomcp-adapters/src/fda_cdx/` (`html.rs` group table, `search.rs` `matches`).

## Keeps

- Keeps: exact matching on application number; snapshot identity; existing individual rows.

## Defers

- Defers: biomarker matching for group rows (already reported as a problem).

## Changes

- Changes: `crates/biomcp-adapters/src/fda_cdx/`, `tests/cases/diagnostic/`
