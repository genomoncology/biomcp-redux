# 1058 — absent-label-section-says-so

Status: OPEN.

Milestone: 7

## Outcome

When a caller asks for a label section the label does not have, or a section name openFDA does not use, the reply says the drug's label was found, says that section is not on it, and lists the sections the label does have.

## Why

The present reply says the drug was not found while counting one record found, so a caller concludes the drug has no label.

## Proof

- Proof: `biomcp get drug Aimovig label --source openfda-label --label-section warnings_and_precautions` names the label, says the section is absent and lists `warnings_and_cautions` among the sections present; it fails on the base commit.

## Starts from

- Starts from: Seen on main be96d8d40 (1.0.0), Linux, 2026-11-19, while agents used the command line build on real tasks. The command above replies `absent` and "found no matching record for Aimovig" while its source line counts 1 record found. openFDA's field is `warnings_and_cautions`, and that name returns the section.
- `warnings_and_precautions` is what the printed label calls the section, so callers will keep trying it. PM decision, 2026-11-19: accept it as another name for `warnings_and_cautions`, and say in the reply which openFDA field was read.

## Keeps

- Keeps: a drug with no label at all still says no matching record.

## Defers

- Defers: other printed-label names that differ from openFDA's field names; the developer lists any found.

## Changes

- Changes: the label section read, its reply text, cases and help.
