# 1105 — show-approval-letter-ids-in-compact-histories

Status: OPEN.

Milestone: 7

## Outcome

Return document IDs and their kinds in a compact approval-history row without requiring the rest of the document metadata or letter text.

## Why

Found by a hand solve of research tasks, 2026-11-20. `biomcp get drug BLA761118 approvals --source drugsfda --limit 200 --fields=submission_number,event,supplement_purpose` returned `--section-limit above 50 reads no supplement letters, so rows carry no supplement_purpose.`. The selected columns also omit the document IDs needed to open individual letters. The full `application_docs` column can return IDs but carries other document data.

## Proof

- Proof: Replay approval rows with several linked document kinds. Check that the compact selection preserves their IDs, is smaller than a reply selecting all of `application_docs`, and each letter ID opens the same document.

## Starts from

- Starts from: `crates/biomcp-query/src/drug/drugsfda/approvals.rs` and `crates/biomcp/src/fields/`.

## Keeps

- Keeps: Full document reads and explicit `application_docs` selection remain available.

## Defers

- Defers: Extracting purpose from readable letters.

## Changes

- Changes: Approval-history output fields, help and cases.
