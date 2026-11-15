# 0966 — cdx-rows-lead-with-shared-fields

Status: COMPLETE.
Landed: e5f946c7a29e74af275cc77a29dbba9f56b53caf

Milestone: 7

## Outcome

An agent reading FDA CDx search rows uses BioMCP's shared fields (diagnostic, indication, device, drug, row_date) rather than FDA's per-table cell names, so rows from both FDA tables read the same way: the shared fields come first in each row (or FDA's cells sit under one key), within the 1.0.0 schema and reply caps.

## Why

dx-lab-director round 4 (2026-11-15): the agent still read FDA's column names; the 5 rows from FDA's second table, which spells its columns differently, came back empty in its scripts (calls 41, 46, 47) and Q4 lost ONCO/Reveal and MI Cancer Seek. 0959's help line naming the shared fields was in its reply at call 6 and did not change this. Evidence: bench `personas/dx-lab-director/runs/2026-11-15-r4/` (job j41109, BioMCP 95a74e246, bench 0051).

## Proof

- Proof: the persona's CDx query replayed from a recording shows both tables' rows with the shared fields first (or FDA's cells nested), and a jq over `.diagnostic`/`.drug` gets all rows.

## Starts from

- Starts from: 0959 and 0933 (CDx rows); the CDx row type in `crates/biomcp-types/src/response/diagnostic/`; the round 4 log calls 41-47.

## Keeps

- Keeps: every FDA cell value; the 1.0.0 schema rules (field order and nesting changes must pass `schema-release-check`; if a nesting change is not allowed, order alone); reply caps.

## Defers

- Defers: a skill line, since the skill has 3 bytes free under its 4 KB limit after 0948.

## Changes

- Changes: CDx row serialization, cases, regenerated docs.

## Progress

- 2026-11-15, evidence read: round 4's agent keyed on the individual table's header `cells["Diagnostic Name (Manufacturer)"]`, so the 5 group-table TAGRISSO rows read as null (calls 41, 46, 47) and ONCO/Reveal and MI Cancer Seek dropped out of Q4. The CLI printed each row as `kind`, `snapshot`, `index`, `cells`, ids, and only then the shared fields, so FDA's cells were the first thing an agent saw.
- Changed: each CDx row now prints `kind`, then `row_date`, `diagnostic`, `indication`, `device` and `drug`, then `snapshot`, `index`, FDA's `cells` and the ids. FDA's cells already sit under one key (`cells`), so no nesting changed; only field order moved, which leaves the reply schema's fields unchanged; only the row type's description in `diagnostic-companion.json` now says both kinds lead with the shared fields (schema-release-check passes, j41175). Generated docs have no diff. Every FDA cell value stays. Over MCP, reply fields are listed alphabetically, so the order holds for CLI JSON only; 0962 covers MCP agents.
- Proof: `tests/cases/diagnostic/fda-companion-shared-first.yaml` replays the recorded FDA page. For July 2026 (one individual and one group row) `drug` must start before each row's `cells` (bytes 701 and 2,918, measured in j41158); for TAGRISSO's NDA208065, all 15 rows (10 individual, 5 group) carry `diagnostic`, `drug`, `device` and `row_date`, so one jq over `.diagnostic`/`.drug` reads every row. Reply size is unchanged.
- Gate: j41179 passed `make --keep-going landing-gate docs-check schema-release-check` on ab6d9eeef (branch merged with origin/main at aab6451c9).
- Review fixes: `help search diagnostic` now says FDA CDx rows of both tables start with row_date, diagnostic, indication, device and drug, before FDA's cells; the shared `device` field's description says it holds FDA's device number and date, not the test's name, which is in `diagnostic` (description only). j41189: schemas and docs regenerate with no diff, check-cheap, diagnostic and mcp-parity cases, and schema-release-check pass; help measures 8,567 bytes, inside its 9,846 cap, so the cap is unchanged.
