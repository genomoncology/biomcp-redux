# 1144 — serve-hpo-electronic-annotations

Status: OPEN.

Milestone: 7

## Outcome

HPO disease searches answer from every annotation row in the named release, including rows with evidence code `IEA`, and the reply no longer says annotations were excluded.

## Why

Ian decided on 2026-11-22 to serve these rows (`sdlc/decisions/2026-11-22-serve-hpo-electronic-annotations.md`). The filter from ticket 1099 dropped every `IEA` row before caching, so a three-phenotype search returned six entries where the release's own file gives nine (round 1 task rd-3: OMIM:121050, OMIM:236200 and OMIM:609008 were lost).

## Changes

- Remove the `IEA` filter in `crates/biomcp-adapters/src/hpo/release.rs`; cached bodies hold the whole file.
- Replace the coverage notice added by ticket 1137 with one that names the source and says HPO marks `IEA` rows as derived from OMIM's clinical features.
- Rows with a NOT qualifier or a zero frequency keep their existing handling.
- A store cached under the filter is re-read, not served as complete.

## Proof

- A replay fixture with an `IEA`-only match: the entry is returned, in CLI and typed MCP.
- The rd-3 search on the held 2026-09-01 release returns the nine entries the release's file gives.
- A store written by the filtered build is refreshed on first read.

## Depends on

- 1137 (resolve-hpo-release-and-state-coverage), which adds the coverage notice this ticket rewrites.
