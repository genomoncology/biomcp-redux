# 0857 — widened-names-drop-code-like-synonyms-everywhere

Status: COMPLETE.
Landed: c6eb17d5aa651680623c91802c80d99db9d1ae02

Milestone: 7

## Outcome

Every search that widens a drug name (article, trial and the 0830 sources) leaves out code-like and context-only synonyms such as "COMPOUND 20" or "compound 20 [PMID: 32250617]", lists them as skipped with a short reason in the one notice, and never widens to a name that would match unrelated records.

## Why

From the bench 0039 persona review (2026-11-12, runs on BioMCP 58d83b6da), `personas/<slug>/runs/2026-11-12/review.md` and `log.md`: msl log.md:116-119: `search article adagrasib` widened to "COMPOUND 20" and returned 500 papers, 413 without the drug's name, back to 1948; `--exact-term` gave PubMed's 385. Widening is now the default, so this is a silent wrong answer. 0850's filter covers only FAERS and labels.

## Proof

- Proof: a routine case with a recorded MyChem reply carrying "COMPOUND 20" shows article and trial search leaving it out with its reason; the 0850 FAERS and label cases still pass.

## Starts from

- Starts from: 0850's name filter and 0830's shared widening; the article and trial widening paths in `crates/biomcp-query/src/drug/search_names.rs` and callers.

## Keeps

- Keeps: widening by default; typed text always searched; never fewer than literal; one compact notice.

## Defers

- Defers: a general relevance check on widened results.

## Changes

- Changes: `crates/biomcp-query/src/drug/`, `tests/cases/search/drug-names/`
