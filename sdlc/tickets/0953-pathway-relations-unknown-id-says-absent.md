# 0953 — pathway-relations-unknown-id-says-absent

Status: OPEN.

Milestone: 7

## Outcome

`pathway genes` and `pathway events` with an ID the source doesn't hold (for example WP9999999 or R-HSA-9999999) say `absent`, as `get pathway` already does, instead of `failed` with reason `not-found`. A real failure (timeout, bad reply) still says `failed`.

## Why

0940's developer and review (2026-11-14): `crates/biomcp-query/src/pathway/relation.rs:81-84` treats any error from the identity check as a failed read, so an agent can't tell an unknown pathway from a broken source; `absent` means the source doesn't hold the identity (`crates/biomcp-types/src/response/status.rs:8-9`).

## Proof

- Proof: recorded cases for an unknown Reactome ID and an unknown WikiPathways ID through `pathway genes` and `pathway events` show `absent` and a source marked ok; they fail on main.

## Starts from

- Starts from: `relation.rs:81-84` and the card reads at `relation.rs:272-281` and `gpml_relation.rs:29-39`; 0940's case `tests/cases/pathway/empty-relations.yaml`.

## Keeps

- Keeps: 0940's `complete` for a found pathway with an empty list; `failed` for real source failures.

## Defers

- Defers: nothing.

## Changes

- Changes: `crates/biomcp-query/src/pathway/`, pathway cases.
