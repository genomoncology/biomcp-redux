# 0841 — openfda-text-filters-survive-punctuation

Status: OPEN.

Milestone: 8

## Outcome

An openFDA text filter whose value holds punctuation or a non-ASCII letter, such as an applicant named "Nestlé", returns matches or an honest empty result, never a source failure.

## Why

The review of approvals-by-applicant found that a value containing `! # $ ; ? ^ | ~`, a backtick or a non-ASCII letter gets a 400 from openFDA. The reviewer expects BioMCP to report that as a source failure and did not run it. Other openFDA text filters probably share the path.

## Proof

- Proof: first reproduce on the base commit with one value per character class against each openFDA text filter, and record which fail.
- Proof: a routine case per failing filter asserts a match or an empty result for a recorded punctuated value; it fails on the base commit.

## Starts from

- Starts from: the shared openFDA query building under `crates/biomcp-adapters/src/`, and the applicant filter landed by approvals-by-applicant.

## Keeps

- Keeps: results for plain values unchanged; no wildcard or boolean operator from user text reaches openFDA as syntax; no new source, key or dependency.

## Defers

- Defers: nothing.

## Changes

- Changes: the openFDA adapters' query building, `tests/cases/`
