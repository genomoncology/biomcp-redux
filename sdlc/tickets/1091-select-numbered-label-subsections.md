# 1091 — select-numbered-label-subsections

Status: OPEN.

Milestone: 7

## Outcome

A current drug label read can return a named or numbered subsection with its heading and source, and report when the heading is missing or ambiguous.

## Why

Found by a hand solve of research tasks, 2026-11-20. `biomcp get drug Ibrance label --source openfda-label --label-section dosage_and_administration`. The recorded Ibrance dosage read opened at `2.1 Recommended Dosage` and returned 8,104 bytes. Whole dosage sections can be much longer than the requested paragraph.

## Proof

- Proof: A saved dosage section with multiple numbered headings returns just the requested subtree and reports an unknown subsection precisely.

## Starts from

- Starts from: `crates/biomcp-query/src/drug/openfda_label.rs` and the source text admitted by `crates/biomcp-adapters/src/openfda_label/`.

## Keeps

- Keeps: Current section names and whole-section reads remain valid.

## Defers

- Defers: Phrase matching is a separate option.

## Changes

- Changes: `crates/biomcp-query/src/drug/openfda_label.rs`, label command options and cases.
