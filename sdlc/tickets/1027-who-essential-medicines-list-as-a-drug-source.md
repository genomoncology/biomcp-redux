# 1027 — who-essential-medicines-list-as-a-drug-source

Status: OPEN.

Milestone: 8

## Outcome

`get drug NAME` can read the WHO Model List of Essential Medicines: whether the drug is listed, core or complementary, its section, the edition it was added, therapeutic alternatives, and the Expert Committee's text, from the WHO eEML's own published data, with its licence checked before any reply data is committed.

## Why

Milestone 8 lists "the WHO essential medicines list" (`sdlc/planning/milestones.md`, section 8; `sdlc/planning/sources-to-add.md:52`). The pt-committee run curled list.essentialmeds.org and could not tell core from complementary, missing four facts (bench `personas/pt-committee/runs/2026-11-16/analysis.md` 5.1-5.3, 10.4).

## Proof

- Proof: a replayed case for dabigatran and warfarin asserting list status, core or complementary and edition; a live check recorded in Progress.

## Starts from

- Starts from: an existing adapter of the same shape, and the source's own documentation and terms.

## Keeps

- Keeps: every existing command form.

## Defers

- Defers: nothing.

## Changes

- Changes: a new adapter or section, its help, schemas, cases, recordings and a release-notes line.

## Progress

- 2026-11-16, access (checked live with curl): the eEML (list.essentialmeds.org) has no documented API, but its pages serve schema.org JSON-LD when asked with `Accept: application/json`. `GET /medicines/{id}` gives the medicine (name, ATC via its recommendations, `guideline` links to its recommendations, `equivalentFor` links when it is listed only as a therapeutic alternative, `status`). `GET /recommendations/{id}` gives one listing per indication: `list` (`CoreList` or `ComplementaryList`), `section` with parent, `statusForIndication`, `guidelineDate`, `history` (each `DecisionToAdd` with its date and list), `therapeuticEquivalents` (or `therapeuticEquivalenceInClass` for some), `includedInEmlc`, `drugUnit` (formulations), `population`, ICD-11 indication code, and `evidenceOrigin` (the Expert Committee text). Names resolve only through the HTML search page `GET /?query=NAME`, whose results link `/medicines/{id}` under `<h5 class="medicine-name" title="Name">`; `/medicines` and `/recommendations` JSON lists carry ids only (1,100 medicines, 1,437 recommendations). The site's Export (`/print?format=xlsx|docx|pdf`) gives medicine, section, formulations, indication, ATC and status, but not core or complementary, history, alternatives or committee text, even when the field boxes are posted. robots.txt disallows nothing.
- Measured: dabigatran is medicine 479, recommendations 1004 (atrial fibrillation) and 1022 (venous thromboembolism), both `CoreList`, added 2019-04-05, alternatives apixaban, edoxaban, rivaroxaban; "dabigatran etexilate" finds nothing. Warfarin is medicine 128 with four `CoreList` recommendations (153, 948, 949, 950), earliest `DecisionToAdd` 1977-10-21, alternative acenocoumarol, `includedInEmlc` true. Apixaban search finds no medicine and one therapeutic equivalent (medicine 478, `guideline` empty, `equivalentFor` 1004 and 1022). Imatinib recommendation 415 is `ComplementaryList`.
- Terms: https://list.essentialmeds.org/licencing says the EML "is distributed under the terms of the Creative Commons Attribution IGO License (CC BY 3.0 IGO)" and that users are "free to copy, distribute, transmit, and adapt this work, including for commercial purposes", except "in conjunction with the advertising or promotion of commercial products" or in a way that "suggests that WHO endorses any specific organisation or products"; no WHO logo; required attribution "WHO electronic Essential Medicines List (eEML), World Health Organization, 2020. https://list.essentialmeds.org/ (beta version 1.0). Licence: CC BY 3.0 IGO." The About page repeats CC BY 3.0 IGO. The caveats page says the 2025 EML and EMLc PDFs are the binding versions. So BioMCP may read it and commit real replies; it is labelled `attribution` (as other CC BY sources), and the terms name carries the two use limits.
