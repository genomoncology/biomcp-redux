# BioMCP findings from the 1.0 persona review

November 12, 2026. Bench ticket 0039 reviewed the eleven blind-graded persona runs of bench 0037 (BioMCP 58d83b6da) for Ian's 1.0 bar (`decisions/2026-11-12-1-0-is-done-when-tools-are-efficient-and-personas-reach-a-minus.md`). Each finding cites the run log in biomed-agent-bench `personas/<slug>/runs/2026-11-12/`. The PM filed the wrong-or-misleading-answer findings as tickets; the rest wait here, ranked by how many calls they cost, for scheduling against the A- bar after the next persona rerun.

## Ticketed

- 0857 widened names drop code-like synonyms everywhere (msl: "COMPOUND 20").
- 0858 empty and failed replies say why (label sections, disease sections, batched trial gets, failed replies, provisional CPIC pairs).
- 0859 every page reports a total (and trial locations take larger pages).
- 0860 errors name the real problem (normalize, incidence, snapshot, get article limit).
- 0861 generic label get prefers the current label (rivaroxaban).
- 0862 CIViC join and section limits.
- 0863 other name lookups skip the ChEMBL wait (WHO prequalification).
- 0864 device-recall paging at small offsets.
- Already open: 0856 variant spellings (in review). Not covered by it: trials that never write the change (life-sciences-ci log.md:101, CodeBreaK 202); a notice that variant search is text-only belongs with 0856's follow-up.

## Waiting: gaps that cost many calls

- Drug-interaction pair filter, "checked, none" (pv-scientist log.md:55-58, pt-committee log.md:44-47).
- FAERS counts by year or quarter in one call (pv-scientist log.md:107-112, pt-committee log.md:88).
- MAUDE event-type filter (pv-scientist log.md:44-47, 125).
- VAERS tables sorted by count (pv-scientist log.md:76-78).
- Trial locations sorted by distance (patient-navigator log.md:167-178; 0083 deferred).
- Site counts across trials with normalised facility names (trial-ops log.md:47-49).
- `why_stopped` on trial search rows, with a filter (trial-ops log.md:78).
- Parsed eligibility fields (trial-ops log.md:57).
- Composite variant evidence pack (tumor-board log.md:441).
- Drugs@FDA approval date sort and window (reg-affairs log.md:34-37, 57).
- Applicant filter matching the company family (reg-affairs log.md:36-39).
- Recall rows with reason, lots and device link (reg-affairs log.md:127, 147; dx-lab-director log.md:77).
- fda-cdx name search with typed PMA number and date (reg-affairs log.md:90-91; dx-lab-director log.md:25).
- De novo decision filter (reg-affairs log.md:66-70).
- Pathway members limited to genes (comp-bio log.md:102).
- Study reads copy-number data (comp-bio log.md:199-205).
- Phenotype query over several HPO terms, ranked (rare-disease log.md:18-25).
- GTR rows with test type and lab contact (rare-disease log.md:96-97).
- Gene-level ClinVar counts by significance (dx-lab-director log.md:85-93).
- Article rows with affiliations (msl log.md:73-77; 0722 said this waits for evaluation evidence, which this is).
- PubMed rows with open-access status (msl log.md:108, 121; 0827 covered Europe PMC rows only).

## Check live before ticketing

- rs121913529 resolved to G12V, not G12D (patient-navigator log.md:634-636).
- Trial rows show no sponsor though 0826 was in the build (msl log.md:97).
- CIViC's first page ignores the limit (dx-lab-director log.md:62-64); folded into 0862.

## Missing sources

Added to `planning/sources-to-add.md`.
