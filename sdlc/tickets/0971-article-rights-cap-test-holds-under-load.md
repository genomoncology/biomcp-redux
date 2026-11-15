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

## Progress

- 2026-11-15, cause: a test problem, not a product timing bug. The test's search finds 25 papers; the license lookup checks the first 20 (the cap) inside a 3-second budget (`crates/biomcp-adapters/src/article_rights/lookup.rs`). Each checked paper with a PMC record costs two record requests, so the old fixture sent 40 record requests through a network stub that serves one connection at a time. On a loaded M5 the budget ran out partway, as designed, and the client dropped the request it had just opened. The stub treats a dropped connection as a fatal fixture error and stops listening (`crates/biomcp-test-support/src/network_stub.rs`), so the next request, the page's title read, failed with reason `Proxy`; the page read returns that error, so the reply had no page. j41195 shows it: right before the failure, an article-rights stub stopped at ordinal 31 (`CONNECT header`, `failed to fill whole buffer`), which is litsense, one summary read and 28 record requests (14 papers checked), then the dropped 31st.
- Users can't hit this: a real network keeps serving after one dropped request, so the title read works and the reply is Partial with the unchecked papers marked `unavailable`, which is the designed behaviour.
- Reproduced in j41283 (6f45f8c10, the old fixture with a panic message that prints the outcome): 240 runs of the `lookups::` tests, 12 parallel workers plus a CPU hog per core, 1 failure: `page: None`, status Failed, PubMed `Failed` with reason `Proxy`, LitSense ok with 25 papers and the unchecked-license notice.
- Fix (test only): in this test only the first page's three papers (2001-2003) have PMC records. The others are settled by their PubMed summary alone, so the cap still decides which papers go unchecked, but the lookup now sends 6 record requests instead of 40 and stays well inside its budget under load. The test's checks are unchanged; its first unwrap now prints the outcome on failure.
- Proved in j41287 (42196f37f): `cargo fmt --check` clean; 400 runs of the `lookups::` tests, 16 parallel workers of 25 runs each plus a CPU hog per core, all 400 passed the cap test with no failure of any kind. With the cap deliberately broken (`MAX_RECORDS` 20 to 100), the same test fails: `capped pools are not whole`, left `Complete`.
