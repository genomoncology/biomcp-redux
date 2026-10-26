# Pharmacogenomics slice design and implemented accounting

This investigation starts from `d18bfbba511e3852c99228f1a5166a14633e5cee`. It covers steps 1 to 4 of [the slice method](../slices.md). It proposes source retrieval, typed disclosure and exact follow-up commands. It does not calculate a patient phenotype, select therapy, recommend a dose or implement product code.

## Evidence boundaries

Official documentation was read on 2026-10-25. Source response counts, field presence and single-request latency are measured below. Implemented product measurements below supersede the historical body projections. The contained 0.9 comparison remains unmeasured. A documented capability is not a successful public-tier request. The source captures must precede acceptance of shared types. Historical source adapters and public papers do not replace that proof.

The reviewed drug, variant and disease designs establish source-owned identities, explicit selectors, independent collection pages and absence distinct from zero. The shared model in [biodata variants and annotations](../biodata-variants-and-annotations.md) establishes `Statement`, `Subject`, source attribution and native fallback. Its proposed `Subject` is broader than the `VariantRef` currently implemented at the starting revision. This slice must not silently implement either expansion or a star-allele subject.

## 1. Questions and accepted answers

A fresh clinical reviewer read the passages below on 2026-10-25 and accepted these source-level answers as confirmed with high confidence for the stated versions. These are explicit investigation questions drawn from published guidance, not transcripts of patient requests. No licensed curation answers or patient details were supplied. Clinical dose text below describes source content for retrieval testing; it supplies no patient-specific instruction. Source reachability and API support remain separate measurements.

| Key | Primary source, version and passage |
| --- | --- |
| D | [CPIC DPYD guideline PA166251462](https://www.clinpgx.org/guideline/PA166251462), [fluorouracil annotation PA166122686](https://www.clinpgx.org/guidelineAnnotation/PA166122686), PMID 29152729, DOI 10.1002/cpt.911. Read the current table and November 2018, January 2024 and July 9, 2026 updates. The July notice explicitly says its changes await incorporation into website/database tables. |
| T | [Thiopurine update](https://ascpt.onlinelibrary.wiley.com/doi/10.1002/cpt.70209), PMID 41618934, published 2026-01-31; [April 2026 correction](https://pmc.ncbi.nlm.nih.gov/articles/PMC13652152/), PMID 42050374. Corrected Table 1 and Tables 2–3 control the answers. Guideline PA166251442. [Allele-function study](https://pmc.ncbi.nlm.nih.gov/articles/PMC13446156/), PMID 42560154, supports current function assignments. |
| N | [CPIC thiopurine update notice](https://blog.clinpgx.org/cpic-guideline-for/), 2026-02-06, passages on retired NUDT15 *2, current *3 suballeles and phase ambiguity. |
| M | [Joint AMP/CPIC/CAP/DPWG genotyping consensus](https://pmc.ncbi.nlm.nih.gov/articles/PMC9808500/), TPMT *3A/*3B/*3C cis/trans discussion. |
| C | [CPIC tamoxifen guideline](https://pmc.ncbi.nlm.nih.gov/articles/5931215/), 2018, PMID 29385237; Table 2, therapeutic recommendations, endpoint evidence and other considerations. Guideline PA166251458. Current source genotype/phenotype mapping is separate from these historical activity-score categories; [2019 translation consensus](https://ascpt.onlinelibrary.wiley.com/doi/10.1111/cts.12692), PMID 31647186, documents changed translation. |
| I | [DPWG UGT1A1/irinotecan guideline](https://pmc.ncbi.nlm.nih.gov/articles/PMC10474017/), online 2022-11-28, PMID 36443464, DOI 10.1038/s41431-022-01243-2; Table 2, titration and comparison with RNPGx/GPCO. |
| A | [Current ClinPGx DPYD/fluorouracil annotations](https://www.clinpgx.org/combination/PA128406956%2CPA145/clinicalAnnotation), read 2026-10-25; rs3918290 rows and positive-chromosomal-strand warning. Individual annotation identities must come from native capture. |
| E | [PharmGKB evidence framework](https://ascpt.onlinelibrary.wiley.com/doi/10.1002/cpt.2350), 2021; association-summary purpose, guideline/label weighting and limits on ranking. [CPIC level definitions](https://cpicpgx.org/wp-content/uploads/2021/03/FDA-vs-CPIC-march-2021-c-002.pdf), 2021, distinguish pair actionability from guideline publication. |

| Case | Question | Accepted source answer | Capture coverage to inspect |
| --- | --- | --- | --- |
| Q01 | Which guideline covers DPYD with fluorouracil and capecitabine? | D: PA166251462; preserve both drugs and update dates. | C03, C09, C21; P01, P07. |
| Q02 | Which identifier accompanies DPYD *2A? | D: rs3918290 and NM_000110.3:c.1905+1G>A are source-given mappings. | C16, C25; P01, P10. |
| Q03 | What does the November 2018 update say for DPYD scores 1 and 1.5? | D: 50% starting-dose reduction with titration; the later specific HapB3 notice remains distinct. | C09; P07; source notice. |
| Q04 | What changed for heterozygous causal c.1129-5923C>G in July 2026? | D: value 0.75, score 1.75 and 75% initial dose; database tables are explicitly not yet updated. | C09; P07; notice/table discrepancy. |
| Q05 | Does rs56038477 establish causal rs75017182? | D: no; the January 2024 notice distinguishes proxy and causal testing. | C25; P01, P10; missing direct causal context. |
| Q06 | Does this guideline provide actionable tegafur guidance? | D: tegafur is listed without actionable recommendations. Preserve evaluated/no-action. | C03, C09; P07. |
| Q07 | What source phenotype accompanies TPMT *1/*3A? | T: intermediate metabolizer. | C17; P02. |
| Q08 | What source phenotype accompanies NUDT15 *1/*3? | N: intermediate metabolizer with current nomenclature. | C18; P03. |
| Q09 | What source phenotype accompanies NUDT15 *5/*5? | T: intermediate; *5 has decreased function. | Current mapping not reserved in C18; remains a capture gap. |
| Q10 | What does one no-function and one decreased-function TPMT allele mean in corrected Table 1? | T: intermediate, not possible intermediate. | C10; exact function-pair mapping remains unmeasured. |
| Q11 | What does the analogous NUDT15 function pair mean? | T: poor. Do not reuse TPMT's mapping. | C11; exact function-pair mapping remains unmeasured. |
| Q12 | What mercaptopurine guidance applies with both genes normal? | T: standard starting dose, with protocol and toxicity adjustment. | C10, C11; P02, P03. |
| Q13 | What applies with one intermediate and the other normal in malignancy? | T: 30–80% when standard starts at least 75 mg/m²/day; retain the lower-dose qualification. | C10, C11; complete joint keys/population. |
| Q14 | What changes when both genes are intermediate? | T: 20–50% at that same threshold; preserve the joint context. | C10, C11; independent rows, no flattened gene result. |
| Q15 | What thioguanine guidance applies to either poor metabolizer? | T: tenfold daily reduction and thrice-weekly frequency, with monitoring. | C10, C11; P02, P03. |
| Q16 | Does an unavailable second gene count as normal? | T: no; preserve missingness. | C10, C11; authored partial-key negative control. |
| Q17 | Which setting does the tamoxifen guideline address? | C: adjuvant ER-positive breast cancer; other settings are not automatically covered. | C12, C23; P04. |
| Q18 | What does its normal-metabolizer table say? | C: 20 mg/day and avoid moderate/strong inhibitors, attributed to the 2018 source. | C12; historical/current version contrast. |
| Q19 | What is its postmenopausal poor-metabolizer recommendation? | C: alternative endocrine therapy, such as an aromatase inhibitor. | C12; retain population/context. |
| Q20 | What qualifies the premenopausal alternative? | C: ovarian-function suppression accompanies the aromatase inhibitor recommendation. | C12; do not discard qualifying text. |
| Q21 | Does tamoxifen 40 mg normalize poor-metabolizer endoxifen? | C: no; levels rise but do not normalize. | C12; preserve comments and endpoint. |
| Q22 | Can tumor CYP2D6 automatically supply the germline result? | C: no; tumor loss of heterozygosity can distort it. | No tumor-to-germline join; authored unsafe-context control. |
| Q23 | Are all tamoxifen endpoint associations equally established? | C: no; preserve endpoint-specific strength and source scope. | C12; P04; no common evidence score. |
| Q24 | What does DPWG say for UGT1A1 *28/*28 and irinotecan? | I: 70% initial dose, with source-qualified escalation. | C07, C13, C20; P05, P08; DPWG attribution. |
| Q25 | What does it say for *1/*28? | I: no genotype-based reduction; no inference of zero toxicity. | C13; exact diplotype not in reserved capture. |
| Q26 | Does its deficient-allele scope include *6, *28 and *37 combinations? | I: yes; preserve its defined phenotype grouping. | C13; P05, P08; exact group fields may be absent. |
| Q27 | Do guidelines agree below irinotecan 180 mg/m²? | I: no; preserve DPWG versus RNPGx/GPCO attribution. | P08; no source ranking or consensus. |
| Q28 | Which level/category accompanies current rs3918290/fluorouracil annotations? | A: Level 1A, Toxicity, Neoplasms; separate Other rows also exist. | P01, P10; exact native annotation identities. |
| Q29 | Does Level 1A itself specify a dose? | E: no; annotation evidence and prescribing guidance are distinct. | C03 versus P01 and P07. |
| Q30 | Does CPIC pair level A prove a CPIC guideline exists? | E: no. Attribute an irinotecan DPWG guideline to DPWG. | C07, C13; P05, P08. |
| Q31 | Can unphased NUDT15 c.415C>T plus c.50_55dup select one diplotype? | N: no; *1/*3 and *3/*6 can differ clinically. | C18; authored phase-ambiguity control. |
| Q32 | Can unphased TPMT c.460G>A and c.719A>G establish *1/*3A? | M: no; *3B/*3C in trans remains possible. | C17; authored phase-ambiguity control. |
| Q33 | What do empty and unavailable results establish? | Retrieval-only controls: source-local no matches versus failure; neither supplies a normal result or dose. | C27/P09 if native zero; explicit authored failure controls. |

### Per-question source and section coverage

Status here describes whether the retained API captures answer the entire question. The accepted clinical literature answer above remains valid in its stated version. `answered` means the needed API fact is present; `partial` means some source identity/context is present but the full answer is not; `unanswered` means the required answer was not captured. Every source is independent. A five-row page never proves absence beyond its declared scope.

| Question | Status | Source and section | Actual coverage or missing part |
| --- | --- | --- | --- |
| Q01 | answered | CPIC pairs/guidelines | C03/C21 identify guideline and both drugs. |
| Q02 | partial | ClinPGx annotation; CPIC diplotype | P10 gives source variant mappings with a newer reference version; C16/C25 literal star-name lookups are empty. |
| Q03 | answered | CPIC recommendations | C09/C15 carry both source score groups and their reduction/titration text. |
| Q04 | unanswered | CPIC recommendations; website notice | C09/C15 have no 1.75 row; the reviewed July notice is not an API capture. |
| Q05 | partial | ClinPGx annotation | Captured identifiers do not implement the reviewed proxy/causal testing qualification. |
| Q06 | answered | CPIC pairs | C03 has tegafur with `usedforrecommendation: No`. |
| Q07 | answered | CPIC diplotype | C17 reports the declared TPMT expression as intermediate. |
| Q08 | answered | CPIC diplotype | C18 reports the declared NUDT15 expression as intermediate. |
| Q09 | unanswered | CPIC diplotype | No exact *5/*5 request was reserved. |
| Q10 | unanswered | CPIC diplotype | No exact function-pair mapping capture. |
| Q11 | unanswered | CPIC diplotype | No exact function-pair mapping capture. |
| Q12 | unanswered | CPIC recommendations | C10/C11 first pages concern azathioprine, not mercaptopurine. |
| Q13 | unanswered | CPIC recommendations | No malignancy-specific mercaptopurine row captured. |
| Q14 | unanswered | CPIC recommendations | Joint keys exist, but this mercaptopurine phenotype row is not in the sampled page. |
| Q15 | unanswered | CPIC recommendations | No thioguanine recommendation row captured. |
| Q16 | partial | CPIC recommendations | Joint keys and indeterminate values are preserved; the literature's missing-second-gene caveat is not fully captured. |
| Q17 | partial | CPIC guidelines | C23 identifies the tamoxifen guideline, but not its cancer-setting prose. |
| Q18 | unanswered | CPIC recommendations | C12 first page concerns amitriptyline, not tamoxifen. |
| Q19 | unanswered | CPIC recommendations | No postmenopausal tamoxifen row captured. |
| Q20 | unanswered | CPIC recommendations | No premenopausal tamoxifen row captured. |
| Q21 | unanswered | CPIC recommendations | No tamoxifen endoxifen comment captured. |
| Q22 | unanswered | CPIC recommendations | Tumor/germline qualification remains in reviewed literature only. |
| Q23 | unanswered | CPIC recommendations | Endpoint evidence distinction remains in reviewed literature only. |
| Q24 | partial | CPIC diplotype/pairs; ClinPGx label references | C20 maps the expression; C07 identifies the pair without CPIC guideline, P08 supplies references, not DPWG dosing text. C13 concerns atazanavir. |
| Q25 | unanswered | ClinPGx annotation/label | No exact diplotype-guidance detail captured. |
| Q26 | partial | ClinPGx annotation/label | References exist; source-defined deficient-allele guidance is not captured in min tier. |
| Q27 | unanswered | ClinPGx label/guideline detail | P08 min references do not expose competing guideline prose. |
| Q28 | answered | ClinPGx annotation | P10 has exact PA166134421 Toxicity/Neoplasms and distinct PA166224841 Other. |
| Q29 | partial | CPIC pairs; ClinPGx annotation | Independent evidence and guidance objects are observed; the framework interpretation is literature-reviewed. |
| Q30 | answered | CPIC pairs/recommendations | C07 has level A without guideline; C13's complete UGT1A1 collection concerns another drug. |
| Q31 | unanswered | CPIC diplotype | Declared-expression lookup is proved; no phase resolution or ambiguity API was captured. |
| Q32 | unanswered | CPIC diplotype | No unphased-variant-to-diplotype conversion is offered or captured. |
| Q33 | partial | CPIC pairs; ClinPGx annotations; authored failures | C27 zero and P09 positive establish source-locality. Authored failure controls execute in `cpic.rs`, `clinpgx/failures.rs` and `pgx_retirements.rs`; their results are not native capture answers. |

CYP2D6 has 1,316 native recommendation rows in C12. W06 later proved the exact `drugid` filter on the distinct native recommendation table, now implemented with an explicit gene selector. That filter proof does not promote the unsampled tamoxifen answers in Q18–Q23. No answer is inferred from the original amitriptyline page.

No further provider call belongs to this investigation. Implementation tickets must retain these gaps and capture their named exact selectors or detail tier before accepting them. The contained 0.9 PGx comparison is a named later proof ticket, with verified containment and an explicit request ledger; no legacy executable runs here.

### Clinical ruling boundaries

The reviewer separated logical guideline identity, source-reported allele function, source-reported diplotype/phenotype, recommendation context and patient treatment. It rejected inferred phase, calculated activity scores, proxy-to-causal substitution, complementing annotation alleles and treating a missing second gene as normal. It also rejected collapsing CPIC pair levels, ClinPGx evidence levels, recommendation strength and PharmVar definition confidence into one scale. Identity, phase, dose, scope and stale-version errors carry high severity; evidence-namespace errors carry medium severity. New dated guidance or actual current table captures can change version-specific answers.

The July 2026 DPYD notice and the April 2026 thiopurine correction are explicit discrepancy tests. A source API can faithfully return a stale table. The product preserves that source/version limitation; it does not calculate an updated answer from a notice. Supporting website notices as an executable section requires its own captured, structured source contract. Until then retain the source link and explicit design gap. Do not claim complete current clinical guidance from an API table alone.

## 2. Source findings

### Official documentation

| Source | Primary reference and date | Finding relevant to the design |
| --- | --- | --- |
| CPIC | [Database documentation](https://github.com/cpicpgx/cpic-data/wiki), edited 2026-03-09; read 2026-10-25 | Documents a relational API, source gene and drug keys, named alleles, diplotypes, independent recommendation lookup methods, populations and native frequency records. A tracked pair need not have a guideline. API data can change between releases. |
| CPIC | [March migration announcement](https://blog.clinpgx.org/updates-to-the-cpic-database-and-api/), 2026-03-04, updated 2026-03-13 | API host remains `api.cpicpgx.org`. `clinpgxid` replaces former PharmGKB identifiers; guideline identity changes from an array of annotation IDs to one guideline entity ID. Pair annotation level becomes `clinpgxlevel`. Do not populate old fields by assumption. |
| CPIC | [Releases](https://github.com/cpicpgx/cpic-data/releases), read 2026-10-25 | Published application and database versions can differ. Retrieval date alone is not a dataset release. Preserve a release only when the response or retained export states it. |
| ClinPGx | [API introduction](https://api.clinpgx.org/), read 2026-10-25 | Public JSON API advertises annotations and reference objects; parameters and replies may change. Published request limit is two per second. Capture recipe uses serial requests with one second between admissions. |
| ClinPGx | [API documentation](https://api.clinpgx.org/swagger/), read 2026-10-25 | Documentation shell requires JavaScript. The bounded capture below proves actual min/base fields and keyless access for these requests. It does not prove reuse rights or native server pagination. |

### Executed native findings and measured source cost

The separate capture delegate ran `j12644` from landed main `b0a690d878bec55039f53ea9fa6e26feb6ea11c4` on 2026-10-25. The first request began at 05:50:40 UTC; final metadata retrieval began at 05:51:23 UTC. All 44 reserved attempts ran exactly once: 28 CPIC data, ten ClinPGx data and six metadata. Every body completed. All replies were HTTP 200 or 206. There were no retries, redirects, credentials, altered queries or automatic continuation. The investigator independently checked every raw body's SHA-256 against its receipt before the delegate removed its temporary reader copy. No provider failure was induced.

The retained archive is on Yellow at `data/biomcp-redux/0153-pgx-native-capture/j12644-pgx-native-capture.tar.gz`, 209,068 bytes, SHA-256 `ddfb5fda5c2329f3aedc3f02d4e4f1cdc9a23965d51f354f5c1e890038da3a42`. Its root `2294-pgx-native-capture` holds all 220 manifest-verified files: recipe, actual request/receipt/body records, field and optionality inventories, ledger, measurements and report. This is the durable evidence reference; no raw source body enters Git.

Each table row cost one physical request. These are upstream payloads, not product outputs. Approximate tokens are `ceil(compact_json_bytes / 4)`, not tokenizer counts. Raw bytes can exceed compact bytes because of source whitespace. Single-request latencies are not formal benchmarks or cold/warm observations. CPIC data ranged 240.236–422.249 ms, median 253.388 ms; ClinPGx ranged 233.638–580.104 ms, median 325.997 ms. Admission spacing is excluded. Data payloads totaled 73,687 CPIC bytes and 898,452 ClinPGx bytes.

| Row | HTTP | Rows | Native Content-Range | Raw / compact bytes | Approximate tokens | Elapsed ms |
| --- | --- | --- | --- | --- | --- | --- |
| C01 | 200 | 6 | `0-5/6` | 21090 / 21075 | 5269 | 393.891 |
| C02 | 200 | 8 | `0-7/8` | 2154 / 2133 | 534 | 253.194 |
| C03 | 200 | 3 | `0-2/3` | 1024 / 1018 | 255 | 263.000 |
| C04 | 200 | 3 | `0-2/3` | 1124 / 1118 | 280 | 249.935 |
| C05 | 200 | 3 | `0-2/3` | 1129 / 1123 | 281 | 242.491 |
| C06 | 206 | 5 | `0-4/79` | 1878 / 1866 | 467 | 247.960 |
| C07 | 206 | 5 | `0-4/7` | 1316 / 1304 | 326 | 240.236 |
| C08 | 206 | 5 | `0-4/49` | 1472 / 1460 | 365 | 245.905 |
| C09 | 206 | 5 | `0-4/10` | 3999 / 3967 | 992 | 253.918 |
| C10 | 206 | 5 | `0-4/105` | 4395 / 4338 | 1085 | 254.953 |
| C11 | 206 | 5 | `0-4/105` | 4395 / 4338 | 1085 | 253.267 |
| C12 | 206 | 5 | `0-4/1316` | 7331 / 7259 | 1815 | 255.214 |
| C13 | 200 | 4 | `0-3/4` | 4996 / 4971 | 1243 | 266.819 |
| C14 | 206 | 5 | `0-4/165` | 2954 / 2927 | 732 | 251.604 |
| C15 | 206 | 5 | `5-9/10` | 4665 / 4633 | 1159 | 253.509 |
| C16 | 200 | 0 | `*/0` | 2 / 2 | 1 | 258.210 |
| C17 | 200 | 1 | `0-0/1` | 1198 / 1192 | 298 | 261.041 |
| C18 | 200 | 1 | `0-0/1` | 1119 / 1113 | 279 | 257.011 |
| C19 | 200 | 1 | `0-0/1` | 1158 / 1152 | 288 | 301.574 |
| C20 | 200 | 1 | `0-0/1` | 1131 / 1127 | 282 | 246.186 |
| C21 | 200 | 1 | `0-0/1` | 233 / 230 | 58 | 260.371 |
| C22 | 200 | 1 | `0-0/1` | 306 / 299 | 75 | 249.766 |
| C23 | 206 | 5 | `0-4/7` | 1778 / 1727 | 432 | 259.159 |
| C24 | 206 | 5 | `0-4/301` | 891 / 828 | 207 | 422.249 |
| C25 | 200 | 0 | `*/0` | 2 / 2 | 1 | 247.191 |
| C26 | 200 | 1 | `0-0/1` | 1249 / 1232 | 308 | 246.456 |
| C27 | 200 | 0 | `*/0` | 2 / 2 | 1 | 248.783 |
| C28 | 200 | 2 | `0-1/2` | 696 / 693 | 174 | 246.337 |
| P01 | 200 | 83 | `-` | 99050 / 99050 | 24763 | 468.322 |
| P02 | 200 | 27 | `-` | 43211 / 43211 | 10803 | 233.638 |
| P03 | 200 | 9 | `-` | 13458 / 13458 | 3365 | 312.036 |
| P04 | 200 | 122 | `-` | 223077 / 223077 | 55770 | 536.702 |
| P05 | 200 | 38 | `-` | 40995 / 40995 | 10249 | 364.827 |
| P06 | 200 | 21 | `-` | 24860 / 24860 | 6215 | 339.958 |
| P07 | 200 | 10 | `-` | 1544 / 1544 | 386 | 262.039 |
| P08 | 200 | 5 | `-` | 703 / 703 | 176 | 260.954 |
| P09 | 200 | 3 | `-` | 3408 / 3408 | 852 | 285.437 |
| P10 | 200 | 83 | `-` | 448146 / 448146 | 112037 | 580.104 |

A dash means no independent total or pagination field was supplied. ClinPGx array length is an observed collection count, not an asserted server-wide total. C09/C15 contain ten distinct recommendation IDs across two observed nonoverlapping DPYD pages. No `order` parameter was requested in that original run. W02–W05 below supply the later same-run ordered witnesses.

### Public-tier identities, fields and scope

| Capture | Observed native contract | Consequence for the proposal |
| --- | --- | --- |
| C01 gene | Six genes have HGNC, NCBI, Ensembl and ClinPGx IDs, reference accessions, native version and substantial method/usage notes. DPYD/CYP2D6 lookup method is `ACTIVITY_SCORE`; the other four use `PHENOTYPE`. | Preserve method and record version. Do not print whole methods in a default header or call a gene version a dataset release. |
| C02 drug | Exact native drug keys include capecitabine `RxNorm:194000`, fluorouracil `RxNorm:4492`, mercaptopurine `RxNorm:103`, thioguanine `RxNorm:10485`, tamoxifen `RxNorm:10324`, irinotecan `RxNorm:51499`, rasburicase `RxNorm:283821`, ondansetron `RxNorm:26225`. Rows supply ClinPGx/DrugBank IDs, ATC arrays and nullable UMLS/flowchart/guideline fields. | Preserve namespace and multiplicity. Irinotecan has null guideline/flowchart. No native PA/DrugBank link automatically selects a MyChem or ChEMBL document. |
| C03/C28 pairs | Pair ID, drug ID, gene, guideline URL, PMID list, CPIC/ClinPGx levels, testing label, provisional flag and `usedforrecommendation`. Level-A filter yields the exact two DPYD A pairs. Tegafur has level C and native `usedforrecommendation: No`. | Level filter has a positive native witness. Keep no-action separate from missing guideline/evidence. Testing can be null. |
| C07 UGT1A1 pairs | Irinotecan pair `110106` is level A with null guideline name/URL. Other pairs carry literal level `B/C`. | Do not close the level enum to A/B/C/D only. Pair actionability does not establish a CPIC irinotecan recommendation. |
| C09–C15 recommendation views | Recommendation IDs, complete lookup keys, implication maps, text, classification, phenotype/activity maps, population and comments. TPMT and NUDT15 first pages are byte-identical and retain both genes; CYP2D6 rows can include CYP2C19. All observed populations are `general`. | No drug ID, guideline integer ID, record date/version, publication IDs, separate allele-status field or structured dose field appears in this view. No pair-specific recommendation join or population filter is proved. Preserve source names/URLs; never join by name. |
| C13 UGT1A1 recommendation | Four rows concern atazanavir only. | They cannot answer the irinotecan questions by gene association. The queried complete UGT1A1 collection supplies no irinotecan recommendation. |
| C16/C25 | DPYD literal `*1/*2A` diplotype and `*2A` definition are successful complete zero replies. | Preserve exact queries and emptiness. Do not repair notation or infer absent clinical relevance. No sequence-location body shape was captured. |
| C17–C20 diplotypes | TPMT `*1/*3A`, NUDT15 `*1/*3`, CYP2D6 `*1/*4` report intermediate; UGT1A1 `*28/*28` reports poor. Source fields include two functions/activity strings, total activity string, description, `diplotypekey`, result, EHR priority, consultation text and result lookup key. | No numeric diplotype ID, record version or allele reference IDs are supplied. The exact gene/expression tuple identifies the lookup. Preserve literal `n/a`; no local numeric conversion or calculation. |
| C21–C23 guideline summaries | Only `guideline_name`, `guideline_url`, drug-name array and gene symbol/URL array. | This tier has no publication records, guideline integer ID or version. Pair PMIDs remain pair facts, without a name-based join. |
| C24 frequency | Five of 301 aggregate rows contain gene, allele name, population group, subject count and source average/weighted/min/max frequency. | No per-study ID, numerator, denominator-unit proof or release. Do not invent study rows or patient ancestry. Product frequency stays deferred. |
| C26 copy allele | CYP2D6 `*2x2`, native ID `779691`, definition `777938`, version `335`, activity string `2.0`, distinct nullable function fields, citations, frequency map and `inferredfrequency: false`. | Preserve copy-bearing source name; no standalone numeric copy field was supplied. `functionalstatus` can be null while clinical function exists. |
| C27/P09 | CPIC TP53 pairs are empty; ClinPGx TP53 annotations have three records. | Empty is source/endpoint-local. Never suppress one source because another returned zero. |
| P01–P06 clinical min | Numeric IDs and PA accession IDs, evidence terms, source location/build/reference information, gene and chemical arrays, chemical-combination logic, guideline/label links and native types. | No disease array, genotype phenotype text, history or recommendation body. Never extract missing context from display name. A position alone is not a complete genomic allele. |
| P07/P08 min references | Only class, PA identity, display name and cross-reference array. | Not full guideline/label recommendations, dates or structured gene/drug context. |
| P10 clinical base | Same ordered 83 DPYD IDs as min, plus allele-phenotype rows, conflicts, history, pediatric/override fields, related alleles/diseases and score details. | A larger tier is required for structured disease/genotype context. Still no native server pagination is proved. Source scores are not a local ranking rule. |

P10 confirms native annotation PA166134421 for rs3918290/fluorouracil Level 1A Toxicity with a structured Neoplasms relation. PA166224841 is a separate Level 1A Other record with an empty disease array. Do not merge them. P10's NM_000110.4-related allele expression stays distinct from the historical paper's NM_000110.3 reference version. C09/C15 have no score-1.75 lookup key; the reviewed July notice explicitly warns that tables await updating. No missing score becomes a normal phenotype or a locally calculated new dose.

Actual current fields `clinpgxid` and `clinpgxlevel` confirm the migration; legacy field spellings are absent. M06 contains the complete dynamic OpenAPI document. It documents richer `recommendation` table fields such as `drugid`, `guidelineid`, `allelestatus`, `version` and dose-related fields, which the original data run did not capture. W06 below supplies a later native table and drug-selector witness. Documentation alone cannot accept those types or selector semantics. Its `info.version: 12.0.2` is PostgREST software metadata, not a CPIC dataset release.

M01 and M02 return identical JavaScript shells, so they retain no actual policy/footer terms. M03 retains the API's policy link and rate limit. M04/M05 are documentation shells; no script follow-up ran. Official footer text read through the web reader remains separate evidence. Complete current policy wording and CPIC-specific scope remain unresolved; HTTP success grants no reuse license.

### Follow-up native witnesses

Ticket 0163 executed one bounded job, `j12785`, from pushed landed revision `a9d7cfbecebfe9afc5f29b3692cf2607d01272bf` on 2026-10-25. Seven fixed keyless GETs ran once in the order below, with one second between admissions, a 20-second total deadline per request and a 4 MiB body bound. All bodies completed. No retry, redirect, query repair, fallback, script crawl, cookie or credential was used. The eighth permitted request remained unused. The retained M06 OpenAPI marks `pairid`, `recommendationid` and `recommendation.id` as primary keys and documents `drugid` on `recommendation`, not `recommendation_view`; those facts fixed the ledger before execution.

The durable archive is `data/biomcp-redux/0163-pgx-native-witnesses/j12785-pgx-witnesses.tar.gz` on Yellow, 17,394 bytes, SHA-256 `4375d382791955e857a748b8b2c94f4be104a519e5001048bbe1c9b707560de4`. Its root `2290-pgx-native-witnesses` retains the exact planned and actual URLs, raw bodies, safe response headers, timestamps, elapsed times, hashes, recipe, job log and mechanical witness analysis. Independent verification on Yellow checked the archive hash and every one of its 35 manifest files. This reference applies to every W row below; raw source content remains outside Git.

| Row | Exact request URL | Result and capability consequence |
| --- | --- | --- |
| W01 | `https://www.clinpgx.org/page/dataUsagePolicy` | HTTP 200, 9,820 bytes, JavaScript shell. Current policy text remains unproved. The `noncommercial` category continues to rest on the PM ruling, with this limitation explicit. No script follow-up is authorized. |
| W02 | `https://api.cpicpgx.org/v1/pair_view?select=%2A&genesymbol=eq.DPYD&order=pairid.asc&limit=2&offset=0` | HTTP 206, 696 bytes, Content-Range `0-1/3`, pair IDs 110086 and 110099. |
| W03 | `https://api.cpicpgx.org/v1/pair_view?select=%2A&genesymbol=eq.DPYD&order=pairid.asc&limit=2&offset=2` | HTTP 206, 326 bytes, Content-Range `2-2/3`, pair ID 110398. Together W02/W03 prove the bounded native ascending unique-key walk of all three DPYD pairs without duplicates or gaps. |
| W04 | `https://api.cpicpgx.org/v1/recommendation_view?select=%2A&lookupkey-%3E%3EDPYD=not.is.null&order=recommendationid.asc&limit=5&offset=0` | HTTP 206, 4,332 bytes, Content-Range `0-4/10`, recommendation IDs 8480742–8480746. |
| W05 | `https://api.cpicpgx.org/v1/recommendation_view?select=%2A&lookupkey-%3E%3EDPYD=not.is.null&order=recommendationid.asc&limit=5&offset=5` | HTTP 206, 4,332 bytes, Content-Range `5-9/10`, IDs 8480747–8480751. Together W04/W05 prove the bounded native ascending unique-key walk of all ten DPYD recommendations without duplicates or gaps. |
| W06 | `https://api.cpicpgx.org/v1/recommendation?select=%2A&lookupkey-%3E%3ECYP2D6=not.is.null&drugid=eq.RxNorm%3A10324&order=id.asc&limit=5&offset=0` | HTTP 206, 2,920 bytes, Content-Range `0-4/22`. All five rows carry exact drug ID `RxNorm:10324`, a CYP2D6 lookup key, guideline ID 100415 and record version 1; IDs are 8481460–8481464. This proves the native gene-plus-drug selector on this table. It does not prove a `drugid` selector on the view or a local name filter. |
| W07 | `https://api.clinpgx.org/v1/data/clinicalAnnotation/PA166134421?view=base` | HTTP 200, 22,406 bytes. The single object at `data` returns `accessionId: PA166134421`, equal to the requested accession, and numeric ID 827843617 separately. This proves the exact clinical single-record base get and its candidate disambiguation route. Guideline and label detail remain unproved. |

The ordered page witnesses establish a same-run bounded walk, not an immutable snapshot across dates or concurrent source changes. Later implementations preserve these source orders and totals. W06 unlocks the native recommendation drug selector for an adapter using the captured `recommendation` table. That table has `id` rather than the view's `recommendationid`, and supplies `drugid`, `guidelineid`, `allelestatus`, `version` and the three native prescribing-information booleans. Preserve complete returned rows and missingness. Do not splice drug names, guideline names or URLs from a separate view by name. The existing view contract stays distinct; its absent fields remain absent. The captured five-row filtered page leaves seventeen rows unexamined and does not answer Q18–Q23 or establish clinical completeness.

W07 permits the proposed clinical annotation detail schema and exact-accession command to proceed through implementation and review. Compare the returned accession before cache admission. Preserve the entire base object and keep its numeric ID separate. This witness does not establish server pagination, other annotation kinds, allele equivalence or a new cross-entity join. No clinical answer was calculated. The implemented envelope sizes, reviewed default and CLI/MCP results appear in the product sections below. These results discharge the drug-filter, order and clinical single-get source proof obligations within the scopes stated above. They do not authorize any additional provider request.

### Use and terms

The PM ruled on 2026-10-25 that CPIC and ClinPGx both use `noncommercial`, group `noncommercial`, under the shared [ClinPGx data use policy](https://www.clinpgx.org/page/dataUsagePolicy). Both remain on by default under the existing decision; commercial users can disable the group. The follow-up W01 policy request also returned a JavaScript shell. Current policy text remains unproved; M01/M02/W01 retained shells, not that text, so this design distinguishes the PM's product ruling from an independently verified policy receipt. Current official footer text read through the web reader names Creative Commons Attribution-NonCommercial-ShareAlike 4.0. No paid or commercial permission is assumed.

API access, source database facts, website content, journal manuscripts and third-party components keep their own terms. A keyless success is not a redistribution grant. The [key rule](../../decisions/2026-10-24-keyed-sources-run-only-with-the-user-s-key.md) still prevents calls to a source whose terms require the user's key when none is configured. Neither bounded capture used credentials. Keep terms names and links in source outcomes and health; use the existing disable groups, with no separate PGx policy system.

The source registry name is `clinpgx`; `pharmgkb` remains only a retired spelling that returns the replacement without provider work. CPIC remains `cpic`. Do not register two names for the same API. No licensed curation-team answer or raw source body enters Git.

### Identity and joins

Source identities remain separate. A CPIC gene symbol is a source key; a returned HGNC, NCBI or Ensembl identifier can support a typed gene next step after equality checks. A CPIC drug key is its published source-qualified identifier. Preserve every supplied cross-reference and its namespace. A ClinPGx PA accession is a ClinPGx object identity. It is not the shared `PaId`, which belongs to a different identifier namespace, and it is not a CAid, rsID or universal drug identifier. Use a separate checked ClinPGx accession type, never `VariantId::PaId`. A guideline ID, an annotation ID and a publication PMID refer to different objects.

Named alleles keep gene context, native allele ID, exact source spelling, optional definition ID, copy representation and source version when supplied. Diplotypes keep the entire source expression, gene context, source tuple and original lookup data; the captured view supplies no numeric record identity or allele IDs. Do not invent either. Preserve unknown, uncertain, duplicated and multiple returned mappings. Do not sort two allele strings, infer phase, strip copy suffixes, turn a star allele into one genomic variant, or calculate an activity score.

The source may return phenotype, activity score or allele status as different lookup methods. Keep the declared method and exact source value; never coerce them into a common phenotype string. Preserve the full multi-gene lookup key, drug, population, guideline and every matching recommendation. A TPMT row must not discard its NUDT15 context. A partial key is not a uniquely selected recommendation.

Cross-entity actions require explicit source links in the captured record. Emit `get gene` only with an identifier the gene slice accepts. Emit `get drug ID --source SOURCE` only with a source-supported identity relation proved in that slice. An RxNorm ID with no supported direct drug route remains a cross-reference. A single source-given reference-qualified HGVS or checked allele ID may supply a variant lookup; an rsID may return several alleles. No name comparison links drugs, genes, conditions, profiles or patients.

### Bounded native capture plan

The build coordinator gives this recipe to a separate build delegate. Run it through `pm job` from a pushed full revision. Use a new experiment with an OWNER naming this ticket. Keep all replies, headers, request bodies, actual recording identities, failure logs, recipes and manifests outside Git and archive them on Yellow. The investigator makes no source data API calls itself.

The risk is whether current public CPIC and ClinPGx tiers carry lossless identities, context, lookup mappings and bounded pages needed by this design. The recipe uses GET only, no cookies, no credentials unless the coordinator explicitly supplies an already authorized user key, no downloads of a whole database and no legacy executable. All query parameters are passed through a URL encoder, not interpolated as shell code.

**Approved request ceiling: 38 physical data HTTP attempts: 28 CPIC and 10 ClinPGx.** Every attempt, including a failed connection, consumes its reserved row. No retries, redirects, refreshes, fallback hosts, altered queries or automatic continuation. A redirect is recorded and refused. A missing column or invalid filter is retained as a failed witness; it does not authorize a repair request. A 401/403 stops remaining rows for that source. A 429 stops that source with its headers. Continue independent rows on the other source. Timeout each request after 20 seconds; bound each body at 4 MiB. Space admissions by one second. These are investigation limits, not proposed product configuration.

Each CPIC request sends `Accept: application/json` and `Prefer: count=exact`. Base URL is exactly `https://api.cpicpgx.org/v1/`. Unless a row overrides them, add `select=*`, `limit=5` and `offset=0`. Preserve full source rows. Native ordering is recorded as returned; no unproved sort column is sent. A later product page requires a proved deterministic order or an explicit source snapshot contract.

| Row | Path and exact filter parameters, before URL encoding | Questions/types examined |
| --- | --- | --- |
| C01 | `gene?symbol=in.(DPYD,TPMT,NUDT15,CYP2D6,UGT1A1,G6PD)&limit=10` | Gene IDs, lookup methods and current ClinPGx links. |
| C02 | `drug?name=in.(fluorouracil,capecitabine,mercaptopurine,thioguanine,tamoxifen,irinotecan,rasburicase,ondansetron)&limit=10` | Native drug IDs and explicit cross-references. Literal source search; not identity resolution. |
| C03–C08 | `pair_view?genesymbol=eq.GENE`, one each in C01 gene order | Guideline presence, pair identities, CPIC and ClinPGx levels, testing labels and empty/absent distinctions. |
| C09–C14 | `recommendation_view?lookupkey->>GENE=not.is.null`, one each in C01 gene order | All lookup-key genes, populations, recommendations, comments, activity scores, source row IDs and source date/version. No clinical selection. |
| C15 | `recommendation_view?lookupkey->>DPYD=not.is.null&offset=5` | Independent second-page witness. No automatic page walk. |
| C16 | `diplotype?genesymbol=eq.DPYD&diplotype=eq.*1/*2A` | Native exact expression; zero is a literal lookup result, not a rejected guideline. |
| C17 | `diplotype?genesymbol=eq.TPMT&diplotype=eq.*1/*3A` | Source-reported mapping and native allele references. |
| C18 | `diplotype?genesymbol=eq.NUDT15&diplotype=eq.*1/*3` | Independent gene result and compound-key context. |
| C19 | `diplotype?genesymbol=eq.CYP2D6&diplotype=eq.*1/*4` | Source-reported activity/phenotype and copy-sensitive expression. |
| C20 | `diplotype?genesymbol=eq.UGT1A1&diplotype=eq.*28/*28` | Native mapping without assuming an irinotecan recommendation. |
| C21 | `guideline_summary_view?genes=cs.[{"symbol":"DPYD"}]` | Publication links, logical guideline vs revision and current entity ID. |
| C22 | `guideline_summary_view?genes=cs.[{"symbol":"TPMT"}]` | Multi-gene guideline identity and supporting citations. |
| C23 | `guideline_summary_view?genes=cs.[{"symbol":"CYP2D6"}]` | Multiple drug guidelines and source order. |
| C24 | `population_frequency_view?genesymbol=eq.TPMT` | Native study/group denominators, units, labels and any source aggregation. |
| C25 | `allele_definition?genesymbol=eq.DPYD&name=eq.*2A&select=*,allele_location_value(*,sequence_location(*))` | Definition-to-location relation, absence, reference accession/build and rsID multiplicity. |
| C26 | `allele?genesymbol=eq.CYP2D6&name=eq.*2x2` | Copy-number-sensitive source identity; do not substitute `*2`. |
| C27 | `pair_view?genesymbol=eq.TP53` | Wrong-gene routing regression: actual CPIC pair result is zero; P09 remains independently positive. |
| C28 | `pair_view?genesymbol=eq.DPYD&cpiclevel=eq.A` | Known-positive native level filter witness before exposing the option. |

ClinPGx base is exactly `https://api.clinpgx.org/v1/`. Requests send `Accept: application/json`. Rows P01–P06 query `data/clinicalAnnotation?location.genes.symbol=GENE&view=min` in C01 gene order. P07 queries `data/guidelineAnnotation?relatedGenes.symbol=DPYD&view=min`. P08 queries `data/labelAnnotation?relatedChemicals.name=irinotecan&view=min`. P09 queries `data/clinicalAnnotation?location.genes.symbol=TP53&view=min`. P10 queries `data/clinicalAnnotation?location.genes.symbol=DPYD&view=base`. These exact legacy request forms were executed. The findings above distinguish their observed tiers; `view=base` is not a universal full-detail guarantee. No client-side truncation proves source pagination. If this API returns an entire collection, record its body and count within the body bound, do not issue an unbounded follow-up, and keep a public product page unsupported until a bounded source or checked local collection recipe is reviewed.

Record each physical attempt in an append-only ledger with row ID, exact URL and method, safe headers, UTC start/end, elapsed milliseconds, response status/content type, source response request ID if present, byte length, SHA-256, retrieval date, redirects refused, complete/truncated/read-failed state, parse outcome, top-level count, collection totals and continuation fields. Record native identity paths, absent/null/empty distinctions, nested row multiplicities and all scopes. Never invent a request ID, hash, count or date. Do not store secrets in the ledger. The recorder must begin in an empty directory and capture actual requests, not reconstruct identities later.

The delegate reports raw compact JSON bytes and a consistently labeled approximate token count, separate from hypothetical 1.0 bytes. Do not compare complete upstream rows to compact command output as a product regression. No 1.0 PGx command exists at the starting revision, so its output, request cost and cold/warm timings remain unmeasured. Documentation/policy retrieval has a separately approved fixed six-request allowance: M01 `https://www.clinpgx.org/page/dataUsagePolicy`, M02 `https://www.clinpgx.org/gene/PA142672008`, M03 `https://api.clinpgx.org/`, M04 `https://api.clinpgx.org/swagger/`, M05 `https://api.cpicpgx.org/`, M06 `https://api.cpicpgx.org/v1/`. Bound M01–M05 at 1 MiB and M06 at 4 MiB. Use the same deadline and admission spacing; no redirects, retries, credentials, script-link crawling or inferred specification paths. A shell or failure stays a limitation. Preserve exact text/bytes and distinguish website/footer evidence from API data rights.

## 3. What would delight the agent

1. **First question.** “Which guidance does CPIC track for this declared gene?” `search pgx --gene DPYD --source cpic` returns a bounded native pair page. No bare input is classified as gene or drug.
2. **Header.** Exact source, gene, pair ID, drug ID/name and native guideline/level/testing facts let an agent choose a follow-up. CPIC level, ClinPGx evidence level and recommendation strength stay independent.
3. **Guidance.** `get pgx --gene DPYD recommendations --source cpic` returns source rows with native drug names and whole lookup keys. It never locally filters by drug name or silently claims a selected patient recommendation.
4. **Diplotype.** `get pgx --gene TPMT diplotype --diplotype '*1/*3A' --source cpic` reports the source's complete lookup result. No allele calling, phase resolution or score calculation occurs.
5. **Annotations.** One ClinPGx kind is disclosed at a time. Min pages let the agent choose an exact accession; the W07 exact single-record base-get contract permits clinical detail. Missing disease fields in min never become disease absence.
6. **Disagreement and dates.** Keep source records independent, complete contexts and source versions distinct from retrieval date. A website notice absent from API tables remains an explicit limitation, never a local correction.
7. **Failure.** A successful zero, missing key, disabled source, unavailable transport, identity rejection and unsupported field remain different. None implies a normal phenotype or safe dose.
8. **Cost.** Start with a bounded page and one source. Measured product envelopes and local timings appear below. Provider latency remains the original capture measurement; contained 0.9 comparison remains separate.
9. **Stop early.** Pair facts can answer guideline presence; a declared diplotype can answer a source-reported label. Neither is a patient treatment choice.
10. **Trust.** Return exact native IDs, source links, original scope, terms and available dates. Every next step keeps the selectors and selection scope that produced it.

## 4. Reviewed types, schemas and commands

The PM settled these interface choices on 2026-10-25. Domain types preserve source facts without I/O. Consumer types own queries, header/card/page disclosure, envelope outcomes and typed actions. Schemas and help derive from one accepted type definition. This design changes no product code or schema.

### Domain and identity boundaries

`PgxPair` holds exact source pair ID, gene, drug ID/name and optional guideline URL, PMID list, CPIC/ClinPGx levels, testing, provisional and recommendation-use facts. Preserve compound labels such as `B/C`. `PgxGeneIdentity` holds source symbol, exact cross-references, lookup method and native record version. A record version is not a dataset release. `PgxGuidelineReference` holds the captured summary's name, URL, ordered drug names and gene symbol/URL objects. It supplies no invented publication ID or version.

`PgxRecommendation` keeps recommendation ID, complete multi-gene lookup key, drug name, guideline name/URL, implications, full source recommendation, classification, phenotype/activity maps, population and comments. The captured view has no drug ID, separate allele-status field, version or date. W04/W05 prove ordered DPYD view paging. W06 separately proves the native `recommendation` table with `id`, `drugid`, `guidelineid`, `allelestatus`, `version` and prescribing-information booleans. Keep that table shape distinct from the view; it has no captured drug name or guideline name/URL. The optional exact `--drug-id` recommendation selector uses this native table, with the declared gene retained. No local drug-name filter or name-based enrichment is permitted.

`PgxNamedAllele` preserves exact gene/name, optional native allele and definition IDs, source version, distinct function statuses and source activity strings. `PgxDiplotypeResult` preserves source expression, gene, complete `diplotypekey` counts, both function/activity values, source result, consultation/EHR text and result lookup key. The source view has no numeric row ID or allele IDs. Preserve `n/a`, uncertainty and multiple results. Do not strip copy notation or compute function, phase or phenotype.

Star alleles and diplotypes stay native PGx records. They produce no `Statement` and no `statement_unmapped` flag. No ClinPGx evidence level maps to a shared proposition. This slice adds no shared `Subject` kind. Source-given genomic identifiers can provide exact variant lookup actions only where the target accepts that identifier and the source establishes its scope; a single rsID is not a whole haplotype. ClinPGx accessions use a distinct namespace and never the shared `PaId`.

`PgxAnnotationHeader` preserves every captured min-tier row field, including native numeric/PA identities, evidence term, location and native gene/chemical/link arrays. `PgxAnnotationDetail` retains all captured base-tier fields under the W07 exact-accession direct-get contract. Clinical, guideline and label kinds remain separate. Min guideline/label rows are references, not full guidance. Missing min-tier disease, history or genotype text is unsupported disclosure, not a negative fact.

### Concrete JSON body shapes and fields

These are design shapes, not executed outputs. Common envelopes reuse architecture's `schema`, `status`, exact `query`, `result`, applicable `page`, source outcomes/terms, problems and typed `next`. Offline projection sizes count the bodies below only; they do not claim measured full envelopes. Names inside native rows retain the native spelling and complete fields. Do not duplicate a native row into a second summary within the same response.

Header and compact gene card:

```json
{"header":{"source":"cpic","gene":"TPMT"},"identity":{"symbol":"TPMT","hgncid":"HGNC:12014","ncbiid":"7172","ensemblid":"ENSG00000137364","clinpgxid":"PA356","lookupmethod":"PHENOTYPE","version":448}}
```

The illustrated card lists only source-supplied identity fields. Implemented output must read their actual captured values, never this illustration as a fixture. The default card is gene identity, not the deferred exact pair card. Bulky source methods and frequencies do not appear in it.

Every bounded collection body follows this shape:

```json
{"header":{"source":"cpic","gene":"DPYD"},"items":[],"page":{"offset":0,"returned":0,"total":0,"total_kind":"source"}}
```

The empty values demonstrate fields only; native fixtures supply actual counts and rows. Pair items contain the entire C03–C08/C28 row. Gene-keyed recommendation view items contain the entire C09–C15 row. Exact drug-ID recommendation items contain the distinct entire W06 table row, preserving its source shape rather than filling view-only fields. Guideline-reference items contain the entire C21–C23 row. A ClinPGx collection header adds `annotationkind`; its page uses `total_kind: "observed"`, the received collection length, and ordinary checked local page bounds. It does not claim native server total or stable provider pagination. An explicitly bounded retained collection supplies all later local pages in observed order. No row or context field is dropped to satisfy output cost.

Diplotype result:

```json
{"header":{"source":"cpic","gene":"TPMT"},"diplotypes":[]}
```

The array carries every complete returned row. Zero, one and multiple rows remain distinct; missing source row IDs stay absent. `--diplotype` is echoed unchanged in the outer query.

Proposed single annotation detail:

```json
{"header":{"source":"clinpgx","id":"PA166134421"},"annotation":{}}
```

The annotation object retains the full captured base-tier row. The existing offline size projection uses P10. W07 separately proves the native direct-get request and returned-accession equality; it does not measure a product envelope. An annotation numeric ID and its PA accession remain separate fields.

Current pair search uses `biomcp.pgx-search/2`, its embedded query response uses `biomcp.pgx_page/2`, and the shared request uses `biodata.request/8`. These versions narrow the search source schema to the CPIC-only behavior the checked request already enforces; get keeps both independently scoped sources. The pair envelope changes only its equal-length schema identity. Native source rows and query values remain unchanged. The other schemas are `biomcp.pgx_card/1`, `biomcp.pgx_guidelines/1`, `biomcp.pgx_recommendations/1`, `biomcp.pgx_diplotype/1` and `biomcp.pgx_annotations/1`. W07 also permits `biomcp.pgx_annotation/1` and its exact clinical single-annotation command to proceed through implementation and review. No frequency schema is approved. The source adapters own native decoding and checked identity; query composes disclosure without clinical inference. No new client, cache or variant parser is introduced.

### Command tree and per-command next actions

`--source cpic` is required for gene search, the gene card, recommendations, guidelines and diplotype. The annotations section requires `--source clinpgx`. An omitted source or any other source/section pairing is a usage error before I/O. In particular, `get pgx --gene X --source clinpgx` without a section is refused. No source is inferred from a section or input.

| Command | Body and proven scope | Next actions |
| --- | --- | --- |
| `search pgx --gene SYMBOL --source cpic` | Native pair page from C03–C08. `--cpic-level` has C28 positive proof. `--limit`/`--offset` select source pages; W02/W03 prove `pairid.asc` for the bounded DPYD walk. No positional search. | Page first when present, then the exact gene card, then the gene-keyed `recommendations` section. Page retains gene, source, level and paging selectors. No disambiguate step while exact pair get remains deferred. |
| `get pgx --gene SYMBOL --source cpic` | Compact source gene identity card from C01. | Exactly the available section steps in this order: `recommendations --source cpic`, `guidelines --source cpic`, then `annotations --source clinpgx`, preserving the declared gene. No help, diplotype lookup or frequency step. |
| `get pgx --gene SYMBOL recommendations --source cpic` | Complete gene-keyed recommendation view rows from C09–C15, native drug names, complete lookup keys and populations. Optional exact `--drug-id` uses the distinct W06 native table contract. `--section-limit`/`--section-offset` retain every selector. W04/W05 prove `recommendationid.asc` for DPYD view pages; W06 captures `id.asc` for the filtered table page. No local drug filter. | Page first when more rows remain; exact source-supported gene pivots only, then at most three useful actions. Do not emit a drug-ID action from native drug name alone. |
| `get pgx --gene SYMBOL guidelines --source cpic` | Native summary-reference page from C21–C23, with section paging. | Exact source-returned linked genes pivot in native order; page follows; source publication actions only when that exact record actually supplies a PMID. No invented article action from guideline name. |
| `get pgx --gene TPMT diplotype --diplotype '*1/*3A' --source cpic` | Full native source lookup from C17–C20; C16 preserves literal DPYD zero. | At most five steps. A gene-keyed recommendation section remains available; never emit a uniquely selected recommendation from an incomplete multi-gene key. Exact alternate candidate selections require real source identities. |
| `get pgx --gene SYMBOL annotations --kind clinical --source clinpgx` | Local page from the captured min collection, observed total and native order. `--kind clinical|guideline|label`, default clinical. Section limit/offset are local collection selectors. | One `disambiguate` single-annotation get per clinical candidate by exact PA accession under W07, then page, then at most three useful pivots. Kind/source/gene and collection identity persist. Guideline/label kinds expose only proved references until richer detail is captured. |
| `get pgx --annotation-id PA… --source clinpgx` | Exact clinical single base annotation under W07; check returned accession before cache admission. | At most five exact source gene/drug/variant actions and available section actions. Names never become identifiers. |

These rules apply architecture 6.10. The pair-search exception above emits page, gene card and recommendations without pair disambiguation. The gene card emits only its three named section steps. Help and diplotype lookup are not next steps. One pure PGx `next` function returns typed requests; presentation spells the arguments. Selectable pages emit every candidate disambiguation before the page action, bounded by checked page size, then at most three extras. Association pages emit one pivot per distinct linked record in native order before page and extras. Other responses have at most five actions. `fresh` retries the same request after failed/stale source outcome; disabled/missing-key recovery uses existing shared actions. No frequency action appears anywhere. A deferred selector or unsupported detail command does not appear as an executable next until its source proof and implementation exist.

### Deferred selectors and proof obligations

Exact pair card, `--population`, `--lookup-key` and `--pgx-testing` remain unproved and deferred. W06 permits `--drug-id` only on the native recommendation table, with the explicit gene selector; drug-ID search and an exact drug or pair card remain deferred. `--pgx-testing` requires a native positive capture before implementation. No positional search or drug-text search is proposed; the question bank does not need a drug-name search. W02–W06 supply the bounded native order and recommendation drug-ID witnesses without local drug filtering. W07 supplies the exact clinical single base get. W01 leaves current policy text unproved; the PM category ruling remains authoritative. The implemented cases and complete envelope measurements appear below. No remaining gap authorizes expansion of the provider ledger.

### Case map and failure boundaries

| Surface | Harvested cases and required assertion |
| --- | --- |
| Gene search, `--gene`, `--source` | Q01/Q06/Q30: exact pair/gene/drug identities, separate level namespaces, native no-action and missing guideline. Zero source switching or name inference. |
| `--cpic-level` | C28/Q30: exactly source-proved positive native selector, compound other levels preserved in unfiltered pages. |
| Search `--limit`/`--offset` | Q01/Q30: complete native rows, source total and retained selectors; W02/W03 prove the same-run ordered DPYD walk; preserve `pairid.asc` and every selector without claiming an immutable snapshot. |
| Gene card | C01/Q01: source-given IDs/method/version only; absent relations stay absent. No pair or frequency claim. |
| Recommendations | Q03/Q04/Q12–Q16/Q18–Q27: complete view drug names, all lookup genes, original scope/comments and no locally repaired notice/table discrepancy. W06 table rows retain exact drug IDs and native missing names. The drug selector and every page selector persist in next. Unsampled questions retain their unanswered status. |
| Guidelines | Q01/Q17/Q30: native URL/name and ordered arrays, without invented publication metadata. |
| Diplotype and `--diplotype` | Q02/Q07/Q08/Q24/Q31/Q32: exact expression, all source mappings, native result versus literal zero, no phase or score calculation. |
| Annotation kinds | Q28/Q29/Q33: independent clinical/guideline/label collections, default clinical, observed total and all native context fields. No merging Toxicity and Other records. |
| `--section-limit`/`--section-offset` | C09/C15 and ClinPGx retained collections: every row remains reachable with exact source/query/collection selectors. No hidden filter or lost multi-gene context. |
| Single clinical annotation selector | Q28/W07: disambiguation by native accession and returned-identity equality before cache admission; other annotation kinds remain reference-only. |
| Source retirement | `pharmgkb` spelling returns `clinpgx` replacement with zero provider requests. |
| Failures, empty, shared options | Q33: source-local zero versus unavailable, 429/500/timeout/broken JSON/HTTP-200 error/identity mismatch, required key, disabled source and warm-cache exclusion. Synthetic failure controls remain explicitly authored. CLI/MCP equality and byte ceilings are implementation proof. |

### Executable case and source-ID inventory

The native capture IDs below identify source observations, not a claim that each observation is a new standalone product case. The 33 Q rows retain their capture-backed statuses above. Literature answers, sampled pages, source inventories and executable product checks remain separate.

| Design IDs or boundary | Existing executable owner or explicit limitation |
| --- | --- |
| C01 gene identities; C02 gene symbols | CPIC `card` scenario and external G01/G02 exercise the checked identity. The original six-row C01 and eight-row C02 captures remain source inventories; no six-card product measurement is claimed. |
| C03–C08 pairs; C27 zero; C28 native level | `runner/cpic.rs` pairs/empty; external CPIC P01–P06. Native source counts and exact native level filtering remain separate from clinical guideline presence. |
| C09–C15 recommendations | `runner/cpic.rs` view/table/missing-second-gene and page walk; external R01–R06. C10–C14 sampled pages do not answer the unsampled drug questions. |
| C16–C20 and C25 declared diplotypes | `runner/cpic.rs` diplotype/empty/truncated-diplotype; external D01–D05. Literal zeros remain zeros; no phase conversion. |
| C21–C23 guidelines | `runner/cpic.rs` guidelines/guidelines-many; external L01–L03. Every source-linked gene and page survives. |
| C24 frequency aggregate | Retained source inventory only. Frequency page is deferred, with fixed guidance in `runner/pgx_retirements.rs`. |
| C26 allele source inventory | Retained source observation only; no allele lookup command is implemented or implied. |
| P01–P06 clinical min; P09 TP53 | `runner/clinpgx.rs` clinical/default/wide and retained external DPYD walks at limits 5/3/1 plus TP53. Other gene captures remain source inventories, not unexecuted product proof. |
| P07 guideline and P08 label min | `runner/clinpgx.rs` guideline/label; retained external DPYD guideline and UGT1A1 label walks. The original P08 irinotecan-name capture is not the later UGT1A1 gene-selector proof. |
| P10 base collection; W07 exact accession | `runner/clinpgx.rs` annotation and wrong-accession; external exact PA166134421 get. Original P10's 83 base rows are not 83 executed exact-accession gets. |
| W01 policy; W02/W03 pair order; W04/W05 view order; W06 filtered table | W01 retains the PM terms ruling but does not prove current policy text. CPIC pairs/view/table and external ordered commands execute the witnessed selectors. No extra provider request was made for these docs. |
| Q16, Q31, Q32, Q33 authored controls | `runner/cpic.rs` missing-second-gene/empty; `runner/clinpgx.rs` empty; `runner/pgx_retirements.rs` phase refusals and fixed migration. These controls do not promote any unanswered native Q row. |
| Transport, identity and cache rejection | `runner/cpic.rs`, `runner/clinpgx/failures.rs`; disabled cache/replay paths and snapshot binding live in their owning runner cases. Each source retains terms and local failure state. |
| Help, executable examples and discovery | `tests/cases/pgx/help.yaml`, `examples.yaml`, `policy.yaml`; `runner/help_examples.rs` executes the four embedded examples; `runner/protocol.rs` bounds MCP discovery. |
| Public data and checked request schemas | `crates/biomcp-types/tests/pgx_contracts.rs` (including biodata contracts) and the existing schema registry. These are public library contracts, not substitutes for command cases. |
| Source registry and policy | `runner/pgx_registry.rs` and `runner/pgx_registry/admission.rs` test source declarations and admission before cached reads. |

The native CPIC cases remain in the verified `j13036` archive named below. ClinPGx native commands, raw CLI/MCP outputs and complete local walks remain in `j13182`; the separate `j13195-focused-schema-and-help.tar.gz` archive validates 155 retained CLI envelopes against final embedded schemas without replay. Its Yellow location is `data/biomcp-redux/0167-clinpgx-product-proof/`, SHA-256 `0f19e83de7e061d08dcf02f5c7b18d6a6ee2ea0f42638beda40810e6e9dbf5be`. All 11 content hashes and the archive hash were independently checked before unique scratch cleanup. Its help sizes precede the final shortening and are not current help measurements.

### Executable ceilings and help measurements

`j13358` regenerated the schemas from code, executed 83 PGx cases, checked the installed help examples, whole help and MCP discovery, and passed the ten structural ratchets. The case outputs below were measured from that built binary in isolated temporary homes using only authored recordings or offline source-policy failures. The owning cases verify CLI/MCP JSON equality and validate their source labels before comparable sizing. No provider or legacy binary ran. Compact full output includes publisher fields; the runner comparison removes exactly validated `use`, `terms` and `terms_url`. Raw CLI stdout includes its newline. Existing caps stayed unchanged because every output fits.

| YAML case and zero-based step | Full compact bytes | Full stdout bytes | Runner comparison compact bytes | Runner comparison stdout bytes | Existing stdout max_bytes |
| --- | ---: | ---: | ---: | ---: | ---: |
| `pgx/help.yaml` 0 | 1069 | 1070 | 1069 | 1070 | 1189 |
| `pgx/help.yaml` 1 | 2165 | 2166 | 2165 | 2166 | 2178 |
| `pgx/examples.yaml` 0 | 993 | 994 | 810 | 811 | 933 |
| `pgx/examples.yaml` 1 | 1047 | 1048 | 864 | 865 | 995 |
| `pgx/examples.yaml` 2 | 859 | 860 | 676 | 677 | 779 |
| `pgx/examples.yaml` 3 | 970 | 971 | 787 | 788 | 907 |
| `pgx/policy.yaml` 0 | 360 | 361 | 177 | 178 | 205 |
| `pgx/policy.yaml` 1 | 376 | 377 | 193 | 194 | 223 |
| `pgx/policy.yaml` 2 | 415 | 416 | 232 | 233 | 268 |
| `pgx/policy.yaml` 3 | 405 | 406 | 222 | 223 | 257 |
| `pgx/policy.yaml` 4 | 424 | 425 | 241 | 242 | 279 |

Rust cases keep their exact full compact baselines in the existing `runner/pgx_retirements.rs::bounded` table, with `ceil(baseline × 1.15)` enforced for CPIC retrieval, source failures and the ClinPGx success kinds. The added success baselines measured in `j13345` are clinical 1451, default 2711, explicit five-row 4015, guideline 916, label 904 and exact annotation 1235 bytes; source-local zero remains 569. These synthetic sizes do not replace native clinical page or exact-base measurements. The independent `CLINICAL_PAGE_MAX_BYTES` check retains the PM-reviewed native default budget. Fixed usage refusals measure their complete authored error envelope and enforce the same headroom rule. Public registry checks retain the existing shared health/configuration bounds.

Generated help now offers only `cpic` for search and both independent sources for get. Search lists its exact source level and source paging; get lists all four sections, annotation accession, declared diplotype, kind, exact recommendation drug key, section paging and retained collection selectors. Each command embeds exactly two successful case-derived examples. Help explicitly says no phenotype, phase or dose is calculated. The final whole help measures 55935 compact bytes and 55936 stdout bytes. MCP discovery measures 1895 bytes. Both remain inside their existing ceilings; neither cap changes. Detailed source pairing and arguments remain in generated command help rather than duplicating the full help tree in discovery.

### Disclosure and offline byte projections

The default pair search and gene card answer identity/guideline-presence questions. Recommendation prose belongs behind its named section. Min annotation pages expose choices; source base detail stays behind exact selection. Start with three clinical annotation rows and five rows for the other page kinds. Smaller pages never delete a returned row's clinical context.

The separate offline projection recipe uses only the verified j12644 archive, compact UTF-8 JSON with `ensure_ascii=False` and separators `(',', ':')`. It constructs the exact result-body shapes above, keeping complete native rows. Pair and recommendation/guideline pages use captured Content-Range total and requested offset. Diplotypes preserve the whole captured list. Gene cards select the seven named identity fields, exactly as supplied. ClinPGx min arrays are paged five at a time through every retained collection, with observed totals. Each P10 annotation is projected as one design-only base detail body. No provider call, program build, old binary or clinical transform runs.

Each output is bounded at 1 MiB and the full projection set at 20 MiB. Record row identities/provenance, compact bytes, `ceil(bytes/4)` approximate tokens and provisional `ceil(bytes*1.15)` body ceiling; retain recipe/outputs/measurements on Yellow. These are offline design projections, not actual native product outputs, tokenizers or passing case ceilings. The product sections below supersede these historical projections with complete envelope bytes, source outcomes, typed-next cost, CLI/MCP parity and local timings. The later contained 0.9 PGx comparison ticket first proves network containment, then measures the same harvested questions within a separately approved ledger. It does not run in this investigation.

### Measured output budget

This table replaces the historical result-body projections with measured complete product envelopes. CPIC measurements come from the 22-command `j13010` run, with guideline action sizes corrected in `j13081`. ClinPGx measurements come from `j13182` and retain its scope and partial-run qualification below. Values are compact UTF-8 bytes; raw CLI stdout adds its newline. Comparison values remove only validated publisher terms and URL and retain `use`; the ordinary case runner also removes validated `use`. Tokens below are approximate `ceil(full bytes / 4)`, not tokenizer counts. Ceilings use `ceil(bytes × 1.15)` and never turn an old projection into an actual measurement.

| Implemented command or section | Measured full compact bytes | Approximate full tokens | Full compact ceiling | Qualified 0.9 figure |
| --- | ---: | ---: | ---: | --- |
| CPIC gene card, current synthetic card | 859 | 215 | 988 | Not measured: contained same-question comparison belongs to 0098. |
| CPIC pair page, P01–P06 | 640–2099 | 160–525 | 736–2414 | Not measured: 0098 containment and comparator execution deferred. |
| CPIC recommendations, R01–R06 | 3611–7931 | 903–1983 | 4153–9121 | Not measured: 0098 containment and comparator execution deferred. |
| CPIC declared diplotype, D01–D05 | 570–1760 | 143–440 | 656–2024 | No separate legacy helper; no old-binary execution claimed. |
| CPIC guideline references, L01–L03 | 738–2640 | 185–660 | 849–3036 | Not measured: 0098 containment and comparator execution deferred. |
| ClinPGx clinical min, largest three-row DPYD page | 5279 | 1320 | 6071 | Not measured: legacy merged kinds differ; contained comparison deferred to 0098. |
| ClinPGx guideline references | 1512 | 378 | 1739 | Not measured: 0098 containment and comparator execution deferred. |
| ClinPGx UGT1A1 label references | 1609 | 403 | 1851 | Not measured: 0098 containment and comparator execution deferred. |
| ClinPGx exact PA166134421 base annotation | 22785 | 5697 | 26203 | No corresponding legacy exact-accession command; old-binary comparison not run. |

The current synthetic CPIC card includes its ClinPGx annotations action; the historical native G01/G02 measurements below precede that addition. The extra native card stage failed on omitted fixture Content-Range and supplies no current native card result. Its historical timings are not current card timings. The complete base annotation remains about 22 KB. Its native fields are intact. Five-row and one-row clinical alternatives, all 83 DPYD rows, TP53, timings, comparison bytes and independent kind measurements are retained in the detailed product table below. No measurement here asserts coverage of every gene or every source row. The six historical C01 identities and the projected 83 base details are not six measured product cards or 83 executed exact gets.

Historical `j12701` body projections remain outside Git in `data/biomcp-redux/0153-pgx-offline-projections/j12701-pgx-offline-projections.tar.gz` on Yellow, SHA-256 `f28ff861655673ec02d317d8ed31d52eea6d9cae19ea00850def546cec1898f5`. They are design history, not case ceilings or executed product envelopes. Product envelopes are larger than some historical body-only ceilings because they include query, source terms and next actions. The PM reviewed the intact clinical page alternatives and chose three rows; the previously approved CPIC and ClinPGx product ceilings govern the implemented cases. A future oversized source collection must fail its admission bound instead of silently truncating.

### Structural risks

Each adapter defines native fields and identities once; help/schema derive from accepted types. Separate annotation kinds preserve collection scope. Shared transport owns requests, quotas, cache and retries. Checked identity precedes cache admission. The current cache can retain a bounded source collection; use existing checked page/snapshot primitives rather than a second source-specific store. Every projected datum remains attributable to a retained row. Failures use isolated runner fixtures, never live induced errors. No activity calculator, clinical consensus ranker or patient inference engine is justified.

## 0.9 command, option and section accounting

Static inventory uses the legacy checkout at `4050fdc7dbdf4d713d94cf02a39a147b863665be`, especially `src/cli/pgx/mod.rs`, `src/cli/pgx/dispatch.rs`, `src/entities/pgx.rs`, source adapters and `docs/user-guide/pgx.md`. This is source inspection, not executed output, not evidence of a released 0.9 binary's behavior and not a timing or parity claim. The dispositions below describe the shipped tree and the recorded deferrals.

| Legacy surface | Static behavior | Proposed disposition and case need |
| --- | --- | --- |
| `search pgx` | CPIC pair rows; gene/drug search required. | Keep with explicit source, native identities, faithful totals and direct source selectors. |
| `-g`, `--gene` | Gene text uppercased and used as primary query. | Keep explicit gene selector; use the source's supported key grammar, not free-text gene detection. |
| positional search `QUERY` | Alias for gene option; shared conflict checks. | Drop ambiguous positional alias; retirement names `--gene`. |
| `-d`, `--drug` | Source drug-name substring query; combined gene/drug filters locally. | Drop drug-text search from the initial tree because no harvested question needs it. W06 permits exact `--drug-id` only for the recommendation table; drug-ID search remains deferred. No local name join. |
| `--cpic-level` | Local filter after a bounded overfetch. | Keep only after direct native positive filter capture C28. Separate from recommendation strength and ClinPGx level. |
| `--pgx-testing` | Local match over five accepted labels. | Deferred until a native positive selector capture and current vocabulary proof; no hard-coded carry-forward. |
| `--evidence` | Free-text substring of guideline name or CPIC level. | Drop under no-text-heuristics ruling. Return source labels and use exact proved native selectors. |
| search `-l`, `--limit` | Default 10, allowed 1–50; fetches up to five times requested rows, capped at 200; truncates after local filters. | Keep bounded native paging, remove hidden overfetch and source-total/locally-filtered-count ambiguity. The measured pair-search default is five. |
| search `--offset` | Native offset precedes local filtering. | Keep only with a proved native stable page contract and all selectors retained in next. |
| `get pgx QUERY` | Word-shape helper chooses gene or drug; default interactions. | Replace with explicitly declared gene identity card and named sections; old bare form retires. Drug-keyed cards and exact pair cards remain deferred; W06 permits only the explicit gene-plus-drug recommendation selector. |
| `interactions` | Gene-drug tracked-pair rows, not drug-drug interactions. | Rename pair disclosure into search/card; retirement explains the selected explicit route. |
| `recommendations` | CPIC recommendations, with extra drug discovery and potentially multi-gene contexts. | Keep proved gene-keyed view pages and complete lookup keys/populations with native drug names. W06 adds the distinct native gene-plus-drug recommendation table selector; W02–W05 supply ordered DPYD page witnesses. No local drug filter or hidden fan-out. |
| `frequencies` | CPIC allele frequency records, including additive drug-to-gene requests. | Defer product page because no harvested clinical question needs it; drop automatic drug-name fan-out. C24 inventories the existing source aggregate. |
| `guidelines` | CPIC guideline summaries. | Keep proved source guideline references; full publication/version detail remains absent in the captured summary tier. |
| `annotations` | Three PharmGKB endpoints merged and deduplicated with a shared limit. | Keep separate clinical/guideline/label min collections under `--kind`, default clinical, observed total; W07 proves the exact clinical single base get. Guideline and label detail remain unproved. |
| `all` | Includes every section. | Drop automatic section fan-out; named sections remain one action away. |
| get `--full` | All sections, up to 50 rows each. | Drop bulk expansion. Explicit section pages retain all facts. |
| multiple `SECTION` arguments | Several sections allowed at offset zero. | Replace with one named section per request and exact next actions. |
| get `-l`, `--limit` | Bounds rows of each selected section, default 10, allowed 1–50. | Pair-search paging uses `--limit`; named sections use shared `--section-limit`, with measured complete CLI/MCP budgets below. |
| get `--offset` | Nonzero only for one named section and without full. | Rename named-section paging to `--section-offset`; reject incompatible selectors before I/O. |
| `--json`, `-j`, section-position JSON shorthand | Shared CLI rendering selection and a legacy section-token extraction. | Global output follows current JSON-default/`--format md` contract. No PGx-specific renderer or section-token trick. |
| PGx helper family | No separate helper in legacy CLI. | The new `get pgx --gene SYMBOL diplotype --diplotype TEXT --source cpic` is an explicit source lookup, not a claim of legacy parity. |
| recommendation drug summary and annotation note | Legacy output fields, not commands/options. | Replace hidden fan-out summaries with source row contexts and ordinary problems; preserve any captured unique fact in its owning section. |

The generated accounting counts leaf commands and long command options. It excludes shared formatting, aliases, positional text and section names. `--pgx-testing` remains undecided for implementation because its positive native selector is unproved; its deferred outcome is explicit rather than silently dropped.

<!-- parity-accounting -->
| 0.9 leaf command | Command outcome | Ported or renamed options | Dropped options | Deferred options | Undecided options | Ruling |
| --- | --- | --- | --- | --- | --- | --- |
| `search pgx` | Ported or renamed | `--gene`, `--cpic-level`, `--limit`, `--offset` | `--drug`, `--evidence` | — | `--pgx-testing` | Explicit gene and CPIC source; native pair pages replace heuristic filtering and overfetch. Testing remains deferred pending positive native proof. |
| `get pgx` | Ported or renamed | `--limit`, `--offset` | `--full` | — | — | Explicit gene card and one section; section-limit and section-offset replace the old page options. Frequency and exact pair pages remain deferred; automatic all and bulk expansion retire. |
<!-- /parity-accounting -->

| Accounting surface | Shipped spelling or disposition | Owning cases |
| --- | --- | --- |
| Search gene, source, level, limit, offset | `search pgx --gene SYMBOL --source cpic`; exact `--cpic-level`, `--limit`, `--offset` | `cpic.rs` pairs and page walk; `pgx/examples.yaml`; external CPIC P01–P06. |
| Positional search; `-g` | Explicit `--gene`; ambiguous positional route retires. | `pgx_retirements.rs`. |
| `-d`, `--drug`, `--evidence` | Drug-text and guideline-text search dropped; no local name join. | `pgx_retirements.rs`. |
| `--pgx-testing`, `--population`, `--lookup-key` | Deferred until positive native selector proof; fixed unsupported guidance. | `pgx_retirements.rs`. |
| Bare get, drug-keyed card, exact pair | Gene card uses `get pgx --gene SYMBOL --source cpic`; drug and pair cards deferred. | `cpic.rs` card; `pgx_retirements.rs`. |
| `interactions` | Pair search and explicit gene card replace the old section. | `pgx_retirements.rs`. |
| `recommendations`; exact `--drug-id` | Named CPIC section; only the W06 gene-plus-drug native recommendation table is supported. Drug-ID search remains deferred. | `cpic.rs` view/table/missing-second-gene; external R01–R06. |
| `frequencies` | Deferred because no harvested question needs the page; automatic drug-name fan-out dropped. | `pgx_retirements.rs`; C24 remains source inventory only. |
| `guidelines` | Named CPIC summary references, complete linked genes and explicit page. | `cpic.rs` guidelines/guidelines-many; external L01–L03. |
| `annotations` | ClinPGx `--kind clinical`, `guideline` or `label`; no merge or deduplication. Clinical default three; other kinds five. | `clinpgx.rs` clinical/default/wide/guideline/label; external retained walks. |
| Exact annotation and diplotype helpers | New explicit `--annotation-id` ClinPGx base get; declared CPIC `--diplotype` source lookup. Neither claims a legacy helper. | `clinpgx.rs` annotation; `cpic.rs` diplotype/empty; external base get and D01–D05. |
| `all`, multiple sections, `--full` | Name one section; automatic fan-out and bulk expansion dropped. | `pgx_retirements.rs`. |
| Search `-l` and get `-l`, `--limit`, `--offset` | Search uses `--limit`/`--offset`; get uses `--section-limit`/`--section-offset`. | `pgx_retirements.rs`; CPIC and ClinPGx page walks. |
| `--json`, `-j`, section-position JSON shorthand | Shared `--format json`; no PGx-specific renderer. | `pgx_retirements.rs`; existing shared formatting cases. |
| Hidden drug summary and annotation note | Complete source context stays in the owning row; no hidden fan-out summary. | `cpic.rs` view/table; `clinpgx.rs` exact row equality. |
| `pharmgkb` spelling | Fixed `clinpgx` replacement without I/O. | `pgx_retirements.rs`; `pgx_registry.rs`. |

Shared transport/configuration options remain shared. Keys, disabled-source groups, replay/record, freshness, output format and help need no PGx-specific duplicate. Section failures keep current envelope outcomes. The old local CPIC-level rank and case-insensitive guideline filtering do not define a new clinical ranking. The comparison still owes actual contained old-binary outputs for the same questions; a legacy request plan alone does not prove the old runtime remained contained.

## Sources kept and deferred

Keep `cpic` for proved gene identity, pair/guideline-reference pages, gene-keyed recommendation views, W06 gene-plus-drug recommendation table pages and native declared-diplotype results. Keep `clinpgx` for separate min annotation collections and the W07 exact clinical single-record base get. Both have PM-ruled noncommercial terms and stay on by default unless disabled. Retire `pharmgkb` spelling without provider work. Defer frequency commands and actions because the question bank needs none. Dropped: PharmCAT calling (patient genotype calculation is outside source retrieval), PharmVar downloads (no indispensable question), and DPWG scraping (kept source references already carry attribution; richer detail remains unproved). No patient genotype calculator is added. Linked external guidelines remain attributed source references without a new adapter.

The remaining source gap is current policy text; W01 retains the PM ruling without proving that text. W02–W07 discharge the bounded native order, recommendation drug-ID and exact clinical base-get source obligations. The implemented cases and measured product envelopes appear below. Contained 0.9 comparison is a later proof ticket. No further provider request is authorized by this design review correction.

## Ian's questions

None.

### Implemented CPIC output measurements

The separate product delegate replayed all 22 fixed 0166 captures through the actual CLI and MCP at `84526cee854c255eb5c7aa6b511c5821ab5808d6` in `j13010`. Native bodies remained outside Git. A local TLS fixture delivered unchanged captured bodies and native Content-Range values through the actual recorder. These local deliveries do not replace the original publisher capture dates. Every complete native row and explicit query selector survived CLI/MCP equality checks. The comparison removes only the exact checked `terms` and `terms_url` values; it retains noncommercial use and all clinical context. Compact UTF-8 bytes, approximate tokens `ceil(bytes/4)` and ceilings at `ceil(bytes*1.15)` follow. Cold local replay and warm cache timings describe local execution on the same machine, not provider latency.

| Case | Full bytes | Comparable bytes | Approximate full tokens | Full ceiling | Comparable ceiling | Cold local replay ms | Warm cache CLI ms | Warm cache MCP ms |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| P01 | 1656 | 1495 | 414 | 1905 | 1720 | 26.048 | 15.228 | 15.081 |
| P02 | 1434 | 1273 | 359 | 1650 | 1464 | 18.443 | 16.379 | 16.14 |
| P03 | 964 | 803 | 241 | 1109 | 924 | 17.287 | 17.123 | 16.975 |
| P04 | 1348 | 1187 | 337 | 1551 | 1366 | 19.448 | 20.779 | 16.869 |
| P05 | 2099 | 1938 | 525 | 2414 | 2229 | 21.073 | 16.997 | 16.959 |
| P06 | 640 | 479 | 160 | 736 | 551 | 18.586 | 16.575 | 15.987 |
| G01 | 757 | 596 | 190 | 871 | 686 | 18.504 | 16.733 | 15.776 |
| G02 | 751 | 590 | 188 | 864 | 679 | 18.401 | 16.458 | 16.423 |
| R01 | 4964 | 4803 | 1241 | 5709 | 5524 | 19.598 | 18.418 | 16.974 |
| R02 | 4843 | 4682 | 1211 | 5570 | 5385 | 20.527 | 17.842 | 16.777 |
| R03 | 7131 | 6970 | 1783 | 8201 | 8016 | 19.835 | 19.158 | 17.964 |
| R04 | 7137 | 6976 | 1785 | 8208 | 8023 | 18.989 | 18.624 | 17.114 |
| R05 | 7931 | 7770 | 1983 | 9121 | 8936 | 20.398 | 17.766 | 19.492 |
| R06 | 3611 | 3450 | 903 | 4153 | 3968 | 19.397 | 17.966 | 16.616 |
| L01 | 738 | 577 | 185 | 849 | 664 | 19.409 | 17.72 | 16.537 |
| L02 | 866 | 705 | 217 | 996 | 811 | 21.114 | 20.952 | 17.21 |
| L03 | 2492 | 2331 | 623 | 2866 | 2681 | 21.521 | 18.61 | 19.477 |
| D01 | 570 | 409 | 143 | 656 | 471 | 18.498 | 16.836 | 16.514 |
| D02 | 1760 | 1599 | 440 | 2024 | 1839 | 19.219 | 18.963 | 16.558 |
| D03 | 1686 | 1525 | 422 | 1939 | 1754 | 19.879 | 18.968 | 17.559 |
| D04 | 1725 | 1564 | 432 | 1984 | 1799 | 19.508 | 17.289 | 16.539 |
| D05 | 1702 | 1541 | 426 | 1958 | 1773 | 18.863 | 18.482 | 15.446 |

`help search pgx` measured 897 bytes with a 1032-byte ceiling. `help get pgx` measured 1763 bytes with a 2028-byte ceiling. The counts describe the implemented source contracts. They do not establish current clinical guidance beyond the captured tables or a CPIC dataset release. The exact native drug filter uses the recommendation table; its rows never acquire view-only names or guideline URLs.

The durable native product proof is `data/biomcp-redux/0166-cpic-product-proof/j13010-product-proof.tar.gz` on Yellow, SHA-256 `a1a1e3e89b53cd526702b2cd999a961a7bd15967cc86df744e374dacab8d5966`. The separate source capture is `data/biomcp-redux/0166-cpic-native-capture/j12978-native-capture.tar.gz`, SHA-256 `13feccbd90e4365fa6d6f45780cc9408411267a640da665f0fb4f9d9c784106a`. Synthetic executable examples use entirely authored rows and actual local-recorder identities.

The 22 executable native cases and real-recorder fixtures remain outside Git at `data/biomcp-redux/0166-cpic-capped-cases/j13036-capped-cases.tar.gz` on Yellow, SHA-256 `3fd1267e69d02dcdae541cd604feef6e6e8beb9db026095f49176796106717e0`. All 47 manifest entries were independently verified. The included shell recipe uses the existing external case interface and fixture-root setting. Its case ceilings follow the existing runner convention, which validates and removes `use`, `terms` and `terms_url` before measuring comparable bytes and tokens. The product comparison table above retains `use`. One G01 case passed as a loader and cap smoke check in `j13036`; this step did not repeat the 22-case native product proof.

After embedding two executable examples for each PGx command, `j13052` measured search help at 1,033 compact bytes and get help at 1,893. Their executable case ceilings are 1,188 and 2,177 bytes, respectively, before the CLI newline. A synthetic truncated-diplotype response failed on the old adapter in `j13048` and passes as an explicit `bad_reply` failure after the correction. The adapter rejects a Content-Range total larger than its complete declared-diplotype array before cache admission. It emits no diplotype continuation action.

Guideline references expose every distinct supported linked gene as a `pivot` in native order, then the native page action, then bounded extra actions. The shared step contract includes this additive kind. Replaying the three retained guideline cases in `j13081` preserved every result row and passed their existing CLI/MCP output ceilings. L01 remains 738 compact bytes. L02 changes its NUDT15 action from `section` to `pivot` and falls from 866 to 864 bytes. L03 changes its five existing gene actions to `pivot`, restores the sixth linked gene CYP2C19 and the page action, and grows from 2,492 to 2,640 bytes. These measurements supersede only those action sizes above; they do not remeasure local timings or contact the provider. A gene card also requires the native count to describe its complete zero-or-one identity before selecting a row.

### ClinPGx annotation endpoint verification

The fixed five-request keyless capture `j13128` ran from landed revision `8f6ed7f28b6ac0bd6b658aaf937efcee78d08400`. Every request returned a complete HTTP 200 JSON envelope with `status: "success"` and `data`; no response stated a server total or Content-Range. DPYD clinical min returned 83 rows, DPYD guideline min returned 10 references, `data/labelAnnotation?relatedGenes.symbol=UGT1A1&view=min` returned 36 references, and TP53 clinical min returned three rows. The UGT1A1 request proves the gene selector independently of the earlier irinotecan-name label capture. The exact PA166134421 base get returned one object with that accession and separate numeric ID 827843617. The DPYD min collection retains PA166134421 and PA166224841 as separate source rows.

The complete nested inventories match the existing clinical min, guideline/label reference and base annotation types. Min references supply only `objCls`, `id`, `name` and `crossReferences`; they supply no full guidance. Requests were serial with at least one second between admissions and no redirects or retries. Original bodies and receipts remain outside Git in `data/biomcp-redux/0167-clinpgx-native-capture/j13128-clinpgx-native-capture.tar.gz`, SHA-256 `281fd813754ca72149f62e65c8b21aca60e2fc0d821f42ad636799bccb6de336`. The archive contains 30 independently verified files. These are native source observations, not product envelope or latency measurements.

ClinPGx continuation uses the existing immutable source snapshot store. The first read admits and retains the complete checked native collection before any local slice. Returned page commands carry `--snapshot` and preserve gene, source, kind and section limits. The store binds a deterministic token to the complete collection and its gene/kind key. Later pages read only that retained collection, independently of transport-cache freshness; missing, expired or mismatched tokens return the shared expired-pool response and an explicit restart without contacting the provider. Source and noncommercial-group admission precede retained reads. An explicitly replayed walk uses the existing caller-owned case cache so that its local collection survives each process. This adds no source-specific store or provider paging.

### ClinPGx product envelopes and reviewed default

The retained product run at `c95b92f04402fb16c98c10500c049c7366d69aae` verified complete native rows and CLI/MCP equality for all clinical local walks at limits five, three and one, independent guideline and label collections, TP53 clinical records and the exact PA166134421 base object. The DPYD walks contain the same 83 rows in native order, including both distinct Q28 accessions, with 17, 28 and 83 local pages respectively. Every continuation preserves its snapshot, gene, source, kind and effective section limit. No provider request occurred; the original archived bytes were delivered by a local fixture once and subsequent product reads used retained replay/cache state. Local fixture receipts do not replace original provider retrieval provenance.

| Command/page | Largest full compact bytes | Comparison bytes | Full ceiling | Comparison ceiling |
| --- | ---: | ---: | ---: | ---: |
| DPYD clinical, five rows | 8193 | 8032 | 9422 | 9237 |
| DPYD clinical, three rows | 5279 | 5118 | 6071 | 5886 |
| DPYD clinical, one row | 2364 | 2203 | 2719 | 2534 |
| DPYD guideline references | 1512 | 1351 | 1739 | 1554 |
| UGT1A1 label references | 1609 | 1448 | 1851 | 1666 |
| TP53 clinical collection | 4250 | 4089 | 4888 | 4703 |
| PA166134421 base get | 22785 | 22624 | 26203 | 26018 |

Comparison bytes remove exactly the validated publisher terms name and URL, while retaining use category, complete query, snapshot and all native context. The ordinary case runner separately excludes all validated source labels, including use, when applying its existing comparable ceiling convention. Each table ceiling is `ceil(measured bytes × 1.15)`; approximate tokens are `ceil(bytes / 4)`. Full raw stdout includes its newline. The largest three-row comparison is approximately 1,280 tokens. The PM reviewed all three alternatives and selected three intact clinical rows by default to retain comparison among records while bounding output. Explicit larger pages remain available. Other section defaults remain unchanged. `CLINICAL_PAGE_MAX_BYTES` enforces the reviewed clinical comparison ceiling.

These measurements precede only the reviewed default change and checked-builder tightening. The three-row result, count, native fields and snapshot remain intact. The command encoder now omits the effective three-row default from continuation arguments and retains an explicit larger limit; this can reduce continuation bytes. The existing CPIC gene-card example gains the annotations section action and measured 676 comparable compact bytes, 677 with newline. The PM approved its case ceiling of 779 comparable stdout bytes.

Across 139 local-walk CLI processes, median elapsed time was 0.02005 seconds, minimum 0.01624 and maximum 1.02581; the MCP median was 0.01875, minimum 0.01475 and maximum 0.02634. The exact get measured 0.01835 seconds for cold local replay, 0.01703 for warm CLI and 0.01658 for warm MCP. These shared-host local measurements do not state provider latency. The durable archive is `data/biomcp-redux/0167-clinpgx-product-proof/j13182-partial-product-proof.tar.gz`, SHA-256 `a20d93fcc46418a4073b2f2e734242785fbfb5b41b154e714510885f534d8a44`, with 2,090 verified files. Its extra CPIC fixture stage omitted the archived Content-Range header and supplies no CPIC native result; the ClinPGx walks and base get are independent of that fixture error. No additional product replay or source delivery is needed to retain those completed measurements.
