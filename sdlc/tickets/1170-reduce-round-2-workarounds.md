# 1170 — reduce-round-2-workarounds

Status: COMPLETE.
Landed: bd0c70dac8f292959665b280675abc7ba3a2b95b

Milestone: 7

## Outcome

Agents get from BioMCP what they went around it for in round 2: the population record for variants BioMCP could not join to gnomAD, the text of an FDA approval letter BioMCP already links, and whole-paper search through `article text --find` and `--save`, which the skill and help teach where an agent first reads about full text. Tables in full text come through in `article text`, or the reply says they do not.

## Why

Round 2 eval (the eval repository, runs/round-2-sol.md; batches r2-apb-{l,b,o}, per-run call logs): with BioMCP available, CLI agents still made about 75 direct gnomAD calls (BioMCP's variant card reported `no-join-id` for population, e.g. rs80282562 and ClinVar 864024), about 60 FDA approval-letter fetches (BioMCP returned the letter URL under `application_docs` but not its text, e.g. BLA761352 approvals), and about 75 whole-paper Europe PMC fullTextXML reads plus about 40 PMC article page reads to search a paper for a number. Product owner, 2026-11-23: "Is there anything we can do to make BioMCP support more of this stuff?"

## Proof

- Proof: replays: the named variants return gnomAD population data or say exactly why they cannot be joined; a call reads the BLA761352 approval letter text (FDA records may be fetched by BioMCP; agents never curl FDA sites); `article text` on a paper with a results table returns the table's rows or marks them missing; the skill's full-text guidance names `--find` and `--save`. Existing ceilings and limits stay as they are.

## Changes

- Changes: the gnomAD join, a reader for linked FDA letters, full-text table handling, the skill and help text, and replay cases. Split into commits per item.
