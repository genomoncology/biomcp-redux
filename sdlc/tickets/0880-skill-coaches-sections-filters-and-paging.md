# 0880 — skill-coaches-sections-filters-and-paging

Status: OPEN.

Milestone: 7

## Outcome

The skill and help steer an agent away from the guesses that cost reg-affairs about a third of its calls: sections are words after the ID, a filter works only on the sources that list it, and paging stops at the total.

## Why

From bench 0041 reg-affairs smoke run on BioMCP b87cb3b3e (pm job j38923), `personas/reg-affairs/runs/2026-11-13/log.md` in biomed-agent-bench (until filed: `~/workspace/data/biomed-agent-bench/0041-persona-reg-affairs/` on the M5): of 57 BioMCP calls, 8 were help reads before the first data call and about 10 were wrong forms or extra pages: `--section approvals` (log.md:23-28), unknown `--fields` names twice (log.md:80-83, 160-161), `--device-id` on fda-pma three times (log.md:112-114), and three empty pages after 62 of 62 WHO rows (log.md:150-151). Each error's own guidance fixed it in one step.

## Proof

- Proof: a before-and-after count on the same persona questions in the next reg-affairs run (bench playbook round 2), plus routine help cases for the new wording.

## Starts from

- Starts from: `skills/biomcp` in the build, `biomcp help` text, and the error messages' next steps.

## Keeps

- Keeps: skill length small; no persona-specific hints.

## Defers

- Defers: other personas' coaching until their runs.

## Changes

- Changes: `skills/`, `crates/biomcp/src/grammar/`, `docs/`, `tests/cases/`

## Progress

- Diagnosis from the run's call record (j38923): the agent read the skill and six help pages, then guessed.
  - `--section approvals`: the skill's only word on sections was the MCP line "sections go in `section`", and `help get drug` lists the argument as `section` with a position but no call that shows it; its one example has no section.
  - `--device-id` on fda-pma: help printed every filter with "Command sources: fda-510k, fda-cdx, fda-device-recall, fda-pma, gtr, who-ivd; source restrictions in the field description apply", and the `--device-id` text ("Exact PMA application, application/supplement or K identity") names no source. The schema already knew it was fda-cdx only (`x-required-component`).
  - `--fields decision_date,supplements` and `manufacturer` on who-pq: the agent took field names from the reply (dates sit inside `header`) or from the other entity's list. The error listed valid names but its next step was only `help`.
  - Three empty WHO pages: the agent batched offsets 0–200 in one script before it saw the total of 62. Nothing in the skill said to stop at the total.
- Changed:
  - Skill: four short lines under "take names only from help": sections are words after the ID (with a recorded call) and `--section` only where help lists it, a filter works only with the sources named after `Only with --source` or in its description, `--fields` names come only from help's `Fields:` list, and when a page reports a total stop paging at it. The skill stays under its 4 KB cap.
  - Help: each option whose schema names only some of the command's sources now ends "Only with --source X" in place of the long all-sources line, using the same schema fact as the typed MCP catalog; options without that fact keep the neutral all-sources line, since many are still limited by their description (trial `--sponsor`, article `--author`). Each positional argument shows the call shape, e.g. `Form: get drug INPUT SECTION...; not --section.` `help search diagnostic` shrank from about 9.7 KB to 6.7 KB.
  - `--fields` error: next now leads with the same call, keeping the valid names (for the run's call `--fields id,product_name,manufacturer,decision_date,supplements`, it suggests `--fields=id,product_name,manufacturer`), then help. When the query selects a source (trial search), the kept names come from that source's rows, so `--source ctgov --fields phase,title,bogus` suggests `--fields=phases,title`.
- Proof now: routine cases `grammar/help-coaches-forms` and `local/skill`. The before-and-after call count waits for the next reg-affairs bench run.
