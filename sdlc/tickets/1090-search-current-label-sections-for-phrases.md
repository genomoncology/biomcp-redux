# 1090 — search-current-label-sections-for-phrases

Status: OPEN.

Milestone: 7

## Outcome

Search literal text in one or more selected sections of a current label and return bounded excerpts with paragraph or table context and label identity. A batch of known labels can be read together.

## Why

Found by a hand solve of research tasks, 2026-11-20. `biomcp get drug Keytruda label --source openfda-label --label-section clinical_studies --find KEYNOTE-189` returned `error.option is not accepted with the other options as given`. The report paged nine clinical-studies replies to find a few lines. Across current-label reads, whole sections were also returned for simple presence checks.

## Proof

- Proof: A saved label returns a phrase across a fragment boundary and the matching table row with its header where the source preserves the structure. The batch cites each label and tells a readable section with no match from a section that could not be read.

## Starts from

- Starts from: `crates/biomcp-query/src/drug/openfda_label.rs` and ticket 1066’s literal label-history matching.

## Keeps

- Keeps: Full-section reads and history calls retain their current behavior.

## Defers

- Defers: Discovery by mechanism belongs to ticket 1086.

## Changes

- Changes: `crates/biomcp-query/src/drug/openfda_label.rs`, label command options and cases.
