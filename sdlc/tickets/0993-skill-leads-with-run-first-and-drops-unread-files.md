# 0993 — skill-leads-with-run-first-and-drops-unread-files

Status: COMPLETE.
Landed: cf0ba84d9bb999474f740c5c586216e8adf8d192

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

## Progress

- 2026-11-16, use-case reads: searched the 43 kept persona run folders on the bench's main (`personas/*/runs/*/`, bench commit 8fec383, 157 files, rounds 2026-11-12 through 2026-11-16) for `use-cases`, `use-case` and `skill show`: 0 hits each. `bot find use-cases --runs` indexes none of these folders (they hold Markdown records, not call records) and reports 0; over the dx-lab-director round-1 call records it finds 1 hit, the skill's own text in call 1, not a file read. `biomcp skill list` appears in 5 runs, all from 2026-11-12 (comp-bio, dx-lab-director 12b, patient-navigator 12b, trial-ops 12 and 12b); each listed the 17 titles, found none fitting, and opened none. So no run opened a use-case file.
- Change: `skill install` writes only `SKILL.md` and its stamp; `skill status` judges only those files; the plugin zip leaves out `skills/use-cases/`. The 17 files (2,618 words) stay in the repo: the binary still embeds them for `skill list`, `skill NAME` and the first-read examples, and `scripts/check_front_map.py` still checks them. The install case asserts no `use-cases` folder is written; `release/test_workflow.py` asserts the zip leaves them out. Release notes say so under "Agent skill".
- Skill order: no change. At origin/main 616e68d22 (and at round 4's 95a74e246) the run-first paragraph (line 13) already directly follows the command list (lines 8-11), so the move this ticket asks for is already the order round 4 read. `skills/SKILL.md` stays 4,085 bytes.
