# 1081 — two-small-reply-faults-from-the-hand-check

Status: OPEN.

Milestone: 7

## Outcome

A lowercase application number is read as the application it names, and a companion diagnostic search whose biomarker filter worked carries no note calling that filter unsupported.

## Why

Found 2026-11-20 on a Linux build of main bb986535f with clean state.

- `biomcp get drug nda212526 approvals --source drugsfda` exits 2 with "expected a supported command with checked arguments" and `option: input`. `NDA212526` works. The message does not say what was wrong; its `next` (a drug search for the same word) runs.
- `biomcp search diagnostic --source fda-cdx --biomarker PIK3CA` returns the right rows (5 of 7 on the first page, status `complete`) and a `problems` entry `{"source": "fda-cdx", "field": "table_1.biomarker", "code": "filter.unsupported"}`. The filter was applied, so the note reads as a failure that did not happen.

Added 2026-11-20 from the review of ticket 1085 (commit 2e137438b). None gives a wrong answer:

- The error schema says an error "never contains rejected caller values" (`crates/biomcp-types/schemas/error.json`), and the new `unplaced` field repeats field names the caller typed. PM decision: keep `unplaced` and correct the schema's sentence to say what is allowed (names of letters, digits, `_` and `.`, up to 64 characters).
- `--fields=submission_number,submission-type` (a hyphen typo) now suggests only help, where it used to suggest `--fields=submission_number`. Keep the names that can be placed in the suggestion and say that one name could not be repeated.
- `get drug NDA022363 label-history --source drugsfda --offset 10` says to add `--every-version`, which still fails without `--find`; an offset with a section label-history does not read reports the offset first and the section second. Report every cause in one refusal.
- The label-history section refusal names `get drug <NAME> label --label-section NAME` in its message and offers only help in `next`; offer that call.
- `help get drug markets` describes `--source` with the generic drug text ("Omit to infer the source"); for markets, omitting it reads every market.
- `search diagnostic fulvestrant --source fda-cdx` is refused with "value not accepted" though `--name fulvestrant` works; say to use `--name`, or accept the word as the name.
- `get drug imatinib markets --source ema` reports `absent` without the stale-data flag the plain markets call gives for EMA.

## Proof

- Proof: the lowercase call returns the same reply as the uppercase one.
- Proof: the PIK3CA search has no `filter.unsupported` problem, or the ticket says what the note is for and its wording says that.
- Note: the PIK3CA search keeps one `filter.unsupported` problem with field `group_table`. FDA's group table has no biomarker column, so `--biomarker` cannot search its rows; the problem says those rows were not searched and may hold matches the page leaves out, not that the filter failed.

## Starts from

- Starts from: the two commands above. Ticket 1060 made BioMCP accept the spellings it prints; application numbers take any letter case like other listed choices.

## Keeps

- Keeps: a word that is not an application number still gets the search suggestion.

## Defers

- Defers: nothing.

## Changes

- Changes: drug input parsing, the fda-cdx search reply, cases.
