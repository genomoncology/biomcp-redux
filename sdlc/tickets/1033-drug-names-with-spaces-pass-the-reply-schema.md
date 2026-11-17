# 1033 — drug-names-with-spaces-pass-the-reply-schema

Status: OPEN.

Milestone: 7

## Outcome

`get drug "dabigatran etexilate" ...` and every other drug name with a space answers with a reply that validates against its schema, on every drug source: the reply schema's drug ID pattern accepts what the command parser accepts.

## Why

1027's developer (2026-11-17) found that a drug name with a space gets a reply that fails schema validation, because the parser accepts spaces in drug IDs and the reply schema's ID pattern does not. This was already true on main. Salt and full names ("dabigatran etexilate", "imatinib mesylate") are common in persona questions.

## Proof

- Proof: a case for a spaced drug name on at least two drug sources asserting the reply validates; it fails on the base commit. The schema release check accepts the widened pattern, or the 1.0.0 review entry records it.

## Starts from

- Starts from: the drug ID pattern in the drug reply schemas and the parser's drug ID rule.

## Keeps

- Keeps: every ID form that validates today.

## Defers

- Defers: nothing.

## Changes

- Changes: the pattern, schemas, a case, one release-notes line.
