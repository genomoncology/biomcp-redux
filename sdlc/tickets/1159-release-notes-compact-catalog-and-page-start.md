# 1159 — release-notes-compact-catalog-and-page-start

Status: COMPLETE.
Landed: 6ff8a5aa89c5eb0431261fa16f7b201ecd1569c9 (with 1154)

Milestone: 7

## Outcome

The release notes name compact MCP tool discovery (ticket 1134) and `trial document --page-start` (ticket 1153), each linking its existing reference.

## Why

Found 2026-11-23 in the BioMCP 1.0 adversarial review. Ian's November 16 ruling (`sdlc/decisions/2026-11-16-review-follow-ups.md`) requires a release-note entry for user-visible changes; both landings omitted one.

## Proof

- `release/notes.md` holds both entries and `make docs-check` passes.

## Changes

- `release/notes.md` only.
