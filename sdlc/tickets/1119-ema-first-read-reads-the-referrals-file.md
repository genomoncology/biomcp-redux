# 1119 — ema-first-read-reads-the-referrals-file

Status: OPEN.

Milestone: 7

## Outcome

A first read of an EMA referral procedure that has not been synced returns the Commission decision date an explicit sync would give, or says the read is incomplete. It never says no date is reported when the referrals file was simply not fetched.

## Why

Found by the review of ticket 1070 (2026-11-20), read from code, not yet run; present on main. `biomcp get drug EMEA/H/C/005248 regulatory --source ema` on a cold copy, after the medicines file is fetched, skips the missing referrals file and says no Commission decision date is reported. The replay case `tests/cases/drug/ema-dates.yaml` starts with `sync ema`, which loads both files and returns `2026-05-29`, so the suite does not see the cold path.

## Proof

- Proof: a replayed case for the call above on a cold copy that fails on the base commit: the reply gives the decision date, or is marked incomplete with the referrals source named.
- Proof: with the referrals fetch failing, the reply is marked incomplete and does not say no date is reported.

## Starts from

- Starts from: `crates/biomcp-adapters/src/ema/referrals.rs` near line 43. The developer first reproduces the fault and reports if it does not reproduce.

## Keeps

- Keeps: the synced reply is unchanged.

## Defers

- Defers: nothing.

## Changes
