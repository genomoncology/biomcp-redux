# 0922 — skill-tries-the-command-before-help

Status: OPEN.

Milestone: 7

## Outcome

The skill tells an agent to run the command first and read help only when a reply leaves it stuck: use the valid values and the next step a refusal gives. It carries a short command map, a few lines, so an agent needs no overview help page (`biomcp help`, `help questions`, `help examples`) before its first real query.

## Why

The bench read the three round 2 persona runs and one round 1 run with Bot's run reader on 2026-11-14 (bench `personas/_eval/help-reads-2026-11-14.md`). Help reads carried 9 to 17.5% of each run's input tokens. About 43% of that was overview pages read in the first three requests, because the skill says "Use `biomcp help COMMAND ENTITY` before guessing an option" (`skills/SKILL.md:16`) and lists five help entry points. Those pages then stayed in context for 55 to 135 more requests. A refusal that listed the valid values cost 100 to 170 tokens; a help page cost 200 to 2,000. Agents fixed most wrong forms in one step from the refusal, and the regulatory-affairs agent sent `--bogus x` on purpose to get the short list. 0880 pushed help harder, and help reads did not fall (paper W3).

## Proof

- Proof: the skill text check (the existing skill or docs check that covers `skills/SKILL.md`) asserts the new wording and the command map; the measure is the next persona round 2's help reads before its first real query, against 10, 5 and 5 in reg-affairs, rare-disease and pv-scientist round 2.

## Starts from

- Starts from: `skills/SKILL.md:9-20`; 0880's ticket for what it changed and why; the analysis above.

## Keeps

- Keeps: the skill's rules on sections, filters, `--fields`, `next`, totals, errors versus empty, and transcript mismatches; the skill size budget.

## Defers

- Defers: help page length (0923) and reply next steps (0904).

## Changes

- Changes: `skills/SKILL.md`, and the skill checks and generated docs that follow from it.
