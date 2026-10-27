# 0333 — Register AlphaGenome as a keyed source with a narrow request encoder

Status: OPEN.

Milestone: 5

Depends on: 0323

## Outcome

Landed main registers `Source::AlphaGenome` with its `restricted` use label and terms, and `SecretName::AlphaGenomeApiKey` read from config and then `ALPHAGENOME_API_KEY` only when the source is called. A narrow encoder builds the one documented PredictVariant request the clinical reviewer selected, and a validator checks a complete response envelope and its tensors, so the 0198 capture runs on accepted landed code.

- Why: after 0323 landed at faabe05e, the 0198 capture delegate found main lacks the source, the key name and a public request path, and it must not alias another source or patch the transport privately. PM ruling 2026-10-27 (m9840).
- Starts from: the landed 0320 to 0322 transport and codec, the source registry, the keyed-source pattern from OncoKB, `2026-10-26-new-sources-follow-open-by-default.md`, `2026-10-27-alphagenome-streaming-transport.md` and the reviewer's selected variant and outputs.
- Rules: no product command or section yet; the encoder covers only the selected request shape; the validator reuses the 0320 limits and the 0322 admission check. A missing key skips with the key named and sends nothing. No key read and no provider call in this ticket.
- Proof: cases for the registry entry and label, config-then-environment key precedence with a placeholder key in isolated config, the missing-key skip with zero requests, the encoded request bytes for the selected shape, and validator acceptance of a complete synthetic stream and refusal of incomplete ones.
- Changes: source registry, secret names, the encoder, the validator and cases.
- Defers: the one capture and the variant section, which are 0198.
