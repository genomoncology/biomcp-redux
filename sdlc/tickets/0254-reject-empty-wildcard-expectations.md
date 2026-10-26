# 0254 — Reject empty wildcard expectations when the case runner prepares assertions

Status: COMPLETE.
Landed: 40e612f35f151fb3570edec24f1fbafe5473b967

Milestone: 1

## Outcome

The case runner refuses a wildcard pointer paired with a literal `[]`, `equals: []` or `count: 0` when it prepares a step's assertions, before the case runs. The error names the pointer and tells the author to assert on the parent array instead, such as `{'/items': {equals: [], count: 0}}`. Valid wildcard expectations and parent-array assertions behave as they do today.

## Why

Two independent incidents, 0222 in j14790 and 0231 in j14937, wrote three such expectations that can never pass, and each was found only after a full run.

## Proof

- Proof: extend the `boundaries` table in `crates/biomcp/tests/runner/proof.rs` so each of the three spellings fails in `assertions::prepare` with the new message, while `{'': {equals: [], count: 0}}` on `[]` and `{'/*': [null]}` on `[null]` still pass. The existing evaluation-time `empty wildcard selection` rows stay as they are. No source calls.

## Starts from

- Starts from: `crates/biomcp/tests/runner/assertions.rs` `prepare` and its wildcard detection, the `boundaries` proof in `proof.rs`, `crates/biomcp/tests/runner/README.md`, and the 0060 count line for this class.
- PM ruling 2026-10-26: accepted from the coordinator's bounded root cause analysis in m7004.

## Keeps

- Keeps: selector behavior, evaluation-time errors, schema-driven assertion proof and every existing case. No new framework or gate.

## Defers

- Defers: nothing.

## Changes

- Changes: `crates/biomcp/tests/runner/assertions.rs`, `crates/biomcp/tests/runner/proof.rs` and `crates/biomcp/tests/runner/README.md`, with one example of the parent-array form.
