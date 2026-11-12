# 0850 — widened-names-the-provider-can-take

Status: COMPLETE.
Landed: 79c044c19409154fb67d5411d1a3bb8804efb659

Milestone: 7

## Outcome

`--expand-drug-names` on FAERS and label search never fails because of a name it added: names the provider cannot take (non-ASCII, `&` and HTML entities such as "Eliquis&reg;", markup) are left out, as are ChEMBL "X component of Y" names and names with a bracketed qualifier such as "Pradaxa (dabigatran)". If the provider still rejects the widened query, BioMCP searches the typed term and says so. The reply lists each name left out with a short reason.

## Why

0830's design review (2026-11-11, base 888187ef, jobs j38107 and j38118) found the opt-in widening landed by 0812 and 0815 fails outright for apixaban on both FAERS and labels (exit 4): MyChem supplies "Eliquis&reg;" and "apixabán" and openFDA rejects any query containing them. Without them the FAERS union is 282,308 reports against 69,029 for "apixaban" alone. Junk names also use up the name limit (12 of valsartan's 19 ChEMBL names are "component of" names). This is a live defect in a persona-facing option, ahead of 0830 making widening the default.

## Proof

- Proof: routine cases with recorded replies: apixaban widened on FAERS and labels completes with the bad names listed as left out; a provider rejection of the widened query falls back to the typed term with a notice; valsartan's widened list holds no "component of" names.

## Starts from

- Starts from: `crates/biomcp-query/src/drug/search_names.rs` and `aliases.rs`, the FAERS and label expansion paths from 0812, 0815 and 0816, and the review's job logs (j38107, j38118).

## Keeps

- Keeps: bounded name lists frozen across pages; native totals never summed; the typed name always searched.

## Defers

- Defers: widening by default (0830); the ChEMBL wait (0843).

## Changes

- Changes: `crates/biomcp-query/src/drug/`, `tests/cases/adverse-event/`, `tests/cases/drug/`
