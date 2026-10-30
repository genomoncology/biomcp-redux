# 0497 — Stop the nightly live check failing on PubMed removals and Reactome's dropped field

Status: OPEN.

Milestone: 5

## Outcome

The nightly live check classes two source changes as churn instead of failure: a PubMed result list that loses an ID from the middle so later IDs shift up a page, and the Reactome reply that no longer carries `releaseStatus`. Scheduled j22745 at dee9172a9 failed only on these three cases, with no product defect.

- Why: j22745 (2026-10-30) failed `article/edges/no-contexts.yaml` (PMID 31548347 left the citing list and the next IDs moved up, with 26712084 added at the tail), `article/fold/live-reverse.yaml` (33377972 replaced by 35312980) and `gene/sections/pathways-tp53.yaml` (`/*/releaseStatus` removed). 0389 deferred the Reactome field because the adapter never reads it, but the classifier still counts a removed field as an invariant failure, so the check has failed on it every night since j21096.
- Starts from: `scripts/nightly_live_check.py` (`same_window`, `dependent_change`, `classify_case`), `scripts/test_nightly_live_check.py`, the j22745 report on Yellow, and the pathways-tp53 recording.
- Rules: refresh the pathways-tp53 recording from one bounded public Reactome capture rather than teaching the classifier to ignore removed fields; a removed or swapped ID counts as churn only when every other ID in the same discovery list is unchanged in order; no new framework or category; nothing is hidden from the report.
- Proof: classifier cases built from the j22745 shapes class both PubMed cases as churn and fail on the current script; a case with an ID removed and a new request parameter still fails; the replay suite passes on the refreshed Reactome recording; the next scheduled run reports no failure for these three cases unless the source fails.
- Changes: the classifier, its tests and the Reactome recording.
- Defers: nothing.
