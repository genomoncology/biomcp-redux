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

- 2026-11-15: All three fixed on `ticket/0976-openfda-notices`.
  - Combining accent marks (U+0300 to U+036F) are now dropped instead of spaced, so a decomposed `infusion pümp` searches `infusion pump`. New step in `tests/cases/diagnostic/recall-punctuation.yaml` replays the existing `product_description:"infusion pump"` recording; on the base commit the request was `"infusion pu mp"`, which is not recorded.
  - A value emptied inside an OR group (FAERS search text with `--drug`, MAUDE `--product-code` repeated) now says "so that value matched nothing"; a lone value keeps "so it matched nothing". Recorded live in pm job j41434, no key: `--exact-term aspirin --drug ®` returns 888,047 FAERS reports and `--product-code FRN --product-code ®` 1,842,058 MAUDE reports, so the old notice was wrong. New steps in `faers-punctuation.yaml` and `maude-punctuation.yaml`.
  - Device recall `--firm` still folds into `manufacturer` (the query echo is unchanged), but the query keeps an unserialized flag so the notice names `--firm`. New step `--firm 'Abbott#'` in `recall-punctuation.yaml` replays the existing `recalling_firm:"Abbott"` recording. A continuation page, rebuilt from the echoed query, names `--manufacturer`.
  - Plain values are untouched (`openfda_words` still returns them borrowed); reply schema unchanged. `docs/cautions.md` notes both behaviours; `docs/reference/commands.md` regenerated (its `--firm` example now comes from the new case).
