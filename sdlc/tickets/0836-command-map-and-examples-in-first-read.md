# 0836 — command-map-and-examples-in-first-read

Status: OPEN.

Milestone: 7

## Outcome

The first text an agent reads, the MCP `initialize` instructions in every mode and the skill, is no larger than on main (skill 5,557 characters rendered, typed instructions 2,009, hosted 1,996, plain 3,669) and aims for about 600 tokens for the skill, and names how to ask for more. The generated map of every command with the question it answers, and one worked example per use case in command-line and typed form, are served on request (the help command and a per-tool help page), each example checked against the grammar and the tool schemas.

Re-scoped 2026-11-11: the first build put the map and examples into the first read, growing the skill to 12,974 characters. `sdlc/records/2026-11-11-token-gap-root-cause.md` found the skill is re-read every turn and costs about 15% of a typed repeat's tokens, so the review rejected that and the map moved to on-request. The branch's generator, typed-call builder and checks are kept.

## Why

In `biomed-agent-bench/results/baseline-1.0.md` (build c2511ee90, which is current main) agents made more curl calls than BioMCP calls in the command-line mode on one model, and 19 of 200 MCP runs read the skill and never called BioMCP. Earlier experiments (`notes/archive/BioMCP and Code Mode - hypothesis and plan.md` in the workspace) found any map that names the commands halves what an agent reads.

## Proof

- Proof: a check fails when the skill or any mode's instructions exceed main's size; a check fails when a documented example does not parse or does not fit its tool's schema, and when a registry command is missing from the map; the bench reruns one model and reports curl calls per BioMCP-mode run before and after.

## Starts from

- Starts from: `skills/SKILL.md` and its `{{front-map}}` generator, `crates/biomcp/src/mcp/protocol.rs` (instructions), `skills/use-cases/`.
- Depends on: 0833

## Keeps

- Keeps: the instructions' size ceiling, now with a ceiling on plain mode too; examples come from recorded cases, never hand-typed output.

## Defers

- Defers: a separate short profile of instructions for chat clients waits for this ticket's measurement.

## Changes

- Changes: `skills/`, `crates/biomcp/src/mcp/protocol.rs`, `crates/biomcp/src/skill/`
