# 1140 — make-observed-refusals-replayable

Status: COMPLETE.
Landed: 08c489c8482777ee49b25e55337ff85f7e6e6c7d

Milestone: 7

## Outcome

The selector, list and field mistakes agents actually made either normalize without loss or return a working call for the same question, so the first retry succeeds.

## Why

In the 2026-11-22 comparison, connector-only agents hit 291 usage refusals over 72 runs, MCP agents with a shell hit the same top ones, and each refusal costs a turn. The largest groups: drug label-history with `section_limit` (38) or `decision_from` (33); `find` without or with the wrong label selector (20); `section` given as a list (7); PMA `--supplement` with `section` (9); empty withdrawal-list searches (8); drug and disease `id` lists on routes that take one ID (37), while the typed schema advertises string-or-array for every ID; field names nested under `search` or misnamed (trial 21, drug 14, gene constraint 6), where seven corrections dropped the projection. Today's refusal lists the options involved, not a combination that works. Code: `crates/biomcp-types/src/drug/{history.rs:75-145,query.rs:39-91,label_find.rs:14-75}`, `crates/biomcp-types/src/query/diagnostic/get.rs:217-238`, `crates/biomcp/src/mcp/typed/{arguments.rs:83-107,forms.rs:110-158,237-253}`, `crates/biomcp/src/fields/select.rs:94-155`.

## Changes

- Each conflict refusal carries a `next` call with the caller's real ID and dates that answers the named sub-question: date-window approvals (`get drug NDA203214 approvals --source drugsfda --decision-from … --decision-to …`), first mention in history (`label-history --label-section … --find …`), a current-label match (`label --label-section … --find …`), a PMA supplement read.
- Withdrawal-list refusals show a query by drug, indication or year.
- The typed schema states each action's real ID and section cardinality.
- Unambiguous nested `fields` move to the top level; the valid subset is kept and unknown names are named.
- Never silently drops a date or other constraint from the caller's question.

## Proof

- A table of the saved refusing requests (pv-2, pv-3b, tb-2b, ra-3, ci-3) runs through the typed tool and the CLI; each returned next call runs against a fixture and answers the named sub-question with the same ID, dates and fields.

## Defers

- Accepting ID lists on every get; catalog-wide field enums (adds prompt bytes, does not make an unprojectable view projectable).
