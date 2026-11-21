# 1092 — read-dailymed-saved-label-versions

Status: WITHDRAWN. The motivating case (NDA215309 boxed warning, MACE) now reads the original label through Drugs@FDA on main 30c84d372: complete, five versions read, first found in ORIG 1 approved 2021-09-21 (pm job j44478). Refile DailyMed saved versions if a later run shows a label-history read it cannot bound.

Milestone: 7

## Outcome

A label-history read can list and read saved DailyMed SPL versions for a label set and search a selected section with the version, published date and source link attached to each match.

## Why

Found by a hand solve of research tasks, 2026-11-20. `biomcp get drug NDA215309 label-history --source drugsfda --label-section boxed_warning --find "major adverse cardiovascular events" --find MACE --every-version` returned `the text appeared on or before 2022-07-18`. The original is available as a saved DailyMed SPL version. Ticket 1082 fixes boundary detection in FDA PDFs.

New source: DailyMed saved SPL versions, https://dailymed.nlm.nih.gov/dailymed/app-support-web-services.cfm, U.S. National Library of Medicine

## Proof

- Proof: A saved SPL history includes the original, a later match and an unreadable version. The reply distinguishes each and uses no FDA approval date unless independently supplied by the source.

## Starts from

- Starts from: `crates/biomcp-query/src/drug/openfda_label.rs`, ticket 1066’s history design and DailyMed’s `/spls/{SETID}/history` listing and versioned SPL ZIP download.

## Keeps

- Keeps: The Drugs@FDA search keeps its separate source, dates and uncertainty.

## Defers

- Defers: Joining DailyMed publication dates to FDA approval actions.

## Changes

- Changes: Drug label history, DailyMed adapter, help and cases.
