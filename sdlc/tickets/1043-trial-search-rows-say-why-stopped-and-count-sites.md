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

## Progress

- Source, live 2026-11-17: `AREA[WhyStopped]("slow" AND "enrollment")` counts 646, the same as the unquoted words, so `--why-stopped` sends each word quoted and joined with AND; `fields=NCTId,LocationFacility,LocationCity,LocationCountry&pageSize=1000` returned all 49 KRAS G12D pancreatic trials (84 KB) and the first 1,000 of 5,135 pancreatic cancer trials (830 KB, 2.2 s) in one request.
- Change: search rows carry optional `why_stopped` (the same unescaped text the card shows), because the search `fields` list now asks for `WhyStopped`; `--why-stopped TEXT` adds `AREA[WhyStopped](...)`. `--site-count` reads the match 1,000 trials a page, up to 5 pages (5,000 trials), counts each facility, city and country once per trial with names exactly as registered, and returns the `--limit` most as optional `site_counts` on the trial page (`trials_matched`, `trials_read`, `facilities_total`, `facilities`), with no rows; the attribution says names are never merged. It refuses `--count-only` and `--next-page`. NCI refuses both options as unsupported filters. Schema change is additive: two optional query fields, one optional row field, one optional page field, two new filter names.
- Recordings: the 128 existing search recordings were re-keyed for the new field list (bodies unchanged); new live recordings from job j42492 back `tests/cases/trial/why-stopped.yaml` and `tests/cases/trial/site-count.yaml`, whose values match the ticket's live counts (MGH 9, Moffitt 7, MD Anderson 6). Both cases fail on the base commit, where the options are unknown.
- Decisions: the site count is an optional `site_counts` field on the existing trial page, not a new reply schema, so the schema change stays additive; the read stops at 5 pages (5,000 trials) and says how many trials it read; the CLI flag is `--site-count` and the facility count is `--limit`.
- Jobs: j42492 (record, regenerate schemas and docs: passed). j42497 (`check-cheap`: a too-long function, the optional-value ratchet on `if let Ok`/`.ok()` in the query module, one over-long doc line; `schema-release-check` passed). j42502 on fc5090a96: ratchets passed; cases `trial/`, `search/`, `grammar/`, `saved/`, `mcp/`, `help/` all passed; `cargo fmt` changed nothing; `check-cheap` failed only on clippy `too_many_lines` in `ctgov/filters.rs::advanced` (85/80).
- Paused 2026-11-17 (agent switch): the commit after fc5090a96 moves the stop-reason clause into its own `why_stopped` helper to fix that clippy failure; it has not been built. Left: one job running `make check-cheap` plus the touched cases, then the full gate `make --keep-going landing-gate docs-check schema-release-check`. Expect the gate's secrets scan may flag ClinicalTrials.gov `nextPageToken` cursors in the re-keyed recordings (new paths); add reviewed entries to `release/secrets.ignore.jsonl` as 1018 did. If the help or tool budget fails, raise the cap to measured plus 15 percent.
