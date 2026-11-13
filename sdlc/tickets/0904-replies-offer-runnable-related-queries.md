# 0904 — replies-offer-runnable-related-queries

Status: OPEN.

Milestone: 7

## Outcome

An agent learns what it can ask next from the reply in hand, not from help. Every card and first page carries its related queries as runnable commands. The options that apply to that reply come as fill-in templates, such as `get drug BLA125514 approvals --decision-from <YYYY-MM-DD>`. A refusal's next step is the corrected command, not a help page. CLI and MCP read the same JSON, so both get it.

## Why

Ian, 2026-11-13, on reg-affairs round 2: "do we need to look at adding HATEOS-like 'related queries' on the JSON that informs both the CLI and MCP users."

Evidence from that run (biomed-agent-bench `personas/reg-affairs/runs/2026-11-13-r2/`, `calls.md` in `0041-persona-reg-affairs-r2` on the M5):
- 10 of its 11 help reads came before its first real query (shell calls 2-11). Four of them were spent learning one section's options (`help get drug approvals` and variants).
- It sent `--bogus x` three times (calls 22, 29, 33) to get the list of valid options from the refusal. It was using errors as a menu.
- Replies already carry `next` steps (section, page, pivot, disambiguate, configure). But 27 of the run's next steps were `configure` pointing to `help get drug` or `help search drug`, not to a command the agent could run.

## Proof

- Proof: recorded cases for a drug card, a drug approvals page, a diagnostic search page and an option refusal show:
  - the card lists each section with a runnable command and that section's options as templates;
  - the approvals page offers its date window, document and page forms as templates;
  - the refusal's first next step is the corrected command;
  - no `next` entry on these replies points to help when a runnable form exists.
- Each reply stays within its measured size limit (re-measured, with the growth stated).
- Measure after landing: help reads plus probing refusals per run, in the next persona round 2s (rare-disease, pv-scientist), against reg-affairs round 2's 11 help reads and 3 probes.

## Starts from

- Starts from:
  - the `next` step builder (`crates/biomcp-query/src/next.rs`, including its five-step cap at line 207, which 0881 ran into);
  - the per-command option lists that help and refusals already use;
  - the MCP structured content.

## Keeps

- Keeps:
  - existing `next` kinds;
  - the connect-time text caps (0836);
  - lean replies (0849): put templates on cards, first pages and refusals, not on every row.

## Defers

- Defers: suggesting other entities' queries (cross-entity pivots beyond the ones that exist). A suggestion that is accurate but narrow can steer an agent wrong (paper W6, rare-disease SCN1A), so add those only with evidence.

## Changes

- Changes:
  - `crates/biomcp-query/src/next.rs` and the reply builders that call it;
  - `crates/biomcp-types/`, adding an optional template form to a next step;
  - `tests/cases/`;
  - `docs/`;
  - the skill's paragraph on reading `next`.
