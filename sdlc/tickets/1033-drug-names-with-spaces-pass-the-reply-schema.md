# 1033 — drug-names-with-spaces-pass-the-reply-schema

Status: OPEN.

Milestone: 7

## Outcome

`get drug "dabigatran etexilate" ...` and every other drug name with a space answers with a reply that validates against its schema, on every drug source: the reply schema's drug ID pattern accepts what the command parser accepts.

## Why

1027's developer (2026-11-17) found that a drug name with a space gets a reply that fails schema validation, because the parser accepts spaces in drug IDs and the reply schema's ID pattern does not. This was already true on main. Salt and full names ("dabigatran etexilate", "imatinib mesylate") are common in persona questions.

## Proof

- Proof: a case for a spaced drug name on at least two drug sources asserting the reply validates; it fails on the base commit. The schema release check accepts the widened pattern, or the 1.0.0 review entry records it.

## Starts from

- Starts from: the drug ID pattern in the drug reply schemas and the parser's drug ID rule.

## Keeps

- Keeps: every ID form that validates today.

## Defers

- Defers: nothing.

## Changes

- Changes: the pattern, schemas, a case, one release-notes line.

## Progress

- Cause: `DrugGet`'s hand-written schema (`crates/biomcp-types/src/drug/query.rs`) gave the echoed `query.input` the `DrugId` pattern, which has no spaces, while the parser takes any `Text` as a name. A resolved name is echoed as its native ID, so only replies that keep the name fail.
- Live probe on main (jobs j42191, j42195, M5, `--fresh`): `get drug NAME` for "dabigatran etexilate", "imatinib mesylate", "dabigatran etexilate mesylate" and five other spaced names on mychem, chembl, opentargets, civic, openfda-label and drugsfda all resolved and echoed a native ID. `--source fda-drug-recall` answered absent `biomcp.drug-card/7` with the spaced name in `query.input`, which fails the pattern. `markets` echoes the name too but its schema already takes `Text`. A name that resolves nowhere is a usage error with no query echo. The other name-keeping path is the absent reply with another source's name hint (`crates/biomcp-query/src/drug/aliases.rs`), on any name-resolving source.
- Audit of reply schema ID patterns narrower than the parser for the same field: only drugs. `DrugGet.input` is the one query field whose schema is overridden by a narrower type than the field it serializes. The other overrides checked: pathway get and relation `id` (pattern `^(R-HSA-|WP)...`; the parser accepts only those IDs and the echo is the canonical form), diagnostic `--condition-id` (pattern `^MedGen:C[0-9]+$`, the parser checks the same rule), cell line input (pattern matches `CellLineInput`, tested in `cell_line_contracts.rs`), variant `LookupInput`/`RecurrenceInput` (same grammar as `VariantInput`, which refuses spaces), and every `schemars(with = X)` field of type `Option<X>` (same type both ways). Type-level parser/pattern agreement for every BioData ID type is already tested by `text_value_patterns_match_parser_admissions`. The adverse-event and GWAS command schemas are wider than their replies by design (the reply keeps normalized values).
- Change: the echoed `input` schema takes the `Text` pattern (no length bounds, so nothing narrows), with a description saying an unresolved name is echoed as given. Release notes line added.
- Live probe j42197: names no source holds, such as "metamizole sodium" (Drugs@FDA, openFDA labels) and "domperidone maleate" (Open Targets), answer absent `biomcp.drug-card/7` with the spaced name in `query.input` and another source's record in the hint. So on main the failure reaches drugsfda, openfda-label, opentargets and fda-drug-recall, and by the same path any name-resolving source.
- Case `tests/cases/drug/spaced-names.yaml`: those three calls, recorded live in j42207, each asserting absent, the schema id and the spaced `query.input`; the case runner validates every reply against its schema, which the old pattern fails. An fda-drug-recall step for "dabigatran etexilate" was dropped: its recording (j42203) replayed differently (the name lookup came out "unfinished" in replay and retried "Pradaxa", a replay miss), so that source's recall replay is not deterministic for this call. Left open; not this ticket.
- Schema reviews (`release/schema-reviews.json`): the query contracts `biomcp.request/32` `/@command=get-drug/input` and `biomcp.drug-get/3` `/input` are reviewed in `pattern_narrowings`, the list the policy reads for a query pattern change (here a widening of what callers may send); the 11 drug reply schemas' `/query/input` and the CIViC page `next/input` paths in `drug-civic/12` and `drug-sections/2` are reviewed in `pattern_widenings`. The reply schemas' baseline majors are older, so the baseline check needs only the two query entries.
- Jobs: j42191, j42195, j42197 (live probes on main 26c501ef2); j42201 (cancelled, diff left out the recordings); j42202 (compile error: serde_json is not a biomcp-types dependency; switched to `schemars::json_schema!`); j42203 (schema generation refused the query change until reviewed; recall step replay miss); j42207 (re-recorded, discovered the four CIViC paths, regenerated schemas and docs; check-cheap, drug/, help/, mcp-parity cases and schema-release-check pass).
