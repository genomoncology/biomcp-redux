# 0198 — Add an AlphaGenome predictions section to variant get

Status: OPEN.

Milestone: 5

Depends on: 0271, 0333

## Outcome

- Ian ruled on 2026-10-27 that AlphaGenome must work (`2026-10-27-alphagenome-streaming-transport.md`); this ticket follows the 0320 transport, and a clinical reviewer chooses the public variant and outputs before the one capture.
- Unblocked 2026-10-26 by `2026-10-26-new-sources-follow-open-by-default.md`: AlphaGenome is labeled `restricted`, runs only with the user's `ALPHAGENOME_API_KEY`, and captures may inherit the machine's key by name. PM ruling 2026-10-26 (m7290): AlphaGenome needs streaming gRPC, which the shared transport lacks, so this ticket moves to milestone 5 behind the 0271 transport design, with no source call or dependency before then.

`get variant ID alphagenome` returns AlphaGenome's numeric predictions for that one variant, as the API sends them, only when the user has set their own AlphaGenome key. Without a key it sends nothing and names the key to set.

- Why: Ian's entity boundary ruling of 2026-10-25 keeps AlphaGenome as a variant section with numbers only. 0.9 offered AlphaGenome predictions, including plots.
- Starts from: `decisions/2026-10-25-entity-boundary-rulings.md`; variant.md's AlphaGenome row; 0.9's AlphaGenome source; the keyed-source pattern from OncoKB and the NCI key; the publisher's API documentation and terms.
- Capture first: one bounded pm job inherits the key by name only and never prints, copies or records it. It sends one documented prediction request for one public variant (for example a public variant chosen by a clinical reviewer, as 0271 says on GRCh38) once, with zero retries, and archives the reply to Yellow outside Git. If the publisher's terms bar storing replies, the case replays from outside the repo.
- Rules: return the API's own outputs, labels, units, model version and intervals as sent; no plots, images, rankings or interpretation, and no claim that a prediction is a classification. The source registers as keyed with the publisher's terms and use label. Results carry the variant's build and the exact coordinates sent. A failure or missing key is a failed or skipped outcome, never an empty prediction.
- Installed proof: the installed CLI reaches a local TLS gRPC fixture through the AlphaGenome section on each available release target (moved from 0323, m8126 ask 67).
- Proof: outside-in cases first: the keyed section from the capture, the keyless refusal naming the key with zero requests, and synthetic 401, 429, 500 and broken replies as failed outcomes. Ceilings at measured +15 percent. The landing commit ticks the AlphaGenome boxes. Then make landing-gate runs through pm job.
