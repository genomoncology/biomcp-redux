# 0186 — Prove that indel IDs printed by variant get round-trip

Status: OPEN.

Milestone: 2

## Outcome

`get variant rs876657378` returns its card, and every variant ID and next step it prints, including the MyVariant repeat form such as `chr19:g.11106928AAG[1]`, is accepted by `get variant` and returns the same variant.

- Why: public issue 249 reported that 0.9 rejected the indel IDs it printed itself, and 0.9 fixed it. The 1.0 variant slice is complete without a case for that round trip.
- Starts from: the variant slice's ID grammar and next steps; issue 249 and 0.9's fix; existing variant captures.
- Rules: an ID that BioMCP prints is accepted as printed, or BioMCP prints the accepted spelling instead. No identifier is rewritten silently.
- Proof: outside-in cases first, watched to fail. If no existing capture holds this rsID and its repeat-form get, one bounded keyless pm job captures them once with zero retries. Cases walk the rsID's card and each printed next step. Ceilings at measured +15 percent. Then make landing-gate runs through pm job.
