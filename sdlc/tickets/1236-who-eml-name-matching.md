# 1236 — who-eml-name-matching

Status: OPEN.

Milestone: 7

## Outcome

`get drug <name> essential-medicines` finds a medicine on the WHO list when the list's name carries a second name in brackets or the caller uses a common synonym, and the DDInter `--partner` filter accepts the names `get drug` accepts.

## Why

Paracetamol reads as not on the Essential Medicines List today.

## Evidence

Checked 2026-11-27 against the live source and the 1.0.0 build at 4efd1dcb2426. Full tables: [source reach census](../records/2026-11-27-source-reach-census.md).

- Failing case 1: `biomcp get drug paracetamol essential-medicines --source who-eml` returns `status: absent`. The list names it "Paracetamol (acetaminophen)". The list holds 650 medicines.
- Failing case 2: `biomcp get drug osimertinib interactions --source ddinter --partner Coumadin` returns `complete` with 0 rows; `--partner warfarin` returns one. Osimertinib's 470 partners otherwise match DDInter's file exactly.

## Proof

- Proof: outside-in cases on the recorded list: paracetamol and acetaminophen both return the entry; the brand-name partner returns the warfarin row or a notice naming the name it could not resolve.
- Proof: a gate test over the shipped list: every listed medicine name resolves to its own entry.

## Starts from

- Starts from: `crates/biomcp-adapters/src/who_eml/read.rs`, `crates/biomcp-adapters/src/ddinter/`.

## Keeps

- Keeps: exact names return the same entry; a removed application is not shown as a listing.

## Defers

- Defers: nothing.

## Changes

- Changes: `crates/biomcp-adapters/src/who_eml/`, `crates/biomcp-adapters/src/ddinter/`, drug cases.
