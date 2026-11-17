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
