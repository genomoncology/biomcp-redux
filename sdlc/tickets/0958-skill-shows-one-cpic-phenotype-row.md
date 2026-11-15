# 0958 — skill-shows-one-cpic-phenotype-row

Status: COMPLETE.
Landed: 3113a8a71b28c0a1ea78e939f81983271bee65c6

Milestone: 7

## Outcome

The skill (`skills/SKILL.md`) has one line, like 0937's DDInter pair line, showing how to ask CPIC for one phenotype's recommendation: `biomcp get pgx --gene CYP2C19 --source cpic recommendations --drug-id RxNorm:32968 --phenotype "Poor Metabolizer"`.

## Why

dx-lab-director round 3 (bench 0050, 2026-11-14), the first persona run with the build's own skill: the agent ran the skill's DDInter pair line word for word and closed that half of Q9, but for the third round it left `--phenotype` unused, read the first 5 of 24 CPIC rows, and missed the Poor Metabolizer advice (calls 78, 90; bench `personas/dx-lab-director/runs/2026-11-14-r3/analysis.md`). A worked line is the coaching this agent follows.

## Proof

- Proof: the skill line runs as written in a recorded case (it returns the Poor Metabolizer row); skill size and docs checks pass within the measured-size-plus-15% caps.

## Starts from

- Starts from: 0937's skill line and its case; 0936's `--phenotype` filter.

## Keeps

- Keeps: the skill's other lines and its size budget (trim nothing else to fit; raise the cap by the measured rule).

## Defers

- Defers: nothing.

## Changes

- Changes: `skills/SKILL.md`, a pgx case if none covers the line.

## Progress

- Skill (`skills/SKILL.md`) gains one line after the DDInter pair line: "For one phenotype's CPIC advice, ask CPIC for that phenotype:" with the ticket's command word for word. The skill is 4,038 bytes (was 3,857), inside its 4,096-byte cap in `crates/biomcp/build.rs` and `scripts/check_docs.py`, so no cap moved and nothing else was trimmed.
- `tests/cases/pgx/recommendations-filter-by-phenotype.yaml` (0936's recorded case) gains a step that runs the skill line in its written argument order (section after `--source`) against the same recorded CPIC replies; it expects the three Poor Metabolizer rows and the "Avoid clopidogrel if possible" advice. `tests/cases/local/skill.yaml` asserts the line in the rendered skill.
