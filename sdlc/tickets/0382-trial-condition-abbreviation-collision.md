# 0382 — Expose ClinicalTrials.gov condition expansion that matches a different disease

Status: OPEN.

Milestone: 5

## Outcome

When a trial search for a condition such as "Snyder-Robinson syndrome" returns trials whose conditions do not contain the searched words, the reply says that some returned trials do not contain all searched words in their listed conditions, that ClinicalTrials.gov also searches titles, keywords and indexed terms and may expand synonyms, and offers an exact-phrase next step only if a documented expression is shown to narrow the result; otherwise it states the source limitation and suggests checking each trial's listed conditions (PM rulings m10791 asks 14 and 15; a capture showed `EXPANSION[None]` returned the same six trials). The cause is confirmed first: whether ClinicalTrials.gov's synonym expansion matched the shared abbreviation SRS (stereotactic radiosurgery).

- Why: experiment 499's rerun of six 0.9.3 rare disease witnesses and one published case on release/1.0 at ff2e9b64 (Jarvis m10854, 2026-10-27); public identifiers only found brain-metastasis radiosurgery trials returned for Snyder-Robinson syndrome without warning. Ian's ruling that 1.0 ships every known fix.
- Starts from: the trial search request builder, ClinicalTrials.gov's documented condition search and exact-term syntax, and 0318's broader search guide.
- Rules: BioMCP never drops or reranks returned trials, and decides by exact word containment against each trial's own condition list, not by guessing meaning. At most three bounded live requests.
- Proof: a replay case for Snyder-Robinson syndrome shows the expansion notice and the exact-phrase next step; any exact-phrase step offered has a replay that returns fewer non-matching trials; a condition whose results all contain the words shows no notice.
- Changes: trial search notices, next steps and cases.
- Evidence: experiment 499 kept only BioMCP output: 1.0 at ff2e9b64 `search trial --condition "Snyder-Robinson syndrome" --limit 25` returned NCT01345539, NCT01345552, NCT03285932, NCT03297788, NCT03303365 and NCT07668869, including FET-PET-guided stereotactic radiosurgery for recurrent glioblastoma (NCT07668869). The raw ClinicalTrials.gov reply was not kept.
- Defers: nothing.
