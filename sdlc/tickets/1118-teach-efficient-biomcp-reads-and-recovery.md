# 1118 — teach-efficient-biomcp-reads-and-recovery

Status: COMPLETE.
Landed: f57d366f84771d21b9a691101d1abdcdaa23a764

Milestone: 7

## Outcome

The BioMCP skill gives short copyable examples for the supported path and names limits that affect the completeness or size of a reply.

## Why

Found by a hand solve of research tasks, 2026-11-20. `biomcp skill render` says `A refusal names the valid values and a next step: use them.` and `Stop paging at a page's total.` The hand solves needed better guidance on byte limits, data sources, field selection and continuations.

## Proof

- Proof: Run the examples against saved replies. Cover batch syntax and caps, combined label selectors and offsets, history evidence and resuming, trial text totals and arm tags, site coordinates and paging, trial search fields and flow-note paths, curation source direction and stale GenCC, HPO versus Monarch, pathway counts and constraint release, approval document IDs, and companion diagnostic source coverage and application keys. Include how to tell Drugs@FDA approval history and product flags from Purple Book presentation licensure, and how to check application IDs after a label fallback.

## Starts from

- Starts from: `skills/SKILL.md`, `docs/cautions.md` and the current CLI help.

## Keeps

- Keeps: Examples use public records and keep the skill short.

## Defers

- Defers: Document new command behavior as each capability lands.

## Changes

- Changes: `skills/SKILL.md` and its existing docs checks.
