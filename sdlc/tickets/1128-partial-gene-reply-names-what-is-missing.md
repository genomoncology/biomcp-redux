# 1128 — partial-gene-reply-names-what-is-missing

Status: OPEN.

Milestone: 7

## Outcome

Every reply whose status is `partial` names, in `incomplete`, each part that is missing and the source that failed to supply it, so an agent can retry or report the gap. A gene batch read with MyGene unavailable is the known case.

## Why

The developer for multi-ID syntax (1103) found, 2026-11-22, that disabling MyGene on the existing `BRAF 3845` gene replay gave a `partial` reply with no `incomplete` entry. An agent then sees the reply is short but not what is short, and may state a gap as a fact.

## Proof

- A replay case reads the `BRAF 3845` gene batch with MyGene disabled and asserts `/status: partial` and an `incomplete` entry naming MyGene and the part it would have supplied. It fails on main first.
- A sweep or contract test asserts that no reply in the existing cases has `partial` status with an empty or missing `incomplete`, unless the case records why.

## Starts from

- Starts from: `tests/replies` gene replay used by the `BRAF 3845` batch case; ask the 1103 developer's branch `ticket/1103-make-multi-id-syntax-consistent` for the exact command if needed.

## Keeps

- Keeps: every existing reply that is complete; the status values.

## Defers

- Defers: nothing.

## Changes

- Changes: where gene batch replies are assembled, and wherever else the sweep finds a partial reply without `incomplete`.
