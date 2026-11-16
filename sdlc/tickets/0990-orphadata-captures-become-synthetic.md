# 0990 — orphadata-captures-become-synthetic

Status: OPEN.

Milestone: 7

## Outcome

The public repo holds no real Orphadata bytes. Every Orphadata case replays synthetic recordings, as the other licensed and noncommercial sources already do.

## Why

Ian, 2026-11-16 (`sdlc/decisions/2026-11-16-review-follow-ups.md`, item 1), on finding C3 of `sdlc/records/2026-11-14-adversarial-review.md`:

- `tests/replies/disease/orphadata/` holds real files from `www.orphadata.com` whose inline `Licence` element states Orphanet's terms;
- the CC BY 4.0 claim cites a different site's FAQ;
- a synthetic set already exists in `tests/replies/disease/orphadata-synthetic/`.

## Proof

- Proof: `tests/replies/disease/orphadata/` is gone (or holds only synthetic bytes), and no case or test reads real Orphadata bytes.
- Proof: the synthetic-captures check from 0920 and 0973 covers Orphadata, and it would fail if real Orphadata bytes came back.
- Proof: every case `tests/cases/disease/orphadata.yaml` covered is still covered, against the synthetic set.

## Starts from

- Starts from: both case files and both reply folders above; 0889; 0920 and 0973's check.

## Keeps

- Keeps: Orphanet's statement on Orphadata results (`sdlc/decisions/2026-11-15-source-terms-live-in-help-sources.md`, item 2). No change to the adapter.

## Defers

- Defers: rewriting git history. The real bytes stay in past commits until the history append (0294).

## Changes

- Changes: Orphadata cases and recordings, the synthetic-captures check, any README in the reply folders.
