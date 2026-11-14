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
