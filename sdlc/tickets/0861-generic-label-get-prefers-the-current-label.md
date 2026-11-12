# 0861 — generic-label-get-prefers-the-current-label

Status: COMPLETE.
Landed: 4471ded6a96eab7c6922ceb1e973c17ee4407f90

Milestone: 7

## Outcome

`get drug NAME label` given a generic name reads the current originator label (or the label with the most indications) and says which it chose and why, or warns that several labels exist and lists them.

## Why

From the bench 0039 persona review (2026-11-12, runs on BioMCP 58d83b6da), `personas/<slug>/runs/2026-11-12/review.md` and `log.md`: pt-committee log.md:36: `get drug rivaroxaban label` silently picked a generic label with 2 of 10 indications.

## Proof

- Proof: a routine case with recorded labels for rivaroxaban shows the XARELTO label chosen and the choice disclosed.

## Starts from

- Starts from: drug label resolution after 0815 and 0851.

## Keeps

- Keeps: label ID reads unchanged; exact-match rules of 0851.

## Defers

- Defers: label version history.

## Changes

- Changes: `crates/biomcp-query/src/drug/`, `tests/cases/drug/`

## Progress

- Choice rule: among labels whose name matches, an exact brand match first; then NDA/BLA labels under their own brand name, other NDA/BLA labels, then generic (ANDA) or other labels; then the most numbered indications (1.1, 1.2, …); then the newest revision. The reply's name lookup adds a `reason` line naming the chosen label, why, and how many others of each kind share the name; the first five others stay as ready get commands.
- Live (job j38722, 2026-11-12): rivaroxaban chose XARELTO 10db92f9 (Janssen, revised 2026-09-10, 10 numbered indications) over 26 others (6 XARELTO repackager sets, 20 generics, most listing only 1.7 and 1.8); main chose the Apotex generic e98f6d83 (job j38713). Apixaban chose ELIQUIS 7be1f4c1 (6 numbered indications) over 9 others.
- Metformin fails on main (j38713) and on this branch (j38722) before any choice: `get drug metformin label` exits 4 with the openfda-label source `state: failed`, `reason: too-large` while the name lookup reads its 50-row pages of full labels. Not the label choice; left for its own ticket.
