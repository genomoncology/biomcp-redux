# 0999 — openfda-reply-checks-have-one-owner

Status: OPEN.

Milestone: 7

## Outcome

One shared module decides whether an openFDA reply page is valid, and every openFDA adapter uses it.

## Why

Finding F5 of `sdlc/records/2026-11-14-adversarial-review.md`: the openFDA page-envelope check exists in three copies, and the copies have drifted (they drop `Item::counts`). On main 3f14da1a6, `adverse_event_sources/received.rs:56` and `faers/counts.rs:50` still re-declare `check_status`. F1's shared escaper (0841, 0976) landed without this, and the review's stance 2 says one publisher fact has one owner.

## Proof

- Proof: no openFDA adapter declares its own page or status check; a grep in a ratchet test shows it.
- Proof: the existing replayed openFDA cases pass unchanged.

## Starts from

- Starts from: the openFDA `Envelope`, the two copies named above, and `biomcp-types/src/openfda.rs`.

## Keeps

- Keeps: every reply byte for byte.

## Defers

- Defers: nothing.

## Changes

- Changes: the openFDA adapters, one shared module, and a ratchet test.
