# 0910 — who-pq-recordings-become-synthetic

Status: OPEN.

Milestone: 7

## Outcome

The repo holds no real WHO prequalification export again, as 0695 established. The WHO cases from 0878 and 0881 run on synthetic replies that keep the same shapes and edge cases.

## Why

The review of 0898 (2026-11-13) found the problem.
- **WHO's terms:** its copyright policy puts its material under CC BY-NC-SA 3.0 IGO and requires permission to use it "in electronic database products and services". The adapter labels the export noncommercial with "export-specific reuse grant unverified" (`crates/biomcp-adapters/src/who_pq/mod.rs:14-16`).
- **What 0695 did:** it replaced the WHO export with synthetic rows ("no live fetch of noncommercial data", and `AGENTS.md:5`, "Respect source redistribution terms").
- **What happened since:** 0878 (regulatory codes and dates read plainly) recorded the full 129,537-byte export again as `tests/replies/drug/who-pq-dates-read/`, and 0881 (one product across markets) copied it into `tests/replies/drug/markets/`.

## Proof

- Proof:
  - no file under `tests/replies/` holds real WHO PQ rows. Check by the export's SHA-256 and by its product IDs, such as HA634 and HA110 with their real dates;
  - the 0878 and 0881 cases pass on synthetic replies that keep their edge cases: blank and padded dates, rows with no date, and ties broken by export order;
  - each replies README says the rows are synthetic;
  - synthetic rows use invented dates and strengths, never ones copied from real WHO rows. The review of 0898 found a "synthetic" export reusing three real oncology dates with trastuzumab's real strengths. Check every synthetic WHO reply, 0695's included, against the live export by date and strength.

## Starts from

- Starts from: 0695's synthetic WHO replies as the pattern; `tests/cases/drug/who-pq-dates-read.yaml` and `tests/cases/drug/markets.yaml`.

## Keeps

- Keeps: what each case asserts about behaviour; the cases may assert different values where they quoted real ones.

## Defers

- Defers: deleting the real export from git history, which is Ian's call if wanted.

## Changes

- Changes: `tests/replies/drug/who-pq-dates-read/`, `tests/replies/drug/markets/`, the two case files, and the READMEs beside them.

## Progress

- `tests/replies/drug/who-pq-dates-read/` and `tests/replies/drug/markets/` now share one synthetic 29-row export in the real export's shape (header, quoting, padded date cells, blank date cells, no final newline). References, applicants, strengths and dates are invented; no date in it appears in WHO's export of 2026-11-13. It keeps the edge cases: dolutegravir rows that start and end undated, a later-listed row dated before an earlier one, no imatinib row, and three lamivudine rows tied on the earliest date so export order picks the first.
- The two cases assert the synthetic values: dolutegravir total 13, item 0 undated, item 5 SYN201 `22  Sep,  2013` read as 2013-09-22; markets WHO rows SYN201 (13 matches) and SYN101 (16 matches, 2001-06-07). What each case asserts about behaviour is unchanged.
- Checking 0695's synthetic replies against the committed real export by date and strength found 35 hits in `who-dates` (its date cells were the real ones, with `10mg` strengths) and one in `who-synthetic` (Trastuzumab `150 mg` on `18  Dec,  2019`, the real BT-ON001 pair). `who-dates` dates now move to unused days in the same month, keeping windows, order and ties, and its strengths are `17mg`; `who-synthetic` uses `137 mg` on `23  Dec,  2019`. `who-dates.yaml` and `who-sync.yaml` follow.
- Each of the four WHO replay directories has a README saying its rows are synthetic.
- Checked live 2026-11-14 in pm job j39976: WHO's export was unchanged (657 rows, SHA-256 `abaeaf03…`). None of the four synthetic WHO files is the real export, and none has a real reference, a real date, or a real date and strength pair. Gate `make --keep-going landing-gate docs-check schema-release-check` passed in pm job j39977 at b14446905.
