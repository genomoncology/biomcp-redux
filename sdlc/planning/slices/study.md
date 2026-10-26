# Study slice investigation

This proposed design starts from main `85e6ba864c3f13ec50432a58670ec2af13e12857`. It follows steps 1–4 of [the slice process](../slices.md). It adds no product code.

The approved boundary keeps exact study retrieval and counts stated by cBioPortal. Local top-mutated, filter, query, co-occurrence, cohort, survival and comparison calculations retire. A source count never supplies an inferred denominator, prevalence, prognosis or treatment recommendation.

## Investigation method

Capture the public cBioPortal detailed study catalog, twenty exact published study records, molecular profiles and sample-list metadata for representative studies, the DataHub catalog, archive size headers, and current source documentation. Each request has one attempt, a bounded response and a deadline. Keep original bodies and receipts outside Git. No participant records, mutation tables or bulk study archives are needed to decide this metadata design. Preserve source-stated counts without calculating biological summaries.

The 0.9 command inventory is inspected statically. Execute no 0.9 binary until network containment is independently verified. Unmeasured command timing and output remain explicit limits.

## Shared model boundary

The existing [CohortObservation contract](../biodata-variants-and-annotations.md#37-population-frequency-cohort-counts-coverage-and-predictor-scores) requires `subject: VariantRef`. Study-wide and gene-wide counts cannot satisfy that requirement. Reuse it only when a captured source supplies an exact variant subject and the stated counts. Preserve study totals and profile/sample-list counts as source metadata. Request PM review before extending the shared subject type.

## Questions that should delight an agent

1. **Which cohort do I need?** `search study TEXT` searches only the publisher's study ID, name and description. It never rewrites a disease name or treats a cancer-type label as an ontology identifier. Exact `--cancer-type` filters a captured native source code.
2. **What can I learn from one call?** The header gives exact study ID, name, source cancer-type code, stated sample count, stated patient count when present, citation and source date. It distinguishes absent fields from zero.
3. **What comes next?** Candidate pages offer exact `get study ID` actions, then paging, then at most three other actions. Cards offer only supported sections under architecture 6.10. No hidden study/profile/sample-set choice occurs.
4. **What counts can I trust?** Each count retains its native scope and field name. Total samples, patients, sequenced samples, CNA samples and sample-list membership stay separate. A sequenced total is not locus coverage; no rate is computed.
5. **How do I find an alteration frequency?** This slice supplies only an explicitly stated source value with its scope and profile identity. It never derives a frequency from mutation rows or total samples. Without that evidence, the answer says the frequency is unavailable through the kept surface and links the source study.
6. **How do joins work?** Study IDs join profiles and sample lists by exact `studyId`. Gene references require native Entrez IDs. Source cancer-type abbreviations remain source codes. Neither case conversion nor text matching makes an OncoTree, MONDO or gene identity.
7. **What fails?** A checked successful empty search differs from source failure, disabled source, missing bundle and absent exact study. A stale accepted snapshot stays labeled stale and offers refresh. A rejected replacement never destroys the last accepted snapshot.
8. **How expensive is a call?** A metadata bundle avoids downloading whole mutation matrices or iterating every cohort on reads. Cold source timings, payloads and output projections appear with the capture evidence. Warm command timing remains unmeasured until a product exists.
9. **What can I cite?** Preserve the source study URL, publication identifier, citation, publisher metadata and snapshot revision. Do not substitute bundle acquisition date for biological release date.
10. **What can I stop doing?** An agent can stop after the bounded default card when it has identity, scope and count provenance. It need not retrieve participant records, render charts or install analysis tools.

## Proposed command tree and disclosure

This is a proposal for PM review. It does not claim that commands already exist.

| Command | Default answer | Deeper answer and next steps |
| --- | --- | --- |
| `search study TEXT [--cancer-type CODE] [--limit N] [--offset N] [--pool ID]` | One page of five study headers, stable source order; literal case-insensitive text matching on the native ID/name/description | One exact get per candidate, then pool paging; one sync action for absent/stale bundle. No clinical ranking, synonyms, recursive tumor descendants or gene-content heuristic |
| `get study ID [SECTIONS] [--section-limit N] [--offset N] [--pool ID]` | Exact study header, description, source-stated sample/patient counts, citation and terms | Named `profiles`, `sample-lists`, and `files` only where captures prove available fields; bounded pages, precise native IDs and source links. `all` remains bounded and never fetches participant-level matrices |
| `sync cbioportal-studies` | One checked metadata catalog snapshot, its source revision and acquisition status | Atomic replacement; accepted previous snapshot survives parse, identity, quota or transport failures. Reading never triggers implicit sync |

A get uses only an exact study ID selected from discovery or explicitly supplied by the caller. Unsupported aliases return usage guidance or source absence, never a guessed cohort. Exact study IDs remain case-sensitive if the source treats them that way. Search filters narrow only checked fields; a cancer type such as `mixed` must not expand into cohorts inferred from sample diagnoses.

The metadata bundle contains catalog metadata, not downloaded study matrices. The implementation must decide snapshot coherence before types land: catalog headers and optional remote section metadata may have different acquisition times, which must remain visible. A continuation binds its snapshot and filters; changing selectors with a continuation fails locally. Missing or expired pools offer restart without provider requests.

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
| Failure envelope | 429, 500, timeout, broken body, HTTP200 error object; failure differs from zero and absent; stale provenance remains explicit |
| Interface and cost | CLI/MCP parity; generated schemas; every emitted next action round trips; quoted file/ID examples; Markdown derived from JSON; measured default/section ceilings |
| Retirement | Every legacy leaf and chart option below; actionable source retrieval guidance; no analysis calls or hidden tool installation |

## 0.9 accounting

Static inventory uses the 0.9 checkout `4050fdc7dbdf4d713d94cf02a39a147b863665be`: `src/cli/study/mod.rs`, `src/cli/types.rs`, `src/cli/list/literature.rs`, `src/entities/study.rs`, `src/sources/cbioportal_study.rs`, and `spec/entity/study.md`. No legacy executable has been run. Network containment and executable baseline costs remain unmeasured.

The following ledger is proposed accounting for PM review, outside the generated closed-accounting markers until the PM accepts the design. A positional study ID is not an option slot. Global output/settings/help flags remain shared; they are outside the historical command-specific denominator.

| 0.9 leaf | Proposed disposition | Every command-specific option and alias | Migration |
| --- | --- | --- | --- |
| `study list` | Rename | No command-specific flags | `search study` over the checked source catalog; local installation list is not public-source absence |
| `study download` | Rename data acquisition | `--list`; positional study ID | `sync cbioportal-studies` acquires metadata only; exact get returns safe source file references. Whole-study analytical installs retire |
| `study query` | Drop calculations | `--study`/`-s`, `--gene`/`-g`, `--type`/`-t`; mutation/mutations, cna/copy_number/copy-number, expression/expr, sv/fusion/fusions/structural_variants | Exact get and explicit native counts; no recomputed rates, expression summary, CNA distribution or SV table |
| `study top-mutated` | Drop calculations | `--study`/`-s`, `--limit`/`-l` | Exact study get; source website for analysis |
| `study filter` | Drop calculations | `--study`/`-s`, repeatable `--mutated`, `--amplified`, `--deleted`, `--expression-above`, `--expression-below`, `--cancer-type` | No replacement analytical intersection; source retrieval guidance only |
| `study cohort` | Drop calculations | `--study`/`-s`, `--gene`/`-g` | No mutant/wildtype split; exact source study retrieval |
| `study survival` | Drop calculations | `--study`/`-s`, `--gene`/`-g`, `--endpoint`/`-e`; os/overall/overall_survival, dfs/disease_free, pfs/progression_free, dss/disease_specific | No KM, endpoint inference, survival statistics or prognostic conclusion |
| `study compare` | Drop calculations | `--study`/`-s`, `--gene`/`-g`, `--type`/`-t`, `--target`; expression/expr and mutations/mutation | No expression or mutation-rate group comparison |
| `study co-occurrence` | Drop calculations | `--study`/`-s`, `--genes`/`-g` | No contingency table, odds ratios or significance calculations |

The shared ChartArgs flags on query, survival, compare and co-occurrence all retire with their leaf: `--chart`, `--terminal`, `--output`/`-o`, `--title`, `--theme`, `--palette`, `--cols`, `--rows`, `--width`, `--height`, `--scale`, and hidden `--mcp-inline`. All chart kinds, themes, palettes, PNG/SVG, terminal and MCP inline rendering retire. `list study` and catalog discovery are shared help surfaces and must describe the new entity commands. `BIOMCP_STUDY_DIR` no longer selects an analysis directory; migration guidance names source sync rather than silently scanning that directory.

Legacy output sections are all accounted: local availability, downloadable IDs and install result become source metadata/sync guidance; Mutation Frequency / Top Variant Classes / Top Protein Changes / CNA Distribution / Expression Distribution / Structural Variants / Top Mutated Genes / Filter Intersection / Cohort Groups / Survival / Group Comparison / Co-occurrence / Charts retire with the calculation ruling. The old `not_in_local_cohorts` state becomes explicit missing snapshot or source absence, never a zero frequency. Legacy study commands have no general get-section selector grammar to preserve.

### Closed public issues

The complete 57-issue closed public inventory was read on 2026-10-25 through `gh issue list --repo genomoncology/biomcp --state closed --limit 1000 --json number,title,body,url`. Inventory SHA-256 is `7eaf219c6f36a891089c98a1eebf18370ed56ba8efbaa2a5f6c3e8b95228e4a6`. Only [issue 113](https://github.com/genomoncology/biomcp/issues/113) concerns this entity's source. Keep its requirement to reject cross-gene data; exact profile/study/gene identity checks are mandatory before any count admission. The old locally aggregated hotspot summary retires; its correctness requirement does not. The other parts of issue 113 concern FDA drug data and are outside this slice. Keyword matches 87, 37 and 20 refer to trial studies or shared MCP tools and are not study-entity requests. No closed study-specific shipped user request has been silently omitted.

## Clinical review boundary

The independent reviewer confirmed from the official FAQ and gene-panel matrix documentation that a sequenced-sample total or sequenced-list membership is not proof of gene-assayed denominator. A missing mutation row cannot mean unaltered. cBioPortal supports multiple samples per patient and overlapping cohorts, so preserve samples, patients, events and profiles distinctly. The native lower-case cancer-type abbreviation is not an OncoTree identifier. Final review must bind these statements to the actual captured record shapes.

## Ian's questions

No clinical or configuration question is sent to Ian. The PM must resolve whether this metadata-only study surface should extend the shared CohortObservation subject beyond VariantRef; the recommendation is to retain native study counts separately and avoid a shared-model change unless a retained captured count endpoint requires it. The PM must also accept the proposed discovery/section/bundle boundaries and any measured output ceilings before implementation tickets exist. These are design decisions, not requests for a paid service, private data or a production change.
