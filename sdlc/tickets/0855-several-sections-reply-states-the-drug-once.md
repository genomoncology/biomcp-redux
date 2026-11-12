# 0855 — several-sections-reply-states-the-drug-once

Status: COMPLETE.
Landed: 2f9af0affa023aa8cac3bc75413ec54ac816e5db

Milestone: 7

## Outcome

A several-sections drug reply (`biomcp.drug-sections/1`, from 0846) states the drug's identity, including the name-lookup block, once at the top level instead of in every part, and a drug name with several label headings reads the label record once, as a label ID already does.

## Why

0846's review measured about 1.5 KB (about 400 tokens) of envelope repeated per part, mostly the identity; six headings by drug name repeat about 7.7 KB. The schema is unreleased (1.0 is not published), so changing it now costs no new major.

## Proof

- Proof: a routine case reads a drug by name with three label headings and asserts the identity appears once, each part keeps its own status, page and next calls, and the recorded replies show one label record fetch; a case with an unknown heading inside the list is refused as a usage error.

## Starts from

- Starts from: 0846 (accepted at 709d6152f) and its review notes; `crates/biomcp-query/src/drug/`; `biomcp.drug-sections/1`.

## Keeps

- Keeps: single-section replies unchanged; combined status rule (partial if any part failed or partial); per-part paging; typed tool list within 1% of main.

## Defers

- Defers: naming the bad value in usage errors (a shared error-schema change for every option, its own ticket if wanted).

## Changes

- Changes: `crates/biomcp-query/src/drug/`, the drug-sections schema, `tests/cases/drug/`

## Notes

- Review: `crates/biomcp-types/schemas/drug-sections.json` was deleted and regenerated at `biomcp.drug-sections/1` rather than moved to /2. Deleting the file skips the schema compatibility check in `crates/biomcp-types/tests/schemas.rs`, which is right only because /1 came from 0846, has never been released, and is not in `release/schema-baseline.json` (1.0.0 release candidate). A schema already in a released baseline must not be changed this way.
