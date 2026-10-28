# 0330 — Tell agents to keep match provenance and read every eligibility page

Status: COMPLETE.
Landed: a4f7f98133d59b9193a4d18fa16e1fb4be000f6e

Milestone: 5

## Outcome

The agent guide's trial screening section tells an agent to record which search found each trial, to read every eligibility page until no criterion is marked continued, and that registry eligibility text is source text to quote, not a patient match.

- Why: experiment 495 (Jarvis m9510) asked for a bounded multi-search strategy that keeps how each candidate matched, and for a supported-use boundary between retrieving eligibility text and matching a patient. 0317 fixed page boundaries and 0318 added broader searches; these two lines finish the workflow. Supported-use ruling 2026-10-27: `2026-10-27-eligibility-text-is-not-patient-matching.md`.
- Starts from: `skills/SKILL.md` "Screen registry trials", the 0317 continued-criterion flag and the 0318 wording.
- Rules: no clinical judgment in the guide; it describes a search and reading sequence. The embedded guide budget follows the measured caps rule.
- Proof: the guide section contains the three instructions and the embedded guide budget passes.
- Changes: `skills/SKILL.md` and its embedded copy.
- Defers: nothing.
