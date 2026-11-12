# 0830 — drug-names-widen-by-default-everywhere

Status: OPEN.

Milestone: 8

## Outcome

Every search that takes a drug name widens it to the drug's known names by default, says in the reply which names it searched and which it could not, and searches the literal text only when given `--exact-term`. This covers FAERS, openFDA labels, Drugs@FDA, EMA and drug recalls, as trial and article search already do; MAUDE and VAERS stay literal (see design review).

## Why

The same drug gives different counts by spelling: the persona run reported 292 fatal reports for the generic name, 414 for the brand and 703 for both. Trial and article search already widen by default, so the difference between commands is itself a trap.

## Proof

- Proof: for each source above, a routine case with a recorded reply shows a generic-name search returning the union with the searched names listed, and `--exact-term` returning the literal count; continuation pages keep the first page's names.

## Starts from

- Starts from: `crates/biomcp-query/src/drug/search_names.rs` and `aliases.rs`; the opt-in `--expand-drug-names` paths landed by 0812, 0815 and 0816; trial search's default widening as the pattern.
- PM decision: 2026-11-11. This overrides the "explicit opt-in only, literal default" line in 0812, 0815 and 0816. One rule for every command beats a safer default that differs by command; `--expand-drug-names` stays accepted as a no-op for one release.
- PM decision: 2026-11-11. Design review done (below). Depends on 0850, 0851 and 0843.


## Amendment, 2026-11-11

Design review, accepted by the PM. Base 888187ef, jobs j38107 and j38118, live sources. FAERS pembrolizumab, myocarditis, fatal: 292 generic, 414 KEYTRUDA, 703 both via `--drug`, 704 via `--expand-drug-names`. Label: apixaban 1 set, ELIQUIS 8.

Blockers, split into their own tickets and landing first: widened names openFDA rejects make apixaban fail outright (0850); the label 1-vs-8 gap is letter case in the exact step (0851). The ChEMBL wait (0843) lands first or alongside: pembrolizumab goes through ChEMBL, and PRADAXA on labels took 7.4 s and came back partial.

Rules: leave out names the provider cannot take and junk names (0850); if the provider rejects the widened query, search the typed term and say so. For sources that try exact matches before word matches (labels, Drugs@FDA), choose the step by the typed name, then search the union at that step: a widened search must never return fewer records than the literal one (dabigatran on labels must not drop below 23 sets). Never add per-name counts: use the provider's total for one OR query, remove duplicates by record ID otherwise, and report no total rather than a sum. Name lookups for FAERS and labels use 0832's skipped rule. One disclosure notice on the searched source lists names searched and each name not searched with a reason. Continuations keep page 1's names: FAERS carries them in the next-page request; labels keep the snapshot plus the chosen step; the snapshot check stops requiring `--expand-drug-names`. `--exact-term` searches the literal text only.

Scope: FAERS first, then labels, then Drugs@FDA, recalls and EMA. MAUDE and VAERS stay literal (MAUDE has no drug field; VAERS takes fixed vaccine codes) and their help says so.

Proof per slice: pembrolizumab finds 704 with its names listed; `--exact-term` finds 292; apixaban finds about 282k rather than failing; dabigatran on labels stays at or above 23 sets.

## Amendment, 2026-11-12

0850's review (accepted at 79c044c19) found widened label search still matches names case-sensitively: `apixaban --expand-drug-names` finds 1 label set while plain `apixaban` finds 10 since 0851. 0851 left widened names to this ticket. Fix it here before widening becomes the default, by applying 0851's rule (exact hits, then word hits, exact first, no threshold) to the OR query over the names, within the single-query size limit and with the chosen step carried to later pages. Proof adds: widened apixaban on labels finds at least plain apixaban's 10 sets; re-record 0850's label case, which today expects 1.

## Keeps

- Keeps: every existing field, schema ID rule (a changed reply schema takes a new major), source attribution and page behavior; no new source, key or dependency. Bounded name lists, frozen across pages, native totals never summed per name, resolver failure reported and not hidden, source exclusions honored.

## Defers

- Defers: phrase and spelling variants for conditions and protein changes are a separate ticket in the composite batch; NCI trial search stays literal with its notice.

## Changes

- Changes: `crates/biomcp-query/src/drug/`, `crates/biomcp-adapters/src/faers/`, `crates/biomcp-adapters/src/openfda_label/`, `crates/biomcp-adapters/src/drugsfda/`, `tests/cases/drug/`, `tests/cases/adverse-event/`
