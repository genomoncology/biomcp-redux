# 0311 — Find drug labels whatever the case of the brand name

Status: OPEN.

Milestone: 5

## Outcome

A drug label search by brand name finds the label when openFDA stores the brand in a different case or with extra words than the user typed, and the exact field stays the first, precise request.

- Why: the 0098 drug run on 2026-10-27 (experiment 496, keyless) found that 1.0's label search sends `openfda.brand_name.exact`, which is case sensitive, so a brand stored as "Lifyorli" was missed, and a two-word brand was missed in both versions. 0.9.4 found the first. 1.0 otherwise led, answering 8 of 11 approval questions against 5.
- Starts from: the drug label adapter and `crates/biomcp/tests/runner/drug_label.rs`, the openFDA query syntax documentation, and the experiment 496 raw outputs on the evidence store under 0098.
- Rules: at most one fallback request, sent only when the exact request returns no result, using openFDA's tokenized `openfda.brand_name` field; the reply's identity check still requires the returned brand to match the typed brand ignoring case and spacing. No extra request when the exact request finds a label.
- Proof: replay cases for a lower-case brand and a brand with an extra word find the label through the fallback, a case where the exact request succeeds makes one request, and a case whose fallback returns a different brand reports no label.
- Changes: the drug label request and its cases.
- Defers: nothing.
