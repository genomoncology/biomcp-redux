# 1121 — trim-drug-help-back-under-its-old-size

Status: OPEN.

Milestone: 7

## Outcome

`biomcp help search drug` and `biomcp help get drug` each fit in at most 9,000 bytes again, and still teach every option and section they list today. The ceilings in `tests/cases/drug/help.yaml` come down to the new sizes.

## Why

Agents read this help before they call the command, and every turn after re-reads it, so its size is paid many times per task. Three tickets raised the ceilings in turn: `help search drug` from 8,943 to 10,590 bytes and `help get drug` from 7,898 to 11,418 bytes (tickets 1027, 1066 and 1067, by `git log -p -- tests/cases/drug/help.yaml`). Each addition was reasonable alone; together the two pages grew by about a third.

## Proof

- Proof: the two ceilings in `tests/cases/drug/help.yaml` are 9,000 bytes or less and the case passes.
- Proof: every option name and section word the case lists today is still listed.
- Proof: the MCP drug tool description stays under its 3,500-character limit and does not grow.

## Starts from

- Starts from: the help text for `search drug` and `get drug`. Look first for repeated wording across sections, long examples, and source notes that say what `help sources` already says.

## Keeps

- Keeps: every option, section and example a test names. Two executable examples per command where the case asks for them.
- Keeps: what each option does; shorten how it is said, not what it says.

## Defers

- Defers: other entities' help.

## Changes
