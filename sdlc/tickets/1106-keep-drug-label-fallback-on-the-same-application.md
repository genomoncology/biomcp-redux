# 1106 — keep-drug-label-fallback-on-the-same-application

Status: COMPLETE.
Landed: a16f1d7389c46c7904f766be3ef2865f36cc2245

Milestone: 7

## Outcome

A fallback from an unresolved drug brand returns label text only after checking the label belongs to the requested application. If the link cannot be verified, it reports the unresolved brand and offers the candidates.

## Why

Found by a hand solve of research tasks, 2026-11-20. `biomcp get drug Hulio label --source openfda-label --label-section how_supplied` returned `The source did not resolve "Hulio"` and `"application_numbers":["BLA761219"]`. The exact BLA761154 lookup returns Hulio’s application label. The generic name shared across applications allowed a different application’s label to appear as the answer.

## Proof

- Proof: Replay the Hulio brand miss, the cross-application generic result and the exact application result. Assert the label returned is verified or the reply clearly leaves the identity unresolved.

## Starts from

- Starts from: `get_parts` in `crates/biomcp-query/src/drug/aliases.rs` and label lookups in `crates/biomcp-query/src/drug/openfda_label.rs`.

## Keeps

- Keeps: A fallback backed by a verified identity still returns its label.

## Defers

- Defers: Broader changes to drug name ranking.

## Changes

- Changes: Drug name fallback and openFDA label resolution, with cases.
