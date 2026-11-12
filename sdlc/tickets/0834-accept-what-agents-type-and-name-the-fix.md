# 0834 — accept-what-agents-type-and-name-the-fix

Status: COMPLETE.
Landed: 3e21941a8ddfcc90f1f4bd06c3d76193e42a018e

Milestone: 7

## Outcome

The commands agents actually typed in the baseline either run or fail with an error that names the option at fault and the exact form to use: short flags `-q`, `-c` and `-i`; a section given as a positional word or as `--section`; retired command names, which answer with the replacement command written out for this request; and no usage error leaves the option unnamed. In typed MCP mode, a section value is plain text listed in the tool's description rather than a fixed list in the schema, so a wrong section such as article `text` or gene `hgnc` reaches the server, which answers with the working call (article `text` names `fulltext`).

Added 2026-11-11 from the typed-tools review: after typed tools landed, the only calls a strict client still refused before BioMCP saw them were section values outside the fixed list.

## Why

In `biomed-agent-bench/results/baseline-1.0.md` (build c2511ee90, which is current main) Sonnet hit a usage error naming no option 40 times, wrong section forms 43 times, short flags 22 times and retired commands 20 times. Each is a wasted turn and a reason to leave for curl.

## Proof

- Proof: routine cases under `tests/cases/grammar/` replay each failing call text from the baseline's failed-call table and assert it runs or returns the named fix; a check fails if any `usage.arguments` reply lacks an option or argument name.

## Starts from

- Starts from: `crates/biomcp/src/grammar.rs`, `crates/biomcp-types/src/response/error.rs`, the retired-command table, and the failed-call tables at the end of the baseline report.

## Keeps

- Keeps: strict checking of values; nothing is guessed silently: an accepted alias is echoed in the reply's query as the canonical form.

## Defers

- Defers: fuzzy matching of unknown option names waits for evidence that it is needed.

## Changes

- Changes: `crates/biomcp/src/grammar.rs`, `crates/biomcp-types/src/response/`, `tests/cases/grammar/`, `crates/biomcp/typed_front_map.rs` and the typed tool-list snapshot (tool-list size stays within 1% of main)
