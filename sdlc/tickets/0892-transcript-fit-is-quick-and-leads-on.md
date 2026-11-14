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

- Proof: the fit check asks about the gene's transcripts in groups of about 20, each call under its own configurable deadline with a measured default, and stops at the first group that fits; a recorded case where the fit query outlasts a configured deadline shows no fits, no added row and no step; the `next` after a fit names `get variant <CA id>` with its clinvar section, and offers no step through CAR when CAR failed on that allele; the retries run in parallel; each source is listed once, and a failed call's state stays (`status::retain_source_state`).

## Starts from

- Starts from: 0885 (`crates/biomcp-adapters/src/variantvalidator/fit.rs`, `crates/biomcp-query/src/variant/normalize.rs`), `fan_out::run`.

## Keeps

- Keeps: never suggesting a base change; the original mismatch message.

## Defers

- Defers: gene-symbol HGVS input without an accession.

## Changes

- Changes: `crates/biomcp-adapters/src/variantvalidator/`, `crates/biomcp-query/src/variant/`, `tests/cases/variant/`, `docs/`

## Progress

- 2026-11-13 measured (from the Beelink, curl): VariantValidator's transcript list took 0.40-1.46 s. Its fit query on all the other NM_ transcripts took 0.68-4.47 s for SCN1A (14), DMD (17), NF1 (2), BRCA2 (5), TTN (6 of its 7 NM_ transcripts), CDKL5 (2), NRXN1 (24), CACNA1C (22), ANK3 (4), GAPDH (6) and TP53 (24). A base that fits nowhere also answered 200 within that time (BRCA2 c.5000, NRXN1 c.1000T>C). The 0885 "no fit" input SCN1A c.4729G>C now answers in 1.9-2.5 s and fits NM_001353954.2 and NM_001353955.2; its 60 s HTTP 500 in job j39173 did not recur here or on the M5 (job j39768), so it was a passing server fault, not VariantValidator's answer for "no fit".
- 2026-11-13 review (job j39842): the fit query's time grows with the transcript count: ANK2's 73 took 11.6 s, CAMK2D's 47 took 25.2 s, TPM1's 39 took 5.8 s, all past an 8 s deadline. Measured again from the Beelink: all of CAMK2D's 48 NM_ transcripts took 6.1-7.7 s, ANK2's 74 took 7.9-9.6 s, TPM1's 40 took 5.3 s; in groups of 20 (the last group smaller), each group took 1.2-3.6 s (ANK2 c.500C>G group 1 slowest).
- 2026-11-13 change: the fit check lists the gene's transcripts, then asks about them in groups of 20 (`FIT_GROUP` in `crates/biomcp-query/src/variant/normalize.rs`), lowest accession first, and stops at the first group that fits or fails. Each of these calls has its own deadline, `transcript_fit_seconds` in config.json, default 8 s (`FIT_DEADLINE` in `crates/biomcp-query/src/context.rs`: the slowest group plus one second of VariantValidator pacing is 4.6 s, and 8 s leaves room for the service running slower under load). The retries on the fitting transcripts run at once through `fan_out::run`. The `next` after a fit is `get variant <CA id> clinvar` when CAR resolved the allele; when CAR was not asked it is `get variant <fitting HGVS> clinvar`, and when CAR failed on that allele there is no step. It no longer repeats the normalize call. `sources` lists each source once through `status::retain_source_state`, so a failed call's state stays. The mismatch message is unchanged and no base change is ever suggested.
- 2026-11-13 recorded (job j39768): for NM_001165963.4:c.4729T>C with CAR, `next` is `get variant CA284979 clinvar` and `sources` is car failed (rejected) and variantvalidator ok; `get variant CA284979 clinvar` returns VCV000068552, Pathogenic/Likely pathogenic, the variant the code-mode agent found (`normalize-transcript-fit.yaml`). BRCA2 NM_000059.4:c.5000A>C fits no other transcript and returns no fits, no added row and no step (`normalize-transcript-nofit.yaml`). With `transcript_fit_seconds` 1 over the recorded 1,939 ms fit reply, variantvalidator fails deadline-exceeded with no fits, no added row and no step (`normalize-transcript-fit-deadline.yaml`). The server-error case is synthetic: it replays the status, body and 60.5 s delay VariantValidator gave in job j39173 with a hand-written text/html content type (`normalize-transcript-fit-server-error.yaml`).
