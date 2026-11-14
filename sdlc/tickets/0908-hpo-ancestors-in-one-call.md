# 0908 — hpo-ancestors-in-one-call

Status: COMPLETE.
Landed: b2748c9cae49a1bec5b118f8d99543d0f3e586ca

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
- 2026-11-14: rechecked OLS v2 `classes?curie=` live (HP:0012469 gives the same 7 ancestors; HP:9999999 gives `elements: []`). Merged origin/main; the written code compiled unchanged (j40156). Contract test covers the `ancestors` section (source ols, limit 50, snapshot and `--source ols` without a section refused) and the `PhenotypeAncestors` envelope. Recorded live in j40162: `get phenotype HP:0012469 ancestors` returns 7 labelled ancestors including HP:0001250 Seizure and HP:0000118, `sources[0]` ols, status complete; HP:9999999 is `absent`. Cases: `tests/cases/phenotype/ancestors.yaml` (live, with and without `--disable-sources monarch`) and `ancestors-monarch-down.yaml` (Monarch capture edited to three 502s: hierarchy fails with exit 4 and offers `get phenotype HP:0012469 ancestors`, which OLS answers). Docs regenerated; `docs/cautions.md` and `sdlc/features.md` (phenotype-ancestors) describe the section.
- 2026-11-14 review fixes (review of f4d1cb42d): an ancestor OLS lists without an ID now makes the adapter return a refusal and the reply `partial` (case `ancestors-missing-id.yaml`, OLS reply edited to drop HP_0011097's curie, labelled as edited). The request adds `includeObsoleteEntities=true`; measured live, HP:0012469's ancestors are unchanged, and HP:0000057 returns `isObsolete: true` with IAO_0100001 "HP:0008665" (Clitoral hypertrophy). The reply gains optional `obsolete` and `replaced_by`, status is `complete` once OLS holds the term (root or obsolete included), and each HP replacement gets a next step `get phenotype HP:0008665 ancestors`. Recorded live in j40296; case `ancestors-identity.yaml` covers HP:0000057 (obsolete) and HP:9999999 (absent, exit 1).
- Open: the phenotype source enum is shared by search and get, so `search phenotype` help now lists "hpo, monarch, ols" though search accepts only Monarch (it already listed hpo the same way). The HPO card does not itself offer the ancestors step; the hierarchy reply does.
