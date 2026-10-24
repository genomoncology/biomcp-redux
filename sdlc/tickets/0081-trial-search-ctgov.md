# 0081 — Search trials on ClinicalTrials.gov with native filters and exact totals

Status: COMPLETE.
Landed: fa87182adea94a472949c1a3eebbb5a7e6256067

Milestone: 2

Depends on: 0079

## Outcome

`search trial [TEXT]` with `--condition`, `--intervention`, `--gene`, `--variant`, `--criteria`, `--status`, `--phase`, `--study-type`, `--sex`, `--age`, `--facility`, `--lat/--lon/--distance`, `--sponsor`, `--sponsor-type`, `--updated-since`, `--updated-until`, `--has-results`, `--id`, `--count-only`, `--limit` and opaque pages returns trial headers in the registry's order, the registry's `totalCount`, and the exact request, through the CLI and MCP.

- Why: the first question an agent asks about trials; 0.9's age post-filter, unknown totals and ignored filters with an NCT ID are the failures this fixes.
- Starts from: trial.md (source findings, disclosure, types, known-positive witnesses, field table, budgets); `decisions/2026-10-24-trial-slice-rulings.md`; the gene resolver for `--gene`; architecture section 3 types and the shared enum macro.
- Builds: biodata `NctId`, `TrialHeader`, `TrialPhase`, `TrialStatus`, `StudyType`, `TrialSex` and `biomcp.trial_page/1`; the ClinicalTrials.gov adapter declaring its quota at one admission per second and its terms; the query and the registry processing timestamp from the version endpoint, cached through the transport.
- Risks the proof must cover: each filter maps to the right field; the age expression admits missing bounds; an NCT ID with filters builds the conjunction; facility with radius is refused before a request; status values never translate to "recruiting"; a failed source never reads as zero.
- Proof: one replay case per row of trial.md's known-positive table asserting its captured total and first NCT ID, recorded once with `--record` through pm job; T23 asserts the conjunction request and a complete empty answer; the invalid status 400 asserts usage; the five failure cases of architecture 5.5; the retired-option messages of ruling 5 and 8; `--count-only`; a two-page walk; MCP parity. Ceilings are measured plus 15 percent, capped by 0.9's bytes for the same question. Then make landing-gate runs through pm job after current main.
- Defers: get and sections (0082 to 0084), NCI (0085), help and accounting (0086).
