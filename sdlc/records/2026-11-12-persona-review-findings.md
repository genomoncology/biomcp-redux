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
- 510(k) and other device searches keep a 2 MiB reply limit (`crates/biomcp-adapters/src/device_search.rs:64`). 0864 found single recall records up to 9.9 MB and raised only the recall limit; its reviewer flagged this as the same risk. No failure seen yet.

## Missing sources

Added to `planning/sources-to-add.md`.

## Disposition, 2026-11-17

Status: accepted. Checked against origin/main 1fea41534 and the persona rounds since (biomed-agent-bench `personas/PLAYBOOK.md`, rounds table, and each persona's 2026-11-16 `analysis.md`). Of 36 findings, 28 are fixed, 1 is obsolete, 1 was checked and is fine, and 6 are still open. All six have tickets, because `decisions/2026-10-27-1-0-ships-every-known-fix.md` puts every known gap in 1.0.

Fixed (28).

- Ticketed: 0856 to 0864 are all COMPLETE. The two items 0856 was thought not to cover are also done. CodeBreaK 202 (NCT05920356) does write the change, as "KRAS p.G12C": a ClinicalTrials.gov term search for `G12C` misses it and `p.G12C` finds it (checked 2026-11-17), and 0856 searches that spelling and says in the reply which spellings it searched.
- Drug-interaction pair: 0898 and 1023. FAERS counts by year: 0894. MAUDE event type: 0893 and 0903. VAERS by count: 0895.
- Drugs@FDA date sort and window: 0874, 0875 and 0900. Applicant family: 0825's applicant filter is a prefix match on the sponsor name (`NOVARTIS*`, kept by 0874).
- Recall rows: 0901 (reason), 0877 (FDA fields), 0897 (search by device name). fda-cdx: 0933. De novo: 0870 and 0967.
- Composite variant evidence pack: 0846 returns several sections in one get. Tumor-board's 2026-11-16 round asked only for 1008 and 1009 beyond it.
- Study copy number: 1031. Phenotype set ranked: 0886. GTR test type and lab contact: 0887 and 0891. Gene-level ClinVar counts: 0932 and 0946. Article affiliations: 1014.
- Trial rows with sponsor: 0826, confirmed by patient-navigator's 2026-11-16 round. CIViC first page and limit: 0862.

Obsolete (1).

- rs121913529 resolving to G12V: patient-navigator's 2026-11-16 round got three candidate alleles, with `clinvar:12582` resolving G12D (`analysis.md:22`). The 2026-11-10 rollup had already withdrawn the allele-pick claim as a misread.

Checked and fine (1).

- The 2 MiB bound on 510(k) and other device searches (`device_search.rs:75`): no failure seen. A live 100-row page measured 0.78 MB for 510(k) and 0.27 MB for PMA on 2026-11-17, well inside the bound. No ticket.

Still open and ticketed (6).

- Trial site distance, nearest first: seen in three patient-navigator runs, the last on 2026-11-16.
  - [1042 Trial sites give distance, nearest first](../tickets/1042-trial-sites-give-distance-nearest-first.md)
- Site counts across trials and `why_stopped` on search rows (it is on the trial card since 0823). Both change the trial search, so they share a ticket.
  - [1043 Trial search rows say why stopped and count sites](../tickets/1043-trial-search-rows-say-why-stopped-and-count-sites.md)
- Parsed eligibility: the card still gives the criteria as one text.
  - [1044 Trial eligibility splits inclusion and exclusion](../tickets/1044-trial-eligibility-splits-inclusion-and-exclusion.md)
- Pathway gene lists include non-gene members: the Reactome genes relation still returns every exact reference, ChEBI compounds included.
  - [1045 Pathway gene lists can leave out non-gene members](../tickets/1045-pathway-gene-lists-can-leave-out-non-gene-members.md)
- Open-access status on PubMed search rows: still filled only on Europe PMC rows; 1016 explained licence codes but did not fill PubMed rows.
  - [1046 PubMed search rows say open access](../tickets/1046-pubmed-search-rows-say-open-access.md)
