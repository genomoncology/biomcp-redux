# 0923 — help-pages-say-each-thing-once

Status: OPEN.

Milestone: 7

## Outcome

A command's help page states each sentence once. A line that applies to every option, such as which sources the command reads or that listed choices accept any letter case, appears once at the top, not after every option. `help search adverse-event` loses its repeats with no loss of information.

## Why

The bench's help-read analysis of 2026-11-14 (bench `personas/_eval/help-reads-2026-11-14.md`): command help pages were about 35% of help cost, read whole before each command's first use. In `help search adverse-event` (9k characters, about 2k tokens; pv-scientist round 2), "Command sources: faers, maude, vaers; source restrictions in the field description apply." appears 17 times, "Listed choices accept any letter case…" 6 times and "; help search adverse-event" 6 times: 24% of the page.

## Proof

- Proof: a help case under `tests/cases/` for `help search adverse-event` that fails if a sentence repeats; the help size caps (0905) re-measured downwards.

## Starts from

- Starts from: where help assembles option descriptions (find the source of "Command sources:" and "Listed choices accept any letter case" in the crates); 0905's measured caps.

## Keeps

- Keeps: every option, value list and source restriction the page states today; the CLI and MCP help reading the same text.

## Defers

- Defers: a short form (flag, values, one line each) as the default page, until this lands and the next persona round shows what help costs then.

## Changes

- Changes: the help rendering code, help cases, size caps and generated docs.
