# 0198 — Add an AlphaGenome predictions section to variant get

Status: OPEN.

Blocked: 2026-10-26. AlphaGenome's current API terms exclude commercial entities, even for noncommercial work. No capture runs until Ian confirms a separate applicable license.

Milestone: 4

## Outcome

`get variant ID alphagenome` returns AlphaGenome's numeric predictions for that one variant, as the API sends them, only when the user has set their own AlphaGenome key. Without a key it sends nothing and names the key to set.

- Why: Ian's entity boundary ruling of 2026-10-25 keeps AlphaGenome as a variant section with numbers only. 0.9 offered AlphaGenome predictions, including plots.
- Starts from: `decisions/2026-10-25-entity-boundary-rulings.md`; variant.md's AlphaGenome row; 0.9's AlphaGenome source; the keyed-source pattern from OncoKB and the NCI key; the publisher's API documentation and terms.
- Capture first: one bounded pm job inherits the key by name only and never prints, copies or records it. It sends one documented prediction request for one public variant (for example BRAF V600E on GRCh38) once, with zero retries, and archives the reply to Yellow outside Git. If the publisher's terms bar storing replies, the case replays from outside the repo.
- Rules: return the API's own outputs, labels, units, model version and intervals as sent; no plots, images, rankings or interpretation, and no claim that a prediction is a classification. The source registers as keyed with the publisher's terms and use label. Results carry the variant's build and the exact coordinates sent. A failure or missing key is a failed or skipped outcome, never an empty prediction.
- Proof: outside-in cases first: the keyed section from the capture, the keyless refusal naming the key with zero requests, and synthetic 401, 429, 500 and broken replies as failed outcomes. Ceilings at measured +15 percent. The landing commit ticks the AlphaGenome boxes. Then make landing-gate runs through pm job.
