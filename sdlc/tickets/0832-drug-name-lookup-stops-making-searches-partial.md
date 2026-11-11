# 0832 — drug-name-lookup-stops-making-searches-partial

Status: OPEN.

Milestone: 7

## Outcome

A trial or article search that names a drug returns `complete` when its own source answered, under the load of 25 searches at once; the drug-name lookup no longer turns the reply `partial` by timing out.

## Why

In `biomed-agent-bench/results/baseline-1.0.md` (build c2511ee90, which is current main), 70 of 109 article searches and 48 of 95 trial searches came back partial because the ChEMBL and openFDA name lookup timed out. An agent holding a partial answer leaves for curl, which is the main reason BioMCP's modes did not beat curl on tokens.

## Proof

- Proof: a routine case with a slow recorded name source shows the search complete with a notice naming the names it could not widen; a release-suite run of 25 concurrent searches against the recorded sources returns no partial reply caused by the lookup. The bench reruns the baseline's Sonnet MCP mode and the partial count falls below 10%.

## Starts from

- Starts from: `crates/biomcp-query/src/drug/search_names.rs` and `aliases.rs`, the shared resolver budget, and idea 'Explain an unattempted drug-name fallback' in `pm idea list`. One run to read: `baseline-1.0-sonnet/r2/v3-10-bixlenvo-ddi-2026/biomcp-mcp/mcp.out` in the bench's data folder on the build machine.
- PM decision: 2026-11-11. Diagnose first with a bounded timing run and put the cause in this ticket before choosing the fix: a cached name table, one shared lookup per call, a longer budget, or the lookup made non-blocking with the reply saying which names were searched. Prefer the fix that removes the wait over one that lengthens it.

## Keeps

- Keeps: a real source failure of the searched source still reports partial or failed; name lists stay bounded and frozen across pages; no result is presented as widened when it was not.

## Defers

- Defers: making widening the default for every drug filter is ticket 0830 on branch `1.1`, which builds on this.

## Changes

- Changes: `crates/biomcp-query/src/drug/`, `crates/biomcp-fetch/src/`, `tests/cases/drug/`
