# 1093 — search-withdrawn-cancer-accelerated-approvals

Status: COMPLETE. Built and landed on the 1088 batch branch (460df3f08)

Milestone: 7

## Outcome

Search FDA’s withdrawn cancer accelerated approvals by drug, indication text or withdrawal date and return one row per indication with both dates and the FDA link.

## Why

Found by a hand solve of research tasks, 2026-11-20. `biomcp search drug "withdrawn cancer accelerated approvals" --source drugsfda` returned `"total":0`. Application histories cannot supply the indication-level rows FDA publishes in this table.

New source: Withdrawn Cancer Accelerated Approvals, https://www.fda.gov/drugs/cancer-approvals/withdrawn-cancer-accelerated-approvals, U.S. Food and Drug Administration

## Proof

- Proof: A saved FDA table with two withdrawals for one drug returns both rows with their own indication text and approval and withdrawal dates. A first read returns rows without a manual sync.

## Starts from

- Starts from: Ticket 1067’s stored FDA web-list design.

## Keeps

- Keeps: FDA’s indication wording and dates are preserved.

## Defers

- Defers: Comparing withdrawals to current labeling.

## Changes

- Changes: Drug search, a stored FDA withdrawal-list reader, help and cases.
