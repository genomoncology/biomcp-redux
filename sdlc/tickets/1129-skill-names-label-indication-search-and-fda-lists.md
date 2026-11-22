# 1129 — skill-names-label-indication-search-and-fda-lists

Status: OPEN.

Milestone: 7

## Outcome

The BioMCP skill tells an agent how to find every drug whose label's indications name a word, and names the FDA list sources an agent cannot otherwise guess (withdrawn cancer accelerated approvals, potential safety signals, safety communications, Purple Book), each with one short copyable command. The skill stays under 4,600 bytes.

## Why

Ticket 1118 cut the skill from 6,616 to 4,087 bytes. A Sol 6.2 run of the twelve round-1 tasks with it (batch `sol62-biomcp-v6-1`, 2026-11-22) used 1.85 million tokens against 2.26 million with the old skill on the same build, but missed two facts the old skill's runs stated: a drug whose own label carries a biomarker use (the old skill had `search drug --indication WORD --source openfda-label`, the new one does not), and a complete count from FDA's withdrawn accelerated approvals list (no skill names that source). One run per task, so the cause is likely but not proven.

## Proof

- `biomcp skill render` shows the label indication search and each named FDA list source with a command; each command replays against a saved reply.
- The skill's byte size is under 4,600, stated before and after.
- Examples use no record from an evaluation task (the PM checks this before accepting).

## Starts from

- Starts from: `skills/SKILL.md` on main after 1118; the old skill's label line for comparison: "Every drug whose label's indications name a word: `search drug --indication \"EGFR\" --source openfda-label --limit 50`. Each row is one label."

## Keeps

- Keeps: everything 1118 added.

## Defers

- Defers: nothing.

## Changes

- Changes: `skills/SKILL.md` and its docs checks.
