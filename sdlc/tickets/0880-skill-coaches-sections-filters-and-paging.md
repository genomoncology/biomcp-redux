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
