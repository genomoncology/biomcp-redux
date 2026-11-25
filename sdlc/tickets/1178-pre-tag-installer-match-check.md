# 1178 — pre-tag-installer-match-check

Status: COMPLETE.
Landed: b196995b32f0ae4f42c61356622a005eb9d2fcb3

Milestone: 7

## Outcome

A read-only pre-tag check (run by the release rehearsal and documented in the release steps) fetches `https://biomcp.org/install.sh` and fails, naming both hashes, when it will not match the `install.sh` the tag will publish, before any package is published.

## Why

`release/publication.py:131` commits `install.sh` to `gh-pages`, then `release/published.py:77-95` requires biomcp.org/install.sh to equal it byte for byte before `publish-latest`. biomcp.org now serves the 0.9.3 installer from the hosting site, so a tag would publish PyPI, Homebrew and the registry, then stop before promoting latest. Found by the 2026-11-24 adversarial review. The hosting side (serving the public repo's copy) is separate work.

## Proof

- Proof: The check, run against a served file that differs, exits non-zero naming both hashes; against a matching file it passes. Cases use a local server, not biomcp.org.

## Starts from

- Starts from: `release/publication.py`, `release/published.py`, the release rehearsal entry point.

## Keeps

- Keeps: The post-publish check in `published.py` stays as it is.

## Defers

- Defers: Making biomcp.org serve the right file: hosting repo work.

## Changes

- Changes: release scripts and the release steps doc.
