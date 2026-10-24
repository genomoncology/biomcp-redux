# 0094 — Pass HGVS to a named normalization service

Status: COMPLETE.
Landed: 13eccef91a24eb0c5d5be88239105b2f7d590595

Milestone: 2

Depends on: 0087

## Outcome

`variant normalize --source car|mutalyzer|variantvalidator|all --hgvs TEXT [--build B]` returns each service's result, mappings, warnings and version unchanged, with `all` returning independent labeled results and no consensus.

- Starts from: variant.md (Mutalyzer and VariantValidator rows, helper timings); the rulings.
- Risks the proof must cover: an HTTP 200 with a validation warning is a rejection; no transcript is chosen for the caller; one failing service does not hide the others under `all`.
- Proof: `NM_004333.4:c.1799T>A` through each service and `all`; the LRG protein warning; the TP53 paper mismatch rejected; failure cases; MCP parity; ceilings. Then make landing-gate runs through pm job after current main.
