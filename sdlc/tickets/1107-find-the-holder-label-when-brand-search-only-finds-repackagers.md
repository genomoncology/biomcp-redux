# 1107 — find-the-holder-label-when-brand-search-only-finds-repackagers

Status: OPEN.

Milestone: 7

## Outcome

When a brand lookup finds only repackager copies and a verified same-application holder label is available, return the holder label first and retain the copies as alternatives.

## Why

Found by a hand solve of research tasks, 2026-11-20. `biomcp get drug Hadlima label --source openfda-label --label-section how_supplied` returned `a brand-name NDA/BLA label repackaged by another firm`. The name candidates omit the holder SPL. The exact BLA761059 lookup includes it among five candidates. The same command by set ID c24655e5-796e-4127-b716-92f08d649c83 returns the BLA761059 holder label. Ticket 1041 ranks candidates already found.

## Proof

- Proof: Replay a name search with only repackagers and an application lookup with the holder label. Check the selected set and prove a holder from another application is excluded.

## Starts from

- Starts from: `crates/biomcp-adapters/src/openfda_label/search/` and the drug query’s `label_application.rs` and `label_choice.rs`.

## Keeps

- Keeps: Explicit repackager SPL IDs remain readable and ambiguity remains visible.

## Defers

- Defers: Resolving a brand miss across applications.

## Changes

- Changes: OpenFDA name candidates and selection, with cases.
