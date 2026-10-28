# 0418 — Apply the bad-value rule in every adapter

Status: OPEN.

Milestone: 5

Depends on: 0417

## Outcome

Every adapter maps non-identity fields through the shared `mapped` helpers, so one bad value is left out with a problem and the reply returns. Unchecked pass-through and silent drops (`unwrap_or_default`, `if let Ok`, `#[serde(default)]`) are gone from adapters and from biomcp-query. The audit ratchet sees every drop form in every adapter file and in biomcp-query, and the `#[serde(default)]` ban and 300-line limit apply to every file under an adapter folder.

- Why: the pre-1.0 code quality review `sdlc/records/2026-10-28-code-quality-review.md`. About 30 of 57 adapters follow other policies; semantic_scholar, erepo, fda_510k, nci, uniprot, litsense, mutalyzer and variantvalidator are named examples. 120 of 143 audit sites are still pending.
- Starts from: `crates/biomcp-adapters`, `crates/biomcp-query`, `tests/ratchets/adapter_values.rs`, `tests/ratchets/source.rs` and the audit inventory.
- Rules: migrate family by family in batches that do not collide with 0408 to 0416; identity fields still fail the item; every pending audit site gets a class.
- Proof: per adapter, a case with one bad non-identity value keeps the reply; the extended ratchet fails on a planted `unwrap_or_default` drop in an adapter and in biomcp-query.
- Changes: the adapters, query modules, ratchets and cases.
- Defers: nothing.
