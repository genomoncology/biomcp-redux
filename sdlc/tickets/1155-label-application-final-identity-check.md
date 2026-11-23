# 1155 — label-application-final-identity-check

Status: OPEN.

Milestone: 7

## Outcome

An explicit FDA application lookup never returns label text whose final read names a different application; it refuses or says the label could not be matched.

## Why

Found 2026-11-23 in the BioMCP 1.0 adversarial review (Astra, snapshot cf9a822889ce), reproduced by the reviewer under replay. `get drug BLA761154 label --source openfda-label --label-section how_supplied`, with the second recorded response changed to name `BLA761219`, exited 0, `complete`, and returned BLA761219's text. Ticket 1106 put a final identity check on the brand fallback; the explicit application route skips it (`crates/biomcp-query/src/drug/label_application.rs:51`).

## Proof

- A replay that keeps the SPL set ID constant and changes the application between reads returns no label text attributed to the requested application, with a status that says why.

## Changes

- `crates/biomcp-query/src/drug/label_application.rs`; reuse the existing verifier, no extra source request.
