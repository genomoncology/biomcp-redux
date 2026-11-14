# 0911 — drugsfda-whole-read-states-what-it-read

Status: OPEN.

Milestone: 7

## Outcome

When a Drugs@FDA search sorts or windows by original approval, its notice says how many matches it actually read. A read that stops early is marked partial. The several-reads path (500 to 3,000 matches) has recorded proof.

## Why

The review of 0900 (Drugs@FDA company search by original approval, 2026-11-13) found two gaps.
- **Notice:** if a later 500-row read comes back empty early, the loop stops, but the notice still says "read all {matched}" (`crates/biomcp-adapters/src/drugsfda/whole.rs:46`, `crates/biomcp-query/src/drug/drugsfda.rs:42,49`). A regulatory tracker would then trust an incomplete ordering.
- **Proof:** the several-reads loop (`whole.rs:37-52`) has no recorded proof, because Novartis (204) fits in one read.

## Proof

- Proof:
  - a recorded or synthetic case with more than 500 matches, for example a sponsor prefix with 600 to 1,000 applications, ordered across reads;
  - a synthetic case where a later read returns empty early, so the reply is partial and the notice says how many it read out of how many matched.

## Starts from

- Starts from: `crates/biomcp-adapters/src/drugsfda/whole.rs`; `crates/biomcp-query/src/drug/drugsfda.rs`.

## Keeps

- Keeps: 0900's order, windows, 50-row pages and the 3,000 refusal.

## Defers

- Defers: nothing.

## Changes

- Changes:
  - those two files;
  - `tests/cases/drug/`;
  - optionally, the too-large message (`crates/biomcp-query/src/drug/drugsfda.rs:40-45`), which could say that dropping `--sort original-approval` or the window pages in openFDA's own order.
