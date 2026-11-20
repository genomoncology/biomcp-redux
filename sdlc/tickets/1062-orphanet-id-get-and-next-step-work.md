# 1062 — orphanet-id-get-and-next-step-work

Status: COMPLETE.
Landed: 38fdfd859e6c4f934a2b5e06aaf6cfea8205ab3e

Milestone: 7

## Outcome

`biomcp get disease ORPHA:115` returns the disease. Every `next` step a refusal offers runs without failing.

## Why

An agent given an Orphanet number was refused, followed the step BioMCP suggested, and that failed too. It reported that BioMCP holds no Orphanet data and downloaded Orphanet's file instead.

## Proof

- Proof: `get disease ORPHA:115` returns congenital contractural arachnodactyly with its MONDO ID; and a check over the recorded refusals that each `next` command exits 0. Both fail on the base commit.

## Starts from

- Starts from: Seen on main be96d8d40 (1.0.0), Linux, 2026-11-19, in 25 agent runs that used the command line build on real tasks.
- `get disease ORPHA:115` replies usage: option `--source`, valid value `ols`, with `next` of `get disease ORPHA:115 --section-limit 5 --source ols`. That command replies `status: failed`.
- `search disease TEXT --source orphanet` is refused; the valid sources are ols, mydisease, monarch and seer, although help lists `mydisease:orphanet`, `monarch:orphanet` and `orphadata`.
- `get disease MONDO:0007947 xrefs` does return cross-references, so the mapping exists once the caller has a MONDO ID.
- PM decision, 2026-11-19: an ORPHA, OMIM or DOID identifier given to `get disease` is resolved to its MONDO record through the cross-references BioMCP already holds, and the reply says which mapping was used and whether it is exact.

## Keeps

- Keeps: a name still goes through `search disease` first.

## Defers

- Defers: Orphanet's own mapping relation (exact, broader, narrower) if the held cross-references do not carry it; say so in the ticket.
- Deferred, 2026-11-19: neither held cross-reference carries it. MyDisease's `mondo.xrefs` lists bare codes, and the OLS MONDO term's `hasDbXref` rows name their supporting source but no relation. The reply names the mapping in `alias` (the MONDO record's `mydisease:mondo.xrefs.orphanet`, `.omim` or `.doid` row) and states no exactness. In MyDisease on 2026-11-19 no Orphanet or OMIM code appeared on more than one MONDO record.

## Changes

- Changes: disease get, its refusal steps, cases and help.
