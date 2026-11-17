# 1043 — trial-search-rows-say-why-stopped-and-count-sites

Status: OPEN.

Milestone: 7

## Outcome

Two trial-operations questions take one `search trial` call each on ClinicalTrials.gov:

- Each search row carries the registry's `why_stopped` text when the trial has one, and `--why-stopped TEXT` keeps only trials whose stop reason contains those words (sent as `AREA[WhyStopped]`).
- A site count across the matched trials: a search can return, instead of trial rows, the facilities that appear in the most matched trials, each with its city, country and the number of trials, over every page of the match. Facility names are counted exactly as the registry writes them, with no name merging, and the reply says so and gives the number of trials read.

## Why

The trial-ops persona asked both in its 2026-11-10 and 2026-11-12 runs and answered by paging cards and counting by hand (biomed-agent-bench `personas/trial-ops/runs/2026-11-12/log.md:47-49` and `:78`, and `personas/rollup.md`; `records/2026-11-12-persona-review-findings.md`, Waiting). `why_stopped` reached the trial card in 0823 but not the search rows. Ian's `decisions/2026-10-27-1-0-ships-every-known-fix.md` puts every known gap in 1.0.

## Proof

- Proof: a replayed case runs `search trial` with a condition whose matches include a terminated trial and expects that row's `why_stopped`; a second runs `--why-stopped enrollment` and expects only trials whose stop reason says so. Both fail on the base commit.
- Proof: a replayed case asks for the site count of a small match (live 2026-11-17: `query.cond=pancreatic cancer`, `query.term=KRAS G12D` matched 49 trials; Massachusetts General Hospital, Boston, appears in 9, Moffitt in 7, MD Anderson in 6) and expects the counts and the trials-read total. It fails on the base commit.

## Starts from

- Starts from: `crates/biomcp-adapters/src/ctgov/search_query.rs` (the search request and its `fields`), the search row mapping beside it, `ctgov/detail/posted.rs` (`why_stopped` on the card) and `ctgov/detail/sites.rs` (facility, city, country). Checked live 2026-11-17: `query.term=AREA[WhyStopped]enrollment` counts 4,471 trials, and `fields=NCTId,LocationFacility,LocationCity` with `pageSize=1000` returns every site of the 49-trial match in one request.

## Keeps

- Keeps: today's search rows, paging and totals when neither option is given; NCI search unchanged (it refuses `--why-stopped` and the site count with a usage error).

## Defers

- Defers: merging different spellings of one facility, which would need a name rule this ticket does not invent.

## Changes

- Changes: the trial search query, ClinicalTrials.gov search adapter and row schema, a new site-count reply, cases and their recordings, help and one release-notes line.
