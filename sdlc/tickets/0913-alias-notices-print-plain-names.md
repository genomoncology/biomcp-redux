# 0913 — alias-notices-print-plain-names

Status: COMPLETE.
Landed: f62614b6d07c70f5bbfef0e0c658aab23992d0be

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

## Progress

- Five drug alias notices in `aliases.rs` printed names through Rust's debug format: "returned no rows", "did not resolve", the two no-alias hints ("has candidate records for" and "found no matching record for"), and the ChEMBL "preferred-name search" notice, which printed `Some(Text("…"))`. All now print the plain quoted name, as 0898's partner notice does. The wording is unchanged.
- Proof: `tests/cases/drug/source-aliases.yaml` and `source-aliases-ambiguous.yaml` (synthetic alias replies) now assert each notice's exact text, for example `The search for "EX-66" returned no rows. mychem record KTUFNOKKBVMGRW-UHFFFAOYSA-N supplies the name "Canonical therapy"; …`. The two hints come second, after the source's dataset-date notice.
