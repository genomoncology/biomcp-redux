# 0929 — phenotype-empty-sections-say-complete

Status: OPEN.

Milestone: 7

## Outcome

A phenotype section (synonyms, cross references, hierarchy, associations) asked for a term the source holds but whose section is empty comes back `complete` with an empty list, not `absent`. `absent` stays for a term the source doesn't hold, as BioMCP defines it: "every identity was confirmed absent" (`crates/biomcp-types/src/response/status.rs:8-9`).

## Why

0908's review (2026-11-14): the new ancestors section reports the root term and obsolete terms as `complete` with an empty list, which matches that definition. The older phenotype sections call an empty section `absent` (`crates/biomcp-query/src/phenotype/sections.rs:124`, `associations.rs:52`). An agent then can't tell "this term has no synonyms" from "this term doesn't exist", which is the same mix-up 0908 fixed for ancestors.

## Proof

- Proof: a recorded case for a real HPO term with no synonyms (or no cross references) shows `complete` and an empty list; an unknown ID still shows `absent`.

## Starts from

- Starts from: `crates/biomcp-query/src/phenotype/sections.rs:124`, `associations.rs:52`, and 0908's handling in `crates/biomcp-query/src/phenotype/ancestors.rs:25`.

## Keeps

- Keeps: `partial` for replies with refused rows; the 1.0.0 schema rules (status values are unchanged; only which one is chosen).

## Defers

- Defers: other entities' empty sections; check them and list any that drift in Progress.

## Changes

- Changes: the phenotype section status code and its cases.

## Progress

- Measured live 2026-11-14 by curl: JAX HPO returns `synonyms: []` and `xrefs: []` for HP:0000001 and `xrefs: []` for HP:0000118, and 404 for HP:9999999. Monarch `/entity/HP:0000118` has `xref: null` (the old code turned a null section into `absent`, not only an empty one). Monarch's direct association page has total 0 both for HP:0000001 (held, no direct diseases or genes) and for HP:9999999 (unknown), so the page alone cannot tell them apart; `/entity` answers 200 and 404.
- Changed: a synonyms, xrefs or hierarchy section is read only after the source returned the term, so it is now `complete` whether its list is empty or the source field is null (`sections.rs`); the ancestors section shares that page helper and drops its own override. A diseases or genes page with total 0 now reads Monarch's term once more: held is `complete`, 404 is `absent`, any other failure is `failed` with the source marked. Pages with rows make no extra call. `partial` for refused rows is unchanged; no schema change.
- Recorded live in j40432; case `tests/cases/phenotype/empty-sections.yaml` covers HP:0000001 synonyms (HPO), HP:0000118 xrefs (HPO and Monarch), HP:0000001 diseases (all `complete`, empty) and HP:9999999 synonyms, Monarch xrefs and diseases (`absent`, exit 1).
- Other entities that drift the same way (deferred, not changed): disease Monarch sections (`crates/biomcp-query/src/disease/monarch.rs:49`, total 0 is `absent`) and pathway relation pages (`crates/biomcp-query/src/pathway/relation.rs:218` and `:240`, total 0 is `absent`). `drug/who.rs:36` also calls an empty list `absent`, but it may be the identity lookup itself; not checked further.
- Gate j40454 failed only `phenotype-associations/monarch-phenotype-cache-admission`: its "empty" control blanks the recorded HP:0001250 genes page and expected `absent`, and the new term read had no recording. Recorded Monarch's real `/entity/HP:0001250` reply live (j40495) into `tests/replies/cache-admission/monarch/`, and the control now expects `complete` for this owner (`crates/biomcp/tests/runner/cache_admission/controls.rs`).
