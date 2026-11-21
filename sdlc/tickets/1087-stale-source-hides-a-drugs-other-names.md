# 1087 — stale-source-hides-a-drugs-other-names

Status: COMPLETE.
Landed: 8beaba573684a74b9b21d2f93b4eec9e18d749a8

Milestone: 7

## Outcome

When a drug lookup would have used another source's names for the drug and that source is stale or unreachable, the reply never says the input is wrong or the record absent. It names the source that could not be used and offers the sync or network step.

## Why

Found by the re-review of ticket 1081 (2026-11-20, jobs j43561 and j43562); present on main before that ticket. With MyChem's cached replies expired and the network down, and FDA's sources live:

- `biomcp get drug STI-571 approvals --source drugsfda` and `biomcp get drug STI-571 label --label-section boxed_warning --source openfda-label` exit 2 with "expected a supported command with checked arguments". With MyChem current, both find imatinib's record through MyChem's other names.
- `biomcp get drug blarcamesine approvals --source drugsfda` and `biomcp get drug zzznotadrug markets` with a stale EMA copy answer `absent`. They list the stale source, but the status reads as a current absence.

A caller is told a real drug does not exist when the cause is out-of-date data.

## Proof

- Proof: replayed cases for the four calls above in which the reply's status is not a usage refusal and not `absent`, names the stale or unreachable source, and offers its sync or network step; each fails on the base commit.
- Proof: with every source current, the same calls answer as they do today.

## Starts from

- Starts from: `crates/biomcp-query/src/drug/resolve.rs` and ticket 1081's change there; the runner tests under `source-data-age/` show how to make a stored copy stale and how to expire cached replies.
- PM decision, 2026-11-20, carried from ticket 1081: only a lookup whose sources were all current may say a name was not found.
- The developer checks whether gene, disease and variant lookups have the same step (retrying a name under another source's names) and reports which do.

## Keeps

- Keeps: a stale lookup that finds a record still answers `partial`, marked stale.

## Defers

- Defers: nothing.

## Changes

- Changes: the drug name lookup, its cases and runner tests.
