# 0979 — name-expanded-article-searches-state-their-totals

Status: OPEN.

Milestone: 7

## Outcome

An article search that queries one source once per drug name (Semantic Scholar, LitSense) says in plain words that the per-name counts overlap and are not added, instead of showing rows read as an unlabelled total.

## Why

0873 review (2026-11-15): when a drug name expands into one request per name, the source appears several times, its count is dropped, and the page total falls back to rows read with nothing labelling it until the end of the window. The item 0873 listed (one source named twice) is now refused as a usage error, so this path was left.

## Proof

- Proof: a routine case with a drug name that expands to several names, on recorded replies (Semantic Scholar replies synthetic, per 0920), asserts the sentence on the first page; it fails on the base commit.

## Starts from

- Starts from: 0873's changes to `crates/biomcp-query/src/article/page.rs` and `article/sources.rs`.

## Keeps

- Keeps: never sum or invent totals; the reply schema.

## Defers

- Defers: nothing.

## Changes

- Changes: article search paging notices, a case.

## Progress
