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

## Progress

- Skill (`skills/SKILL.md`): the five help entry points (`help`, `help questions`, `help examples`, `help ENTITY`/`help COMMAND ENTITY`, `help sources` and `skill list`) and "Use `biomcp help COMMAND ENTITY` before guessing an option" are replaced by a two-line command map (search and get with their entities; the follow-up commands on one record) and one paragraph: run the command first, use a refusal's valid values and next step, and read only the one help page needed when a reply still leaves the agent stuck. 0880's rules on sections, markets, filters, `--fields`, `next` and totals stay word for word, as do errors versus empty, transcript mismatches and the MCP section (its help line no longer points at `questions`). The skill file was 2,698 bytes before this ticket.
- Checks: `tests/cases/local/skill` asserts the command map and the new wording in place of the old help line; `scripts/check_front_map.py` no longer requires the skill to name the overview pages. The server instructions still name `help questions` and `help examples` on request; that check is unchanged.
- The measure (help reads before the first real query, against 10, 5 and 5 in round 2) waits for the next persona round.
- Review of 00c68cebc (rejected for text): the first map wrote `search ENTITY TEXT` and `get ENTITY ID` for every entity. Checked against `docs/reference/commands.md`: variant, pgx, biomarker, organization and diagnostic search by flags, not text (`search variant --gene BRAF --protein V600E`); get pgx takes `--gene` and `--source`, no ID; get diagnostic needs `--source`; `variant normalize` takes `--hgvs` and `--source`, and `variant recurrence` needs flags. The map now says so in one clause and adds the missing follow-ups `protein complex-members`, `article citation-evidence`, `article recommendations` and `article pdf`. "Take names only from a reply" would have reopened 0880's `--fields` bug (names taken from returned data), so the skill now says names come from a refusal or the help page, never keys from returned data, and the `--fields` line says the same. Skill file now 3,467 bytes, under its 4 KB cap.
- `scripts/check_front_map.py` no longer pins the literal map wording: it requires every command that `help search`, `help get`, `help author`, `help cell-line`, `help protein`, `help pathway`, `help article`, `help trial` and `help variant` list to appear in the skill, requires "Run the command first.", and refuses `help questions`, `help examples` and a bare `` `biomcp help`: `` entry in the skill.
- Follow-up, not in this ticket: the MCP server instructions (`crates/biomcp/build/front_map.rs:14`) still name `help sources`, `help questions` and `help examples` up front.
