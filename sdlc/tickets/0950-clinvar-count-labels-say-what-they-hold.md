# 0950 — clinvar-count-labels-say-what-they-hold

Status: COMPLETE.
Landed: 43d9b4eeb188a39c2a40da92640cde97b9fea39c

Milestone: 7

## Outcome

The ClinVar gene section's field descriptions say what each bucket holds: "records whose aggregate label includes X", so combined labels ("Pathogenic/Pathogenic, low penetrance; risk factor", "Conflicting…; risk factor", bare VUS-high) land where the description says. A test pins that a 429 echoing the key in a header is still refused.

## Why

0932's review (2026-11-14): `crates/biomcp-types/src/response/gene/clinvar.rs:31,37,47` say "Pathogenic." and "risk factor" while combined labels land elsewhere; `request.rs:353`'s comment says NCBI though the 429 exception is global; the leftover parallel `Job`/`Done` dispatch in `clinvar/gene.rs:110-148,192-210` can be plain calls.

## Proof

- Proof: descriptions updated and regenerated; a transport test for a 429 with the key in a header; a unit or case check for one combined label.

## Starts from

- Starts from: the review lines above.

## Keeps

- Keeps: counts and buckets.

## Defers

- Defers: nothing.

## Changes

- Changes: descriptions, a comment, a small refactor, one test.

## Progress

- 2026-11-14 measured live (E-utilities, Build261108-1750.3): a `"clinsig X"[prop]` filter matches a record when X is one of the terms ClinVar joins with `/` and `;` in its aggregate germline label, so "Pathogenic, low penetrance" and "VUS-high" are terms of their own. "Pathogenic; risk factor" and "Pathogenic/Pathogenic, low penetrance" match only the pathogenic filter; "Likely pathogenic/Pathogenic, low penetrance" only the likely pathogenic one; "Pathogenic/Likely pathogenic/Pathogenic, low penetrance/Established risk allele; risk factor" (F2) both; "Conflicting classifications of pathogenicity; risk factor" (CHEK2, APC, F2) the conflicts filter; "Uncertain significance/VUS-high" (CHEK2) the VUS filter. Bare "risk factor", "Pathogenic, low penetrance", "VUS-high" and "VUS-mid" match none and count as other. Against all 639 CD46 esummary records the buckets match the labels exactly (pathogenic 45 = 44 + 1 combined; likely pathogenic 26 = 25 + 1; P/LP 12 = 11 + 1; other 38), and F2's 517 records do the same. Across all of ClinVar one label lists terms from two buckets (VCV000032255, "Uncertain significance; Pathogenic/Likely pathogenic"), so the buckets overlap there.
- Changed: the `ClinVarGermlineCounts` field descriptions now say "records whose label includes X", with combined examples, and name the overlap; `docs/cautions.md` says the same; `gene-cards.json` regenerated (descriptions only, j41097). The 429 comment in `request.rs` says the exception covers every source. The ClinVar section's `Job`/`Done` dispatch is now plain calls in the same order. `crates/biomcp-fetch/tests/transport/credentials.rs` `throttle_replies_echoing_the_key_in_a_header_are_refused_without_retry` proves a 429 echoing the key in a header is refused as a bad reply after one request (j41091).
- Case: `tests/cases/gene/sections/clinvar-labels.yaml` replays CD46 recorded live in j41091 and asserts the buckets above; the clinvar, clinvar-labels and clinvar-throttled cases pass (j41097).
