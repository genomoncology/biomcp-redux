# 1032 — nih-reporter-grants-as-a-source

Status: OPEN.

Milestone: 8

## Outcome

BioMCP can search NIH RePORTER projects by investigator and text, with organization, institute, fiscal year and amount, from RePORTER's public API.

## Why

`sdlc/planning/sources-to-add.md:54`; msl 8.3 and 8.4 (bench `personas/msl/runs/2026-11-16/analysis.md`).

## Proof

- Proof: a replayed case asserting an investigator's project count and a text search's count.

## Starts from

- Starts from: an existing adapter of the same shape, and the source's own documentation and terms.

## Keeps

- Keeps: every existing command form.

## Defers

- Defers: nothing.

## Changes

- Changes: a new adapter or section, its help, schemas, cases, recordings and a release-notes line.

## Progress

- 2026-11-17, docs (https://api.reporter.nih.gov/ and its "Data Elements for RePORTER Project API_V2.pdf"): `POST https://api.reporter.nih.gov/v2/projects/search` takes `{"criteria": {...}, "include_fields": [...], "offset", "limit", "sort_field", "sort_order"}`. Criteria used here: `pi_names` (`[{"any_name": ...}]`, "by default wildcard enabled"), `advanced_text_search` (`operator` and, or or advanced; `search_field` from projecttitle, terms, abstracttext; `search_text`), `org_names` (partial match), `fiscal_years` (list). `limit` defaults to 50, at most 500; `offset` at most 14,999 and not past the total. The reply has `meta.total` and `results`, one row per project and fiscal year, with `project_num`, `core_project_num`, `fiscal_year`, `project_title`, `activity_code`, `organization.org_name/org_city/org_state/org_country`, `agency_ic_admin.abbreviation/name`, `award_amount` (that year's total cost), `principal_investigators[].full_name/is_contact_pi/profile_id`, `project_start_date`, `project_end_date` and `project_detail_url`. The page asks for no more than one request per second and large jobs on weekends or 9 PM to 5 AM Eastern, or "administrators [may] block your IP address".
- Terms: the API page footer says "The RePORTER database is available to all public users" and reserves the right to block addresses that ignore robots.txt or "submit requests at a rate that negatively impacts service delivery to other users". RePORT's Data Access Policy (https://report.nih.gov/sites/report/files/2020-07/DataAccessPolicy.pdf) lists RePORTER and its API under "NIH Funding Data Available to the Public"; only non-public data needs FOIA or guest-researcher access. NIH's disclaimer (https://grants.nih.gov/disclaimer.htm) disclaims liability and forbids using NIH's views for advertising or endorsement. No licence, attribution request or limit on automated use or redistribution was found; the data is US federal government data. So BioMCP may read it and commit real replies. The recordings ask RePORTER for named fields only (`include_fields`), so abstracts, written by the grantees, are not kept. The source is labelled open, with the 1/s quota.
- Live check with curl, 2026-11-17: `pi_names [{"any_name": "Pasi Janne"}]` gives 70 project-years (the msl run's 8.3 number), the same as `first_name PASI, last_name JANNE`; "Janne" alone gives 290; "Janne, Pasi" gives 0. `advanced_text_search` "KRAS G12C" with `and` over title and terms gives 11, over title, abstract and terms 173. Adding `org_names ["DANA-FARBER"]` and `fiscal_years [2024, 2025]` to the Janne search gives 3. An empty criteria object returns all 2.98 million projects.
- Command shape: `search grant [TEXT] [--investigator NAME] [--organization NAME] [--fiscal-year YEAR] [--limit N] [--offset N]`, a new search-only helper like `search organization`. Read `docs/reference/commands.md` and the CLI design review (`sdlc/records/2026-11-07-cli-design-review.md`, which keeps verb-first rows). A section on `get author` does not fit: `get author` takes exact ORCID or Semantic Scholar IDs, RePORTER does not know them, and a section cannot do the topic search 8.4 needs. A `--source nih-reporter` on `search author` would return grants under an author reply and could not search by topic. `search grant` takes both inputs, follows the `search ENTITY` form every other search uses, and adds no `get`, because each row already carries the RePORTER page link. On MCP it is the `article` tool's `grant_search` action, next to the author helpers, since `decisions/2026-11-07-entity-tools-in-1-0.md` puts helpers inside the tool that uses them and keeps the tool count.
- Reply `biomcp.grant-page/1`: one row per project-year (as RePORTER counts them, so `page.total` is project-years) with `project`, `core_project`, `fiscal_year`, `title`, `activity`, `institute` (abbreviation and name), `organization` (name, city, state, country), `award_amount` (that year's total cost in dollars), `start`, `end`, `investigators` (name, contact flag, RePORTER profile ID) and `url`. Text matches all words in title, abstract and terms, as RePORTER's own site searches; RePORTER's `any_name` takes "First Last". RePORTER pages itself; the next step stops at the total and at its 15,000-row limit.
- Changed: new source `nih-reporter` (adapter `crates/biomcp-adapters/src/nih_reporter/`, quota 1/s as RePORTER asks, terms labelled open, health probe asks for one known project number); query `GrantSearch` (`biomcp.grant-search/1`) and reply `GrantPage` (`biomcp.grant-page/1`); runner `crates/biomcp-query/src/grant.rs`; registry row `search grant`; the MCP `article` tool gains action `grant_search`. Fields RePORTER sends that cannot be read become `problems`, as other adapters do. `--fiscal-year` takes one year: the command grammar has no repeated or comma-separated integer option. Docs: release-notes line, feature line, source-pacing row, nightly live check row; the command reference is regenerated.
- Proof: `tests/cases/grant/search.yaml` replays real RePORTER replies recorded in jobs j42229 and j42240 (`tests/replies/grant/search/README.md`; no abstracts, since only named fields are asked for): Pasi Janne has 70 project-years, the newest FY2026 5P50CA265826-05 (P50, NCI, Dana-Farber, $384,444) with a next-page step; "KRAS G12C" gives 173; Janne at DANA-FARBER in FY2025 gives 1 (5P50CA265826-04, $371,024); offset 68 gives the last two (FY2008); an unknown name gives 0. `tests/cases/grant/validation.yaml` and `tests/cases/local/refusals-search-grant.yaml` cover refusals and help. Replies measured 234 to 2,074 bytes for 0 to 3 rows.
- Jobs: j42229 (first build and recording; the adapter ratchets refused serde defaults and a local text helper, two doc lines were too long, coverage wanted `help grant` and option steps, and two-year `--fiscal-year` was refused by the grammar; all fixed), j42233 and j42235 (a leftover call broke the build; then the one-year form), j42237 and j42239 (the generated grant schemas had to be dropped and regenerated for the one-year field), j42240 (recorded the one-year call; schema and docs regenerate, check-cheap and the grant, local, mcp, help, grammar, schema, readme and search cases pass).
- More jobs: j42242 (gate: the skill's command map lacked `grant`; `skills/SKILL.md` now says search also takes organization, grant, with three phrases tightened to stay at 4,087 bytes), j42243 (first-read check and skill, readme, help and local cases pass), j42244 (gate: the pacing citation must state a published limit; reworded), j42247 (source declarations and docs-check pass), j42251 (after merging main with STRING: regenerated schemas; check-cheap and the grant, mcp, local and protein cases pass).
- Gate j42255 passed on 7a1afa9d4 (branch merged with origin/main d42922dfe): `make --keep-going landing-gate docs-check schema-release-check`.
- Open: `--fiscal-year` takes one year, because the grammar has no list-of-integers option; RePORTER itself takes several. An offset past RePORTER's total is refused by RePORTER (HTTP 400) and reads as a failed source; the next step never points past the total.
