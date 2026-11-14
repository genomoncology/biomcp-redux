# 0915 — section-help-drops-options-the-section-refuses

Status: OPEN.

Milestone: 7

## Outcome

Section help shows only the options a section accepts. `help get drug markets` and the `get variant` section help stop offering options that the section refuses.

## Why

The review of 0902 (section help, 2026-11-13) found two gaps that 0902 did not cause.
- **Markets:** `help get drug markets` lists `--source` (drugsfda, ema, who-pq) and the approvals-only options `--decision-from`, `--document-id` and `--product-number`. Its example is `get drug EMEA/H/C/000406 --source ema`, which doesn't read markets. But markets takes a name and no source, and `crates/biomcp-query/src/drug/query.rs:151` refuses markets with any `--source`.
- **Variant:** the clinvar and population section help still show `--civic-kind`, `--civic-profile` and `--civic-cursor`, because the variant schema doesn't mark them civic-only the way 0902 marked the `get drug` options.

## Proof

- Proof: help cases asserting that `help get drug markets` has no `--source`, no approvals-only options and a markets example, and that `help get variant clinvar` has no CIViC options. Each case keeps a size cap at measured size plus 15%.

## Starts from

- Starts from: 0902's section filter and its "Only with --source X" description wording; `crates/biomcp-query/src/drug/query.rs:151`.

## Keeps

- Keeps: 0902's section help for every other section.

## Defers

- Defers: nothing.

## Changes

- Changes: option descriptions in the drug and variant commands, the section-help filter, `tests/cases/help/`, regenerated schemas and command reference.
