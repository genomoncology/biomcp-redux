# 0913 — alias-notices-print-plain-names

Status: OPEN.

Milestone: 7

## Outcome

Drug alias notices print names as plain quoted text, never as `Text("…")`, so an agent can quote them.

## Why

The 0898 developer (2026-11-13) found that alias notices printed names through Rust's debug format, such as `Text("EX-66")`, and fixed the new partner notice. The older "returned no rows" and "did not resolve" notices in `crates/biomcp-query/src/drug/aliases.rs` still print names that way.

## Proof

- Proof: recorded or synthetic drug alias cases asserting the exact notice text, with plain quoted names, for the "returned no rows" and "did not resolve" notices.

## Starts from

- Starts from: `crates/biomcp-query/src/drug/aliases.rs`; 0898's partner-notice fix as the pattern.

## Keeps

- Keeps: the notices' meaning.

## Defers

- Defers: nothing.

## Changes

- Changes: `crates/biomcp-query/src/drug/aliases.rs`, `tests/cases/drug/`.
