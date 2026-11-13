# 0892 — transcript-fit-is-quick-and-leads-on

Status: OPEN.

Milestone: 7

## Outcome

When a variant's reference base does not match, the transcript-fit check (0885) answers quickly even when no transcript fits, and its next step leads the agent on to the variant's record instead of repeating the lookup.

## Why

0885's review (2026-11-13) found:
- the fit check waits up to VariantValidator's 15-second source deadline when no transcript fits (VariantValidator returns a server error after about 60 seconds), and the recorded fit call took 1.9 s;
- its `next` reruns the normalize call whose answer is already in the reply, rather than pointing to `get variant <allele>` or the ClinVar record;
- retries run one after another;
- `sources` can list the same source twice (failed, then ok).

In the rare-disease run (biomed-agent-bench `personas/rare-disease/runs/2026-11-13/log.md:303-372`) the agent's next move after resolving a variant was the ClinVar and population lookups.

## Proof

- Proof: the fit check has its own configurable deadline with a measured default; a recorded case of VariantValidator's server error shows no fits, no added row and no step; the `next` after a fit names `get variant <CA id>` with its clinvar section; the retries run in parallel; each source is listed once with its final state, and the other states are kept in problems.

## Starts from

- Starts from: 0885 (`crates/biomcp-adapters/src/variantvalidator/fit.rs`, `crates/biomcp-query/src/variant/normalize.rs`), `fan_out::run`.

## Keeps

- Keeps: never suggesting a base change; the original mismatch message.

## Defers

- Defers: gene-symbol HGVS input without an accession.

## Changes

- Changes: `crates/biomcp-adapters/src/variantvalidator/`, `crates/biomcp-query/src/variant/`, `tests/cases/variant/`, `docs/`
