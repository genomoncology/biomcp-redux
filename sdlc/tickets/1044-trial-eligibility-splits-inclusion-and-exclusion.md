# 1044 — trial-eligibility-splits-inclusion-and-exclusion

Status: OPEN.

Milestone: 7

## Outcome

`get trial NCT… eligibility` gives the inclusion criteria and the exclusion criteria as two lists of the registry's own items, beside the age, sex and healthy-volunteer fields it already gives. The split uses only the registry's own "Inclusion Criteria" and "Exclusion Criteria" headings and its list markers. When the text has no such headings, the reply keeps the whole text and says it could not be split. The original text stays in the reply.

## Why

The trial-ops persona got eligibility as one block of prose in its 2026-11-10 and 2026-11-12 runs and split it by hand to screen patients (biomed-agent-bench `personas/trial-ops/runs/2026-11-12/log.md:57`, `personas/rollup.md`; `records/2026-11-12-persona-review-findings.md`, Waiting). Ian's `decisions/2026-10-27-1-0-ships-every-known-fix.md` puts every known gap in 1.0.

## Proof

- Proof: a replayed case on a trial whose criteria carry both headings expects the two lists with the registry's item text. A second, on a trial with no headings, expects the whole text and the "not split" notice. The first fails on the base commit.

## Starts from

- Starts from: `crates/biomcp-adapters/src/ctgov/detail/eligibility.rs`, `TrialEligibilityPage` in `crates/biomcp-types/src/response/trial_get.rs`, and `exclusion_heading` in `crates/biomcp-adapters/src/ctgov/gene.rs`, which already reads the same two headings and should be shared rather than copied.

## Keeps

- Keeps: the eligibility text, its fragments and paging, and the structured age, sex and healthy-volunteer fields. No judgment of whether a patient is eligible (`decisions/2026-10-24-no-text-heuristics.md` applies: headings and list markers only).

## Defers

- Defers: reading values such as lab thresholds or prior lines of therapy out of the items.

## Changes

- Changes: the ClinicalTrials.gov eligibility mapping, the eligibility page schema, cases and their recordings, and one release-notes line.

## Progress

- 2026-11-17: `exclusion_heading` moved from `ctgov/gene.rs` to `ctgov/criteria.rs` as `heading`, now shared by gene notices and the split; it also accepts a space before the colon ("Exclusion criteria :", seen in a batch-of-20 recording). The eligibility page gains three optional fields: `inclusion` and `exclusion` (this page's items, each a top-level line with its more deeply indented nested lines, verbatim) and `not_split` (a notice when the whole text lacks either heading). Lists follow the existing paging: a later page knows its section from the last heading before its offset. Text before the first heading stays only in `criteria`.
- Measured in j42431: NCT02578680 eligibility page grows from under 8,395 to 12,427 bytes (10 inclusion, 25 exclusion items); ceiling 14,291. NCT00003020 (live recording, no headings) measures 3,652 bytes; ceiling 4,200.
