# 0976 — openfda-cleaned-text-notices-and-combining-accents

Status: OPEN.

Milestone: 7

## Outcome

openFDA text cleaning keeps a word whole when its accent is typed as a separate combining mark, and its notices name the option the user typed and never say a whole search matched nothing when only one value did.

## Why

0841 review (2026-11-15, accepted at a496b941a), three notes left for later:

- `crates/biomcp-types/src/openfda.rs:39`: a combining accent mark (U+0300–U+036F, as macOS pastes) becomes a space, so "pu" + combining ¨ + "mp" searches `pu mp`. The notice shows it, but the search is wrong; before 0841 device recall `--name` refused such marks.
- `crates/biomcp-query/src/openfda_text.rs:22`: an emptied value inside an OR group (`--product-code FRN --product-code ®`, FAERS `aspirin --drug ®`) returns rows, yet the notice says the option "matched nothing".
- `crates/biomcp-query/src/diagnostic/mod.rs:63`: device recall maps `--firm` to `--manufacturer` before the notice, so `--firm "Abbott#"` is reported as `--manufacturer`.

## Proof

- Proof: routine cases for each of the three, with recorded openFDA replies, that fail on the base commit.

## Starts from

- Starts from: 0841's shared builder `crates/biomcp-types/src/openfda.rs` and notice helper `crates/biomcp-query/src/openfda_text.rs`.

## Keeps

- Keeps: plain values byte-identical; no user text reaches openFDA as syntax; the reply schema.

## Defers

- Defers: nothing.

## Changes

- Changes: openFDA text cleaning and notices, `tests/cases/`.

## Progress
