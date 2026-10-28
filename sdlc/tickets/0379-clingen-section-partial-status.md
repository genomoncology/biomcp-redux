# 0379 — Keep ClinGen validity and dosage status separate

Status: OPEN.

Milestone: 5

Depends on: 0388

## Outcome

The gene ClinGen section reports gene-disease validity and dosage sensitivity with separate statuses, so a failure in one leaves the other's data in the reply with its own status. The existing case that pins a dosage failure to a whole-section `unavailable` changes to expect validity data with a dosage failure.

- Why: regression against 0.9.3 found by experiment 499's rerun of six 0.9.3 rare disease witnesses and one published case on release/1.0 at ff2e9b64 (Jarvis m10854, 2026-10-27); public identifiers only. 0.9.3 kept separate status for validity and dosage. Ian's ruling that 1.0 ships every known fix.
- Starts from: the gene ClinGen section type, its two source reads and the pinned dosage failure case.
- Rules: each part's status names its own source outcome; no part is filled from the other.
- Proof: cases where dosage fails and validity succeeds, where validity fails and dosage succeeds, and where both fail; the baseline is regenerated.
- Changes: the ClinGen section type, query, cases and the baseline.
- Defers: nothing.
