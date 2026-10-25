# 0117 — Investigate the disease slice and propose its design

Status: COMPLETE.
Landed: 5b1620561eac38fe905f590e383fb69c8a21cd4e

Milestone: 3

## Outcome

`sdlc/planning/slices/disease.md` holds the disease slice investigation and proposed design, through steps 1 to 4 of `planning/slices.md`, ready for PM review and Ian's questions. No product code changes.

- Why: milestone 3 serves disease identity questions, and the trial slice left explicit MyDisease.info resolution for disease identity to a later slice.
- Starts from: `planning/slices.md`; `planning/slices/trial.md` and its MyDisease resolution section; `planning/slices/variant.md`; milestone 3 in `planning/milestones.md`; the disease sources in ROADMAP.md; 0.9's disease commands; decisions `2026-10-24-no-text-heuristics.md` and `2026-10-24-sources-are-open-by-default.md`.
- Examples: 20 to 40 real clinical disease questions from papers and public issues, covering a cancer type by name and by identifier, synonyms and cross-references such as MONDO, NCIt, OncoTree and ICD, subtypes and parents, genes associated with a cancer, phenotypes for a rare tumor syndrome, incidence and survival from SEER, an ambiguous name, an empty result and a failing source, each with its source and accepted answer. Clinical judgments go to a clinical reviewer.
- Sources: ask each candidate source in ROADMAP.md directly for each example and record counts, speed, requests, returned identifiers, the fields its public tier carries, and terms. Each source marked investigate ends kept or dropped with a reason. UMLS stays uncalled unless a product smoke inherits a user key under an approved recipe.
- Identity: record how each source names the same disease, which identifier joins them, and how an ambiguous name returns candidates without a guess. Record how the trial slice's `--condition` could use a resolved disease identity explicitly.
- 0.9: measure 0.9's disease commands on the same examples in time, bytes and facts, find any filter mapped to the wrong field, and account for every command, option and section.
- Design: the delight questions, types and schema, a search that returns disease headers with drill-downs into synonyms, hierarchy, genes, phenotypes, statistics and other sections the examples need, disclosure layers and token budget against 0.9, sources kept and dropped, harvested case list, and the open questions for Ian with options and a recommendation.
- Proof: the PM reviews the design on its branch. Live captures run through pm job with raw evidence on the evidence store. No licensed content enters the repo.
- PM review 2026-10-24 of 3de6ac19, returned with fixes. The identity model, ambiguity handling and 0.9 accounting stand.
  1. Give every source and component its use label and group under the open-by-default decision, including MyDisease's components, Monarch's OMIM and Orphanet rows, HPO, each OLS ontology and SEER.
  2. End OncoTree as kept or dropped. Keep direct HPO and the CIViC disease section only with a question each that needs them.
  3. Reuse the shared `Curie` and `Concept` instead of a new `DiseaseId`. Gene and phenotype associations are association records, never statements, because the shared model requires a classification. Say how a MONDO ID reaches each section source's own identifier, such as EFO or DOID, through stated cross-references.
  4. Write a command table with the header and next steps per command, one source per section, and JSON shapes for the header, card and pages.
  5. Measure every section's bytes against 0.9 on the harvested cases, saying so where bytes stand in for tokens. Ceilings sit near the measurements or give a fact-level reason.
  6. Write a case list that maps every command and option to its cases.
  7. Settled by the PM: OLS for discovery limited to MONDO by default and MyDisease for exact annotation, with no automatic pick; Monarch's native scope is shown with the requested ID and the returned row's ID side by side, without an `expanded_scope` field; trial `--condition-id` is accepted, and the design states the exact text sent to ClinicalTrials.gov and echoed, with NCI taking NCIt only; the drops stand; the query is positional with no `--query` spelling.
  8. Statistics wait for one structured SEER capture, with no HTML parsing and no name-to-site table, and stay inside search and get. If no structured tier exists, the design says so, and the PM takes the dropped 0.9 survival command to Ian with options and cost.
  9. Close these gaps in this ticket, then rename the PM review section to Ian's questions, listing only what is his.
- PM recheck 2026-10-24 of 0d2fa76f, returned with these remaining fixes:
  1. Measure the SEER sites page offline from the retained catalog, and mark each unmeasured row (card beyond D08, the MONDO-only default, per-section 0.9 comparison waiting on a contained 0.9 run) as unmeasured beside it.
  2. Limit `--ontology` to mondo, doid and ncit. Remove "explicit expanded scope" from the Monarch row. `--condition-id` names the source that resolves the ID and sends that concept's primary label.
  3. SEER stays inside search and get: `search disease --source seer` for the site catalog and `get disease <site ID> statistics --source seer`. Drop survival `--period` until a capture returns the requested period. PM ruling: the structured annual survival rows answer the survival question, and the five-year aggregate that only the web page carries is dropped with that reason; this is not Ian's question.
  4. Capture one positive Open Targets disease-genes case through the MONDO to EFO path, or defer that section.
  5. Settle HPO's use label from the publisher's current terms page and any published license file, not from a 404. If HPO allows commercial use, Monarch's genes and phenotypes stay open.
  6. Move `--condition-id` to its own trial-owned ticket, named in the design.
  7. Rename "Decision boundary" to "Ian's questions" and write None.
- PM recheck 2026-10-25 of 8610e99e: fixes closed except the SEER get shape; design accepted with these amendments, applied on the branch before it lands:
  1. SEER statistics use the ID position and named sections: `get disease <site ID> incidence --source seer` and `get disease <site ID> survival --source seer`, in the command table, the JSON next action and the case map.
  2. Get-level paging uses `--section-limit` and `--section-offset` everywhere, SEER included. `-l` is removed; `--limit` has one spelling.
  3. The hierarchy section shows MyDisease parents only. Children wait for an OLS positive capture in the OLS ticket; until then D18 subtypes go through OLS search.
  4. Remove the Open Targets next action from the mapping table, since that section is deferred.
  5. The card row marks cards other than D08 unmeasured, and every 0.9 section comparison names 0136.
- Defers: tickets, which the PM files from the reviewed design.
