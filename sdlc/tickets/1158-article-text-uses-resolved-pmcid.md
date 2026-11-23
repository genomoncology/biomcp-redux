# 1158 — article-text-uses-resolved-pmcid

Status: OPEN.

Milestone: 7

## Outcome

`article text PMID` reads the open PubMed Central copy when the detailed record it fetches names one, instead of reporting `not-open`.

## Why

Found 2026-11-23 in the BioMCP 1.0 adversarial review (Astra, snapshot cf9a822889ce), reproduced by the reviewer under replay. For PMID 28944238 with the PMC link removed from the PubMed summary but kept in the detailed record, `article text 28944238` returned `complete`, showed `PMC5606870` on its card, and marked full text `not-open`; `article text PMC5606870 --part 1` returned the body. The fallback decides before reading the detailed card (`crates/biomcp-query/src/article/text.rs:77`, `article/get.rs:137`).

## Proof

- The replay above returns the body from the PMC copy, rights-checked as today; if that read cannot finish within the deadline, the reply names its outcome instead of `not-open`.

## Changes

- `crates/biomcp-query/src/article/text.rs` and `get.rs`, and an article replay case.
