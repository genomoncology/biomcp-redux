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

## Progress

- Measured live 2026-11-14 in j41025 (branch with the fix): Reactome `/data/query/R-HSA-9999999` answers 404 NOT_FOUND, which the Reactome card adapter already reads as "not held"; WP9999999 is not in the stored WikiPathways catalog. With the fix, `pathway genes WP9999999`, `pathway genes R-HSA-9999999` and `pathway events R-HSA-9999999` answer `absent`, exit 1, source ok, matching `get pathway R-HSA-9999999`. Before it (0940's run on main) they answered `failed` with reason `not-found`. `pathway events` takes only Reactome IDs (a WP ID is refused as invalid input), so there is no WikiPathways events case.
- Changed: the relation fetch (`relation.rs` for Reactome, `gpml_relation.rs` for WikiPathways) now returns "not held" instead of a made-up not-found error when the card read says the source lacks the pathway; `relation::read` then marks the item `absent`, leaves the source ok and saves no relation snapshot. Any real error from the card or relation read still fails as before.
- Cases: `tests/cases/pathway/empty-relations.yaml` gains the three unknown-ID steps, with the Reactome 404 recorded in j41025. `tests/cases/pathway/unavailable.yaml` gains `pathway genes` and `pathway events` on its synthetic Reactome outage, which still answer `failed` with reason `unavailable`. 0940's WP5034 `complete` steps are unchanged.
- Seen again: `sync wikipathways` failed with no reason in j41025, as 0940 saw; `pathway genes WP9999999` still read the catalog and answered `absent`. Not investigated.
- Gate j41035 passed on 9080e6916 (branch = origin/main ba54ad575 merged with 0940's branch ed7bb0e71, plus this fix). rustfmt check passed in j41025.
