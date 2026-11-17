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
- Changed: a new live source `who-eml` (adapter `crates/biomcp-adapters/src/who_eml/`, quota `who-eml` at the conservative 1/s, terms CC BY 3.0 IGO labelled `attribution`) and a new drug section `essential-medicines` with reply `biomcp.drug-essential-medicines/1`. `get drug NAME essential-medicines` finds the medicine on the eEML's search page by its exact name (letter case ignored), reads its JSON-LD medicine page, then one JSON-LD recommendation page per listing, five per default page with a page step. Each listing gives its eEML number and page, `list` (`core` or `complementary`), section, indication and ICD-11 code, eEML status, `added` (the earliest Committee decision to add it), the distinct `decisions`, `children` (also on the EMLc), therapeutic `alternatives`, ATC codes, formulations and the Committee's recommendation text (the first of the eEML's evidence texts, omitted when it repeats the listing before it). A drug the eEML names only as an alternative, such as apixaban, shows the listed medicine's listings with `alternative_to`. `get drug N --source who-eml` reads an eEML medicine number; `search drug NAME --source who-eml` lists the eEML's own search hits. The reply's `hint` says WHO's PDF lists are binding. Help, the command reference, source pacing, the features list and a release-notes line say so. Schemas changed only by the added reply, the added `who-eml` and `essential-medicines` enum values and descriptions.
- Proof: `tests/cases/drug/who-eml.yaml` replays real eEML replies recorded in job j42168 (CC BY 3.0 IGO allows keeping them; `tests/replies/drug/who-eml/README.md`): dabigatran 479 has two `core` listings (1004 atrial fibrillation, 1022 venous thromboembolism) added 2019-04-05 with alternatives apixaban, edoxaban, rivaroxaban, ATC B01AE07 and the Committee text once; warfarin 128 has 4 `core` listings, added 1977-10-21, on the EMLc, alternative acenocoumarol, paged two at a time; apixaban 478 shows dabigatran's two listings with `alternative_to`; imatinib has a `complementary` listing; the brand "pradaxa" answers absent; `search drug warfarin --source who-eml` finds 128.
- Live check (jobs j42160 and j42168, M5, `--fresh`): the calls above answered complete or absent as the case asserts; dabigatran's reply was 3,171 bytes, warfarin's two-listing page 3,021, apixaban's 3,227, imatinib's three listings 7,175 (its committee text covers a whole EMLc cancer review).
- Jobs: j42153 (fmt and clippy: fmt diffs and two compile errors, fixed); j42158 (recording refused because a record folder must start empty; regenerated schemas and docs; cases refused the unlisted feature; doc check refused a machine name in this file; all fixed); j42160 (recorded the replies one folder per call; clippy too_many_lines twice, a bare rustdoc URL, the adapter-value ratchet's `if let Ok` in the query, an `includes` assertion, the `schema biomcp.source/1` cap and the hosted catalog's "who" word check, all fixed: the parity cap is now 4,747, measured 4,128 plus 15%, and the hosted check strips `who-eml`, which is not noncommercial).
- More jobs: j42162 (regenerated schemas, docs and help caps; fmt line, an `includes` assertion and the hosted catalog's check of the eEML's display name, fixed); j42165 (first full gate after merging main: the new schema missed main's changes, and a spaced name such as "dabigatran etexilate" fails the reply schema's existing drug ID pattern, which the parser accepts; the absent step now uses the one-word brand "pradaxa"); j42168 (re-recorded every reply, regenerated); j42171 full gate `make --keep-going landing-gate docs-check schema-release-check` passed at e1f4e3c7c.
- Open: the reply schema's drug ID pattern rejects spaces the parser accepts, so any `get drug "two words"` reply fails schema validation; ticket 1033 (drug names with spaces) holds it, and this case uses "pradaxa".
- Review fixes (coordinator, 2026-11-17): medicine pages link rejected applications too (pembrolizumab 565: 9 of 13 `ApplicationRejected`, still marked `CoreList`), so every linked recommendation page is read and only `AddedToEml` ones are listings; the total counts listings. `alternative_to` is set only for listings reached through `equivalentFor`. A medicine page without `guideline` or `equivalentFor`, a link that is not a recommendation, a recommendation without a status, and a search page without its "Found N recommendations for M medicines and E therapeutic equivalents" line (or a nonzero count with no medicine headings) are bad replies, not absent. The hint carries WHO's required attribution text. Reading every page at one a second needs a 60-second leg and command (pembrolizumab: 16 reads).
- Jobs: j42182 (re-record; pembrolizumab ran past the 15-second leg); j42183 (re-recorded every reply, cargo fmt, regenerated, check-cheap and the drug, help, grammar and mcp-parity cases pass; live pembrolizumab answered 5 listings, 1182 complementary, 1490-1492 core and nivolumab's 1157 with `alternative_to` nivolumab, 31,284 bytes). The recordings grew to 1.4 MB because the case keeps every pembrolizumab recommendation page it must read.
- Jobs: j42186 (after merging main: regenerate, check-cheap and touched cases pass; refreshed the diagnostic help cap); j42187 full gate `make --keep-going landing-gate docs-check schema-release-check` passed at 3a0a80e28.
