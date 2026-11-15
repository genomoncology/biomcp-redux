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

## Progress

- 2026-11-15: a typed call refused for a key its action does not take now gets the command line's fix for the same mistake ahead of the help step. The server spells the key as the option a CLI caller would have typed (`q` as `-q`, `who_pq` as `--who-pq`, `search` as `--search`), adds it to the refused call's command line, and asks the ordinary parser for its first non-help fix, so every `flag_fix` branch (text flag, source flag, respelling, HGVS name) applies and a call the parser would refuse is never suggested. The fix is then shown in typed form like any other next step. Other refusals (a missing or doubled argument) are unchanged, and so is the refusal schema (`crates/biomcp/src/mcp/typed.rs` `command_call`, `refusal::fix` in `crates/biomcp/src/mcp/typed/refusal.rs`).
- Measured in jobs on the M5: `diagnostic` search with `search` or `q` gets `{query: colorectal, source: fda-pma}`; `drug` search with `who_pq: true` gets `source: who-pq`; `gene` get with `source: hgnc` gets the call without it; `trial` search with `c` gets `condition`; `variant` search with `variant: "NM_007294.4(BRCA1):c.68_69del"` gets `{gene: BRCA1, coding: c.68_69del}` then help; a name without a gene and `gene_symbol` keep only help. Schema and docs regeneration changed nothing; `check-cheap` and all `mcp-parity` cases passed.
- Case: `tests/cases/mcp/typed-unknown-key-fixes.yaml` (offline; no source requests). Search text sent as `query`, `text`, `name` or `keyword` is already accepted as typed aliases, so the text-flag case uses `search` and `q`.
- 2026-11-15, review fix: with several unknown keys, the fix took only the first and suggested a call that meant something else (`{c: melanoma, i: pembrolizumab}` became `{condition: [melanoma]}`). Every unknown key now goes onto the command line and the parser fixes them in turn, as the CLI does; no fix is offered when any key cannot be an option (an empty key, one with `=` or a leading `-`, a text value with a leading `-`, or a false, null or object value). The case gained a step for each example. Measured with a typed stdio probe (job j41326): the command line itself fixes `--who-pq -q primaquine` but only gives help for `-q primaquine --who-pq` (job j41321), and a JSON object's keys have no order, so the server tries the keys in both orders; `{q: primaquine, who_pq: true}` now gets `{query: primaquine, source: who-pq}`.
