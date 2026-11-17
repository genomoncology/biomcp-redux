# 1009 — clinvar-assertions-select-record-kind

Status: OPEN.

Milestone: 7

## Outcome

`variant assertions --source clinvar` and the ClinVar section take a record-kind filter (vcv, rcv or scv), with its own total, so per-submitter rows arrive alone.

## Why

tumor-board paged eight times for BRCA1 R1699Q's 58 SCVs and lost a page to a row with no record id (bench `personas/tumor-board/runs/2026-11-16/analysis.md` 9.1, calls.md 1694, 1822); both graders named it a gap in both arms.

## Proof

- Proof: a replayed case for clinvar:37636 asking scv rows only, asserting the submitter total; fails on the base commit.

## Starts from

- Starts from: the analysis named in Why, and the BioMCP code it cites.

## Keeps

- Keeps: every other reply field and command form.

## Defers

- Defers: nothing.

## Changes

- Changes: the named command's code, help, cases and a release-notes line.

## Progress

- Live check (2026-11-16, ClinVar efetch for Variation 37636): VCV000037636.161 holds 58 submitted records (SCV) and 13 per-condition records (RCV). BioMCP lists the VCV rows, then RCV rows, then SCV rows, and pages over all of them, so the 58 submitter rows started after the aggregate rows.
- Changed: `variant assertions --source clinvar` takes `--kind vcv|rcv|scv`, and `get variant ID clinvar` (or `all`) takes `--clinvar-kind`. The filter runs before paging, so `page.total` counts only that kind and `next` keeps it. `--kind` is refused with `--source erepo` or with `--assertion`; `--clinvar-kind` is refused without the clinvar or all section. One optional query field each; the new enum `ClinvarRecordKind` is a new type, not a change to `AssertionKind`.
- The "row with no record id" in the persona run is an unmapped row (a ClinVar label with no common-model statement): its record sits at `unmapped.attribution.record`, not `statement.attribution.record`. This ticket does not move it; `--kind scv` still returns such rows when a submitter's label is unmapped.
- Recorded live in j42016: `tests/replies/variant/assertions/clinvar-kind/` (one ClinVar efetch reply, 580 KB). Schemas and docs regenerated in the same job; `make schema-generate` passed its compatibility check.
- Proof: `tests/cases/variant/assertions-clinvar-kind.yaml`. `--kind scv --limit 50` gives total 58, 50 rows, all `scv`, and `next` keeps `kind: scv`; offset 50 gives the last 8; `--kind rcv` gives 13; unfiltered stays 72 starting with the VCV row; `get variant clinvar:37636 clinvar --clinvar-kind scv` gives a section total of 58; `--kind` with `--source erepo` is refused. On the base commit the first step fails: `--kind` is an unknown option there.
- Checks: j42020 and j42028 (check-cheap, variant, help, query-validation and mcp-parity cases). `help get variant clinvar` is 3,049 bytes, under the case's 3,212 cap; `--clinvar-kind` was added to that case's option list.
- Gate j42040 passed on ca69f2526 (branch merged with origin/main): `make --keep-going landing-gate docs-check schema-release-check`.
