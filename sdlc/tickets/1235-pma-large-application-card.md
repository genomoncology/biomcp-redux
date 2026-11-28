# 1235 — pma-large-application-card

Status: COMPLETE.
Landed: 8ecc6eee7bf8c45b66d0b7ff4e1af2e2865c4757

Milestone: 10

## Outcome

`get diagnostic <PMA number> --source fda-pma` returns the original application's card for a PMA with more than 100 rows, with its supplements paged and their total stated.

## Why

Long-lived devices, the ones with the most history, return no card today.

## Evidence

Checked 2026-11-27 against the live source and the 1.0.0 build at 4efd1dcb2426. Full tables: [source reach census](../records/2026-11-27-source-reach-census.md).

- Failing case: `biomcp get diagnostic P890003 --source fda-pma` returns `status: partial`, no card and no paging, with problem `meta.results.total collection.incomplete`. `P950037` does the same, as do the `supplements` and `regulatory` sections. Only `--supplement S486` works.
- The diagnostic slice design chose this limit; the cases use PMAs of 5 to 62 rows.
- Related: `biomcp get diagnostic H140006` (a humanitarian device exemption) fails on every source, and six of 244 companion-diagnostic rows have no device ID.

## Proof

- Proof: an outside-in case on recorded replies: P890003 returns its original-approval card and the first supplements page with a total and a `next` step.

## Starts from

- Starts from: `crates/biomcp-adapters/src/fda_pma/`, `crates/biomcp-query/src/diagnostic/`.

## Keeps

- Keeps: PMAs under the limit return the same card.

## Defers

- Defers: humanitarian device exemption records as a route; the companion-diagnostic group rows dropped by `--biomarker`, which ticket 1056 defers.

## Changes

- Changes: `crates/biomcp-adapters/src/fda_pma/`, `crates/biomcp-query/src/diagnostic/`, diagnostic cases.
