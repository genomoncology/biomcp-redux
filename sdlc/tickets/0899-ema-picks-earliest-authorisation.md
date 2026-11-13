# 0899 — ema-picks-earliest-authorisation

Status: OPEN.

Milestone: 7

## Outcome

When a drug name matches several EMA medicines, the name lookup (and so the cross-market row from 0881) picks the earliest authorisation, as the FDA lookup already picks the original application, and says that is its rule. The cross-market match-count description stays true when a market has no record.

## Why

A regulatory-affairs user comparing first approvals across markets is misled today: for lamivudine the EMA lookup picks Lamivudine Teva Pharma B.V. (EMEA/H/C/001111, authorised 2009-12-10) over Epivir (EMEA/H/C/000107, 1996), following source search order among equals (found in the independent review of 0881, 2026-11-13; persona evidence in biomed-agent-bench `personas/reg-affairs/runs/2026-11-13/`).

## Proof

- Proof: the recorded lamivudine markets case (`tests/cases/drug/markets.yaml`) chooses EMEA/H/C/000107 with its 1996 date and a `choice` that states the earliest-authorisation rule; dolutegravir and imatinib choices are unchanged unless the rule picks an earlier record, which the case then states. The recorded EMA file in `tests/replies/drug/markets/` gains Epivir if it lacks it, and its README says so.

## Starts from

- Starts from: the EMA name lookup used by `crates/biomcp-query/src/drug/markets.rs`; the FDA lookup's original-application choice as the model.

## Keeps

- Keeps: every market's `matches` count and list call; the five-step ordering from 0881.

## Defers

- Defers: nothing.

## Changes

- Changes: the EMA drug lookup in `crates/biomcp-query/src/drug/`, `crates/biomcp-types/src/drug/markets.rs` and `crates/biomcp-types/schemas/drug-markets.json` (description only: the `matches` text ends "this row is one of them", false when the market reports 0; say "a found row is one of them"), `tests/cases/drug/markets.yaml`, `tests/replies/drug/markets/` (README: NDA204790 and NDA021335 are also asked twice, like NDA020596; or reuse the first read in `fda()` at `markets.rs:78` so none is).
