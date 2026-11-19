# 1061 — lists-page-one-way

Status: OPEN.

Milestone: 7

## Outcome

A caller pages any list the same way. Whatever form is chosen works on a search, on a section of a record, and on trial locations, and a refusal of another form names the form to use.

## Why

Agents tried four forms to get past the first ten rows of an approval history, and one run spent much of its 20 minutes on it.

## Proof

- Proof: cases that page a search, `get drug NDA022363 approvals --source drugsfda`, `get pathway R-HSA-5693579 genes` and `get trial NCT04613596 locations` to the end with the same options; they fail on the base commit.

## Starts from

- Starts from: Seen on main be96d8d40 (1.0.0), Linux, 2026-11-19, in 25 agent runs that used the command line build on real tasks.
- `get drug NDA022363 approvals --source drugsfda` shows 10 of 15. `--offset 10`, `--page 2` and `--limit 40` are each refused as usage; only `--section-limit` and `--section-offset` work.
- `get pathway ID genes --limit 50` is refused, while `pathway genes ID --limit 50` works.
- `get trial NCT... locations --limit 100 --offset 100` is refused: "nonzero --offset requires --pool from the returned continuation". One trial has 776 sites.
- `--section-limit` and `--limit` stop at 50, so 776 rows take 16 calls. `--section-offset 50` on a 45-row list is refused as usage instead of returning an empty page.
- PM decision, 2026-11-19: `--limit` and `--offset` work everywhere a list is returned, as other names for the section forms where those exist. The developer proposes the cap to the PM with reply sizes measured; a cap of 50 is too low for site lists.

## Keeps

- Keeps: the present forms keep working. Replies keep `page` with `offset`, `returned` and `total`.

## Defers

- Defers: an option that returns a whole list in one reply, if the size measurements argue against it.

## Changes

- Changes: argument parsing for get sections and trial locations, help and cases.
