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

## Progress

- 2026-11-16, found: two openFDA adapters re-declared the page, absence and status check that `crates/biomcp-adapters/src/openfda.rs` owns: `faers/counts.rs` (reaction counts) and `adverse_event_sources/received.rs` (receipt-date counts). Every other openFDA adapter (FAERS, MAUDE, 510(k), PMA, device and drug recalls, Drugs@FDA, drug labels, device search) already used the shared `Envelope`. The two copies dropped `Item::counts` for a reason the review did not record: openFDA `count=` replies carry only `meta.last_updated`, with no `meta.results` window or total (the recorded FAERS count replies show this), so the shared search `Meta` could not read them.
- Decided: `Envelope<T, M = Meta>` takes its metadata type. A `PageMeta` trait says what the metadata gives (publisher date, and whether the row count fits the page): `Meta` keeps the old skip/limit/total arithmetic; the new `CountMeta` reads `last_updated` alone and admits any row count, as both copies did. `Item<M = Meta>` keeps its row check and `counts` hook, so every existing `impl Item for X {}` and the FAERS detail override are unchanged. One `Reply` impl now owns `check`, `data_date` and `check_status` for both kinds. The receipt-date day check moved from the copy's `check` into `Item::check` for `Day`, the same test on every row. `Absence::valid` is now private to `openfda.rs`. Wire shapes and checks are unchanged, so replies stay byte for byte the same.
- Ratchet: `crates/biomcp/tests/ratchets/openfda_owner.rs`. An openFDA adapter file is any file in a top-level adapter module that declares `"https://api.fda.gov`, or any adapter file that names `openfda::` or an openFDA module's `SPEC`. Such a file outside `openfda.rs` fails if it declares `fn check_status` or `impl ... Reply for`. Its test plants both copies back, a new openFDA module with its own `Reply`, and checks that `openfda.rs` itself passes.
- Checks: job j41780 on 55f1bf0f5 regenerated schemas and docs (nothing changed), passed clippy, rustdoc and the 32 adverse-event, 48 diagnostic and 83 drug cases unchanged, which replay every openFDA adapter including both count paths. Its ratchets failed twice on the new test itself: the proof planted a copy with no openFDA module beside it, and a `#[test]` outside the listed locations. The proof now plants `faers/mod.rs` with the openFDA host and runs from `ratchets.rs`. Job j41785 on 7982338cb (the branch merged with origin/main 7aeecb3d8) passed all 51 ratchets; its only `cargo fmt` change was one wrapped assert, committed as 495cd46a5.
- Gate: job j41787 passed `make --keep-going landing-gate docs-check schema-release-check` on 495cd46a5 (the branch merged with origin/main 7aeecb3d8): all 1,006 cases, the changed tool proofs, the changed library tests, docs-check and schema-release-check.
