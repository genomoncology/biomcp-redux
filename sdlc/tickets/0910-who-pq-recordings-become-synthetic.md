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
  - each replies README says the rows are synthetic.

## Starts from

- Starts from: 0695's synthetic WHO replies as the pattern; `tests/cases/drug/who-pq-dates-read.yaml` and `tests/cases/drug/markets.yaml`.

## Keeps

- Keeps: what each case asserts about behaviour; the cases may assert different values where they quoted real ones.

## Defers

- Defers: deleting the real export from git history, which is Ian's call if wanted.

## Changes

- Changes: `tests/replies/drug/who-pq-dates-read/`, `tests/replies/drug/markets/`, the two case files, and the READMEs beside them.
