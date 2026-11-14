# 0926 — label-says-when-a-section-is-absent

Status: COMPLETE.
Landed: 2ee1af603bedeb81acfe585aeeac4ddc321919d2

Milestone: 7

## Outcome

When a drug label section is asked for and that label has no such section, the reply says so in plain words, naming the label: for example "KEYTRUDA's label (set 9333c79b…, version 229) has no boxed_warning section". It is a note or problem the agent can quote, not only `label.state: empty`. The section list still comes back. A section name that no label can have is still refused, as today.

## Why

The drug-safety persona on current Bot (bench `personas/pv-scientist/runs/2026-11-14-bot/`, j40190) asked `get drug pembrolizumab label --label-section boxed_warning`. KEYTRUDA's label has no boxed warning, so the empty section was correct, but the reply was `status: complete`, `label.state: empty` with no note. The agent had to work it out from the section list, and its report called this "silent", "the single most dangerous behaviour in a label-vs-signal workflow". A safety scientist must be able to cite "the label carries no boxed warning" as a fact the tool stated.

## Proof

- Proof: a case with a recorded openFDA label reply for a label without a boxed warning (KEYTRUDA's, as the persona saw it) asserts the note. An existing case with a boxed warning keeps its text and has no such note.

## Starts from

- Starts from: the label section reader in the drug query crate (`label_section`, `label.state`); the persona's call at bench `personas/pv-scientist/runs/2026-11-14-bot/`, and the record in `0045-persona-pv-scientist-bot04923f2` on the M5.

## Keeps

- Keeps: the reply schema within the 1.0.0 baseline rules (an added allowed message or optional field only); label size caps.

## Defers

- Defers: nothing.

## Changes

- Changes: the drug label section code, a case, generated docs if a message is added.

## Progress

- 2026-11-14 measured (job j40257, live openFDA): the bot build already said "this label has no boxed_warning section" in `result.hint`, but it neither named the label nor came early: the hint began at byte 1,611 of the 2,127-byte reply, after the 34-name section list, and the persona read only the first 1,500 bytes and looked for a top-level `hint`. KEYTRUDA (set 9333c79b-d487-4538-a9f0-71b91a02b287, version 229) and KEYTRUDA QLEX (set 097d166f-…, version 19) both lack openFDA's `boxed_warnings` field.
- Changed: the hint now names the label, "KEYTRUDA's label (set 9333c79b-d487-4538-a9f0-71b91a02b287, version 229) has no boxed_warning section; …" (brand, else generic; "The label (set …)" when the source gives no name; version left out when the source gives none). `DrugLabel.hint` now serializes after `identity` and before `label` and `sections`, so it starts at byte 752. No schema change: same optional field and description.
- Proof: `tests/cases/drug/label-no-boxed-warning.yaml` replays the persona's call (`get drug pembrolizumab label --label-section boxed_warning`) over the two openFDA replies recorded whole in j40257 (1.07 MB and 2.15 MB); reply 2,189 bytes, ceiling 2,520. `label-section-absent` now expects RASONQUE's named note; `several-sections-by-name` asserts the boxed-warning part keeps its text and has no hint or section list.
