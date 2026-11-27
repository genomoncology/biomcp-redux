# 1203 — civic-drug-submitted-evidence

Status: OPEN.

Milestone: 7

## Outcome

`get drug <therapy> civic --evidence-status submitted` returns CIViC's submitted evidence rows instead of dropping each one with a `status value.unknown` problem, and the drug and variant routes return the same statuses when `--evidence-status` is omitted, or each route's help says which statuses it returns.

## Why

CIViC holds more submitted (not yet moderated) evidence than accepted evidence for many therapies, and the CIViC MCP benchmark counts both. The flag is offered and accepted, but every row it asks for is thrown away, so an agent sees a total with no rows and cannot check them.

## Evidence

Checked 2026-11-27 against live CIViC GraphQL and the 1.0.0 build at d1c6afa3def3.

- `get drug 187 civic --source civic --civic-kind evidence --evidence-status submitted --evidence-significance resistance --section-limit 3` returns `page.total` 9 and `returned` 0, with problems `EID8052 status value.unknown`, `EID7388 status value.unknown` and `EID11098 status value.unknown`.
- Cause: `build` in `crates/biomcp-adapters/src/civic/drug_map.rs` refuses any row whose status is known and not `Accepted`, whatever status the query asked for.
- With no `--evidence-status`, `drug_request.rs` sends `status:ACCEPTED`, while `get variant ... civic` returns accepted and submitted rows (profile 4553: 8 rows, 2 of them submitted). For pembrolizumab (therapy 138) CIViC holds 25 accepted and 33 submitted items; the drug route shows 25. The help says an omitted status "preserves the source's existing default".
- In the CIViC benchmark pilot, two chatbot runs with only BioMCP's tools hit this on osimertinib and palbociclib and had to leave items unresolved.

## Proof

- Proof: an outside-in case on a recorded osimertinib reply returns the submitted resistance rows with status `SUBMITTED`; a case without `--evidence-status` shows the default the help states, the same on the drug and variant routes; existing accepted-only replies are unchanged.

## Starts from

- Starts from: `crates/biomcp-adapters/src/civic/drug_map.rs` (`build`), `drug_request.rs` (status argument), `evidence_filter.rs`, and the variant route's status handling in `variant.rs`.

## Keeps

- Keeps: rejected items stay out unless asked for; accepted-only replies keep their shape.

## Defers

- Defers: lookups for profiles without a CAid, which 1202 owns.

## Changes

- Changes: `crates/biomcp-adapters/src/civic/`, the drug CIViC help text, cases and replies.
