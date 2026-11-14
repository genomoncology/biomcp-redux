# 0920 — noncommercial-recordings-checked-and-synthetic

Status: OPEN.

Milestone: 7

## Outcome

No test reply recorded from a source labelled noncommercial holds that source's real data. Each such reply is synthetic and says so in a README beside it.

## Why

0910's developer (2026-11-13) searched `tests/replies/` for recordings from noncommercial sources after replacing the WHO exports, and found:
- **Semantic Scholar** (`Use::Noncommercial`, `crates/biomcp-adapters/src/semantic_scholar/mod.rs:9-12`) has what look like real recordings:
  - `article/fulltext/live-all/semantic-scholar`, used by `article/sections/grammar.yaml`;
  - `article/semantic-scholar/live`, `article/pdf/timeout` and `article/pdf/unavailable`, which no case file names; Rust tests may use them.
- **ORCID:** `orcid/translated-title` has no case that names it, so whether it is real is unknown.
- **WHO IVD:** 0695's older synthetic replies (`diagnostic/who-synthetic`, `diagnostic/who-search-synthetic`) use the real WHO product ID 0466-046-00 (cobas 4800 HPV Test). It also appears in `who.yaml`, `who-search.yaml` and several Rust tests.

0695 set the rule ("no live fetch of noncommercial data"), and `AGENTS.md:5` says "Respect source redistribution terms".

## Proof

- Proof:
  - a check, run in the gate, that lists every recording under `tests/replies/` for each source whose terms say `Use::Noncommercial`, and fails unless its folder has a README saying the rows are synthetic;
  - the Semantic Scholar, ORCID and WHO IVD items above are synthetic, or shown to be synthetic already, with invented IDs, and the cases that use them still pass;
  - unused recordings are deleted with a note of what used them.

## Starts from

- Starts from: 0910's synthetic WHO replies and READMEs; 0695.

## Keeps

- Keeps: what each case asserts about behaviour.

## Defers

- Defers: deleting real data from git history, which is Ian's call if wanted.

## Changes

- Changes: `tests/replies/` for those sources, their cases and Rust tests, a check script wired into the landing gate.
