# 0988 — nci-trial-paging-follows-source-rows

Status: OPEN.

Milestone: 7

## Outcome

Paging through NCI trial search results returns each NCI trial once. After a page with a refused trial, the next page starts where the source left off and doesn't repeat trials already shown. A page where every trial was refused still offers the next page.

## Why

Finding A1 of `sdlc/records/2026-11-14-adversarial-review.md` (lane A, finding 1), marked "before the cut". Nothing has changed `crates/biomcp-query/src/trial/nci.rs` since that review.

- `nci.rs:129-147` builds the next offset from admitted rows (`page.data.len()`).
- The NCI adapter's `Page` (`crates/biomcp-adapters/src/nci/mod.rs:55-62`) keeps refused rows apart and carries no count of rows received.

So a 100-row page with one refused trial gives a next offset of 99, which repeats a trial already shown. A page where every trial is refused gives no page step, so the rest of the results can't be reached. ClinicalTrials.gov paging was fixed the same way (F4, 0796: `trial.rs:257-258` advances by `page.received`).

## Proof

- Proof: a routine case over a recorded two-page NCI search, with one trial on page one refused (a synthetic fixture). Page two starts at the source's next row and repeats no trial ID. The case fails on the base commit.
- Proof: a page where every trial is refused still offers the next page, at the source's next position.

## Starts from

- Starts from: the review's finding text; `crates/biomcp-adapters/src/nci/mod.rs` `Page`; `crates/biomcp-query/src/trial/nci.rs`; the ClinicalTrials.gov `received` pattern in `trial.rs`.

## Keeps

- Keeps: `returned` and `retrieved_count` count admitted rows, as today. No reply schema change. NCI fixtures are synthetic.

## Defers

- Defers: the CPIC annotation paging (A3), unless the same change covers it in a few lines.

## Changes

- Changes: the NCI adapter page, NCI trial continuation, one case and its recordings.
