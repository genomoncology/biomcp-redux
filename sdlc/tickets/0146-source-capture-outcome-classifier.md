# 0146 — Classify source capture outcomes in one shared helper

Status: OPEN.

Milestone: 3

## Outcome

One helper under `scripts/` classifies each captured HTTP reply by its actual status and the source's native body. Both a capture's stop decision and a comparison's eligibility come from that helper, and no capture script copies its own status rule.

- Why: three source probes stopped early or dropped valid results by reading a valid reply as a failure. The latest, 0136 j11622, had two blanket status rules: it refused 31 requests across 18 openFDA URLs after openFDA's native 404 "No matches found" reply, and it excluded valid empty outputs from the comparison. The counts are in 0060.
- Starts from: the j11622 capture script and its two status rules, and the recorder scripts that decide when a source stops.
- Proof: one offline job with synthetic equivalents of the incidents, testing the helper the recorder imports. A 200 reply whose body contains 429 is a success. A source's native empty 404 on a declared endpoint is an empty result and lets the next declared request run. A 404 on an unknown route, a structured 429, a malformed or cut body, a transport failure, and a budget or unknown-identity refusal each stay distinct. Recorder stop state and comparison eligibility are asserted for each. No provider call. Then make landing-gate runs through pm job.
