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
- Correction (developer's map, 2026-11-19): approvals already took `--section-limit` up to 200, and trial locations and trial search already took `--limit` up to 100; 50 was the cap of most other lists. At 100 rows, 776 sites take 8 calls.
- Correction: no list refused an offset past its end. Pathway, protein and trial sections refused any nonzero offset without a pool or token, which is what refused `--section-offset 50` on 45 pathway genes; with the pool, a page past the end was already empty. The developer's map and measurements are in `sdlc/planning/1061-lists-page-one-way.md`.
- PM decision, 2026-11-19: `--limit` and `--offset` work everywhere a list is returned, as other names for the section forms where those exist. The developer proposes the cap to the PM with reply sizes measured; a cap of 50 is too low for site lists.
- PM decision, 2026-11-19 (on the proposal): points 1 to 4 as proposed. `--limit`/`--offset` work everywhere, with the section names kept as other names. An offset without a token starts a fresh stored copy and returns the page, as article search does, while a token still keeps pages consistent and an unknown or expired token is still refused. An offset past the end returns an empty page with its total. `--page` is refused with a message naming `--offset`. Trial section replies gain the same `page` field as other lists.
- PM decision, 2026-11-19: caps of 200 for lists served from a stored copy or a whole record, and 100 for searches the source pages itself where the source allows. No whole-list option. Defaults unchanged.
- PM decision, 2026-11-19: on `get variant`, when a section is named, `--limit`/`--offset` page that section; with no section they page the list of matching variants as today. The reply's `page` must make plain which list was paged, with a case for each.
- PM decision, 2026-11-19: the second name is accepted without being listed in the MCP tool descriptions, but help for each command says in one short line which names page it, and a refusal of any paging form names the working one.
- PM decision, 2026-11-19: `--fields` is not extended to sections here; it is its own ticket.
- Review, 2026-11-19 (PM rulings on a fresh review): an offset page short of the end is never empty, and a check walks every touched list by offset alone, each call with a new store, against its big pages, with one page past the end; a past-the-end page is empty with its total everywhere (CPIC pgx search, Reactome pathway search); every offered next page and every paging refusal uses `--limit`/`--offset`; a ClinicalTrials.gov offset the ID walk cannot reach inside the deadline is refused as usage before any request, naming `--next-page`, and help states the limit and that `--next-page` keeps pages consistent while offsets read the live registry; an eligibility offset past the text says so.
- PM decision, 2026-11-19: keep `--limit` paging the named section on `get variant`, and add an explicit option to cap the candidates when a section is named. Developer's choice: `--candidate-limit` and `--candidate-offset`, named in help and in the next steps.
- Developer decision, 2026-11-19: the ClinicalTrials.gov offset limit is 5,000 rows. The walk costs about 1.1 s per 1,000 rows skipped and ran out of the 15-second deadline near 14,000; offset 10,000 took 13 s.

## Keeps

- Keeps: the present forms keep working. Replies keep `page` with `offset`, `returned` and `total`.

## Defers

- Defers: an option that returns a whole list in one reply, if the size measurements argue against it.

## Changes

- Changes: argument parsing for get sections and trial locations, help and cases.
