# 1004 — variant-oncokb-section-refusal-names-the-real-cause

Status: OPEN.

Milestone: 7

## Outcome

`biomcp get variant clinvar:13961 oncokb`, and the same with `--tumor-type Melanoma`, never answers with a usage error that rejects `section` while listing `oncokb` among the valid values. With no OncoKB key it answers like every other licensed section: `oncokb` skipped, reason `no-key`, `next` pointing at `help key oncokb`. If OncoKB needs something the input lacks (a build, a protein change), the error names that option, not `section`.

## Why

The tumor-board persona on 3521c503a, the hosting candidate (bench ticket 0057, job j41867, `calls.md` lines 2033-2044 of `0057-persona-tumor-board-pair/biomcp-cli`), ran `biomcp get variant clinvar:13961 oncokb --tumor-type 'Melanoma'` and then without `--tumor-type`. Both answered `usage.arguments` on `option: section`, "option is missing or invalid", with `valid_values` that include `oncokb`. Its report says: "The CLI rejected the `oncokb` section word with a generic usage error rather than a 'no-key' refusal", and "the current generic usage error hides the cause". On 2026-11-12, `get variant CA126713 oncokb --build GRCh37 --tumor-type …` reached the no-key skip, so the refusal depends on the input form.

## Proof

- Proof: routine cases for `get variant clinvar:13961 oncokb` with and without `--tumor-type`, with no OncoKB key: each answers with the `no-key` skip or with an error naming the real missing option. The cases fail on the base commit.

## Starts from

- Starts from: `get variant` section parsing and the OncoKB section's preconditions; the error that names `section`.

## Keeps

- Keeps: OncoKB's licence gate and the no-key refusal wording other licensed sections use.

## Defers

- Defers: nothing.

## Changes

- Changes: the variant get's section validation, cases and their recordings.
