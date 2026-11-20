# 1081 — two-small-reply-faults-from-the-hand-check

Status: OPEN.

Milestone: 7

## Outcome

A lowercase application number is read as the application it names, and a companion diagnostic search whose biomarker filter worked carries no note calling that filter unsupported.

## Why

Found 2026-11-20 on a Linux build of main bb986535f with clean state.

- `biomcp get drug nda212526 approvals --source drugsfda` exits 2 with "expected a supported command with checked arguments" and `option: input`. `NDA212526` works. The message does not say what was wrong; its `next` (a drug search for the same word) runs.
- `biomcp search diagnostic --source fda-cdx --biomarker PIK3CA` returns the right rows (5 of 7 on the first page, status `complete`) and a `problems` entry `{"source": "fda-cdx", "field": "table_1.biomarker", "code": "filter.unsupported"}`. The filter was applied, so the note reads as a failure that did not happen.

## Proof

- Proof: the lowercase call returns the same reply as the uppercase one.
- Proof: the PIK3CA search has no `filter.unsupported` problem, or the ticket says what the note is for and its wording says that.

## Starts from

- Starts from: the two commands above. Ticket 1060 made BioMCP accept the spellings it prints; application numbers take any letter case like other listed choices.

## Keeps

- Keeps: a word that is not an application number still gets the search suggestion.

## Defers

- Defers: nothing.

## Changes

- Changes: drug input parsing, the fda-cdx search reply, cases.
