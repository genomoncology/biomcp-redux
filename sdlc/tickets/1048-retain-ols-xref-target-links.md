# 1048 — Keep the outbound links supplied on OLS disease cross-references

Status: COMPLETE.
Landed: 2b52eb3d7e2416eec65f67ad19366ab42eced6a4

Milestone: 7

## Outcome

`get disease MONDO:0016512 xrefs --source ols --section-limit 50` returns the publisher-supplied target URL beside each cross-reference that has one, so a caller can follow NORD:1318 to the Kabuki syndrome page and find its patient organizations.

## Why

The repeated rare-disease Q9 checks reach GARD and NORD IDs but lose the links needed to find peer support. On November 17 the BioMCP build at 31d7ecd75 returned the OLS IDs GARD:0006810 and NORD:1318 with only MONDO row attribution. The [OLS MONDO term response](https://www.ebi.ac.uk/ols4/api/ontologies/mondo/terms?obo_id=MONDO:0016512) carries `obo_xref.url` on those same rows. Its `https://rarediseases.org/?p=1318` redirects to the [NORD Kabuki syndrome page](https://rarediseases.org/rare-diseases/kabuki-syndrome/) with [All Things Kabuki](https://rarediseases.org/organizations/all-things-kabuki/). The supplied GARD query URL currently opens GARD's homepage. GARD's [Kabuki syndrome page](https://rarediseases.info.nih.gov/diseases/6810/kabuki-syndrome) has a slug that the OLS response does not supply. A raw identifier or an HTTP 200 is insufficient evidence of a disease link.

## Starts from

- Starts from: `ols/raw.rs` leaves out `obo_xref.url`, `ols/map.rs` maps each row to a Biodata `DiseaseRelation`, and `biomcp-query/src/disease/ols.rs` pages those rows. `attribution.url` identifies the source claim. The outbound link identifies the claim's target.
- PM decision (2026-11-17): use a separate optional, checked HTTP(S) target URL on BioMCP's xref view. Retain source-provided URLs without a reachability or endorsement assertion. Do not rewrite legacy targets from the disease name, fetch them during the request, or add a source call to the default MyDisease path. Mention the explicit OLS route in xrefs help if it lacks that entry point.

## Proof

- Proof: an OLS MONDO:0016512 replay through the xrefs CLI binds `NORD:1318` to the exact target URL the row supplies, with unchanged MONDO attribution. A row with no URL retains its ID and has no target URL. A malformed or non-HTTP(S) target is omitted without discarding the row. An explicit OLS call hands off a usable NORD link for the documented support lookup.

## Keeps

- Keeps: row order, IDs, pagination, default MyDisease request set, license and attribution.

## Defers

- Defers: importing any support directory or page, constructing GARD routes and handling legacy provider redirects. This handoff prediction applies when a caller uses OLS and can follow the returned URL.

## Changes

- Changes: `crates/biomcp-adapters/src/ols/`, `crates/biomcp-query/src/disease/ols.rs`, the BioMCP xrefs response and schema, and disease xrefs replay cases.
