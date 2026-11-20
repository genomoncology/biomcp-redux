# 1112 — name-the-cause-of-drug-get-refusals

Status: OPEN.

Milestone: 7

## Outcome

A drug get refusal names the unsupported option, section or field and gives a working next command that preserves the request it can express.

## Why

Found by a hand solve of research tasks, 2026-11-20. `biomcp get drug Keytruda label --source openfda-label --label-section clinical_studies --find KEYNOTE-189` returned `error.option is not accepted with the other options as given`. `biomcp get drug Keytruda label --source openfda-label --label-section clinical_studies --fields=text` offers `"valid_values":["characters","source_offset","source_part"]` and leaves the existing text path unexplained. `biomcp get drug BLA125057 regulatory --source drugsfda` says `choose one of the source selector values` while listing the sections `approvals` and `label-history`. `biomcp get drug BLA125057 biosimilars --source drugsfda` says `unknown section: error.option is not a section of this record; valid_values lists them`.

## Proof

- Proof: Replay the four calls. Check that current-label `--find` explains its actual scope on any build where it is unavailable, the text-field refusal shows a working text read, and regulatory and biosimilar calls distinguish section from source.

## Starts from

- Starts from: The current label and regulatory argument checks and the shared usage wording from tickets 1064 and 1085.

## Keeps

- Keeps: Error codes and strict validation stay in place.

## Defers

- Defers: New label phrase search and the Purple Book capabilities have their own tickets.

## Changes

- Changes: Drug get validation, field refusal guidance and cases.
