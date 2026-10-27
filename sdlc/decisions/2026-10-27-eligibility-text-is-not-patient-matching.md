# Eligibility text is source text, not patient matching

Date: 2026-10-27

Decided by the PM.

BioMCP retrieves a trial's registry eligibility text whole and in order, including cohort alternatives, exclusions, exceptions and time windows, and pages it without splitting a criterion (0317). It never decides whether a patient meets a criterion, never classifies clauses, and never fills unknown patient facts. Agents quote the source text and leave matching to the clinical team. Search output keeps which query and fields found each trial, so a screen can show how each candidate matched. Patient matching would need a new product ruling by Ian.
