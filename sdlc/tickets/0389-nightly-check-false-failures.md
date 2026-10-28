# 0389 — Stop false failures in the nightly live check

Status: COMPLETE.
Landed: 218497d38490715882bc40846a9f5adbca6be87c

Milestone: 5

## Outcome

The nightly live check fails only for a product defect or an unclassified source change. A ClinicalTrials.gov continuation case gets its page token from the live first page instead of replaying a recorded token, and a LitVar publication list that only gains a PMID is classed as source churn.

- Why: scheduled j19320 at 0becd93c5 failed with no outage or product defect (coordinator triage m11979, 2026-10-28). One ClinicalTrials.gov command reused a literal recorded page token, which the source rejected with HTTP 400. The LitVar publications case returned HTTP 200 with one added PMID (1590 to 1591) and was counted as a failure. The retired-gene HGNC and Reactome change matched the data-change category accepted in 0252.
- Starts from: `scripts/nightly_live_check.py`, its tests, the j19320 report and manifest on Yellow, and the case definitions behind the two failures.
- Rules: no new live requests beyond the scheduled recipe while fixing; prove with `scripts/test_nightly_live_check.py` cases built from the retained diagnostic shapes. A removed or added source count stays visible as churn in the report; nothing is hidden. Keep the existing categories and add no new framework.
- Proof: a classifier case that failed before the fix classes the one-PMID addition as churn; a case shows the ClinicalTrials.gov continuation never sends a recorded token live; the next scheduled run reports no failure for these two cases unless the source fails.
- Changes: the nightly classifier and the ClinicalTrials.gov live case setup.
- Defers: Semantic Scholar HTTP 429 throttling stays a source limit. Reactome dropping `releaseStatus` needs no change, because the adapter never reads that field and the live reply completed.
