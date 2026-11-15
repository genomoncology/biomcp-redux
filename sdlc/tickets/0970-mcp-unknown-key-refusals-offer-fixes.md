# 0970 — mcp-unknown-key-refusals-offer-fixes

Status: OPEN.

Milestone: 7

## Outcome

A typed MCP call refused for an unknown argument gets the same runnable fixes a CLI caller gets for the same mistake (the text-flag, source-flag, respelling and, after 0969, HGVS-name suggestions), not only a help step.

## Why

0969 review (2026-11-15): unknown keys in typed MCP calls are refused in `crates/biomcp/src/mcp/typed.rs:224-233` with only a help step and never reach `flag_fix` (`crates/biomcp/src/grammar/errors.rs`), so MCP agents miss every existing refusal fix that CLI agents get.

## Proof

- Proof: cases sending a typed MCP call with each mistake `flag_fix` covers (including a full HGVS name in `variant`) and asserting the same first `next` step the CLI case gets.

## Starts from

- Starts from: `crates/biomcp/src/mcp/typed.rs:224-233`; `flag_fix` in `crates/biomcp/src/grammar/errors.rs`; 0969's `hgvs_flag`.

## Keeps

- Keeps: the refusal schema; the CLI refusals; MCP's next steps in typed form where the CLI gives command text.

## Defers

- Defers: nothing.

## Changes

- Changes: the typed MCP refusal path, its cases.
