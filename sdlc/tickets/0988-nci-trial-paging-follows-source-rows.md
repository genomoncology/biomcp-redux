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

## Progress

- 2026-11-16, base behaviour (job j41592 on 674391f9d, the new case over 5f96d6803's code): the review's static reading was not quite right. On the base commit NCI never refused one trial: a row with a bad `nci_id` or `nct_id` failed the whole page, so `search trial --source nci` answered `failed`, reason `bad-reply`, and showed none of the page's good trials. The duplicate and the stranded rest that the review describes would have followed as soon as a row could be refused. The new case fails on the base commit for that reason. (j41590 was the same job with a typo in its script.)
- Live check (same job, NCI key inherited, never printed): `search trial --source nci --keyword melanoma --limit 3` at `from` 0 returned NCT05929768, NCT04267848, NCT05677490 of 2,129; at `--next-page 2` it started with NCT05677490. So `from` is a zero-based row position, and a next page built one row short repeats one trial.
- Decided (within the outcome): an NCI trial row whose identity is malformed (unreadable row, bad `nct_id`, or an `nci_id` not starting `NCI-`) is refused on its own, as ClinicalTrials.gov already does, instead of failing the page. The page reply turns `partial` and names one `identity` problem, the field name MyVariant uses for a refused row whose bad field is unknown. The adapter `Page` now carries `received` (rows the source sent, refused ones included); the page-size checks use it. The next page starts at offset + `received` and is offered whenever the page received any row, so a page of only refused trials still leads on. `returned` and `retrieved_count` still count admitted trials. No reply schema change.
- Case `tests/cases/trial/nci-native-consumption.yaml` over four synthetic recordings (`tests/replies/trial/nci/native-consumption/`), total 8, two rows a page: page one refuses one row and shows NCT09100002; page two starts at offset 2 with NCT09100003 and NCT09100004; page three refuses both rows (bad `nci_id`, missing `nct_id`), returns none and still offers the next page; page four at offset 6 shows NCT09100007 and NCT09100008 and offers no further page. The search uses `--condition-id NCIT:C3224`, not `--keyword`: help examples come from the first case in `tests/cases/trial/` that uses a flag, and a `--keyword` run here would have replaced the `--keyword` example with this synthetic search.
- A3 (CPIC annotation paging): left alone. The annotations section pages a stored list in memory (`pgx/annotations.rs` `local_page`), so its admitted count is the number of slots read and the fallback in `pgx/next.rs` is exact there.
- No `release/notes.md` line: no command, option, filter, source, config key, credential name, output field or tool changed (`sdlc/decisions/2026-11-16-review-follow-ups.md`, item 5). Paging and refusal follow the rules the notes already state.
- Other NCI pages: organization and biomarker searches never refuse a row (their mappers return `Refusal::None` and keep every row), so their admitted count already equals the rows read and they need no change.
- Checks: job j41594 on dfc91e8fe put back 5f96d6803's NCI code and ran the final case, which failed at step one (`failed`, `bad-reply`); with the fix it regenerated schemas and docs (nothing changed), passed `cargo fmt`, `make check-cheap`, the 18 other cases and runner trials whose names contain `nci`, and `custom_refusals`. The new case got through its first two pages and then failed on its own assertion: an empty NCI page leaves `items` out rather than sending `[]`. 0105a2778 asserts that; job j41597 ran the corrected case alone on 44713bf0a (the branch merged with origin/main b54b4e89f) and it passed.
