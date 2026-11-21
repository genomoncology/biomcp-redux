# 1104 — find-companion-diagnostics-by-regimen-partner

Status: COMPLETE.
Landed: f436c26484a484723127e9e84296b64fa7b2c4af

Milestone: 7

## Outcome

Find FDA companion diagnostic rows by an explicitly named regimen partner alongside the existing application and drug/test name searches, and label each match by how it was found.

## Why

Found by a hand solve of research tasks, 2026-11-20. `biomcp search diagnostic --source fda-cdx --drug-application NDA207103 --limit 50` returned `"total":0`. Rows for the combination are indexed to another application. Ticket 1084 covers compact output and drug or test names, and this adds regimen-partner lookup.

## Proof

- Proof: Replay rows where a combination partner appears in a regimen but has no application-key match. The partner lookup returns the sourced rows with match basis, and application-only lookup keeps its existing scope.

## Starts from

- Starts from: `crates/biomcp-query/src/diagnostic/companion.rs` and `crates/biomcp-adapters/src/fda_cdx/`.

## Keeps

- Keeps: FDA text and application-key semantics remain intact.

## Defers

- Defers: Inferring eligibility, specimen or variant coverage from regimen wording.

## Changes

- Changes: Companion diagnostic filters and row matching, help and cases.
