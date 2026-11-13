# 0905 — size-limits-root-cause

Status: OPEN.

Milestone: 7

## Outcome

A root cause analysis (`skills/rca.md` in the agents repo) of why size limits keep forcing developers to trim wording ticket by ticket. It ends with ranked fixes, and the smallest one is done or ticketed, so adding a source, option or next step no longer means hunting for spare characters.

## Why

On 2026-11-13, five tickets ran into a size limit and trimmed other text to fit:
- 0889 (Orphanet prevalence) hit the connect-time text caps in `scripts/check_front_map.py` (ticket 0836: must not grow). Hosted single now has 1 character to spare.
- 0893 (MAUDE event type) pushed the MCP `search_adverse_event` tool past its 2,000-character limit (`crates/biomcp/tests/runner/typed_schemas.rs:182`). The developer shortened two other flags' titles.
- 0894 (counts by date) hit the same 2,000-character limit and shortened three descriptions.
- 0886 (phenotype ranking) raised a help size budget.
- 0896 (FAERS labels) raised a synthetic card's size ceiling by 205 bytes.

0904 (runnable related queries) will add more to replies.

## Proof

- Proof: the RCA's write-up in this ticket's Progress. It counts the cases, sorts them by cause and names fixes with cost and reach, and the smallest fix lands or has its own ticket.

## Starts from

- Starts from:
  - `scripts/check_front_map.py`;
  - `crates/biomcp/tests/runner/typed_schemas.rs`;
  - the help budget and reply size checks;
  - ticket 0836 and the tickets named above (their Progress sections and branches).

## Keeps

- Keeps: the reasons the limits exist (what MCP clients load on connect, token cost to agents) unless the RCA shows a limit protects nothing.

## Defers

- Defers: nothing.

## Changes

- Changes: depends on the findings.
