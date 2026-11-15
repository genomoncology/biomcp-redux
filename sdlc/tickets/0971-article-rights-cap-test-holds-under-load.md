# 0971 — article-rights-cap-test-holds-under-load

Status: OPEN.

Milestone: 7

## Outcome

`lookups::papers_past_the_lookup_cap_say_they_went_unchecked` (crates/biomcp-query/tests/article_rights/lookups.rs) passes every time on a loaded M5, or fails only for a real defect, with the same checks.

## Why

0967's first gate (j41195, 2026-11-15) failed this test with `page` None at lookups.rs:124 on an unrelated change; the unchanged rerun (j41223) and other gates that day passed, so it is load-dependent and can fail a landing for nothing.

## Proof

- Proof: the cause named from the code (likely the run's time budget against 25 stub lookups while the M5 is busy); the test run many times in one job under parallel load passes every time; a deliberately broken cap still fails it.

## Starts from

- Starts from: crates/biomcp-query/tests/article_rights/lookups.rs:116-135 and its `fixture::live` stub; j41195's log.

## Keeps

- Keeps: what the test checks.

## Defers

- Defers: nothing.

## Changes

- Changes: the test or its fixture, or the code if the flake is a real timing bug in the product.
