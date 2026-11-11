# 0836 — command-map-and-examples-in-first-read

Status: OPEN.

Milestone: 7

## Outcome

The first text an agent reads, the MCP `initialize` instructions and the skill, holds a generated map of every command with what question it answers, plus one worked example for each of the skill's use cases in both the command-line form and the typed-tool form, each example checked against the grammar and the tool schemas at build time.

## Why

In `biomed-agent-bench/results/baseline-1.0.md` (build c2511ee90, which is current main) agents made more curl calls than BioMCP calls in the command-line mode on one model, and 19 of 200 MCP runs read the skill and never called BioMCP. Earlier experiments (`notes/archive/BioMCP and Code Mode - hypothesis and plan.md` in the workspace) found any map that names the commands halves what an agent reads.

## Proof

- Proof: a check fails when a documented example does not parse or does not fit its tool's schema, and when a registry command is missing from the map; the bench reruns one model and reports curl calls per BioMCP-mode run before and after.

## Starts from

- Starts from: `skills/SKILL.md` and its `{{front-map}}` generator, `crates/biomcp/src/mcp/protocol.rs` (instructions), `skills/use-cases/`.
- Depends on: 0833

## Keeps

- Keeps: the instructions' size ceiling; examples come from recorded cases, never hand-typed output.

## Defers

- Defers: a separate short profile of instructions for chat clients waits for this ticket's measurement.

## Changes

- Changes: `skills/`, `crates/biomcp/src/mcp/protocol.rs`, `crates/biomcp/src/skill/`
