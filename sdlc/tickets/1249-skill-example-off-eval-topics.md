# 1249 — skill-example-off-eval-topics

Status: OPEN.

Milestone: 10

## Outcome

The shipped research skill (`skills/SKILL.md`, line 30) shows the ClinGen gene-validity title-prefix call with a disease that no evaluation case uses, in place of "Fanconi anemia". The help example that mirrors it (`crates/biomcp/src/help/source_routes.rs` line 56 and `crates/biomcp/typed_front_map.rs` line 338) uses the same new disease, and the call still returns ClinGen curations matched by publisher title.

## Why

PM ruling 2026-11-28: an example in the shipped skill should not carry an evaluation topic, so an evaluated agent is not handed a worked call for the case it is scored on. "Fanconi anemia" is the subject of an evaluation case.

## Proof

- Proof: the existing skill, help and parity cases pass with the new disease on recorded ClinGen replies, recorded through BioMCP's recorder in a pm job; the new disease appears in no evaluation case (check the local evaluation case folders read-only and report the search).

## Starts from

- Starts from: `skills/SKILL.md`, `crates/biomcp/src/help/source_routes.rs`, `crates/biomcp/typed_front_map.rs`, and the cases that pin that example.

## Keeps

- Keeps: the call's shape (`get disease <name> gene-validity --source clingen --source-title-prefix`) and its point, matching publisher titles. Test fixtures that use "Fanconi anemia" for other purposes, such as `crates/biomcp/tests/runner/recent_parity.rs`, are out of scope unless the skill example's own case depends on them.
