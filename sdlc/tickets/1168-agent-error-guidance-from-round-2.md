# 1168 — agent-error-guidance-from-round-2

Status: OPEN.

Milestone: 7

## Outcome

When an agent makes one of the mistakes seen in round 2's 120 Sol runs, BioMCP's reply tells it the right next call, and the typed MCP tools accept what schema-filling clients send.

## Why

Round 2 (pilot B, runs/round-2-sol.md; retained batches r2-apb-{l,b,o}, calls in results/*/1/record/calls-01-answer.jsonl) logged these against 1.0.0-3ce84f683315:

1. Typed tools reject schema-filled calls. The chatbot sends both the `search` and `get` objects with every field empty or default, e.g. drug `{"action":"get","search":{...all empty...},"get":{...}}`. BioMCP answers `usage.arguments`, 46 times across article, drug, diagnostic and trial. An object that the action does not use and that holds only empty or default values should be ignored; a non-empty one should get an error naming the action's object.
2. The message "unknown option: this command has no error.option; run next, or use an option valid_values lists" reads as an unfilled template (seen in typed tools and `search variant --hgvs`). It should name the rejected option.
3. Wrong recovery suggestions: `variant gwas-get GCST000022` returns unknown_command with next `help variant normalize`; `fetch` with an EMA PDF URL returns unknown_command with next `help get diagnostic`. Each should point at the command that does what was asked, or say none does.
4. `--fields` on `get` cards (60 errors, e.g. `get article 17657473 --fields answer.card`) is refused. Either support it on cards or say which call narrows the card.
5. Every variant card carries dozens of `value.unknown` problems for ClinGen Allele Registry `representations.expression` (about 65 per batch), which buries real problems.
6. Failed source replies with `next: []` (Europe PMC `deadline-exceeded`, 45 times; Semantic Scholar `unavailable`, 17) leave the agent without a retry or alternative.

## Proof

- Proof: a replay per item: the schema-filled typed call succeeds; the rejection names the bad option; the two misdirected calls suggest a relevant command; the card `--fields` call either narrows or names the call that does; a variant card reports the unknown CAR expression once per record or not as a problem; a timed-out or unavailable source offers a next call. Existing ceilings and limits stay as they are.

## Changes

- Changes: typed MCP argument parsing, usage error messages and suggestions, the CAR problem reporting, and failed-source next calls, with replay cases.
