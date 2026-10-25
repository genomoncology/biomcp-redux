# Pharmacogenomics slice investigation and proposed design

This investigation starts from `d18bfbba511e3852c99228f1a5166a14633e5cee`. It covers steps 1 to 4 of [the slice method](../slices.md). It proposes source retrieval, typed disclosure and exact follow-up commands. It does not calculate a patient phenotype, select therapy, recommend a dose or implement product code.

## Evidence boundaries

Official documentation was read on 2026-10-25. Source response counts, field presence and single-request latency are measured below. Product budgets remain unmeasured. A documented capability is not a successful public-tier request. The source captures must precede acceptance of shared types. Historical source adapters and public papers do not replace that proof.

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
| Q33 | partial | CPIC pairs; ClinPGx annotations; authored failures | C27 zero and P09 positive establish source-locality. Failure controls are proposed, not executed. |

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

A dash means no independent total or pagination field was supplied. ClinPGx array length is an observed collection count, not an asserted server-wide total. C09/C15 contain ten distinct recommendation IDs across two observed nonoverlapping DPYD pages. No `order` parameter was requested, so a stable future page walk remains unproved.

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

Actual current fields `clinpgxid` and `clinpgxlevel` confirm the migration; legacy field spellings are absent. M06 contains the complete dynamic OpenAPI document. It documents richer `recommendation` table fields such as `drugid`, `guidelineid`, `allelestatus`, `version` and dose-related fields, but no data request captured that table. Documentation cannot accept those types or selector semantics. Its `info.version: 12.0.2` is PostgREST software metadata, not a CPIC dataset release.

M01 and M02 return identical JavaScript shells, so they retain no actual policy/footer terms. M03 retains the API's policy link and rate limit. M04/M05 are documentation shells; no script follow-up ran. Official footer text read through the web reader remains separate evidence. Complete current policy wording and CPIC-specific scope remain unresolved; HTTP success grants no reuse license.

### Use and terms

The PM ruled on 2026-10-25 that CPIC and ClinPGx both use `noncommercial`, group `noncommercial`, under the shared [ClinPGx data use policy](https://www.clinpgx.org/page/dataUsagePolicy). Both remain on by default under the existing decision; commercial users can disable the group. The type ticket must capture actual current policy text. M01/M02 retained JavaScript shells, not that text, so this design distinguishes the PM's product ruling from an independently verified policy receipt. Current official footer text read through the web reader names Creative Commons Attribution-NonCommercial-ShareAlike 4.0. No paid or commercial permission is assumed.

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
5. **Annotations.** One ClinPGx kind is disclosed at a time. Min pages let the agent choose an exact accession; a captured single-record base get will expose detail. Missing disease fields in min never become disease absence.
6. **Disagreement and dates.** Keep source records independent, complete contexts and source versions distinct from retrieval date. A website notice absent from API tables remains an explicit limitation, never a local correction.
7. **Failure.** A successful zero, missing key, disabled source, unavailable transport, identity rejection and unsupported field remain different. None implies a normal phenotype or safe dose.
8. **Cost.** Start with a bounded page and one source. Offline projections below measure proposed bodies only. Product latency, full envelopes and contained 0.9 comparison remain later proof.
9. **Stop early.** Pair facts can answer guideline presence; a declared diplotype can answer a source-reported label. Neither is a patient treatment choice.
10. **Trust.** Return exact native IDs, source links, original scope, terms and available dates. Every next step keeps the selectors and selection scope that produced it.

## 4. Reviewed types, schemas and commands

The PM settled these interface choices on 2026-10-25. Domain types preserve source facts without I/O. Consumer types own queries, header/card/page disclosure, envelope outcomes and typed actions. Schemas and help derive from one accepted type definition. This design changes no product code or schema.

### Domain and identity boundaries

`PgxPair` holds exact source pair ID, gene, drug ID/name and optional guideline URL, PMID list, CPIC/ClinPGx levels, testing, provisional and recommendation-use facts. Preserve compound labels such as `B/C`. `PgxGeneIdentity` holds source symbol, exact cross-references, lookup method and native record version. A record version is not a dataset release. `PgxGuidelineReference` holds the captured summary's name, URL, ordered drug names and gene symbol/URL objects. It supplies no invented publication ID or version.

`PgxRecommendation` keeps recommendation ID, complete multi-gene lookup key, drug name, guideline name/URL, implications, full source recommendation, classification, phenotype/activity maps, population and comments. The captured view has no drug ID, separate allele-status field, version or date. The adapter ticket captures native `drugid` and deterministic `order` before those capabilities are accepted. The currently proved gene-keyed section ships without a local drug filter.

`PgxNamedAllele` preserves exact gene/name, optional native allele and definition IDs, source version, distinct function statuses and source activity strings. `PgxDiplotypeResult` preserves source expression, gene, complete `diplotypekey` counts, both function/activity values, source result, consultation/EHR text and result lookup key. The source view has no numeric row ID or allele IDs. Preserve `n/a`, uncertainty and multiple results. Do not strip copy notation or compute function, phase or phenotype.

Star alleles and diplotypes stay native PGx records. They produce no `Statement` and no `statement_unmapped` flag. No ClinPGx evidence level maps to a shared proposition. This slice adds no shared `Subject` kind. Source-given genomic identifiers can provide exact variant lookup actions only where the target accepts that identifier and the source establishes its scope; a single rsID is not a whole haplotype. ClinPGx accessions use a distinct namespace and never the shared `PaId`.

`PgxAnnotationHeader` preserves every captured min-tier row field, including native numeric/PA identities, evidence term, location and native gene/chemical/link arrays. `PgxAnnotationDetail` retains all captured base-tier fields when a direct single-get capture proves that request. Clinical, guideline and label kinds remain separate. Min guideline/label rows are references, not full guidance. Missing min-tier disease, history or genotype text is unsupported disclosure, not a negative fact.

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

The empty values demonstrate fields only; native fixtures supply actual counts and rows. Pair items contain the entire C03–C08/C28 row. Recommendation items contain the entire C09–C15 row. Guideline-reference items contain the entire C21–C23 row. A ClinPGx collection header adds `kind`; its page uses `total_kind: "observed"`, the received collection length, and ordinary checked local page bounds. It does not claim native server total or stable provider pagination. An explicitly bounded retained collection supplies all later local pages in observed order. No row or context field is dropped to satisfy output cost.

Diplotype result:

```json
{"header":{"source":"cpic","gene":"TPMT"},"diplotypes":[]}
```

The array carries every complete returned row. Zero, one and multiple rows remain distinct; missing source row IDs stay absent. `--diplotype` is echoed unchanged in the outer query.

Proposed single annotation detail:

```json
{"header":{"source":"clinpgx","id":"PA166134421"},"annotation":{}}
```

The annotation object retains the full captured base-tier row. This body is projected offline from P10, but native direct-get request and returned-accession equality need one future capture before implementation accepts it. An annotation numeric ID and its PA accession remain separate fields.

Proposed schemas are `biomcp.pgx_page/1`, `biomcp.pgx_card/1`, `biomcp.pgx_guidelines/1`, `biomcp.pgx_recommendations/1`, `biomcp.pgx_diplotype/1`, `biomcp.pgx_annotations/1` and `biomcp.pgx_annotation/1`. No frequency schema is approved. The source adapters own native decoding and checked identity; query composes disclosure without clinical inference. No new client, cache or variant parser is introduced.

### Command tree and per-command next actions

| Command | Body and proven scope | Next actions |
| --- | --- | --- |
| `search pgx --gene SYMBOL --source cpic` | Native pair page from C03–C08. `--cpic-level` has C28 positive proof. `--limit`/`--offset` select source pages; deterministic order is captured in the adapter ticket. No positional search. | One exact gene-card selection where accepted; distinct source-returned gene associations pivot in native order. Page retains gene, source, level and paging selectors; then at most three useful section/pivot actions. Exact pair get remains deferred. |
| `get pgx --gene SYMBOL --source cpic` | Compact source gene identity card from C01. | At most five actions, prioritizing recommendations, guidelines, declared diplotype lookup help and annotation search with a proved exact gene relation. No frequency step. |
| `get pgx --gene SYMBOL recommendations --source cpic` | Complete gene-keyed recommendation rows from C09–C15, native drug names, complete lookup keys and populations. `--section-limit`/`--section-offset`. No local drug filter. | Page first when more rows remain; exact source-supported gene pivots only, then at most three useful actions. Do not emit a drug-ID action from native drug name alone. |
| `get pgx --gene SYMBOL guidelines --source cpic` | Native summary-reference page from C21–C23, with section paging. | Exact source-returned linked genes pivot in native order; page follows; source publication actions only when that exact record actually supplies a PMID. No invented article action from guideline name. |
| `get pgx --gene TPMT diplotype --diplotype '*1/*3A' --source cpic` | Full native source lookup from C17–C20; C16 preserves literal DPYD zero. | At most five steps. A gene-keyed recommendation section remains available; never emit a uniquely selected recommendation from an incomplete multi-gene key. Exact alternate candidate selections require real source identities. |
| `get pgx --gene SYMBOL annotations --kind clinical --source clinpgx` | Local page from the captured min collection, observed total and native order. `--kind clinical|guideline|label`, default clinical. Section limit/offset are local collection selectors. | One `disambiguate` single-annotation get per clinical candidate by exact PA accession after direct base-get proof, then page, then at most three useful pivots. Kind/source/gene and collection identity persist. Guideline/label kinds expose only proved references until richer detail is captured. |
| Deferred: `get pgx --annotation-id PA… --source clinpgx` | Single base annotation; one future exact direct-get capture required. | At most five exact source gene/drug/variant actions and available section actions. Names never become identifiers. |

These rules apply architecture 6.10. One pure PGx `next` function returns typed requests; presentation spells the arguments. Selectable pages emit every candidate disambiguation before the page action, bounded by checked page size, then at most three extras. Association pages emit one pivot per distinct linked record in native order before page and extras. Other responses have at most five actions. `fresh` retries the same request after failed/stale source outcome; disabled/missing-key recovery uses existing shared actions. No frequency action appears anywhere. A deferred selector or unsupported detail command does not appear as an executable next until its source proof and implementation exist.

### Deferred selectors and proof obligations

`--drug-id`, exact pair card, `--population`, `--lookup-key` and `--pgx-testing` are unproved and deferred. No positional search or drug-text search is proposed; the question bank does not need a drug-name search. The adapter ticket separately captures exact drug IDs and deterministic native ordering without local drug filtering. The type ticket captures current policy text. A single-record ClinPGx base get needs one exact native capture. These are implementation proof tasks, not unresolved product choices or permission to expand this investigation's provider ledger.

### Case map and failure boundaries

| Surface | Harvested cases and required assertion |
| --- | --- |
| Gene search, `--gene`, `--source` | Q01/Q06/Q30: exact pair/gene/drug identities, separate level namespaces, native no-action and missing guideline. Zero source switching or name inference. |
| `--cpic-level` | C28/Q30: exactly source-proved positive native selector, compound other levels preserved in unfiltered pages. |
| Search `--limit`/`--offset` | Q01/Q30: complete native rows, source total and retained selectors; future exact-order capture required before stable walk acceptance. |
| Gene card | C01/Q01: source-given IDs/method/version only; absent relations stay absent. No pair or frequency claim. |
| Recommendations | Q03/Q04/Q12–Q16/Q18–Q27: complete native drug names, all lookup genes, original scope/comments and no locally repaired notice/table discrepancy. Unsampled questions retain their unanswered status. |
| Guidelines | Q01/Q17/Q30: native URL/name and ordered arrays, without invented publication metadata. |
| Diplotype and `--diplotype` | Q02/Q07/Q08/Q24/Q31/Q32: exact expression, all source mappings, native result versus literal zero, no phase or score calculation. |
| Annotation kinds | Q28/Q29/Q33: independent clinical/guideline/label collections, default clinical, observed total and all native context fields. No merging Toxicity and Other records. |
| `--section-limit`/`--section-offset` | C09/C15 and ClinPGx retained collections: every row remains reachable with exact source/query/collection selectors. No hidden filter or lost multi-gene context. |
| Future single annotation selector | Q28: disambiguation by native accession, one future native base-get capture and returned-identity equality before cache admission. |
| Source retirement | `pharmgkb` spelling returns `clinpgx` replacement with zero provider requests. |
| Failures, empty, shared options | Q33: source-local zero versus unavailable, 429/500/timeout/broken JSON/HTTP-200 error/identity mismatch, required key, disabled source and warm-cache exclusion. Synthetic failure controls remain explicitly authored. CLI/MCP equality and byte ceilings are implementation proof. |

### Disclosure and offline byte projections

The default pair search and gene card answer identity/guideline-presence questions. Recommendation prose belongs behind its named section. Min annotation pages expose choices; source base detail stays behind exact selection. Start with five rows per page. Smaller pages never delete a returned row's clinical context.

The separate offline projection recipe uses only the verified j12644 archive, compact UTF-8 JSON with `ensure_ascii=False` and separators `(',', ':')`. It constructs the exact result-body shapes above, keeping complete native rows. Pair and recommendation/guideline pages use captured Content-Range total and requested offset. Diplotypes preserve the whole captured list. Gene cards select the seven named identity fields, exactly as supplied. ClinPGx min arrays are paged five at a time through every retained collection, with observed totals. Each P10 annotation is projected as one design-only base detail body. No provider call, program build, old binary or clinical transform runs.

Each output is bounded at 1 MiB and the full projection set at 20 MiB. Record row identities/provenance, compact bytes, `ceil(bytes/4)` approximate tokens and provisional `ceil(bytes*1.15)` body ceiling; retain recipe/outputs/measurements on Yellow. These are offline design projections, not actual native product outputs, tokenizers or passing case ceilings. Complete envelope bytes, source outcomes, typed-next cost, CLI/MCP parity and cold/warm latency remain unmeasured until product implementation. The later contained 0.9 PGx comparison ticket first proves network containment, then measures the same harvested questions within a separately approved ledger. It does not run in this investigation.

### Structural risks

Each adapter defines native fields and identities once; help/schema derive from accepted types. Separate annotation kinds preserve collection scope. Shared transport owns requests, quotas, cache and retries. Checked identity precedes cache admission. The current cache can retain a bounded source collection; use existing checked page/snapshot primitives rather than a second source-specific store. Every projected datum remains attributable to a retained row. Failures use isolated runner fixtures, never live induced errors. No activity calculator, clinical consensus ranker or patient inference engine is justified.

## 0.9 command, option and section accounting

Static inventory uses the legacy checkout at `4050fdc7dbdf4d713d94cf02a39a147b863665be`, especially `src/cli/pgx/mod.rs`, `src/cli/pgx/dispatch.rs`, `src/entities/pgx.rs`, source adapters and `docs/user-guide/pgx.md`. This is source inspection, not executed output, not evidence of a released 0.9 binary's behavior and not a timing or parity claim. Proposed dispositions await PM review.

| Legacy surface | Static behavior | Proposed disposition and case need |
| --- | --- | --- |
| `search pgx` | CPIC pair rows; gene/drug search required. | Keep with explicit source, native identities, faithful totals and direct source selectors. |
| `-g`, `--gene` | Gene text uppercased and used as primary query. | Keep explicit gene selector; use the source's supported key grammar, not free-text gene detection. |
| positional search `QUERY` | Alias for gene option; shared conflict checks. | Drop ambiguous positional alias; retirement names `--gene`. |
| `-d`, `--drug` | Source drug-name substring query; combined gene/drug filters locally. | Drop drug-text search from the initial tree because no harvested question needs it. Exact `--drug-id` remains deferred; no local name join. |
| `--cpic-level` | Local filter after a bounded overfetch. | Keep only after direct native positive filter capture C28. Separate from recommendation strength and ClinPGx level. |
| `--pgx-testing` | Local match over five accepted labels. | Undecided until direct native selector and current enum proof; no hard-coded carry-forward. |
| `--evidence` | Free-text substring of guideline name or CPIC level. | Drop under no-text-heuristics ruling. Return source labels and use exact proved native selectors. |
| search `-l`, `--limit` | Default 10, allowed 1–50; fetches up to five times requested rows, capped at 200; truncates after local filters. | Keep bounded native paging, remove hidden overfetch and source-total/locally-filtered-count ambiguity. Proposed default five needs measurements. |
| search `--offset` | Native offset precedes local filtering. | Keep only with a proved native stable page contract and all selectors retained in next. |
| `get pgx QUERY` | Word-shape helper chooses gene or drug; default interactions. | Replace with explicitly declared gene identity card and named sections; old bare form retires. Exact drug selector and pair card remain deferred. |
| `interactions` | Gene-drug tracked-pair rows, not drug-drug interactions. | Rename pair disclosure into search/card; retirement explains the selected explicit route. |
| `recommendations` | CPIC recommendations, with extra drug discovery and potentially multi-gene contexts. | Keep proved gene-keyed native page and complete lookup keys/populations with native drug names; drugid/order capture belongs to adapter ticket. No local drug filter or hidden fan-out. |
| `frequencies` | CPIC allele frequency records, including additive drug-to-gene requests. | Defer product page because no harvested clinical question needs it; drop automatic drug-name fan-out. C24 inventories the existing source aggregate. |
| `guidelines` | CPIC guideline summaries. | Keep proved source guideline references; full publication/version detail remains absent in the captured summary tier. |
| `annotations` | Three PharmGKB endpoints merged and deduplicated with a shared limit. | Keep separate clinical/guideline/label min collections under `--kind`, default clinical, observed total; single base get requires one later capture. |
| `all` | Includes every section. | Drop automatic section fan-out; named sections remain one action away. |
| get `--full` | All sections, up to 50 rows each. | Drop bulk expansion. Explicit section pages retain all facts. |
| multiple `SECTION` arguments | Several sections allowed at offset zero. | Replace with one named section per request and exact next actions. |
| get `-l`, `--limit` | Bounds rows of each selected section, default 10, allowed 1–50. | Pair-search paging uses `--limit`; named sections use shared `--section-limit`, with provisional offline body budgets and later full-output checks. |
| get `--offset` | Nonzero only for one named section and without full. | Rename named-section paging to `--section-offset`; reject incompatible selectors before I/O. |
| `--json`, `-j`, section-position JSON shorthand | Shared CLI rendering selection and a legacy section-token extraction. | Global output follows current JSON-default/`--format md` contract. No PGx-specific renderer or section-token trick. |
| PGx helper family | No separate helper in legacy CLI. | The new `get pgx --gene SYMBOL diplotype --diplotype TEXT --source cpic` is an explicit source lookup, not a claim of legacy parity. |
| recommendation drug summary and annotation note | Legacy output fields, not commands/options. | Replace hidden fan-out summaries with source row contexts and ordinary problems; preserve any captured unique fact in its owning section. |

Shared transport/configuration options remain shared. Keys, disabled-source groups, replay/record, freshness, output format and help need no PGx-specific duplicate. Section failures keep current envelope outcomes. The old local CPIC-level rank and case-insensitive guideline filtering do not define a new clinical ranking. The comparison still owes actual contained old-binary outputs for the same questions; a legacy request plan alone does not prove the old runtime remained contained.

## Sources kept and deferred

Keep `cpic` for proved gene identity, pair/guideline-reference pages, gene-keyed recommendation pages and native declared-diplotype results. Keep `clinpgx` for separate min annotation collections and a single-record base get after one future capture. Both have PM-ruled noncommercial terms and stay on by default unless disabled. Retire `pharmgkb` spelling without provider work. Defer frequency commands and actions because the question bank needs none. Do not add PharmCAT calling, PharmVar downloads, DPWG scraping or a patient genotype calculator. Linked external guidelines remain attributed source references without a new adapter.

The remaining type/adapter proof obligations are current policy capture, native drug-ID and stable-order witnesses, one exact ClinPGx base get and measured full product envelopes. Contained 0.9 comparison is a later proof ticket. No further provider request is authorized by this design review correction.

## Ian's questions

None.
