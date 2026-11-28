# 1255 — next-steps-carry-no-placeholders

Status: OPEN.

Milestone: 10

## Outcome

No next step or error suggestion anywhere in BioMCP carries a literal placeholder value that a model would copy, such as an example ID or the number 1. Each carries the real value from the request or reply, or is a step that needs no placeholder. A test fails on any next step that carries one.

## Why

Found in a run of all 100 items of the public CIViC MCP benchmark (Schimmelpfennig et al., Bioinformatics Advances 2026, vbag209) through BioMCP MCP tools only, GPT 6 Luna, 2026-11-28: 88 of 126 positive labels right. A reading of the 31 missed items found BioMCP usability faults behind 27 of them; reach is complete. The CIViC `civic_profile: 1` next step (ticket 1251) shows the failure mode: models copy suggested calls exactly. Any other next step with an example value has the same risk.

## Proof

- Proof: an audit listing every next step and error suggestion that carried a placeholder, each fixed; a test over the recorded cases and generated suggestions that fails when a next step carries a value that came from neither the request nor the reply.

## Starts from

- Starts from: the next-step builders in `crates/biomcp-query/src/` and the error suggestions in `crates/biomcp/src/grammar/`.

## Keeps

- Keeps: help examples, which are documentation and may use example values.
