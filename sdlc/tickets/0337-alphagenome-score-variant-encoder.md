# 0337 — Encode AlphaGenome ScoreVariant requests and admit their replies

Status: COMPLETE.
Landed: f6c6e199cc1089687a9ae4a27c05619666f86b49

Milestone: 5

Depends on: 0333

## Outcome

Landed main registers the `ScoreVariant` method, encodes the ruled request with a public typed encoder over the private generated messages, and admits only a complete, checked scorer reply, so the authorized `ScoreVariant` capture runs from accepted landed code.

- Why: 0333 exposes only the PredictVariant DNase request and envelope, and the 0198 request contract (PM ruling m9972) needs `ScoreVariant` with 0.9.3's three scorers. PM ruling 2026-10-27 (m9979).
- Starts from: the landed 0320 to 0322 transport and 0333 registration, the 0198 request contract, and 0.9.3's `score_variant_request` and scorers as reference.
- Rules: the encoder builds exactly the ruled shape: organism human, a 524,288 bp interval centered on the variant, gene-mask LFC on RNA_SEQ, gene-mask splicing on SPLICE_SITES with width absent, and a 501 bp center-mask DIFF_MEAN on DNASE, with an empty model version as 0.9.3 sent and no ontology selector. Reply admission reuses the 0320 limits and 0322 completion rules and requires exactly three complete output groups, each associated with its requested scorer by position, since the wire reply carries no scorer tag; output reports that association as requested scorer identity, distinct from native returned metadata (PM ruling 2026-10-27, m9985). No product section, no key read and no provider call.
- Proof: encoded request bytes for the reviewed public variant match the expected shape field by field; synthetic complete three-group streams are admitted; missing or extra groups, incomplete tensors, chunk or descriptor order violations, a variant mismatch and oversized streams are refused.
- Changes: the method registration, encoder, admission check and cases.
- Defers: the capture, runtime output and replay, which are 0198.
