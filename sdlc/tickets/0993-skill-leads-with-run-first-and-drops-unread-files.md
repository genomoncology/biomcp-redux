# 0993 — skill-leads-with-run-first-and-drops-unread-files

Status: OPEN.

Milestone: 7

## Outcome

The skill puts "run the command first; a refusal names the valid values" right after the command list, where agents read it before their first call. The ten `skills/use-cases/` files (2,618 words) either show a measured use or are no longer shipped.

## Why

A review of the persona bench, passed on by Ian on 2026-11-16, applied the Evidence team's `notes/streamlining-agents.md`. Two of its findings concern the skill:

- **The run-first line comes too late.** Agents still read about seven help pages before their first real query, though the skill says to try the command first (0922). The note's practice 5 is that order is part of the instruction.
- **The use-case files go unread.** No persona run since 2026-11-12 has opened one, while `skills/SKILL.md` sits at 4,085 of its 4,096 bytes.

## Proof

- Proof: the bench's kept runs since 2026-11-12, searched with `bot find --runs`, show how often a use-case file was opened. If it is zero, the folder is no longer installed or packaged (`crates/biomcp/src/skill/install.rs`), and the release notes say so (Ian's rule, `sdlc/decisions/2026-11-16-review-follow-ups.md`, item 5). Otherwise, Progress records where it is read.
- Proof: the skill's line order changes so the run-first line follows the command list; the skill size check passes.
- Proof: the next persona pair after landing counts help calls before the first real query, against round 4's seven. This is read on the bench, not gated.

## Starts from

- Starts from: `skills/SKILL.md`, `skills/use-cases/`, `skill/install.rs`, 0922.

## Keeps

- Keeps: the 4,096-byte skill limit; the skill's other lines, unless a moved line frees a duplicate.

## Defers

- Defers: nothing. Lands before the 1.0 persona baseline, not during it.

## Changes

- Changes: the skill, its packaging, the notes, the skill checks.
