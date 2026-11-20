# 1089 — who-lookups-on-stale-data-do-not-say-absent

Status: COMPLETE.
Landed: a8861ac877d96faee94a7dda3149df136ffd7eeb

Milestone: 7

## Outcome

A WHO prequalification lookup or a WHO essential-medicines lookup that misses on an out-of-date copy does not answer `absent` or `no-record`. It answers `failed`, names the stale source and offers the sync or fresh-read step.

## Why

Found by the review of ticket 1087 (2026-11-20) and reproduced by its developer with replay fixtures; present on main. `biomcp get drug imatinib markets --source who-pq` returns `no-record` with zero matches when an expired WHO bundle lacks the name, and the `essential-medicines` get returns `absent` from an expired search page with no exact match. Both paths hold the freshness state before they classify the miss. A caller is told WHO does not list a drug when the cause is out-of-date data.

## Proof

- Proof: replayed cases for both calls with the WHO copy stale, in which the status is not `absent` or `no-record`, the stale source is named and its sync or fresh-read step is offered; each fails on the base commit.
- Proof: with the copy current, both calls answer as they do today.

## Starts from

- Starts from: `crates/biomcp-query/src/drug/who.rs` near line 148 and `crates/biomcp-query/src/drug/who_eml.rs` near line 43; ticket 1087's branch holds the reproducing cases its developer wrote.
- PM decision, 2026-11-20, carried from ticket 1081: only a lookup whose sources were all current may say a name was not found.

## Keeps

- Keeps: a stale lookup that finds a record still answers `partial`, marked stale.

## Defers

- Defers: nothing.

## Changes
