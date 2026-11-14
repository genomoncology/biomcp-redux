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

## Progress

- 2026-11-13 measured (from the Beelink, curl): VariantValidator's transcript list took 0.40-1.43 s and its fit query 0.68-4.47 s for SCN1A, DMD, NF1, BRCA2, TTN, CDKL5, NRXN1, CACNA1C, ANK3, GAPDH and TP53 (2 to 24 transcripts; NRXN1 slowest). A base that fits nowhere also answered 200 within that time (BRCA2 c.5000, NRXN1 c.1000T>C). The 0885 "no fit" input SCN1A c.4729G>C now answers in 1.9-2.5 s and fits NM_001353954.2 and NM_001353955.2; its 60 s HTTP 500 in job j39173 did not recur here or on the M5 (job j39768), so it was a passing server fault, not VariantValidator's answer for "no fit".
- 2026-11-13 change: the fit check runs under its own deadline, `transcript_fit_seconds` in config.json, default 8 s (largest measured transcript list plus fit query, plus one second of VariantValidator pacing, plus 15 percent; `FIT_DEADLINE` in `crates/biomcp-query/src/context.rs`). The retries on the fitting transcripts run at once through `fan_out::run`. The `next` after a fit is `get variant <CA id> clinvar` when CAR resolved the allele, else `get variant <fitting HGVS> clinvar`; it no longer repeats the normalize call. `sources` lists each source once with the state of its last call; each earlier, different state is kept in `problems` as `state: <state>, <reason>` under the HGVS that call was for. The mismatch message is unchanged and no base change is ever suggested.
- 2026-11-13 recorded (job j39768): for NM_001165963.4:c.4729T>C with CAR, `next` is `get variant CA284979 clinvar`, `sources` is car ok and variantvalidator ok, and `problems` keeps CAR's `state: failed, rejected`; `get variant CA284979 clinvar` returns VCV000068552, Pathogenic/Likely pathogenic, the variant the code-mode agent found (`normalize-transcript-fit.yaml`). BRCA2 NM_000059.4:c.5000A>C fits no other transcript and returns no fits, no added row and no step (`normalize-transcript-nofit.yaml`). VariantValidator's 500 after 60.5 s from job j39173 is replayed with the status, body and delay that job saw; its content type was not captured. The check stops at its 8 s deadline with no fits, no added row and no step, and variantvalidator is marked deadline-exceeded (`normalize-transcript-fit-server-error.yaml`).
