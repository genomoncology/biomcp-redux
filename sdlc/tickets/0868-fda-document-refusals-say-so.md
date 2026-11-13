# 0868 — fda-document-refusals-say-so

Status: COMPLETE.
Landed: 6e8e3973c8acaa4728394ea8c8bed3c8e58b52d5

Milestone: 7

## Outcome

When FDA refuses a document download (PMA summary and other FDA PDFs), the reply says FDA refused it, gives the document's URL and suggests a retry, instead of a bare `rejected`.

## Why

Bench 0040's rerun (failure #3 in `sdlc/records/2026-11-12-persona-rerun-failures.md`): reg-affairs and dx-lab-director got 327-byte `status failed`, `reason "rejected"` replies with no problem, status or URL, so an agent cannot tell a blocked download from a missing summary. The cause is FDA's abuse detection, which redirects some downloads intermittently.

## Proof

- Proof: a routine case replays an FDA redirect to its abuse-detection page for `get diagnostic P170019 summary --source fda-pma` and shows the problem, the document URL and the retry step; a recorded successful download still returns the summary.

## Starts from

- Starts from: the FDA PMA adapter's document fetch; the j38804 and j38812 evidence in the failures record.
- PM decision: 2026-11-12. Disclosure only. BioMCP keeps its honest client identity; it does not pose as a browser to get past FDA's detection.

## Keeps

- Keeps: successful downloads unchanged; the failed status.

## Defers

- Defers: caching or retrying downloads automatically.

## Changes

- Changes: `crates/biomcp-adapters/src/`, `tests/cases/diagnostic/`
