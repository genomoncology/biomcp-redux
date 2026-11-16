# 0989 — release-notes-catch-up

Status: OPEN.

Milestone: 7

## Outcome

`release/notes.md` describes what a user of the 1.0 build gets today. It names only credentials the code reads, and it covers every user-visible change landed since the notes were last brought up to date, saved work included.

## Why

Findings D1 and D2 of `sdlc/records/2026-11-14-adversarial-review.md`, which the review calls blockers of an honest release.

- **D1:** `release/notes.md:63` names `UMLS_API_KEY`, but the code reads seven credential names and none of them is UMLS (`crates/biomcp-fetch/src/secret.rs` `environment()`).
- **D2:** since then about 120 tickets have landed (from 2026-11-13). Their user-visible changes are missing from the notes. They include:
  - GTR specimens and lab contacts, and ClinVar gene counts;
  - the FDA search, recall, PMA and companion-diagnostic changes;
  - HPO ancestors and the CPIC phenotype filter;
  - Europe PMC and bioRxiv PDFs, and `sync_timeout_seconds`;
  - the leaner reply envelope (terms moved to `help sources`, `sdlc/decisions/2026-11-15-source-terms-live-in-help-sources.md`);
  - related queries;
  - saved work: lists, searches, notes, `changed`, the typed `saved` tool and the hosted `--saved-folder`.

## Proof

- Proof: every credential name in the notes appears in `secret.rs` `environment()`, and every one there appears in the notes.
- Proof: one pass over `git log` from the commit that last updated the notes' feature list to main. Each landed ticket with a user-visible change (a command, option, filter, source, config key, credential name, output field or tool) has a line in the notes or a reason in Progress for leaving it out.
- Proof: `make docs-check` passes.

## Starts from

- Starts from: the review's D1 and D2 text, `release/notes.md`, `CHANGELOG.md`, and `sdlc/decisions/2026-11-01-no-backwards-compatibility.md` (the notes list every removed or changed command, option, config key and output field).

## Keeps

- Keeps: the notes' existing structure and plain wording. No code change. Notes and public docs name none of our private projects.

## Defers

- Defers: whether every later ticket updates the notes as it lands. That is a rule for Ian to make.

## Changes

- Changes: `release/notes.md`, and `CHANGELOG.md` if it repeats the notes.
