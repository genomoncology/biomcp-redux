# 1008 — civic-evidence-filters-by-significance

Status: OPEN.

Milestone: 7

## Outcome

`get variant … civic` and `get drug … --source civic` take `--evidence-significance` (such as resistance, sensitivityresponse), sent to CIViC like `--evidence-level`, and the profile's count reflects it.

## Why

tumor-board needed T790M's 18 resistance items of 46 and read 15 rows (bench `personas/tumor-board/runs/2026-11-16/analysis.md` 8.3). First confirm CIViC's GraphQL `evidenceItems` takes a significance argument.

## Proof

- Proof: a replayed case for EGFR T790M with `--evidence-significance resistance` asserting the count; fails on the base commit.

## Starts from

- Starts from: the analysis named in Why, and the BioMCP code it cites.

## Keeps

- Keeps: every other reply field and command form.

## Defers

- Defers: nothing.

## Changes

- Changes: the named command's code, help, cases and a release-notes line.

## Progress

- CIViC's schema, asked live (pm job j41984, GraphQL introspection of `Query`): `evidenceItems` takes `significance: EvidenceSignificance`, a single value, beside `evidenceLevel`, `evidenceType`, `evidenceDirection` and `status`. EvidenceSignificance has 24 values: SENSITIVITYRESPONSE, RESISTANCE, BETTER_OUTCOME, POOR_OUTCOME, POSITIVE, NEGATIVE, NA, ADVERSE_RESPONSE, PATHOGENIC, LIKELY_PATHOGENIC, BENIGN, LIKELY_BENIGN, UNCERTAIN_SIGNIFICANCE, REDUCED_SENSITIVITY, GAIN_OF_FUNCTION, LOSS_OF_FUNCTION, UNALTERED_FUNCTION, NEOMORPHIC, UNKNOWN, DOMINANT_NEGATIVE, PREDISPOSITION, PROTECTIVENESS, ONCOGENICITY, LIKELY_ONCOGENIC. The repo held no recorded schema or request using it before this ticket.
- Built like `--evidence-level`: `CivicEvidenceFilter` gains optional `evidence_significance` (lower-case wire values of that enum), and `evidence_filter::arguments` sends `significance:VALUE` on the same root `evidenceItems` query for `get variant … civic` and `get drug … --source civic`. Schemas gain only the optional field and its enum; docs regenerated; release-notes line added.
- Live recordings (`biomcp --record`): j41999, `get variant CA090928 civic --evidence-significance resistance` (EGFR T790M, profile 34) gives evidence total 18 of the profile's 46, first EID238 (PMID 25668228), matching the analysis. j42008, `get drug 5 civic --source civic --civic-kind evidence --evidence-significance resistance` (imatinib) gives 94 of 169 accepted items. Cases `tests/cases/variant/civic-significance.yaml` and `tests/cases/drug/civic-significance.yaml` replay them; on the base commit the flag is a usage error, so both fail there.
- Focused job j42018: schema and docs regenerate clean, `make check-cheap` and `make schema-release-check` pass, and the touched civic, help, enum and hosted cases pass. Two MCP parity trials fail: `mcp-parity/compact-typed-schemas` and `mcp-parity/typed-hosted-catalog` measure the `get_drug` typed tool at 2,517 characters against the 2,400 action budget (`crates/biomcp/tests/runner/typed_schemas.rs:186`). The new field costs 469 characters, about 390 of them the 24 enum values; `get_drug` was about 2,048. `get_variant` stays within budget. A typed action budget change needs a PM ruling (sdlc/decisions/2026-10-26-measured-entity-caps-need-no-ask.md, search_diagnostic precedent), so the full gate has not run.
- PM ruling 2026-11-16: `get_drug` gets its own typed action budget of 2,900 characters in `crates/biomcp/tests/runner/typed_schemas.rs`, measured 2,517 (j42018) plus 15 percent, as `search_diagnostic`'s 2,800 under sdlc/decisions/2026-10-26-measured-entity-caps-need-no-ask.md. Other wording is unchanged.
- Full gate passed: j42035 at 765bc1d27 (branch merged with origin/main 84f5db681), `make --keep-going landing-gate docs-check schema-release-check`, after a schema and docs regenerate that changed nothing.
