# Study slice investigation

The investigation started from main `85e6ba864c3f13ec50432a58670ec2af13e12857`. It follows steps 1–4 of [the slice process](../slices.md). The implementation and its bounded disclosure evidence are recorded below.

The approved boundary keeps exact study retrieval and counts stated by cBioPortal. Local top-mutated, filter, query, co-occurrence, cohort, survival and comparison calculations retire. A source count never supplies an inferred denominator, prevalence, prognosis or treatment recommendation.

## Investigation method

Capture the public cBioPortal detailed study catalog, twenty exact published study records, molecular profiles and sample-list metadata for representative studies, the DataHub catalog, archive size headers, and current source documentation. Each request has one attempt, a bounded response and a deadline. Keep original bodies and receipts outside Git. No mutation tables or bulk study archives are needed to decide this metadata design. The DETAILED sample-list endpoint did return public sample IDs; retain them only in the raw capture and omit them from product metadata output. A SUMMARY endpoint probe tests whether that transfer can be avoided. Preserve source-stated counts without calculating biological summaries.

The 0.9 command inventory is inspected statically. Execute no 0.9 binary until network containment is independently verified. Unmeasured command timing and output remain explicit limits.

## Shared model boundary

The existing [CohortObservation contract](../biodata-variants-and-annotations.md#37-population-frequency-cohort-counts-coverage-and-predictor-scores) requires `subject: VariantRef`. Study-wide and gene-wide counts cannot satisfy that requirement. Reuse it only when a captured source supplies an exact variant subject and the stated counts. Preserve study totals and profile/sample-list counts as source metadata. Request PM review before extending the shared subject type.

## Questions that should delight an agent

1. **Which cohort do I need?** `search study TEXT` searches only the publisher's study ID, name and description. It never rewrites a disease name or treats a cancer-type label as an ontology identifier. Exact `--cancer-type` filters a captured native source code.
2. **What can I learn from one call?** The header gives exact study ID, name, source cancer-type code, stated sample count, citation and source date. Patient totals require a separately attributed endpoint because the captured study cards omit them. It distinguishes absent fields from zero.
3. **What comes next?** Candidate pages offer exact `get study ID` actions, then paging, then at most three other actions. Cards offer only supported sections under architecture 6.10. No hidden study/profile/sample-set choice occurs.
4. **What counts can I trust?** Each count retains its native scope and field name. Total samples, separately attributed patients, sequenced samples, CNA samples and sample-list membership stay separate. A sequenced total is not locus coverage; no rate is computed.
5. **How do I find an alteration frequency?** This slice supplies only an explicitly stated source value with its scope and profile identity. It never derives a frequency from mutation rows or total samples. Without that evidence, the answer says the frequency is unavailable through the kept surface and links the source study.
6. **How do joins work?** Study IDs join profiles and sample lists by exact `studyId`. Gene references require native Entrez IDs. Source cancer-type abbreviations remain source codes. Neither case conversion nor text matching makes an OncoTree, MONDO or gene identity.
7. **What fails?** A checked successful empty search differs from source failure, disabled source, missing bundle and absent exact study. A stale accepted snapshot stays labeled stale and offers refresh. A rejected replacement never destroys the last accepted snapshot.
8. **How expensive is a call?** A metadata bundle avoids downloading whole mutation matrices or iterating every cohort on reads. Cold source timings, payloads and output projections appear with the capture evidence. Warm command timing remains unmeasured until a product exists.
9. **What can I cite?** Preserve the source study URL, publication identifier, citation, publisher metadata and snapshot revision. Do not substitute bundle acquisition date for biological release date.
10. **What can I stop doing?** An agent can stop after the bounded default card when it has identity, scope and count provenance. It need not retrieve participant records, render charts or install analysis tools.

## Proposed command tree and disclosure

The landed commands expose stored metadata and bounded independent sections. The capture limit ruling defers `counts`, `--gene` and patient totals.

| Command | Default answer | Deeper answer and next steps |
| --- | --- | --- |
| `search study TEXT [--cancer-type CODE] [--limit N] [--offset N] [--pool ID]` | One page of five study headers, stable source order; literal case-insensitive text matching on the native ID/name/description | One exact get per candidate, then pool paging; one sync action for absent/stale bundle. No clinical ranking, synonyms, recursive tumor descendants or gene-content heuristic |
| `get study ID [SECTION] [--section-limit N] [--section-offset N]` | Exact study header, description, source-stated sample and assay counts, citation and terms | Named `profiles`, `sample-lists`, `files`, and bounded `all`; `counts` and `--gene` are deferred; bounded pages, precise native IDs and source links. `all` remains bounded and never fetches participant-level matrices |
| `sync cbioportal-studies` | One checked metadata catalog snapshot, its source revision and acquisition status | Atomic replacement; accepted previous snapshot survives parse, identity, quota or transport failures. Reading never triggers implicit sync |

A get uses only an exact study ID selected from discovery or explicitly supplied by the caller. Unsupported aliases return usage guidance or source absence, never a guessed cohort. Exact study IDs remain case-sensitive if the source treats them that way. Search filters narrow only checked fields; a cancer type such as `mixed` must not expand into cohorts inferred from sample diagnoses.

The metadata bundle contains catalog metadata. Catalog search continuations bind the immutable catalog revision and selectors; changing a selector fails locally, and missing or expired pools offer a restart without provider requests. Profiles, sample lists and files use ordinary checked received-reply caches with independent acquisition times and section offsets. Their pages retain those acquisition times. They do not claim one shared snapshot across independently acquired sections.

Source policy applies before any cache, bundle or transport access. Existing `cbioportal` use remains `restricted` pending study-specific reuse proof; it stays on by default and honors explicit source/group disables. A bundle may not relabel the same restricted data as open merely because it is downloaded.

## Types, schema and identity

| Proposed record | Captured/native basis | Boundary |
| --- | --- | --- |
| `StudyId` | Exact `studyId` / `cancer_study_identifier` | Opaque checked source identity; no hyphen/underscore repair |
| Study header/card | ID, native name, optional short name/description/citation/PMIDs, native cancer-type reference, source-stated counts | Optional fields reflect actual capture absence, not a guessed zero or default cancer class |
| Study count metadata | Native sample, patient, sequenced and CNA totals, with native field names and unit | Cohort size never becomes a denominator. Never add overlapping cohorts or deduplicate participant records |
| Profile reference | Exact molecular profile ID and `studyId`, native alteration/data type, name and description | Require both identities to agree before caching. A profile label is not proof every locus was assayed |
| Sample-list metadata | Exact list ID, `studyId`, category/name/description and native stated sample count when returned | Membership count is not altered-sample count or gene-assayed count. Do not include raw member IDs in a default reply |
| File reference | Native file name/path, stable publisher URL, stated bytes and revision when present | No inferred data type from a filename; no whole archive download on get; no bioinformatics executables |
| Gene reference | Source-stated Entrez gene ID, existing checked GeneRef resolution | No symbol-only biological join; no gene ID invented from study title or file name |
| OncoTree reference | Only an explicit native code/namespace/version or source-stated cross-reference | Lowercase cBioPortal `cancerTypeId` remains distinct. No uppercase transform, descendant expansion or equivalence claim |
| `CohortObservation` | Exact variant subject plus source-stated numerator/profiled/cohort counts if a captured endpoint supplies them | Current shared subject is `VariantRef`. Generic study/gene counts do not fit. Missing gene-assayed count remains absent |

Generate and version the request, header, card and section schemas from the shared checked types. Preserve source-native multiplicity, including multiple PMIDs and profiles; parsing comma-separated source identifiers must validate each identifier without reinterpreting source prose. No scalar PMID field may silently discard later publications. Public tier absence is not an empty biological finding.

## Kept and dropped sources

Keep the public cBioPortal study metadata API and DataHub catalog as the one study source family. Keep exact native study/profile/sample-list metadata and safe file references where the captured public tier proves them. The [official REST documentation](https://docs.cbioportal.org/web-api-and-clients/) establishes that the public web visualizations use the REST API. No authentication is required for these public captures. Institutional authenticated portals, Google-sign-in MCP and private GENIE datasets are not silently substituted for the public instance.

The [DataHub README](https://github.com/cBioPortal/datahub) describes staging files and weekly validation against the portal. Weekly validation is not a promised weekly data release or archive publication schedule. Record source ETag/Last-Modified/commit metadata when provided; do not infer a release date from the paper's year. The [FAQ](https://docs.cbioportal.org/user-guide/faq/) names ODbL as the general grant but permits study-specific commercial restrictions. Retain study notices and use the existing restricted source declaration until specific rights are verified. Do not conflate the portal software's AGPL license with the data license.

Drop new analysis engines, plots, local mutation/CNA/expression/SV aggregation, sample filtering, cohort stratification, co-occurrence and survival calculations under [Ian's entity boundary ruling](../../decisions/2026-10-25-entity-boundary-rulings.md). Drop unrelated GDC bulk downloads, whole-database MySQL exports, institutional portals, and tool installation: none is needed to get a public study and the counts it states.

## Cases for the proposed commands

| Surface or option | Required outside-in cases |
| --- | --- |
| Search text | Known-positive native title and ID; case handling; multiple hits; no-match complete empty; no typo repair; no synonyms; empty text rule |
| `--cancer-type` | Known-positive exact native code; unknown code; `mixed` preserved; no uppercase OncoTree conversion; text and code conjunction; no descendant expansion |
| `--limit`, `--offset`, `--pool` | Default five; limits one/maximum; terminal page; no gaps/duplicates; snapshot continuity; changed-selector rejection; expired pool restart with zero network |
| Exact get | Published selected ID; absent ID; wrong returned study rejection; malformed ID before I/O; source-native multiple PMIDs; missing optional patient count versus stated zero |
| `profiles` | Exact study match, multiple profile types, missing versus empty, wrong-study admission failure, bounded page, no gene coverage inference |
| `sample-lists` | Source-stated count, absent count, exact list/study identity, no denominator inference, no participant IDs by default |
| `files` | Verified public file reference and size/revision; LFS pointer size distinguished from actual content bytes; absent manifest; no automatic archive fetch |
| Sections, `all`, `--section-limit` | Known-positive field per public tier; unsupported section usage error; independent partial section; bounded all; no participant matrix or chart |
| Sync | Fresh install, checked snapshot, warm read without requests, revision change, malformed/oversized/identity-conflicting replacement preserves old snapshot, concurrent sync ownership |
| Source/settings | Disabled source and restricted group before cache/network; full terms and notices; no private portal fallback; key-free public access |
| Failure envelope | 429, 500, timeout, broken body, HTTP 200 error object; failure differs from zero and absent; stale provenance remains explicit |
| Interface and cost | CLI/MCP parity; generated schemas; every emitted next action round trips; quoted file/ID examples; Markdown derived from JSON; measured default/section ceilings |
| Retirement | Every legacy leaf and chart option below; actionable source retrieval guidance; no analysis calls or hidden tool installation |

## 0.9 accounting

Static inventory uses the 0.9 checkout `4050fdc7dbdf4d713d94cf02a39a147b863665be`: `src/cli/study/mod.rs`, `src/cli/types.rs`, `src/cli/list/literature.rs`, `src/entities/study.rs`, `src/sources/cbioportal_study.rs`, and `spec/entity/study.md`. No legacy executable has been run. Network containment and executable baseline costs remain unmeasured.

The accepted entity boundary and capture limit rulings close the following accounting. The generated ledger counts long option slots; the detailed rows retain aliases, values, chart rendering and the legacy environment variable. A positional study ID is not an option slot. Global output/settings/help flags remain shared; they are outside the historical command-specific denominator.

| 0.9 leaf | Proposed disposition | Every command-specific option and alias | Migration |
| --- | --- | --- | --- |
| `study list` | Rename | No command-specific flags | `search study` over the checked source catalog; local installation list is not public-source absence |
| `study download` | Rename data acquisition | `--list`; positional study ID | `sync cbioportal-studies` acquires metadata only; exact get returns safe source file references. Whole-study analytical installs retire |
| `study query` | Drop calculations | `--study`/`-s`, `--gene`/`-g`, `--type`/`-t`; mutation/mutations, cna/copy_number/copy-number, expression/expr, sv/fusion/fusions/structural_variants | Exact get preserves native study metadata counts; gene aggregates and patient totals are deferred. No recomputed rates, expression summary, CNA distribution or SV table |
| `study top-mutated` | Drop calculations | `--study`/`-s`, `--limit`/`-l` | Exact study get; source website for analysis |
| `study filter` | Drop calculations | `--study`/`-s`, repeatable `--mutated`, `--amplified`, `--deleted`, `--expression-above`, `--expression-below`, `--cancer-type` | No replacement analytical intersection; source retrieval guidance only |
| `study cohort` | Drop calculations | `--study`/`-s`, `--gene`/`-g` | No mutant/wildtype split; exact source study retrieval |
| `study survival` | Drop calculations | `--study`/`-s`, `--gene`/`-g`, `--endpoint`/`-e`; os/overall/overall_survival, dfs/disease_free, pfs/progression_free, dss/disease_specific | No KM, endpoint inference, survival statistics or prognostic conclusion |
| `study compare` | Drop calculations | `--study`/`-s`, `--gene`/`-g`, `--type`/`-t`, `--target`; expression/expr and mutations/mutation | No expression or mutation-rate group comparison |
| `study co-occurrence` | Drop calculations | `--study`/`-s`, `--genes`/`-g` | No contingency table, odds ratios or significance calculations |

The shared ChartArgs flags on query, survival, compare and co-occurrence all retire with their leaf: `--chart`, `--terminal`, `--output`/`-o`, `--title`, `--theme`, `--palette`, `--cols`, `--rows`, `--width`, `--height`, `--scale`, and hidden `--mcp-inline`. All chart kinds, themes, palettes, PNG/SVG, terminal and MCP inline rendering retire. `list study` and catalog discovery are shared help surfaces and must describe the new entity commands. `BIOMCP_STUDY_DIR` no longer selects an analysis directory; migration guidance names source sync rather than silently scanning that directory.

Legacy output sections are all accounted: local availability, downloadable IDs and install result become source metadata/sync guidance; Mutation Frequency / Top Variant Classes / Top Protein Changes / CNA Distribution / Expression Distribution / Structural Variants / Top Mutated Genes / Filter Intersection / Cohort Groups / Survival / Group Comparison / Co-occurrence / Charts retire with the calculation ruling. The old `not_in_local_cohorts` state becomes explicit missing snapshot or source absence, never a zero frequency. Legacy study commands have no general get-section selector grammar to preserve.


<!-- parity-accounting -->
| 0.9 leaf command | Command outcome | Ported or renamed options | Dropped options | Deferred options | Undecided options | Ruling |
| --- | --- | --- | --- | --- | --- | --- |
| `study list` | Ported or renamed | — | — | — | — | Search the stored public catalog with search study; local installations do not define source absence. |
| `study download` | Ported or renamed | — | `--list` | — | — | Sync metadata with sync cbioportal-studies and inspect exact file references with get study ID files; whole-study installs and downloadable archive listing retire. |
| `study query` | Dropped by ruling | — | `--study`, `--gene`, `--type`, `--chart`, `--terminal`, `--output`, `--title`, `--theme`, `--palette`, `--cols`, `--rows`, `--width`, `--height`, `--scale`, `--mcp-inline` | — | — | The entity boundary retires local analysis and charts; get study ID preserves native metadata, and the source study page supplies analysis. Gene aggregates and patient totals remain explicitly deferred by the capture limit ruling. |
| `study top-mutated` | Dropped by ruling | — | `--study`, `--limit` | — | — | The entity boundary retires local analysis and charts; get study ID preserves native metadata, and the source study page supplies analysis. Gene aggregates and patient totals remain explicitly deferred by the capture limit ruling. |
| `study filter` | Dropped by ruling | — | `--study`, `--mutated`, `--amplified`, `--deleted`, `--expression-above`, `--expression-below`, `--cancer-type` | — | — | The entity boundary retires local analysis and charts; get study ID preserves native metadata, and the source study page supplies analysis. Gene aggregates and patient totals remain explicitly deferred by the capture limit ruling. |
| `study cohort` | Dropped by ruling | — | `--study`, `--gene` | — | — | The entity boundary retires local analysis and charts; get study ID preserves native metadata, and the source study page supplies analysis. Gene aggregates and patient totals remain explicitly deferred by the capture limit ruling. |
| `study survival` | Dropped by ruling | — | `--study`, `--gene`, `--endpoint`, `--chart`, `--terminal`, `--output`, `--title`, `--theme`, `--palette`, `--cols`, `--rows`, `--width`, `--height`, `--scale`, `--mcp-inline` | — | — | The entity boundary retires local analysis and charts; get study ID preserves native metadata, and the source study page supplies analysis. Gene aggregates and patient totals remain explicitly deferred by the capture limit ruling. |
| `study compare` | Dropped by ruling | — | `--study`, `--gene`, `--type`, `--target`, `--chart`, `--terminal`, `--output`, `--title`, `--theme`, `--palette`, `--cols`, `--rows`, `--width`, `--height`, `--scale`, `--mcp-inline` | — | — | The entity boundary retires local analysis and charts; get study ID preserves native metadata, and the source study page supplies analysis. Gene aggregates and patient totals remain explicitly deferred by the capture limit ruling. |
| `study co-occurrence` | Dropped by ruling | — | `--study`, `--genes`, `--chart`, `--terminal`, `--output`, `--title`, `--theme`, `--palette`, `--cols`, `--rows`, `--width`, `--height`, `--scale`, `--mcp-inline` | — | — | The entity boundary retires local analysis and charts; get study ID preserves native metadata, and the source study page supplies analysis. Gene aggregates and patient totals remain explicitly deferred by the capture limit ruling. |
<!-- /parity-accounting -->

| Additional accounting surface | 1.0 disposition | Owning proof |
| --- | --- | --- |
| `list study` | Fixed `search study` guidance; public catalog discovery uses `help search study`, `help get study` and `help sync cbioportal-studies`. | `study-retirements/fixed-guidance-1`; `study/help.yaml`; MCP command discovery. |
| `BIOMCP_STUDY_DIR` | Retired without scanning the directory; guidance names metadata sync. | `study-retirements/environment-does-not-read-directory`. |
| DataHub `study_list.json` | Deferred by the ticket filing ruling: the independent downloadable archive catalog is outside the stored portal metadata catalog. | Source inventory remains in the retained investigation. No download-list parity is claimed. |
| Per-study use labels | Deferred until a need appears under the Study ruling. Source-wide restricted policy remains; native LICENSE links are notices on files pages and `all`, never cached onto a plain card. | `study-files/complete-walk-and-notice`. |
| `counts`, `--gene`, patient total | Deferred under the capture limit ruling; the API has no aggregate-row gene selector and BRCA's unfiltered reply costs 4,258,105 bytes. A cohort-changing gene filter is not substituted. | Bounded admission witnesses below; no product count command is claimed. |
| Analysis engines, plots, local aggregation, institutional portals, private datasets, whole-database exports, tool installation | Dropped under the entity boundary ruling. | `study_retirements.rs`; no tool download or installation. |

### Closed public issues

The complete 57-issue closed public inventory was read on 2026-10-25 through `gh issue list --repo genomoncology/biomcp --state closed --limit 1000 --json number,title,body,url`. Inventory SHA-256 is `7eaf219c6f36a891089c98a1eebf18370ed56ba8efbaa2a5f6c3e8b95228e4a6`. Only [issue 113](https://github.com/genomoncology/biomcp/issues/113) concerns this entity's source. Keep its requirement to reject cross-gene data; exact profile/study/gene identity checks are mandatory before any count admission. The old locally aggregated hotspot summary retires; its correctness requirement does not. The other parts of issue 113 concern FDA drug data and are outside this slice. Keyword matches 87, 37 and 20 refer to trial studies or shared MCP tools and are not study-entity requests. No closed study-specific shipped user request has been silently omitted.

## Clinical review boundary

The independent reviewer confirmed from the official FAQ and gene-panel matrix documentation that a sequenced-sample total or sequenced-list membership is not proof of gene-assayed denominator. A missing mutation row cannot mean unaltered. cBioPortal supports multiple samples per patient and overlapping cohorts, so preserve samples, patients, events and profiles distinctly. The native lower-case cancer-type abbreviation does not establish OncoTree equivalence by case conversion; preserve its source namespace and any explicit mapping. Final review must bind these statements to the actual captured record shapes.

## Ian's questions

No clinical or configuration question is sent to Ian. The PM must resolve whether this metadata-only study surface should extend the shared CohortObservation subject beyond VariantRef; the recommendation is to retain native study counts separately and avoid a shared-model change unless a retained captured count endpoint requires it. The PM must also accept the proposed discovery/section/bundle boundaries and any measured output ceilings before implementation tickets exist. These are design decisions, not requests for a paid service, private data or a production change.

## Harvested human-health questions

S01–S20 are paraphrased research-data follow-ups to the studies and publications named by the native cBioPortal records. They are not purported quotations from clinicians. Each accepted answer is a metadata answer, not a clinical conclusion. The twenty exact API responses were verified in j14206 at 2026-10-26 UTC (2026-10-25 in New York). The source-provided paper identifiers identify the research context; the API field named below supplies the accepted current portal answer.

| Case | Sourced question | Accepted answer and direct source |
| --- | --- | --- |
| S01 | Which public breast invasive carcinoma cohort supplies the PanCancer Atlas data and how many samples does its portal record state? [TCGA, Cell 2018](https://pubmed.ncbi.nlm.nih.gov/29625048/) | `brca_tcga_pan_can_atlas_2018`; `allSampleCount=1084`, source cancer type `brca`. [Native answer](https://www.cbioportal.org/api/studies/brca_tcga_pan_can_atlas_2018?projection=DETAILED). |
| S02 | Which public lung adenocarcinoma cohort supplies the PanCancer Atlas data and how many samples does its portal record state? [TCGA, Cell 2018](https://pubmed.ncbi.nlm.nih.gov/29625048/) | `luad_tcga_pan_can_atlas_2018`; `allSampleCount=566`, source cancer type `luad`. [Native answer](https://www.cbioportal.org/api/studies/luad_tcga_pan_can_atlas_2018?projection=DETAILED). |
| S03 | Which public lung squamous cell carcinoma cohort supplies the PanCancer Atlas data and how many samples does its portal record state? [TCGA, Cell 2018](https://pubmed.ncbi.nlm.nih.gov/29625048/) | `lusc_tcga_pan_can_atlas_2018`; `allSampleCount=487`, source cancer type `lusc`. [Native answer](https://www.cbioportal.org/api/studies/lusc_tcga_pan_can_atlas_2018?projection=DETAILED). |
| S04 | Which public colorectal adenocarcinoma cohort supplies the PanCancer Atlas data and how many samples does its portal record state? [TCGA, Cell 2018](https://pubmed.ncbi.nlm.nih.gov/29625048/) | `coadread_tcga_pan_can_atlas_2018`; `allSampleCount=594`, source cancer type `coadread`. [Native answer](https://www.cbioportal.org/api/studies/coadread_tcga_pan_can_atlas_2018?projection=DETAILED). |
| S05 | Which public skin cutaneous melanoma cohort supplies the PanCancer Atlas data and how many samples does its portal record state? [TCGA, Cell 2018](https://pubmed.ncbi.nlm.nih.gov/29625048/) | `skcm_tcga_pan_can_atlas_2018`; `allSampleCount=448`, source cancer type `skcm`. [Native answer](https://www.cbioportal.org/api/studies/skcm_tcga_pan_can_atlas_2018?projection=DETAILED). |
| S06 | Which public glioblastoma multiforme cohort supplies the PanCancer Atlas data and how many samples does its portal record state? [TCGA, Cell 2018](https://pubmed.ncbi.nlm.nih.gov/29625048/) | `gbm_tcga_pan_can_atlas_2018`; `allSampleCount=592`, source cancer type `difg`. [Native answer](https://www.cbioportal.org/api/studies/gbm_tcga_pan_can_atlas_2018?projection=DETAILED). |
| S07 | Which public brain lower grade glioma cohort supplies the PanCancer Atlas data and how many samples does its portal record state? [TCGA, Cell 2018](https://pubmed.ncbi.nlm.nih.gov/29625048/) | `lgg_tcga_pan_can_atlas_2018`; `allSampleCount=514`, source cancer type `difg`. [Native answer](https://www.cbioportal.org/api/studies/lgg_tcga_pan_can_atlas_2018?projection=DETAILED). |
| S08 | Which public prostate adenocarcinoma cohort supplies the PanCancer Atlas data and how many samples does its portal record state? [TCGA, Cell 2018](https://pubmed.ncbi.nlm.nih.gov/29625048/) | `prad_tcga_pan_can_atlas_2018`; `allSampleCount=494`, source cancer type `prad`. [Native answer](https://www.cbioportal.org/api/studies/prad_tcga_pan_can_atlas_2018?projection=DETAILED). |
| S09 | Which public ovarian serous cystadenocarcinoma cohort supplies the PanCancer Atlas data and how many samples does its portal record state? [TCGA, Cell 2018](https://pubmed.ncbi.nlm.nih.gov/29625048/) | `ov_tcga_pan_can_atlas_2018`; `allSampleCount=585`, source cancer type `hgsoc`. [Native answer](https://www.cbioportal.org/api/studies/ov_tcga_pan_can_atlas_2018?projection=DETAILED). |
| S10 | Which public pancreatic adenocarcinoma cohort supplies the PanCancer Atlas data and how many samples does its portal record state? [TCGA, Cell 2018](https://pubmed.ncbi.nlm.nih.gov/29625048/) | `paad_tcga_pan_can_atlas_2018`; `allSampleCount=184`, source cancer type `paad`. [Native answer](https://www.cbioportal.org/api/studies/paad_tcga_pan_can_atlas_2018?projection=DETAILED). |
| S11 | How many sequenced samples does the published bladder urothelial carcinoma cohort's portal metadata report? [TCGA, Cell 2018](https://pubmed.ncbi.nlm.nih.gov/29625048/) | `blca_tcga_pan_can_atlas_2018`; `sequencedSampleCount=410`, separate from `allSampleCount=411`. No gene-assayed denominator follows. [Native answer](https://www.cbioportal.org/api/studies/blca_tcga_pan_can_atlas_2018?projection=DETAILED). |
| S12 | How many sequenced samples does the published head and neck squamous cell carcinoma cohort's portal metadata report? [TCGA, Cell 2018](https://pubmed.ncbi.nlm.nih.gov/29625048/) | `hnsc_tcga_pan_can_atlas_2018`; `sequencedSampleCount=515`, separate from `allSampleCount=523`. No gene-assayed denominator follows. [Native answer](https://www.cbioportal.org/api/studies/hnsc_tcga_pan_can_atlas_2018?projection=DETAILED). |
| S13 | How many sequenced samples does the published stomach adenocarcinoma cohort's portal metadata report? [TCGA, Cell 2018](https://pubmed.ncbi.nlm.nih.gov/29625048/) | `stad_tcga_pan_can_atlas_2018`; `sequencedSampleCount=436`, separate from `allSampleCount=440`. No gene-assayed denominator follows. [Native answer](https://www.cbioportal.org/api/studies/stad_tcga_pan_can_atlas_2018?projection=DETAILED). |
| S14 | How many sequenced samples does the published uterine corpus endometrial carcinoma cohort's portal metadata report? [TCGA, Cell 2018](https://pubmed.ncbi.nlm.nih.gov/29625048/) | `ucec_tcga_pan_can_atlas_2018`; `sequencedSampleCount=517`, separate from `allSampleCount=529`. No gene-assayed denominator follows. [Native answer](https://www.cbioportal.org/api/studies/ucec_tcga_pan_can_atlas_2018?projection=DETAILED). |
| S15 | How many sequenced samples does the published kidney chromophobe cohort's portal metadata report? [TCGA, Cell 2018](https://pubmed.ncbi.nlm.nih.gov/29625048/) | `kich_tcga_pan_can_atlas_2018`; `sequencedSampleCount=65`, separate from `allSampleCount=65`. No gene-assayed denominator follows. [Native answer](https://www.cbioportal.org/api/studies/kich_tcga_pan_can_atlas_2018?projection=DETAILED). |
| S16 | How many samples does the published kidney renal clear cell carcinoma cohort's portal metadata report with CNA data? [TCGA, Cell 2018](https://pubmed.ncbi.nlm.nih.gov/29625048/) | `kirc_tcga_pan_can_atlas_2018`; `cnaSampleCount=509`, separate from `allSampleCount=512`. No alteration frequency follows. [Native answer](https://www.cbioportal.org/api/studies/kirc_tcga_pan_can_atlas_2018?projection=DETAILED). |
| S17 | How many samples does the published kidney renal papillary cell carcinoma cohort's portal metadata report with CNA data? [TCGA, Cell 2018](https://pubmed.ncbi.nlm.nih.gov/29625048/) | `kirp_tcga_pan_can_atlas_2018`; `cnaSampleCount=283`, separate from `allSampleCount=283`. No alteration frequency follows. [Native answer](https://www.cbioportal.org/api/studies/kirp_tcga_pan_can_atlas_2018?projection=DETAILED). |
| S18 | How many samples does the published thyroid carcinoma cohort's portal metadata report with CNA data? [TCGA, Cell 2018](https://pubmed.ncbi.nlm.nih.gov/29625048/) | `thca_tcga_pan_can_atlas_2018`; `cnaSampleCount=497`, separate from `allSampleCount=500`. No alteration frequency follows. [Native answer](https://www.cbioportal.org/api/studies/thca_tcga_pan_can_atlas_2018?projection=DETAILED). |
| S19 | How many samples does the published acute myeloid leukemia cohort's portal metadata report with CNA data? [TCGA, Cell 2018](https://pubmed.ncbi.nlm.nih.gov/29625048/) | `laml_tcga_pan_can_atlas_2018`; `cnaSampleCount=191`, separate from `allSampleCount=200`. No alteration frequency follows. [Native answer](https://www.cbioportal.org/api/studies/laml_tcga_pan_can_atlas_2018?projection=DETAILED). |
| S20 | Which public study represents the MSK-IMPACT clinical sequencing cohort and what does the portal count? [Zehir et al. Nat Med 2017](https://pubmed.ncbi.nlm.nih.gov/28481359/) | `msk_impact_2017`; native `mixed`, `allSampleCount=10945`, `sequencedSampleCount=10945`. The description says 10,000 clinical cases; that prose is not a patient-count field. [Native answer](https://www.cbioportal.org/api/studies/msk_impact_2017?projection=DETAILED). |

Additional controls supplement this published-question bank: exact absent study returns 404; a fabricated text query has zero metadata matches; source 429/500/timeout/broken-body controls must report failure. They are test controls, not additional published clinical questions. Gene-count and file-reference follow-ups appear with the second capture findings below.

## Question answers through the landed surface

These statuses describe the command's ability to return the already reviewed native answer. They do not change the source answer or infer a clinical conclusion. `scripts/study_get_retained.py` builds the twenty exact card cases and complete profile walks from the verified native archive. The retained proof covers all twenty identities, native codes and counts in S01–S20. S21–S24 and S26 remain source findings whose product access is deferred by the PM's capture limit ruling.

| Question | Command answer | Owning landed assertion |
| --- | --- | --- |
| S01 | Answered | Exact native card in the twenty-card retained proof; `study-get/retained-native` checks identity, native count fields and complete description through CLI and MCP. |
| S02 | Answered | Exact native card in the twenty-card retained proof; `study-get/retained-native` checks identity, native count fields and complete description through CLI and MCP. |
| S03 | Answered | Exact native card in the twenty-card retained proof; `study-get/retained-native` checks identity, native count fields and complete description through CLI and MCP. |
| S04 | Answered | Exact native card in the twenty-card retained proof; `study-get/retained-native` checks identity, native count fields and complete description through CLI and MCP. |
| S05 | Answered | Exact native card in the twenty-card retained proof; `study-get/retained-native` checks identity, native count fields and complete description through CLI and MCP. |
| S06 | Answered | Exact native card in the twenty-card retained proof; `study-get/retained-native` checks identity, native count fields and complete description through CLI and MCP. |
| S07 | Answered | Exact native card in the twenty-card retained proof; `study-get/retained-native` checks identity, native count fields and complete description through CLI and MCP. |
| S08 | Answered | Exact native card in the twenty-card retained proof; `study-get/retained-native` checks identity, native count fields and complete description through CLI and MCP. |
| S09 | Answered | Exact native card in the twenty-card retained proof; `study-get/retained-native` checks identity, native count fields and complete description through CLI and MCP. |
| S10 | Answered | Exact native card in the twenty-card retained proof; `study-get/retained-native` checks identity, native count fields and complete description through CLI and MCP. |
| S11 | Answered | Exact native card in the twenty-card retained proof; `study-get/retained-native` checks identity, native count fields and complete description through CLI and MCP. |
| S12 | Answered | Exact native card in the twenty-card retained proof; `study-get/retained-native` checks identity, native count fields and complete description through CLI and MCP. |
| S13 | Answered | Exact native card in the twenty-card retained proof; `study-get/retained-native` checks identity, native count fields and complete description through CLI and MCP. |
| S14 | Answered | Exact native card in the twenty-card retained proof; `study-get/retained-native` checks identity, native count fields and complete description through CLI and MCP. |
| S15 | Answered | Exact native card in the twenty-card retained proof; `study-get/retained-native` checks identity, native count fields and complete description through CLI and MCP. |
| S16 | Answered | Exact native card in the twenty-card retained proof; `study-get/retained-native` checks identity, native count fields and complete description through CLI and MCP. |
| S17 | Answered | Exact native card in the twenty-card retained proof; `study-get/retained-native` checks identity, native count fields and complete description through CLI and MCP. |
| S18 | Answered | Exact native card in the twenty-card retained proof; `study-get/retained-native` checks identity, native count fields and complete description through CLI and MCP. |
| S19 | Answered | Exact native card in the twenty-card retained proof; `study-get/retained-native` checks identity, native count fields and complete description through CLI and MCP. |
| S20 | Answered | Exact native card in the twenty-card retained proof; `study-get/retained-native` checks identity, native count fields and complete description through CLI and MCP. |
| S21 | Unanswered by the product; PM deferral | No gene-count section; the retained source finding stays unchanged. |
| S22 | Unanswered by the product; PM deferral | No gene-count section; the retained source finding stays unchanged. |
| S23 | Unanswered by the product; PM deferral | No gene-count section; the retained source finding stays unchanged. |
| S24 | Unanswered by the product; PM deferral | No gene-count section; the retained source finding stays unchanged. |
| S25 | Answered | Exact MSK profile walk preserves all three native profile IDs and types; no expression profile is invented. |
| S26 | Partial | Stored card preserves the absent patient field; the separately scoped patient endpoint total is deferred with counts. |

The design case map is implemented by `study_catalog.rs` (search, selectors, sync, revision and settings), `study_get.rs` (exact cards, native profile and list walks, admission and navigation), `study_files.rs` (files and LICENSE notices), `study_failures.rs` (synthetic failures and rejected replacements), `study_retirements.rs` (every retired leaf, option, alias, enumerated value, chart flag and environment setting), and `tests/cases/study/` (generated help and executable examples). The native catalog proof covers the complete 551-study catalog and every five-header window. The native card/section generator and files generator keep their cases outside the repo, each with its own measured full and comparable byte ceilings. Known-positive capture evidence remains unchanged; this accounting adds no provider request.

## Captured source findings

All requests used the public tier without a key, institutional credentials or an alternate portal. j14206 made 36 fixed attempts: 35 HTTP 200 and one expected absent-study 404. Its total response-body transfer was 2,661,293 bytes. j14214 made six fixed attempts, all HTTP 200. j14223 read two exact study LICENSE files, both HTTP 200. No redirects or retries occurred. Source timings below are individual HTTP elapsed measurements on the M5 with cold Python processes, ordinary shared-machine load and no application cache. They exclude job setup and are not warm/cold product-command benchmarks.

| Public endpoint | Observed answer | Requests, bytes and elapsed |
| --- | --- | --- |
| `/api/studies?projection=DETAILED` | 551 study metadata records; assay totals, import dates, source cancer types and publications | One request, 549,064 bytes, 0.447 seconds |
| DataHub `study_list.json` | 512 downloadable archive IDs | One request, 10,358 bytes, 0.304 seconds; catalog membership differs from the live study catalog, so neither is assumed complete for the other |
| Twenty exact `/api/studies/ID?projection=DETAILED` | All requested identities matched; native sample and assay counts, no `allPatientCount` field | One request per example, 868–2,692 bytes each; median 0.182 seconds, range 0.148–0.421 seconds |
| BRCA / LUAD / MSK `molecular-profiles?projection=DETAILED` | 17 /14 /3 native profile records; exact parent identity and profile type | One request per study, 19,199 /15,549 /2,187 bytes; 0.187 /0.196 /0.188 seconds |
| BRCA / LUAD / MSK `sample-lists?projection=DETAILED` | 13 /11 /60 lists, stated `sampleCount` and public sample IDs | One request per study, 207,993 /103,070 /1,327,332 bytes; 0.263 /0.203 /0.480 seconds. Product output must discard sample IDs without computing counts from them |
| BRCA sample lists `projection=SUMMARY&pageSize=100` |13 list descriptions, but neither `sampleIds` nor `sampleCount` | One request, 3,162 bytes, 0.161 seconds. SUMMARY cannot supply list counts; missing counts must not become zero |
| BRCA patients `projection=META&pageSize=1` | Empty body, HTTP header `total-count:1084` | One request, zero body bytes, 0.229 seconds. It is a source-stated patient endpoint count; it does not fill an absent study-card field or a gene denominator |
| `/api/mutated-genes/fetch`, body containing only one `studyIds` value | Source gene aggregates, exact Entrez IDs, source panel IDs, altered/profiled cases; no frequency or explicit unit | BRCA: 16,488 rows,4,258,105 bytes,0.749 seconds. MSK:413 rows,113,843 bytes,0.613 seconds. These are source aggregates, not local calculations |
| DataHub GitHub contents for exact BRCA / MSK directories |58 /18 native file entries with path, publisher links and Git blob SHA |62,536 /16,844 bytes,0.287 /0.243 seconds. GitHub `size` is repository blob size; LFS means it must not be labeled data-content size |
| Exact absent study |HTTP 404,55-byte error body | One request; no fallback, query repair or inferred biological absence |

The source metadata describes samples, patients, profiles, resources and structural variants with different fields. Preserve native `structuralVariantCount` and `treatmentCount` names without renaming them sample counts. `resourceCounts` carries resource-specific sample/patient counts, which are not study-wide totals. Public `groups` and `readPermission` describe source access metadata, not a reuse license. All twenty selected records state `referenceGenome: hg19`; preserve this as source metadata without imputing any per-variant coordinate or converting to another build.

The native BRCA profile set explicitly includes `GENERIC_ASSAY`, `COPY_NUMBER_ALTERATION`, `MUTATION_EXTENDED`, `PROTEIN_LEVEL`, `MRNA_EXPRESSION` and `STRUCTURAL_VARIANT`. MSK has CNA, mutations and structural variants; no expression profile was returned. A public source record lacking an assay must not be described as a biologically negative result. Profile IDs are captured, never constructed by appending `_mutations` to a study ID.

### Source-stated gene counts

The following questions supplement S01–S20. They reuse the studies' published research context and show exactly what a source-count answer can carry.

| Case | Sourced question | Accepted native answer |
| --- | --- | --- |
| S21 | What TP53 counts does cBioPortal report for the [BRCA PanCancer Atlas study](https://pubmed.ncbi.nlm.nih.gov/29625048/)? | Exact Entrez 7157 /TP53: `numberOfAlteredCases:347`, `numberOfAlteredCasesOnPanel:347`, `totalCount:353`, `numberOfProfiledCases:1066`, panels`[WES]`. Unit unspecified in captured aggregate schema; no calculated frequency |
| S22 | What KRAS counts does the same published cohort's source report? | Exact Entrez 3845 /KRAS:6 altered cases,6 altered cases on panel,6 `totalCount`,1066 profiled cases, panels`[WES]`; retain these native field names and scope |
| S23 | What TP53 counts does cBioPortal report for the [MSK-IMPACT study](https://pubmed.ncbi.nlm.nih.gov/28481359/)? | Entrez 7157 /TP53: 4538 altered cases,4538 altered cases on panel,4954 `totalCount`,10945 profiled cases, panels`[IMPACT341,IMPACT410]`; never call4954 patients or mutation events from magnitude alone |
| S24 | What KRAS counts does the published MSK-IMPACT cohort's source report? | Entrez 3845 /KRAS: 1643 altered cases,1643 altered cases on panel,1670 `totalCount`,10945 profiled cases, same two native panel IDs; no prevalence or calculated rate |
| S25 | Which molecular profiles can the public [MSK-IMPACT cohort](https://pubmed.ncbi.nlm.nih.gov/28481359/) provide? | The returned IDs are `msk_impact_2017_cna`, `msk_impact_2017_mutations` and `msk_impact_2017_structural_variants`, with native types; no expression profile returned |
| S26 | Does the public [BRCA PanCancer Atlas cohort](https://pubmed.ncbi.nlm.nih.gov/29625048/) identify source-provided patient count separately from its sample metadata? | Study card has no patient count. Explicit patient META endpoint states `total-count:1084`; retain endpoint and acquisition provenance separately from `allSampleCount:1084` |

The captured aggregate row also carries `entrezGeneIds`, `hugoGeneSymbols` and `uniqueEventKey`, beyond the OpenAPI's listed properties. Some source rows may therefore represent grouped identities. A kept gene-count section must accept only exact, unambiguous source identity and preserve multiplicity; it may not silently use the first gene in a group. Optional `--gene` uses existing checked gene resolution and exact native Entrez joins. No symbol substring or disease-title heuristic enters that join. A query for a gene with no returned aggregate reports missing aggregate data. It is not proof that the gene was assayed and unaltered, and it does not make the study absent.

The clinical reviewer found no primary evidence in the retained OpenAPI that binds `AlterationCountByGene`'s fields to the shared unit enum. Its description says only that the values are grouped by gene. Therefore retain native cases-count metadata with unit unspecified. Do not map `totalCount` to events, `numberOfAlteredCases` to patients, or `numberOfProfiledCases` to a derived denominator. The current `CohortObservation` cannot represent these gene-level aggregates: it requires both a variant subject and a proved unit. The proposed first implementation keeps native count metadata; a future shared-model extension requires PM agreement and evidence for its unit. No default variant subject is proposed.

### Bundles, files, rights and cadence

Archive HEAD requests establish exact publisher sizes without fetching the archives: BRCA 509,019,117 bytes, LUAD 934,112,175 bytes, MSK 12,752,259 bytes. BRCA/LUAD Last-Modified dates are 2026-06-01; MSK is 2026-08-04. The study metadata import dates can differ from those archive dates. Preserve both without assuming release equivalence. These measurements justify the metadata-only default and file-reference disclosure; downloading a study archive for a card would transfer hundreds of megabytes without improving the basic answer.

Exact DataHub LICENSE captures distinguish terms. The BRCA file refers to the Broad Institute GDAC TCGA Analysis Pipeline License and warns that the data may change. It does not itself supply a complete permissive grant, so use `restricted` with that publisher link until reviewed further. The MSK file states ODbL attribution and share-alike obligations; its proposed study-specific label is `attribution`, with the ODbL and native license links. The existing source-wide restricted declaration remains authoritative until PM reviews how verified per-study labels interact with that existing source policy. Preserve both attribution and applicable study-specific restrictions. Public accessibility alone supplies no commercial-reuse proof.

Native LICENSE links are [BRCA](https://github.com/cBioPortal/datahub/blob/master/public/brca_tcga_pan_can_atlas_2018/LICENSE) and [MSK](https://github.com/cBioPortal/datahub/blob/master/public/msk_impact_2017/LICENSE). Do not label the generic software license as the study's data license. Other study licenses remain unverified and restricted. No licensed curation answers, raw source bodies or participant IDs enter Git.

### Measured disclosure budget

The table uses the accepted complete product envelopes from the retained CLI/MCP proofs. The earlier offline projections excluded envelopes, terms and actions; they were lower bounds and supplied no passing runtime cap. Every native case keeps its measured ceiling plus fifteen percent in the retained generator. CLI includes its newline; MCP structured content is one byte smaller. Comparable case budgets omit only the existing shared serializer's terms fields and remain separate from full-envelope ceilings. MCP transport frames remain measurement references rather than payload budgets.

| Layer | Largest measured full CLI bytes | Largest measured MCP structured-content bytes | Full CLI ceiling | Owning measured proof |
| --- | ---: | ---: | ---: | --- |
| One header | Included in the measured five-header envelope | Included in the measured five-header envelope | Covered by the search page ceiling | Every consecutive catalog window, including the largest native header |
| Search page, five headers | 2,654 | 2,653 | 3,053 | Catalog replay; comparable MCP ceiling 3,051 |
| Default study card | 3,127 | 3,126 | 3,597 | Forty-six retained card/section cases |
| Profile page, five | 2,890 | 2,889 | 3,324 | Complete BRCA, LUAD and MSK profile walks |
| Sample-list page, five | 2,088 | 2,087 | 2,402 | Complete BRCA, LUAD and MSK sample-list walks |
| File page, five | 3,390 | 3,389 | 3,899 | Twenty-one retained files/card/all cases |
| Bounded `all` | 9,338 | 9,337 | 10,739 | BRCA native bounded first pages; no participant matrices |
| One selected gene count row | Deferred | Deferred | No fabricated budget | Capture limit ruling defers gene counts and patient totals |
| Sync | 450 | 449 | 518 | Retained compact catalog sync receipt, including native source terms |

The retained sync receipt serializes to 449 compact UTF-8 JSON bytes; the CLI newline adds one. These are technical serialization measurements from the existing verified product receipt, not a new source call or replay. New synthetic examples and help also enforce their measured per-step `max_bytes` through both CLI and MCP. Existing synthetic failure, retirement, navigation and disabled-source cases retain their owning response ceilings. Source cases remain outside Git when publisher terms bar storage.

The synthetic help/example measurements in j14944 preserve the existing full-envelope and comparable-output distinction. Both surfaces passed their ordinary cases before these fixed ceilings were added. The source terms remain present in full responses.

| Case step | Full CLI bytes | MCP bytes | Comparable bytes | Comparable max_bytes | Full max_bytes |
| --- | ---: | ---: | ---: | ---: | ---: |
| help 1 | 1203 | 1202 | 1202 | 1383 | 1384 |
| help 2 | 1471 | 1470 | 1470 | 1691 | 1692 |
| help 3 | 433 | 432 | 432 | 497 | 498 |
| examples 1 | 445 | 444 | 279 | 321 | 512 |
| examples 2 | 445 | 444 | 279 | 321 | 512 |
| examples 3 | 928 | 927 | 762 | 877 | 1068 |
| examples 4 | 936 | 935 | 770 | 886 | 1077 |
| examples 5 | 1141 | 1140 | 975 | 1122 | 1313 |
| examples 6 | 1142 | 1141 | 976 | 1123 | 1314 |

The existing synthetic study runner responses have fixed command-phase full-envelope budgets in `study_budgets.rs`, attributed in j14953. Each call site names its phase, and each surface uses its own measured maximum plus fifteen percent. A page walk keeps its largest measured page. Unknown phases fail rather than borrowing a schema-wide allowance. This supplements the native per-case ceilings without rerunning native proofs. Failure and retirement cases retain their more specific existing ceilings.

Every 0.9 timing and output-size row remains **unmeasured**: this lane has no independently verified contained legacy executable, and it installs no external baseline. The milestone 5 comparison owns those measurements. No speedup, token reduction or output-size ratio is claimed. Native source transfer sizes above remain HTTP measurements and are not product command costs. The byte-derived token approximation used by the runner is not a tokenizer benchmark. No local count, frequency or denominator is calculated.

## Evidence and review limits

Evidence is outside Git on Yellow under `data/biomcp-redux/0194-study-investigation/`:

| Job | Archive | SHA-256 | Independent verification |
| --- | --- | --- | --- |
| j14206 at `1d2da2092c767ce08b2e6f31ce5ff15ac0e41d73` |`j14206-native-metadata.tar.gz` |`3c6c25ccc194e1822506c6caaf260403aa00a32770c0d210b1b65d135166588a` |41 exact members; every manifest digest verified locally and at destination |
| j14214 at `8337152da8983f6e9eaa3e01f1c3cf5113f9cff5` |`j14214-counts-files.tar.gz` |`8dce62719f04569a30e0e035aef594b26e8bb0f52c07a908817573942d7c7153` |11 exact members; every manifest digest verified locally and at destination |
| j14223 at `366f86241861b03a0cb0bdb91043e086e193aded` |`j14223-licenses.tar.gz` |`8768c5bf5cdf561ac9a5a34846a27dcb7d7596d932365661adc8891f05277518` |7 exact members; every manifest digest verified locally and at destination |

Each archive retains the fixed recipe, original response bodies, HTTP headers/statuses, request scopes, timestamps, measured elapsed durations, environment and manifest. The clinical reviewer independently checked all S01–S20 native records and accepted those metadata answers; the reviewer separately required native count names and unspecified aggregate units. Clinical review does not accept technical transport or licensing design. A bounded fixed-route source failure witness is the captured exact 404; synthetic 429/500/timeout/malformed-body cases are proposed, not falsely labeled live failures.

Unmeasured limits remain explicit: 0.9 commands were inspected but not executed because no independently verified network containment was established for this lane; no warm command timing, token count, full dataset download, content checksum for LFS objects, gene-frequency value, full terms review for all 551 studies, or private-tier probe was performed. These limits do not authorize substituting a calculated rate or a guessed source field. The accepted Study and capture limit rulings keep gene counts, patient totals, the DataHub archive catalog and per-study use labels deferred; no shared-model extension is made.


Reproducibility supplement: `metadata-annotations.tar.gz`, SHA-256 `eff5b037f893c4e0fba9723f9d1e8d39dd45f8319d3bcc5fad4d508576f6623f`, lives beside the native archives. Its four members preserve the complete closed-issue inventory, the byte-projection recipe, its JSON measurements and a manifest. Every member hash and the exact member set were independently checked on Yellow. The recipe reads the two retained source archives and reproduces the byte table without contacting providers or calculating biological counts.

The independent clinical review accepts all twenty-six question answers and the final count boundaries. It verified the exact TP53/KRAS native arrays, four count fields, panel IDs and the separately scoped patient endpoint total. No clinical question remains. Shared-model representation, source policy, transport design and output ceilings remain PM and technical review decisions.

## Bounded admission witnesses

Job j14352 ran from landed `6e20aa069bbb981f9c9c6f0da3fd4ba31e556ceb` on the M5 on 2026-10-26 UTC. It verified the retained j14206 archive and inspected its OpenAPI before making requests. It wrote both fixed request plans before admission and invoked `make source-capture PLAN=... OUT=...` for each. Each plan permits one attempt, no redirect or retry, a 20-second deadline and a 1 MiB body bound. The job paced admissions by one second. It made two physical requests and no aggregate or sample-list request. No credential was used.

| Capability | Result and boundary |
| --- | --- |
| Gene-count section | Deferred under the accepted capture limit ruling. `/api/mutated-genes/fetch` accepts `StudyViewFilter`, which has no declared selector for restricting returned aggregate rows to named Entrez IDs. `geneFilters[].geneQueries[][]` includes `entrezGeneId`, `hugoGeneSymbol` and alteration predicates within the study-view cohort filter. That field is not proof of an aggregate-row selector and was not substituted. No filtered count witness ran. Retained unfiltered BRCA and MSK costs remain 4,258,105 and 113,843 bytes. The PM accepted deferral of the gene-count section and the patient total; the counts implementation ticket is withdrawn. |
| Full-catalog search projection | Proven offline for all 551 retained records and all 547 consecutive five-header windows in source order. Largest header: 334 bytes, `brca_hta9_htan_2022`, zero-based index 545. Largest five-header page: 1,192 bytes, index 541, from `rms_msk_2023` through `brca_hta9_htan_2022`. These replace the selected-case maxima as the measured search basis. They exclude envelope, terms and actions, so they do not establish a product ceiling. |
| Largest catalog description | Proven offline: `brca_mbcproject_2022`, index 7, has 1,018 UTF-8 text bytes and 1,024 compact JSON-string bytes. This bounds the retained description projection, not a full product card. |
| Catalog revalidation | The retained catalog headers contain neither ETag nor Last-Modified. They state `Cache-Control: no-cache, no-store, max-age=0, must-revalidate`, `expires: 0` and `pragma: no-cache`. Conditional revalidation is unproved; acquisition time is not a publisher revision. |
| Second source patient total | `GET https://www.cbioportal.org/api/studies/msk_impact_2017/patients?projection=META&pageSize=1` returned HTTP 200, zero body bytes and `total-count: 10336` in 0.228 seconds. This proves a second separately attributed patient-total witness beside BRCA 1084. The accepted capture limit ruling defers the patient-total capability with gene counts. It does not fill a study-card field, count participants locally or establish a gene denominator. |
| Absent DataHub directory | `GET https://api.github.com/repos/cBioPortal/datahub/contents/public/biomcp_absent_study_0207` returned HTTP 404 and a 141-byte native error body in 0.179 seconds. The helper records this as `provider_failure`; the fixed witness interprets the captured 404 as absence of that exact directory. It proves the files section can distinguish exact source absence. It does not establish absence of an unrelated study or grant a fallback. |

The offline recipe uses the earlier header keys `studyId`, `name`, `cancerTypeId`, `allSampleCount`, `citation` and `referenceGenome`, retaining only present fields. It serializes compact UTF-8 JSON with `ensure_ascii=False` and separators `(',', ':')`. Each full five-header window is measured; the description measurement records both raw UTF-8 text and its JSON string. The retained catalog body SHA-256 is `7840bd832d3b6593888cfbeb1788f9bb3ab77469168e7cccedea64216d87545a`. These are technical metadata measurements, not calculated clinical counts.

Every distinct native `cancerTypeId` and its study-record count appears below. The codes retain source spelling; no ontology equivalence or descendant expansion is inferred.

| Native code | Study records |
| --- | --- |
| `acbc` | 1 |
| `acc` | 3 |
| `acyc` | 7 |
| `adifg` | 1 |
| `alal` | 1 |
| `aml` | 9 |
| `ampca` | 1 |
| `angs` | 3 |
| `apad` | 1 |
| `astr` | 1 |
| `bcc` | 1 |
| `bfn` | 1 |
| `biliary_tract` | 2 |
| `bladder` | 5 |
| `blca` | 18 |
| `bll` | 4 |
| `bowel` | 4 |
| `brain` | 2 |
| `brca` | 27 |
| `breast` | 8 |
| `ccrcc` | 10 |
| `cervix` | 1 |
| `cesc` | 3 |
| `chl` | 1 |
| `chol` | 8 |
| `chrcc` | 4 |
| `cllsll` | 6 |
| `coad` | 5 |
| `coadread` | 16 |
| `cscc` | 4 |
| `desm` | 1 |
| `difg` | 20 |
| `dlbclnos` | 6 |
| `egc` | 6 |
| `es` | 3 |
| `esca` | 4 |
| `escc` | 2 |
| `gbc` | 3 |
| `gist` | 3 |
| `hcc` | 12 |
| `hccihch` | 1 |
| `hdcn` | 2 |
| `head_neck` | 1 |
| `hgsoc` | 5 |
| `hnsc` | 8 |
| `ihch` | 5 |
| `lgsoc` | 1 |
| `liad` | 1 |
| `lipo` | 2 |
| `luad` | 17 |
| `lung` | 5 |
| `lusc` | 6 |
| `lymph` | 1 |
| `mbc` | 1 |
| `mbl` | 5 |
| `mbn` | 3 |
| `mcl` | 1 |
| `mds` | 3 |
| `mel` | 7 |
| `mixed` | 49 |
| `mnet` | 2 |
| `mng` | 1 |
| `mnm` | 1 |
| `mpn` | 1 |
| `mpnst` | 1 |
| `mrt` | 2 |
| `mtnn` | 1 |
| `myeloid` | 1 |
| `nbl` | 6 |
| `nccrcc` | 1 |
| `nfib` | 1 |
| `nhl` | 3 |
| `npc` | 1 |
| `nsclc` | 9 |
| `nsgct` | 2 |
| `nst` | 1 |
| `ohnca` | 1 |
| `os` | 1 |
| `ovary` | 4 |
| `paac` | 2 |
| `paad` | 10 |
| `pact` | 1 |
| `pancreas` | 4 |
| `panet` | 4 |
| `past` | 1 |
| `pcm` | 2 |
| `pcnsl` | 2 |
| `plmeso` | 5 |
| `prad` | 25 |
| `prcc` | 3 |
| `prostate` | 6 |
| `ptad` | 2 |
| `rbl` | 2 |
| `rcc` | 2 |
| `read` | 3 |
| `rms` | 5 |
| `scco` | 1 |
| `schw` | 1 |
| `sclc` | 4 |
| `skcm` | 12 |
| `skin` | 3 |
| `soft_tissue` | 13 |
| `stad` | 8 |
| `stmyec` | 1 |
| `stomach` | 2 |
| `testis` | 3 |
| `tet` | 1 |
| `thpa` | 4 |
| `thym` | 3 |
| `thyroid` | 2 |
| `uccc` | 1 |
| `ucec` | 12 |
| `ucs` | 4 |
| `uec` | 1 |
| `um` | 4 |
| `urcc` | 1 |
| `usarc` | 1 |
| `utuc` | 5 |
| `vsc` | 1 |
| `wt` | 2 |

The evidence archive is `data/biomcp-redux/0207-study-bounded-captures/2293-study-admission.tar.gz` on Yellow, SHA-256 `7ce61d45a4fc89024452f5467524a0b759be6cae39ff767fc32d58e0634ecd71`. Its 17 exact members include the retained input archive, request plans, both helper archives, HTTP headers, receipts, offline measurements, OpenAPI inspection and executable recipe. Every one of its 16 manifest hashes was independently verified after transfer, together with both nested helper archives. The input archive's exact members and hashes were verified before use. Raw replies remain outside Git. No product code, participant identifier or licensed curation answer enters this change.


## Implemented catalog disclosure measurements

The retained catalog replay at `d2231c0c7cdac895fc450c8445bd46b3e59049dc` checked all 551 records and every one of the 547 consecutive five-header windows in source order through CLI and MCP. The complete `pan_can_atlas` walk returned 32 distinct identities without gaps or duplicates. The external case runner checked S01–S20 exact study identities and native cancer-type codes. Additional native title-word walks retained each study. No provider request ran in this replay.

| Output | Largest measured UTF-8 bytes | Enforced payload ceiling or frame reference at measured size plus 15 percent |
| --- | ---: | ---: |
| Complete CLI five-header response, including newline | 2,654 | 3,053 |
| MCP structured content and equivalent response text | 2,653 | 3,051, enforced payload ceiling |
| Whole MCP transport frame | 5,893 | 6,777, measurement reference |
| Compact MCP command discovery description | 2,009 | 2,311 |

The case runner enforces the serialized MCP structured content budget and verifies equivalent response text. The whole transport frame is a measured reference rather than an enforced case ceiling; all 547 measured frames fit that reference. Whole executable help measured 61,078 bytes and fits its existing ceiling. The five-header measurements include the envelope, source terms, retained pool metadata and next actions. They replace projected page sizes for the implemented catalog search. Larger requested limits retain their explicit row bound.

The durable archive is `data/biomcp-redux/0213-study-catalog/d2231c0c7cdac895fc450c8445bd46b3e59049dc.tar.gz` on Yellow, SHA-256 `d06e4a1605e6bbd55e3c4e71efb7403ccbf48a7c82817d5545954d978b2c6f02`. Its five exact members preserve the input archive, sync receipt, all window measurements, executable recipe and manifest. Its four manifest hashes and the nested input archive's 41 exact members and 40 hashes were verified at the durable destination. The first native job passed external cases and then stopped on the recipe's noncanonical macOS temporary path; the bounded continuation used its canonical path and ran only the unreached measurement and transfer. The raw catalog remains outside Git. No 0.9 timing or output comparison is claimed.

## Implemented card and section disclosure measurements

The stored card uses `biomcp.study_card/2` because its required `source_url` adds the checked publisher study link. Its native fields and catalog snapshot remain intact. Profiles and sample lists retain their existing versioned envelopes. Each section reads its complete bounded DETAILED source array and pages the accepted metadata locally. A sample-list reply admits at most 1,526,432 bytes, which covers the retained MSK reply plus fifteen percent. A larger or undecodable collection returns a failure and an incomplete-collection problem rather than a shortened success page.

The retained cases cover all twenty card questions and every page of the BRCA, LUAD and MSK profile and sample-list arrays. They compare CLI and MCP replies without removing source facts, check every native profile and list identity and stated count, and reject any output containing a member sample ID from the retained replies. Each external case seeds its catalog through one CLI sync and compares both read surfaces against that same acquisition. Separate sync operations have separate acquisition times and therefore do not supply a read-parity comparison.

| Output | Largest full CLI bytes, including newline | Largest MCP structured-content bytes | Largest enforced full-response ceiling |
| --- | ---: | ---: | ---: |
| Stored card | 3,127 | 3,126 | 3,597 |
| Five-profile page | 2,890 | 2,889 | 3,324 |
| Five-sample-list page | 2,088 | 2,087 | 2,402 |

Each of the forty-six cases has its own measured ceiling plus fifteen percent in the retained-case generator. The native trial checks the complete serialized response plus the CLI newline against its full-response ceiling, retaining source terms. The runner's separate comparable-output assertion uses its existing source-terms serializer; the largest comparable MCP payloads are 2,961, 2,724 and 1,922 bytes respectively. Equivalent MCP response text carries the same JSON. These measurements describe the owned command payloads, not a whole MCP transport frame or a comparison with an older executable.

The original capture archive does not record per-request acquisition timestamps. Native replay preserves that missing provenance as the shared transport's legacy timestamp value of zero. It does not substitute an HTTP Date header, a batch completion time or a fabricated timestamp. Synthetic navigation cases separately prove that a nonzero original reply timestamp survives cache reads and emitted continuation commands. The catalog's local acquisition time and the native import date remain independent.

The first twenty-two native measurements are preserved in `data/biomcp-redux/0238-study-get/j14671-partial.tar.gz` on Yellow, SHA-256 `84f699ec854f2db0e3072d745e6f9558ad18715a0e19c2dc8481d2a38ab40843`. The continuation archive is `data/biomcp-redux/0238-study-get/ccc94f2d5da52b01c2dc510c9c059392e2295eda.tar.gz` on Yellow. Its SHA-256 is `45795b4c1a68e9df0f8a8f473844b39168114294d0c2fe50f0b35c37c84c1191`. Each archive has fifty-six exact members and fifty-five verified manifest hashes, including the original capture archive, external cases, replay records and measured output. The original input's forty-one members and forty manifest hashes are verified before case generation. No provider request ran.

The complete replay with the explicit full-response guard passed all forty-six CLI/MCP cases in j14686. Its durable archive is `data/biomcp-redux/0238-study-get/45ca299c1b6e090e437ec6e3e8ef815cd3651bb8.tar.gz` on Yellow, SHA-256 `5448657b7c0ad0ea82857286b0ef6ba6fd501f4ef477a16ec9df3f5bbd48fb0a`. Its fifty-six exact members and fifty-five manifest hashes were verified at the durable destination before scratch cleanup. All original source bodies remained outside Git, and the replay made no provider requests.


## Native file reference disclosure

The files section reads one exact GitHub contents directory under the existing restricted cBioPortal declaration. It returns native references in source order and pages them locally. `repositoryBlobSize` preserves GitHub's `size`; it states repository blob bytes and makes no claim about LFS content size. No listed data file, LICENSE body or archive is downloaded. A native LICENSE entry supplies its publisher link as `license_notice` on every files page and on `all` through `files.data`. A plain card reads only the stored catalog and never attaches a notice from a warm directory cache. A missing directory leaves the stored card intact and produces an empty files section in `all`.

The bounded directory capture contains 58 BRCA and 18 MSK entries with native LICENSE links. Its nine-member archive is `native.tar.gz` in the `0221-study-files` evidence folder, SHA-256 `5cf07928dcea5ebd755e05864fcb0821e9be3f061b22360381569a7442106677`. The original absent-directory reply remains in the retained study admission archive. The replay recipe checks exact archive digests and every manifest member before preparing external cases. Raw replies stay outside Git.

The original five-entry projection was 1,721 bytes. Full native outputs retain publisher and download references, blob identity, source terms, continuation commands and the LICENSE notice. The following measurements distinguish complete CLI bytes, including its newline, from the comparable envelope budget. MCP has one fewer full byte. `OUTPUT_CEILINGS` in `scripts/study_files_retained.py` derives each byte and token ceiling from its measured output with fifteen percent headroom. The retained native cases enforce both full and comparable ceilings through the ordinary case runner.

| Case | Native view | Full CLI bytes | Comparable bytes | Full ceiling | Comparable ceiling | Token ceiling |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| 000 | BRCA card | 2,430 | 2,264 | 2,795 | 2,604 | 651 |
| 001 | BRCA all | 9,338 | 8,677 | 10,739 | 9,979 | 2,496 |
| 002 | BRCA files offset 0 | 2,709 | 2,543 | 3,116 | 2,925 | 732 |
| 003 | BRCA files offset 5 | 2,983 | 2,817 | 3,431 | 3,240 | 811 |
| 004 | BRCA files offset 10 | 3,074 | 2,908 | 3,536 | 3,345 | 837 |
| 005 | BRCA files offset 15 | 3,390 | 3,224 | 3,899 | 3,708 | 927 |
| 006 | BRCA files offset 20 | 3,094 | 2,928 | 3,559 | 3,368 | 842 |
| 007 | BRCA files offset 25 | 3,026 | 2,860 | 3,480 | 3,289 | 823 |
| 008 | BRCA files offset 30 | 2,962 | 2,796 | 3,407 | 3,216 | 804 |
| 009 | BRCA files offset 35 | 3,078 | 2,912 | 3,540 | 3,349 | 838 |
| 010 | BRCA files offset 40 | 3,338 | 3,172 | 3,839 | 3,648 | 912 |
| 011 | BRCA files offset 45 | 3,194 | 3,028 | 3,674 | 3,483 | 871 |
| 012 | BRCA files offset 50 | 2,930 | 2,764 | 3,370 | 3,179 | 795 |
| 013 | BRCA files offset 55 | 1,832 | 1,666 | 2,107 | 1,916 | 480 |
| 014 | MSK card | 1,316 | 1,150 | 1,514 | 1,323 | 332 |
| 015 | MSK all | 6,543 | 5,882 | 7,525 | 6,765 | 1,692 |
| 016 | MSK files offset 0 | 2,490 | 2,324 | 2,864 | 2,673 | 669 |
| 017 | MSK files offset 5 | 2,636 | 2,470 | 3,032 | 2,841 | 711 |
| 018 | MSK files offset 10 | 2,735 | 2,569 | 3,146 | 2,955 | 740 |
| 019 | MSK files offset 15 | 1,739 | 1,573 | 2,000 | 1,809 | 454 |
| 020 | Absent directory | 363 | 197 | 418 | 227 | 58 |
