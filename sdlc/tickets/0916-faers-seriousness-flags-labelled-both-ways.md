# 0916 — faers-seriousness-flags-labelled-both-ways

Status: OPEN.

Milestone: 7

## Outcome

Every FAERS seriousness flag on a card carries a label, whether openFDA sends "1" or "2", so an importer never needs openFDA's 1/2 rule.

## Why

The pharmacovigilance persona's round 2 (bench `personas/pv-scientist/runs/2026-11-13-r2/`, log.md:80, report Q7) scanned 25 fatal cards. Flags set to "1" carry a label, such as `death_label`. Flags set to "2" keep the bare code with no label, as in report 27316597's `lifethreatening:"2"`. The persona wrote that "a "2" flag needs the importer to know 1/2 semantics (undocumented in the reply)".

## Proof

- Proof: a recorded FAERS card case where a "2" flag carries a label saying the report did not record that outcome, and a "1" flag keeps today's label.

## Starts from

- Starts from: 0896's FAERS code labels.

## Keeps

- Keeps: the raw codes beside the labels.

## Defers

- Defers: nothing.

## Changes

- Changes: the FAERS card mapping, `tests/cases/adverse-event/`, regenerated schema descriptions if a field description changes.

## Progress

- Checked openFDA's field reference (`https://open.fda.gov/fields/drugevent.yaml`, 2026-11-13): each of the six `seriousness*` fields still has no `possible_values` and reads "This value is `1` if the adverse event resulted in ..., and absent otherwise"; `2` is not documented. Only the report-level `serious` field documents `2` ("did not result in any of the above"). Live counts: `seriousnessdeath` `2` 6,303,253 and `1` 1,906,622; `seriousnessother` `1` 7,835,031 and `2` 4,056,657. `2` also appears on non-serious reports (`serious:2 AND seriousnessdeath:2`, 3,029,119), and live report 27316597 still carries `lifethreatening:"2"`.
- Because openFDA does not say what `2` means, its label claims only what is safe either way: "The report did not record that the adverse event resulted in ..." with each flag's own clause. A `1` keeps openFDA's wording. Raw codes stay beside the labels, and neither code counts as off-list.
- Built: `2` words in the six seriousness lists; field and type descriptions, the schema descriptions and `docs/cautions.md` now say `2` is labelled. No field added or removed. The recorded case `faers-recorded-labels` (card 27298481, `death` `1`, the other five `2`) now expects the five `2` labels and the unchanged `death_label`. The synthetic budget card carries only `1` flags, so its size is unchanged.
- Paused 2026-11-13 23:05 for the midnight wind-down. The code and test change is c759f76; the gate job j40055 (`make --keep-going landing-gate docs-check schema-release-check` on c759f76) was still waiting for memory on the M5 at 23:04. Pick up here: read `pm job show j40055`. If it passed, report it, because this commit changes only this ticket file. If it failed or never ran, fix what it names (likely rustfmt layout in `crates/biomcp-adapters/src/faers/words.rs`, or a docs-check regeneration if a generated doc shows the old seriousness descriptions), then gate again. The schema change is descriptions only, so schema-release-check should pass.
