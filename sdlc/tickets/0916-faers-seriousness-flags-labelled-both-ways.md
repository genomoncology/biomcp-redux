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
- Gate j40055 (`make --keep-going landing-gate docs-check schema-release-check` on c759f76) passed, exit 0, 2026-11-13 23:16. Later commits change only this ticket file.
- Review found the report-level `serious` flag unlabelled. openFDA's field reference documents it both ways (`drugevent.yaml`, `serious`): `1` "The adverse event resulted in death, a life threatening condition, hospitalization, disability, congenital anomaly, or other serious condition", `2` "The adverse event did not result in any of the above". Added those words as `SERIOUS` and an optional `serious_label` on the report header, set only on the FAERS card; MAUDE, FAERS search rows and FAERS section replies leave it out. The checked-in schemas gain only that optional field; the 1.0.0 baseline is unchanged.
- A `serious` or seriousness flag value other than `1` or `2` now has no label and is named in `unmapped_codes`, through the same check as the other coded fields. `docs/cautions.md` says so.
- Proof: `faers-recorded-labels` now also reads card 22814945 (recorded `serious` `2`) and checks its label, and checks card 27298481's `serious` `1` label. The synthetic `faers-code-words` card 100 now carries `seriousnessdeath` `3` and expects it in `unmapped_codes` with no `death_label`.

