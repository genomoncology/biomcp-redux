# 0382 — Expose ClinicalTrials.gov condition expansion that matches a different disease

Status: OPEN.

Milestone: 5

## Outcome

When a trial search for a condition such as "Snyder-Robinson syndrome" returns trials whose conditions do not contain the searched words, the reply says that ClinicalTrials.gov expanded the condition term, and offers a next step that searches the exact phrase only. The cause is confirmed first: whether ClinicalTrials.gov's synonym expansion matched the shared abbreviation SRS (stereotactic radiosurgery).

- Why: experiment 499's rerun of six 0.9.3 rare disease witnesses and one published case on release/1.0 at ff2e9b64 (Jarvis m10854, 2026-10-27); public identifiers only found brain-metastasis radiosurgery trials returned for Snyder-Robinson syndrome without warning. Ian's ruling that 1.0 ships every known fix.
- Starts from: the trial search request builder, ClinicalTrials.gov's documented condition search and exact-term syntax, and 0318's broader search guide.
- Rules: BioMCP never drops or reranks returned trials, and decides by exact word containment against each trial's own condition list, not by guessing meaning. At most three bounded live requests.
- Proof: a replay case for Snyder-Robinson syndrome shows the expansion notice and the exact-phrase next step; the exact-phrase step's replay returns no radiosurgery trials; a condition whose results all contain the words shows no notice.
- Changes: trial search notices, next steps and cases.
- Defers: nothing.
