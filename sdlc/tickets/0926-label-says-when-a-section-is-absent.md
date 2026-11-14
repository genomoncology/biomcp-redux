# 0926 — label-says-when-a-section-is-absent

Status: OPEN.

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
