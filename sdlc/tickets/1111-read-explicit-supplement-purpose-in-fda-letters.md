# 1111 — read-explicit-supplement-purpose-in-fda-letters

Status: COMPLETE. Built and landed on the 1105 batch branch (ticket/1105-show-approval-letter-ids-in-compact-histories, 22d80088f): tests/cases/drug/supplement-purpose-changes.yaml returns the letter's change list and cites letter 75677

Milestone: 7

## Outcome

An approval-document read returns the purpose explicitly stated for the same supplement by an FDA letter with variant wording or a list of changes, and cites that letter.

## Why

Found by a hand solve of research tasks, 2026-11-20. `biomcp get drug BLA761118 approvals --source drugsfda --document-id 75677 --limit 10` returned `No complete supplement-purpose statement could be read from the linked FDA letter; no indication was inferred.`. The returned letter text contains an explicit paragraph introducing the proposed changes, followed by a list, which the existing phrase rule misses.

## Proof

- Proof: Use a small synthetic fixture for BLA761118 supplement 007 matching the letter’s heading and listed changes. Assert that the linked source text supplies the purpose and that a letter for another supplement or with no explicit statement still gives no purpose.

## Starts from

- Starts from: `crates/biomcp-query/src/drug/drugsfda/purpose.rs`. The current rule expects `provides for` and specific closing markers.

## Keeps

- Keeps: The text stays attributed to its exact letter and claims are never inferred from a label.

## Defers

- Defers: Reworking the PDF reader or the Purple Book roster.

## Changes

- Changes: Drugs@FDA supplement-purpose reading and cases.
