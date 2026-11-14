# 0908 — hpo-ancestors-in-one-call

Status: OPEN.

Milestone: 7

## Outcome

A counselor's coding-consistency check, "is infantile spasms already under seizure?", takes one call. A phenotype card can return the term's full ancestor list, and that still works when Monarch is down.

## Why

In rare-disease round 2 (biomed-agent-bench `personas/rare-disease/runs/2026-11-13-r2/`, report Q4 and Q5), the agent had to make four calls to climb HP:0012469 Infantile spasms → HP:0011097 → HP:0020219 → HP:0001250, because BioMCP returns one hop per call. HPO hierarchy and text search are served by Monarch only, which returned HTTP 502 for part of that session, so those calls failed.

The agent's curl to EBI's OLS4 returned every ancestor of a term in one request each. BioMCP already uses OLS for disease search (MONDO).

## Proof

- Proof: recorded cases showing that:
  - `get phenotype HP:0012469` with an ancestors section returns the full ancestor chain to HP:0000118, with labels, in one call;
  - the same request, with Monarch replaying as unavailable, is answered from OLS and says which source served it.
  Both values must match the recorded OLS reply.

## Starts from

- Starts from:
  - the phenotype card and its hierarchy section;
  - BioMCP's existing OLS adapter for MONDO;
  - OLS4's `ancestors` endpoint for the HP ontology.

## Keeps

- Keeps: the one-hop parents and children sections; Monarch as the first source where it answers.

## Defers

- Defers: HPO text search through OLS, its own question. Round 2 showed "floppy baby" finds nothing in either OLS or Monarch.

## Changes

- Changes:
  - the phenotype query and its OLS adapter path;
  - `crates/biomcp-types/` (an added section or field);
  - `tests/cases/phenotype/`;
  - `docs/`.

## Progress

- Measured live 2026-11-13 by curl (no job): Monarch v3 `/v3/api/entity/HP:0012469` has only one-hop `node_hierarchy` (super_classes HP:0011097, no ancestor closure), so Monarch cannot serve ancestors; "Monarch first" applies only to the existing hierarchy section, which stays. OLS v1 `/api/ontologies/hp/terms/{double-encoded IRI}/ancestors` needs `%25`-encoded paths, which `PathSegment`/`with_path` in biomcp-fetch refuse, and returns 200 with zero rows for an unknown ID (HP:9999999) exactly as for the root HP:0000001. OLS v2 `GET /api/v2/ontologies/hp/classes?curie=HP:0012469` (query parameters only) returns one element whose `directAncestor` IRIs equal the v1 `ancestors` set for HP:0012469 (7), HP:0002121 (8), HP:0001250 (4), HP:0008947 (7), HP:0410030 (9) and HP:0000001 (0); labels and CURIEs come from `linkedEntities`; an unknown ID returns `elements: []`. Reply about 13-16 KB. HP:0012469 ancestors: HP:0000001 All, HP:0000118, HP:0000707, HP:0001250 Seizure, HP:0011097, HP:0012638, HP:0020219.
- Shape (code written, NOT yet compiled or tested): new section `ancestors`, source `ols` (both enum widenings), default and only source OLS, default section limit 50, snapshot refused. Reply `biomcp.phenotype-ancestors/1` (`PhenotypeAncestors` page of `PhenotypeAncestor {id, label}`, components carry the HPO terms, envelope `sources` names ols). Adapter `crates/biomcp-adapters/src/ols/ancestors.rs`; OLS declares an `hp` component (native source hpo, HPO terms). Query `crates/biomcp-query/src/phenotype/ancestors.rs`, routed from `phenotype/mod.rs`; unknown ID is reported absent. Every `hierarchy` reply, including a failed Monarch one, now offers a next step `get phenotype ID ancestors --source ols`.
- Pick up here (wound down at midnight 2026-11-13, no job run yet):
  1. Push, then `make schema-generate` and a `cargo build`/`cargo test` job; fix compile errors (watch `PhenotypeSource` exhaustive matches, `Read`/`Reply` bounds on the private raw structs in `ols/ancestors.rs`, the help/front-map and coverage text).
  2. Update `crates/biomcp-types/tests/phenotype_contracts.rs` (section loop: add `ancestors` with source ols, limit 50; envelope test for `PhenotypeAncestors`).
  3. Record live in a job with `--record`: `get phenotype HP:0012469 ancestors` and the same with `--disable-sources monarch`; record Monarch `get phenotype HP:0012469 hierarchy` and edit it to three 502 exchanges for a synthetic Monarch-down case asserting the `ancestors` next step. Write `tests/cases/phenotype/ancestors.yaml` (live, nightly) and `ancestors-monarch-down.yaml` (response-edited); expect HP:0001250 Seizure and HP:0000118 in items, page total 7, `/sources/0/source` ols.
  4. Docs (`make docs-generate`), `sdlc/features.md` line, help text naming the ancestors section, then the gate `make --keep-going landing-gate docs-check schema-release-check`.
