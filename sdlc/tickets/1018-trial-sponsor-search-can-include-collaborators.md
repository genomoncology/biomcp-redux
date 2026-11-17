# 1018 — trial-sponsor-search-can-include-collaborators

Status: COMPLETE.
Landed: 92cb1168249769cb4d4a0feb44ffc6d896639d86

Milestone: 7

## Outcome

`search trial --sponsor` can match any sponsor role, lead or collaborator, by an option, mapped to ClinicalTrials.gov's `query.spons`.

## Why

life-sciences-ci 1.2 (bench `personas/life-sciences-ci/runs/2026-11-16/analysis.md`; `crates/biomcp-adapters/src/ctgov/filters.rs` 77-79).

## Proof

- Proof: a replayed case asserting a collaborator trial is found; fails on the base commit.

## Starts from

- Starts from: the analysis named in Why, and the BioMCP code it cites.

## Keeps

- Keeps: every other reply field and command form.

## Defers

- Defers: nothing.

## Changes

- Changes: the named command's code, help, cases and a release-notes line.

## Progress

- Live check of ClinicalTrials.gov on 2026-11-16, sponsor "Revolution Medicines": `query.spons` 26 trials; `AREA[LeadSponsorName]` 20; `AREA[CollaboratorName]` 6; `AREA[SponsorSearch]` 26, the same set as `query.spons` (SponsorSearch is the lead-or-collaborator area `query.spons` searches). A quoted AREA phrase matches within the name ("Revolution" alone gives the same counts). With `--condition pancreatic`: lead 13, SponsorSearch 16.
- Change: `search trial --sponsor-role lead|any` (optional; lead when absent, so every existing command form is unchanged). `any` puts the sponsor phrase in `AREA[SponsorSearch]` instead of `AREA[LeadSponsorName]`; the zero-match broadening to `query.spons` is unchanged. `--sponsor-role` without `--sponsor` is refused as a usage error naming `--sponsor` as the option to fix; NCI refuses it as an unsupported filter like `--sponsor`. `--sponsor` help names the role option. Schemas gain only the optional `sponsor_role` field and the `--sponsor-role` filter name; `--sponsor` gets a new description. A first try added a refusal message, which schema-release-check refused as an incompatible `/error/message` change (job j42065), so the refusal uses the existing usage message.
- Recording job j42057 (schema regenerate, live record, replay): the `any` search returned 16 trials, including NCT07898813 (lead Dana-Farber) and NCT06922591 (lead Tango) with Revolution Medicines as collaborator; replay matched. Case `tests/cases/trial/sponsor-role.yaml` replays it; it fails on the base commit, where `--sponsor-role` is an unknown option (job j42055 ran the old grammar and refused it). Release-notes line added.
- Gate job j42065 at 13ac58dc9 failed on two things: that error message, and the secrets scan flagging the recorded reply's public ClinicalTrials.gov `nextPageToken` (42 characters, outside the scanner's 39-character cursor pattern) as `generic-api-key`. The cursor gets a reviewed entry in `release/secrets.ignore.jsonl`, like earlier recorded cursors. Moved `PhaseFilter`, `SponsorType` and the new `SponsorRole` into `query/trial/filters.rs` to keep `trial.rs` under its 600-line limit (job j42060).
- Gate job j42071 at 4e2e53461 failed only `query-validation/enumerated-letter-case`: it tries each enum option alone, and `--sponsor-role` alone is refused for lacking `--sponsor`. Its context table now gives `sponsor_role` a sponsor, as it gives `biomarker_use` a biomarker code.
- Full gate (landing-gate, docs-check, schema-release-check) passed at 480243c6d in job j42084.
