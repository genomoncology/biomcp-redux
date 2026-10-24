# Disease slice investigation and proposed design

This investigation starts from published revision `ecef1096239f8aeee8fcc6cbefc5720e89506035`. It proposes the disease slice through steps 1 to 4 of the slice method. It changes no product types, adapters, commands or schemas. Clinical answers require the independent clinical review cited with the harvested examples.

## Identity and scope

A disease is a publisher's named concept, with its exact namespaced identifier. A label, synonym, hierarchy edge, cross-reference and equivalence assertion are different source facts. Keep those relation types and the source that supplied them. A shared spelling never establishes a join.

The proposed neutral `biodata` layer owns disease identifiers and source facts. The consumer `biomcp-types` layer owns request selectors, source choices, headers, cards, sections, paging and next commands. Adapters use the existing checked transport and the shared source registry for use category, terms name and publisher page. No second terms table belongs in the disease adapter.

A disease identifier contains its namespace and source value. MONDO, DOID, NCIt, OncoTree, ICD-10, ICD-10-CM and ICD-10-AM remain distinct. A source can supply a cross-reference without asserting equivalence. BioMCP preserves the relation exactly. A source-native ontology identifier cannot be replaced with a name-derived match. Obsolete concepts retain their obsolete flag and publisher replacement candidates. A replacement is never silently selected.

Search returns publisher-ranked candidates. `MF` must remain ambiguous when the source supplies several concepts. Exact label equality does not resolve a search result. Get accepts an explicit checked identifier and validates the returned identity before admission to cache or output. Names stay on search. A candidate next command contains the exact returned identifier.

Trial integration is explicit. `search trial --condition TEXT` keeps its existing literal meaning. A proposed distinct `--condition-id NAMESPACE:VALUE` first resolves that exact disease identity, returns its source label and cross-references, and echoes the chosen trial request. The caller chooses between literal text and a resolved identifier; the program does not infer this from punctuation or a disease name. ClinicalTrials.gov still applies its own condition-text matching. NCI needs a source-supplied NCIt identifier. Neither route grants an eligibility claim. A failed or ambiguous resolution returns candidates or a problem before any trial request.

SEER site identifiers form a separate source-native vocabulary. An ontology disease name cannot pick a SEER site. Statistics require an explicit site selected from the publisher's catalog, plus the native sex, geography, year, age, race, stage, interval and measure where the source returns them. A historical curated name-to-site table is outside the approved design.

## Questions that shape the command

1. The agent first asks for a cancer by its published name. `search disease --query TEXT` returns bounded candidate headers with exact identifiers, publisher labels, source, match context and a next get command. It does not choose an identity.
2. The smallest useful header carries the exact identifier, label, source, available cross-reference namespaces and obsolete state. It must identify the source's result count separately from the displayed row count.
3. The agent next asks for synonyms, parents, children, genes, phenotypes, evidence or statistics. Each proved section has a typed next command and an independent page. Unproved public-tier fields have no advertised section.
4. Each next request needs the exact source identifier: MONDO/DOID for ontology resolution, source-provided NCIt for NCI trials, native association identifiers for evidence, and explicit SEER site IDs for statistics. OncoTree and ICD codes remain source-labeled cross-references until a capture proves their relation and namespace.
5. A disagreement remains two attributed facts. Parentage from different ontologies is never flattened into one clinical hierarchy; native gene association predicates and scores are never turned into a diagnosis or treatment recommendation.
6. A valid empty query is `empty`; disabled, missing-key and unsupported sources are `skipped` with a reason; an upstream error is `failed`; partial native data and problems remain visible. HTTP 200 with GraphQL errors is not a valid zero.
7. Cold source latency and warm product latency must be measured separately on the same machine. A proposed cache is not evidence of warm speed. The capture receipts below distinguish direct requests from product requests.
8. The old command's local reranking, query recovery, hand-made clinical term lists and inferred SEER mapping have no place in the new path. Public source fields decide the supported sections.
9. The agent can stop after selecting the correct candidate and reading the default identity card. Large association, phenotype and evidence lists stay behind explicit sections; no default fanout is justified by a hidden enrichment step.
10. A clinician needs the publisher identity, source version or retrieval date, native relation type and evidence link. Source gene associations alone supply no causality, screening advice or therapy advice.

## Proposed types and command tree

These names describe a design, not implemented schema identifiers. Reuse existing neutral identifier, text, numeric, attribution and assertion records where their contracts already fit.

| Layer | Proposed data | Contract |
|---|---|---|
| Neutral identity | `DiseaseId`, `DiseaseConcept` | Namespace and source value; exact publisher label; optional definition; obsolete flag; source version. |
| Neutral relation | `DiseaseRelation` | Subject ID, predicate as published, object ID, relation provenance and optional source evidence. No inferred equivalence. |
| Neutral synonym | `DiseaseSynonym` | Source text, source synonym type, language when returned. No local alias list. |
| Neutral associations | Existing source-qualified statement or a small disease association record | Source gene/phenotype IDs, native predicate, evidence IDs, native score and qualifiers where supplied. No cross-source score blend. |
| Neutral statistics | `DiseaseStatistic` | Native measure, value, unit, denominator, geography, population, time period, interval and uncertainty where supplied; explicit SEER site ID. No calculated survival. |
| Consumer search | Disease search query and candidate page | Literal query, explicit source, limit, offset/cursor, publisher totals, candidate headers, source outcomes and next actions. |
| Consumer get | Disease card and page | Explicit checked ID, source outcomes, problems, section availability and bounded section actions. |

The proposed commands are `search disease --query TEXT --source SOURCE --limit N --offset N`, `get disease ID`, and `get disease ID SECTION --section-limit N --section-offset N`. Native cursor sources need typed cursors rather than pretending offsets are equivalent. Candidate limits and section limits are independent. Source selectors name providers. An optional explicit ontology selector names an ontology within a provider; it does not overload provider selection.

The default card contains identity, label, a bounded definition when returned, obsolete state and a small exact cross-reference list. Synonyms, hierarchy, genes, phenotypes, source evidence and statistics are separate sections. A hierarchy row carries its ontology and relation type. A gene row carries its native association predicate and evidence source. Source-native subtype edges remain ontology facts. Open Targets scores remain named source measurements. They are not clinical validity classifications.

Statistics use a distinct explicit site query, proposed as `disease statistics --source seer --site ID`, rather than a name-driven get section that guesses a site. The final source capture determines whether incidence and survival have one common record or separate source measures. `all` is an explicit bounded section request with independent source outcomes, never unlimited output.

## Progressive disclosure and proposed ceilings

The following are proposal ceilings. They are not measured product outputs and must not be reported as achieved budgets. The matched harvested examples and released baseline measurements below determine the final limits before implementation. The runner must assert each accepted ceiling with `max_bytes` or `max_tokens`.

| Surface | Proposed maximum bytes | Why it exists |
|---|---:|---|
| Search default, five candidates | 8,000 | IDs, labels, short match context, source totals, terms, bounded next actions. |
| Default identity card | 5,000 | Enough identity and provenance to choose or stop. |
| Synonyms, ten rows | 5,000 | Native types and language without an unbounded alias dump. |
| Hierarchy, ten rows | 7,000 | Exact related concepts and relation types. |
| Genes, ten rows | 10,000 | Evidence and native association qualifiers remain attached. |
| Phenotypes, ten rows | 10,000 | Exact HPO identity, evidence and source frequency qualifiers. |
| Statistics, one explicit site and native stratum page | 8,000 | Source period, population and measure context cannot be dropped to save bytes. |
| Evidence, five rows | 12,000 | Source claims need identity, publication links and native context. |

These byte ceilings include the full response envelope and terms metadata. Token counts require an identified tokenizer; bytes divided by four are not measured tokens. Any proposed default above the same-question baseline needs an explicit fact-level reason. Bulk evidence remains behind pagination or exact source links. Native descriptions must not be silently truncated without a drill-down that reaches their full content.

## Harvested clinical questions

The independent clinical reviewer checked these questions on 2026-10-24. D01–D24 paraphrase real published study aims or guideline topics; they are not quoted clinician questions. D15 also has an explicit diagnostic question in its source guideline. D25 is a real public issue. Accepted answers retain the paper's scope and date. API facts and clinical truth remain separate: a source-local empty result does not negate a disease, and a failed request supplies no accepted API answer.

| Case and literal term | Published question, paraphrased | Accepted answer and limit | Primary anchor |
|---|---|---|---|
| D01 breast cancer | Which disease and classes does the breast-tumor cohort cover? | Broad breast cancer with luminal A/B, HER2-enriched and basal-like study classes. No patient subtype inference. | [TCGA, PMID 23000897](https://pubmed.ncbi.nlm.nih.gov/23000897/) |
| D02 MONDO:0007254 | What identity and cross-references does this exact concept carry? | Publisher label breast cancer. A breast carcinoma cross-reference is not automatically exact equivalence. | [TCGA](https://pubmed.ncbi.nlm.nih.gov/23000897/), [publisher ontology](https://ols.monarchinitiative.org/ontologies/mondo_reorder/classes/http%253A%252F%252Fpurl.obolibrary.org%252Fobo%252FMONDO_0007254) |
| D03 triple-negative breast cancer | What defines the study's TNBC category? | ER/PR/HER2 marker-negative clinical category. No automated biomarker classification. | [Sable, PMID 27612858](https://pubmed.ncbi.nlm.nih.gov/27612858/) |
| D04 basal-like breast cancer | Is basal-like synonymous with triple-negative? | Overlapping categories with distinct expression-based and marker-based definitions. Reject a local exact synonym join. | [Sable](https://pubmed.ncbi.nlm.nih.gov/27612858/) |
| D05 lung adenocarcinoma | Which parent and aliases belong to the tumor? | DOID:3910 has lung non-small cell carcinoma and adenocarcinoma parentage; preserve publisher synonym scopes and both NCIt cross-references. | [TCGA, PMID 25079552](https://pubmed.ncbi.nlm.nih.gov/25079552/), [DOID:3910](https://disease-ontology.org/term/DOID%3A3910/) |
| D06 non-small cell lung carcinoma | Does NSCLC mean adenocarcinoma alone? | Broad group; adenocarcinoma is a child, not an exact synonym. The guideline concerns a specified metastatic setting. | [ESMO, PMID 36669645](https://pubmed.ncbi.nlm.nih.gov/36669645/) |
| D07 diffuse large B-cell lymphoma | Which diagnosis and study classes are described? | A non-Hodgkin lymphoma subtype with expression-defined study subgroups. A historical cohort does not define every current WHO/ICC subtype. | [Alizadeh, PMID 10676951](https://pubmed.ncbi.nlm.nih.gov/10676951/) |
| D08 melanoma | Which broad identity links the terminology? | Broad melanoma; exact captured MONDO:0005105. A search rank cannot select it, and cutaneous-only scope cannot replace it. | [OncoTree, PMID 33625877](https://pubmed.ncbi.nlm.nih.gov/33625877/) |
| D09 cutaneous melanoma | Which disease does genomic classification concern? | Cutaneous melanoma. The study's BRAF/RAS/NF1/triple-WT classes are not hereditary diagnoses or disease synonyms. | [TCGA, PMID 26091043](https://pubmed.ncbi.nlm.nih.gov/26091043/) |
| D10 uveal melanoma | Is this the cutaneous disease or a distinct site? | Uveal-tract melanoma with four study subsets. No transfer of cutaneous prognosis or therapy. | [Robertson, PMID 28810145](https://pubmed.ncbi.nlm.nih.gov/28810145/) |
| D11 colorectal cancer | Which genes associate with the studied tumors? | APC, TP53, SMAD4, PIK3CA and KRAS are among somatic cohort associations. This is not a germline syndrome gene list. | [TCGA, PMID 22810696](https://pubmed.ncbi.nlm.nih.gov/22810696/) |
| D12 Lynch syndrome | Which genes and historical name does the guideline cover? | Germline MLH1/MSH2/MSH6/PMS2/EPCAM context; HNPCC wording needs source scope. MMR-deficient tumor does not alone prove Lynch syndrome. | [BSG/ACPGBI/UKCGG guideline](https://pmc.ncbi.nlm.nih.gov/articles/PMC7034349/), [NCI genetics PDQ](https://www.cancer.gov/types/colorectal/hp/colorectal-genetics-pdq) |
| D13 Li-Fraumeni syndrome | Which gene and cancer spectrum define the guideline's syndrome? | Germline TP53-related predisposition; preserve the broader hTP53-related concept separately. MedGen distinguishes direct TP53 from related CHEK2. | [Frebourg, PMID 32457520](https://pubmed.ncbi.nlm.nih.gov/32457520/), [MedGen](https://www.ncbi.nlm.nih.gov/medgen/88399) |
| D14 von Hippel-Lindau disease | Which gene and phenotypes define the syndrome? | Germline VHL context with source-described tumors and lesions. Phenotype associations supply no diagnosis or individual probability. | [Maher, PMID 21386872](https://pmc.ncbi.nlm.nih.gov/articles/PMC3110036/) |
| D15 Birt-Hogg-Dubé syndrome | When should the diagnosis be considered? | Guideline features include pneumothorax, pulmonary cysts, renal neoplasia and fibrofolliculomas; constitutional FLCN context. One feature does not diagnose the syndrome. Obsolete MONDO metadata must be explicit. | [ERN GENTURIS, PMID 39085584](https://pubmed.ncbi.nlm.nih.gov/39085584/), [publisher guideline](https://www.genturis.eu/l=eng/Assets/20240731_BHD_guideline_public_v1.pdf) |
| D16 hereditary leiomyomatosis and renal cell cancer | Which manifestations and gene association define the syndrome? | Heterozygous germline FH context with cutaneous/uterine leiomyomas and RCC; distinct from biallelic FH fumarase deficiency or every sporadic FH-deficient tumor. | [Menko, PMID 25012257](https://pubmed.ncbi.nlm.nih.gov/25012257/), [NCI HLRCC PDQ](https://www.cancer.gov/publications/pdq/information-summaries/genetics/hlrcc-syndrome-hp-pdq) |
| D17 medullary thyroid carcinoma | How does hereditary versus apparently sporadic MTC relate to RET? | Germline RET association belongs to hereditary context; presentation alone does not exclude it. No inference that every MTC is MEN2. | [ATA, PMID 25810047](https://pmc.ncbi.nlm.nih.gov/articles/PMC4490627/) |
| D18 multiple endocrine neoplasia type 2 | Which subtypes and relations does ATA use? | MEN2A/MEN2B, with source-edition-specific FMTC and phenotype relations. MEN2, MEN2A and MTC are not flat synonyms. | [ATA guideline, sections C/D](https://pmc.ncbi.nlm.nih.gov/articles/PMC4490627/) |
| D19 lung and bronchus cancer | What incidence applies to this exact SEER site? | Snapshot measure must retain age adjustment, population, cases period and units. It is not a LUAD or NSCLC statistic. | [Cancer statistics 2026, PMID 41528114](https://pubmed.ncbi.nlm.nih.gov/41528114/), [SEER lung](https://seer.cancer.gov/statfacts/html/lungb.html) |
| D20 female breast cancer | What five-year survival applies to this population? | Five-year relative survival for the source's female breast, all-stage population and period. No TNBC or individual prognosis inference. | [Cancer statistics 2026](https://pubmed.ncbi.nlm.nih.gov/41528114/), [SEER breast](https://seer.cancer.gov/statfacts/html/breast.html) |
| D21 MF | Which disease does the abbreviation select? | It cannot select one. Published usage includes mycosis fungoides and myelofibrosis; actual MyDisease candidates include myotonia fluctuans and mycosis fungoides. Preserve actual source candidates, not expected expansions. | [CTCL update, PMID 36226409](https://pubmed.ncbi.nlm.nih.gov/36226409/), [myelofibrosis update, PMID 36680511](https://pubmed.ncbi.nlm.nih.gov/36680511/) |
| D22 gastrointestinal stromal tumor | Which disease and gene classes does the care guideline identify? | Digestive-tract mesenchymal malignancy; KIT/PDGFRA-mutant and WT classes with SDH context. Most mutations are sporadic. Associations alone imply neither germline causation nor treatment. | [GEIS, PMID 37655207](https://pmc.ncbi.nlm.nih.gov/articles/PMC10467260/) |
| D23 NUT carcinoma | Which tumor identity does the clinical cohort concern? | NUTM1-rearranged carcinoma; not every NUTM1-fusion tumor is this carcinoma. A source-local empty is only a technical overlay. | [Kloker, PMID 38301600](https://pubmed.ncbi.nlm.nih.gov/38301600/), [NCI PDQ](https://www.cancer.gov/types/midline/hp-child-midline-tract-carcinoma-treatment-pdq) |
| D24 mycosis fungoides | Which clinical parent group contains this diagnosis? | Cutaneous T-cell lymphoma; distinguish Sézary syndrome. A provider failure supplies no API answer and does not negate the diagnosis. | [CTCL update](https://pubmed.ncbi.nlm.nih.gov/36226409/) |
| D25, reuses D05 requests | Why do ICD-O3 diagnoses and OncoTree assignments conflict? | The public dataset team describes separately collected clinical fields and recommends its curated BPC fields for that dataset. Preserve both identities and discordance; no universal ontology precedence rule follows. | [Public discussion](https://www.synapse.org/Synapse:syn27056172/discussion/threadId=11533) |

The clinical reviewer confirmed the answers with high confidence and preserved the cited editions' limits. Exact API crosswalks still need captured publisher fields. A source-supplied ICD-10-GM skin-melanoma code must not become a broad melanoma code or an ICD-10-CM code; the [BfArM 2025 C43 page](https://klassifikationen.bfarm.de/icd-10-gm/kode-suche/htmlgm2025/block-c43-c44.htm) makes its skin scope explicit.

## Released 0.9 accounting

The shared baseline download tried the official v0.9.4 release, received 404, and used explicitly labeled released **0.9.3**. The binary SHA-256 is `54f66769d9193c569d1b65756ad766eafd1a2c5e92d08f8c855cc8964e9a7145`; archive SHA-256 is `e637062c18c6692f3ec49c5847e69b29f6feee7f7909345970bd504d72174994`. The installed 0.9.1 executable was not used. Actual released help confirms the search/get options below. Repository source inspection identifies their implementation contracts; it is not a claim that every optional source was measured live.

| 0.9 surface | Proposed disposition | Reason and proof requirement |
|---|---|---|
| `search disease QUERY`; `-q`, `--query` | Keep literal query; prefer explicit `--query` | The source applies its own matching. No colon-shape routing, hand-made boosts or silent query rewriting. D01–D24 cover literal search. |
| `search disease --source mondo|doid|mesh` | Change to distinct provider and ontology selectors | A provider is not an ontology. Old MyDisease source selection checks field presence. A positive direct-source case must prove each retained selector. |
| `--inheritance` | Do not port its hand-written clinical phrase vocabulary | A future exact HPO ID filter needs a positive captured field and scope. No inherited syndrome diagnosis from a returned association. |
| `--phenotype` | Defer an explicit HPO-ID selector until positive filter proof | The old request searches `hpo.phenotype_related_to_disease.hpo_id`, not phenotype label text. No unsupported label-to-ID guessing. |
| `--onset` | Drop old field mapping; reconsider only a proved native onset filter | Old request targets `hpo.clinical_course.hpo_name`. Record the field mismatch; do not rename clinical-course facts into onset facts. A positive source witness is required before adding an onset option. |
| `--no-fallback` | Retire | The new search has no discover fallback. A valid zero remains zero. |
| Search `-l`, `--limit`, `--offset` | Keep bounded pagination | Native totals and source order remain separate from displayed rows. Native cursors cannot masquerade as offsets. |
| `get disease NAME_OR_ID`; `--name` | Change to explicit ID get; names use search | Ambiguity returns candidates. No best-match selection or free-text semantic parser. Typed IDs retain exact namespaces. |
| Get positional section words | Keep proved named sections | One section cannot consume a disease name accidentally. Explicit ID syntax resolves the old multi-word ambiguity. |
| `genes` | Keep attributed native associations | D11–D18 and D22 need context, relation, original subject/object and evidence. No merged clinical score or local causal label. |
| `phenotypes` | Keep source-qualified rows | D13–D18 need phenotype, evidence, negation and native frequency qualifiers where present. |
| `clinical_features` | Rename to `phenotypes` | It is an alternate presentation of the same Monarch/HPO rows, not an independent clinical interpretation. |
| `pathways` | Drop | Old route derives Reactome pathways from associated genes. No harvested care question establishes a needed disease-to-pathway join. |
| `diagnostics` | Drop from this slice | It fans out to GTR/WHO IVD. The disease examples need no diagnostic catalog. The disease card does not imply a test recommendation. |
| `variants` | Rename to source evidence drill-down | Preserve CIViC molecular-profile identity and full compound context. A disease match is not a chosen variant or a universal disease-gene claim. |
| `civic` | Keep explicit source evidence | Native DOID, evidence/profile IDs, assertion type, direction and publication remain linked. No inference from a matching disease name. |
| `models` | Drop | Model-organism evidence does not serve the cancer-care identity questions. |
| `prevalence` | Change to explicit typed statistics only where public source fields prove it | Open Targets prevalence-like evidence is not interchangeable with SEER incidence or survival. No invented unit or denominator. |
| `survival` | Replace inferred mapping with explicit SEER site/statistic query | D19/D20 require exact native population and period. The old name-to-site table is prohibited. |
| `funding` | Drop | NIH grant amounts do not answer the harvested care questions. |
| `disgenet` | Keep dropped | The gene slice already dropped it; this investigation found no indispensable new question that changes that ruling. No keyed request was made. |
| `all` | Keep only a bounded explicit request for supported sections | The default identity card makes no enrichment fanout. Every requested section keeps its own status and page. |
| Hidden default `treatments`/`recruiting_trials` enrichment | Drop from default; use explicit existing drug/trial commands | The measured breast get calls MyChem indication search and ClinicalTrials.gov without a named section. Identity lookup must not silently perform those tasks. |
| `disease trials NAME`; `--source`, `--limit`, `--offset` | Map to the accepted trial surface | Literal `search trial --condition`; explicit NCIt route for NCI. Proposed `--condition-id` needs PM review. |
| `disease articles NAME`; `--limit`, `--offset` | Map to explicit article disease/text search | No ontology resolution or broadening by default. Article behavior remains owned by that slice. |
| `disease drugs NAME`; `--limit`, `--offset` | Map to explicit drug indication search if drug design accepts it | No implicit drug source fallback or disease synonym expansion. The drug design owns supported filters. |
| `list disease` | Replace with generated disease help | Generate command/section/option accounting from the implemented request tree. No second hand-written clinical vocabulary. |
| Adjacent `search phenotype` and its text/ID/limit/offset inputs | Drop inferred text-to-phenotype ranking from this slice; explicit ID similarity remains a separate PM choice | D13–D18 ask for known-syndrome facts, not a scored patient diagnosis. A source-native similarity route would need its own harvested question and explicit semantics. |
| Shared `-j`, `--json` | Map to existing `--format json` | Preserve the common envelope and CLI/MCP parity. |
| Shared `--no-cache` | Use the existing explicit transport cache policy | No new disease-only cache control. Capture and replay use the shared transport. |
| Shared `--disable-sources`, help | Keep existing shared settings/help | Enforce before cache/network access; report skipped/disabled with registry terms. |

These dispositions are proposals for PM review, not additional approved product rulings. Every retained narrowing filter needs its own known-positive direct request and case. The 0.9 code's `icd10cm` crosswalk querying `umls.icd10am` is a namespace mismatch; the new identity model must preserve those coding systems separately. No interpretation of their clinical interchangeability is made.

## Direct source findings and measured comparison

The source capture jobs made **142 direct requests** from `imaurer-m5` on 2026-10-24, using one attempt, no local response cache, a 20-second request deadline and serial admission. They stayed inside the 180-request cap. Every clinical query returned HTTP 200. The HPO repository `LICENSE` URL returned 404; that is a failed terms lookup, not an HPO disease absence. No 429, 5xx or timeout occurred, so no live outage example was established. Failure, broken-body and timeout acceptance cases must use the existing recorded transport mechanism; they must not be called newly observed provider failures.

MyDisease, OLS, Open Targets and CIViC received every literal D01–D24 query. The other sources received only applicable typed identity, phenotype or site questions. No identifier was guessed to make an unrelated source answer every question. D25 reuses D05 identity requests. Source totals count different things: ontology hits, Open Targets search hits and CIViC evidence items are not directly comparable measures of clinical completeness.

| Source | Direct requests, including terms | Observed latency min/median/max, ms | Public-tier fields proved and limit | Proposed choice |
|---|---:|---:|---|---|
| MyDisease.info | 33 | 259/283/378 | Candidate IDs, MONDO label, typed synonym groups, parents, xrefs; exact annotation includes native HPO fields where present. Literal ID query can return zero while exact annotation succeeds. | Keep for candidate discovery and explicit exact annotation; no automatic first hit. |
| OLS | 34 | 413/566/4468 | Namespaced concepts, labels, definitions, scoped synonyms, xrefs, obsolete flag, `consider` and replacement metadata. | Keep as explicit ontology source and exact metadata drill-down. No automatic fallback after MyDisease failure. |
| Monarch | 7 | 125/158/243 | Native subject/object, predicates, association IDs, original IDs, primary/aggregator sources, evidence, negation and optional qualifiers. Returned edges can match descendants through closure. | Keep native association sections; preserve actual edge endpoints and explicit expanded scope. |
| HPO | 3 | 127/202/214 | Exact HP term identity, name and definition on two term requests. The attempted raw license path was absent. | Keep exact term metadata; resolve publisher terms before implementation. No independent disease diagnosis/ranker. |
| Open Targets | 32 | 216/238/971 | Source-ranked disease candidates, source IDs, association count, target IDs/symbols, native overall and datasource scores. | Keep a named association source. No local score blending or EFO-first identity selection. |
| CIViC | 25 | 190/223/351 | Accepted evidence totals, native disease name/DOID, evidence/profile IDs, type, level, direction, significance and publication identity. | Keep explicit source evidence, not a default identity enrichment. |
| SEER | 3 | 134/199/220 | Two Stat Facts HTML pages carry incidence/survival context; Explorer catalog carries native site vocabulary. No structured statistics endpoint was queried with a selected site. | Keep explicit native site statistics provisionally; extraction contract remains a design question. |
| Reactome | 2 | 190/220/251 | Disease search response and publisher terms page. A broad result is not an established disease-to-pathway association. | Drop from this slice: no harvested question needs the old gene-to-pathway fanout. |
| MedlinePlus | 2 | 219/413/608 | Health-topic XML for Li-Fraumeni query and terms page. Topic discovery is not a disease-identifier equivalence service. | Drop from this slice: reference links can remain source links; no extra patient-education section is required by the cases. |
| OncoTree | 1 | 1169/1169/1169 | Native tumor tree with code, name and source cross-references. | Use only captured cross-reference facts or an explicit future provider choice; no name-based join. |
| UMLS | 0 | Unmeasured | No key request or key inspection. | Drop from the initial slice; no indispensable question justifies a keyed dependency. A future ordinary configured-key smoke needs its own approved recipe. |
| DisGeNET / NIH RePORTER | 0 | Unmeasured | Standing gene-slice drops; no new indispensable clinical question. | Keep dropped. |

Terms remain source and component facts. [MONDO's publisher](https://mondo.monarchinitiative.org/pages/download/) names CC BY 4.0; [OLS's host terms](https://www.ebi.ac.uk/about/terms-of-use/) do not replace ontology-specific rights. [Open Targets](https://platform-docs.opentargets.org/licence) marks its platform data CC0 and documents its integrated source arrangements. [CIViC](https://civicdb.org/releases/licensing) dedicates its data under CC0. [Reactome](https://reactome.org/license) distinguishes data from illustrations and other materials. [MedlinePlus](https://medlineplus.gov/about/using/usingcontent/) distinguishes public-domain summaries from separately licensed material. [Monarch's current licensing guidance](https://monarch-app.monarchinitiative.org/Licensing/) recommends licenses and explicitly leaves repository-specific terms authoritative; it is not a blanket relicensing of every upstream association. The [HPO repository](https://github.com/obophenotype/human-phenotype-ontology) directs readers to its [publisher license page](https://hpo.jax.org/app/license). An old HPO website license and Open Targets' HPO integration are not proof of the current direct API's complete terms. Raw unknown-rights replies stay outside Git. The existing registry must carry each finally reviewed source/component use label, terms name and publisher page; no open endpoint grants additional rights.

The following table records native source totals and the same-question released baseline's time and stdout bytes. The baseline command is `search disease --query TERM --limit 5 --no-fallback --json`, using an isolated home/config/cache with keys absent. All 24 searches exited zero. Times describe this loaded M5 run, not an isolated speed benchmark. Direct HTTP timings above and end-to-end product timings below measure different work.

| Case | MyDisease hits | OLS hits | Open Targets hits | CIViC evidence items | 0.9.3 ms | 0.9.3 bytes | Illustrative proposed search bytes |
|---|---:|---:|---:|---:|---:|---:|---:|
| D01 | 60 | 1414 | 2901 | 317 | 459 | 1356 | 1083 |
| D02 | 0 | 1 | 1 | 0 | 275 | 678 | 254 |
| D03 | 1 | 17 | 1 | 8 | 324 | 723 | 466 |
| D04 | 1 | 5 | 1 | 0 | 329 | 176 | 434 |
| D05 | 14 | 176 | 186 | 126 | 279 | 1418 | 1180 |
| D06 | 2 | 141 | 3 | 0 | 278 | 934 | 681 |
| D07 | 28 | 394 | 2 | 19 | 293 | 1173 | 1216 |
| D08 | 126 | 1825 | 2322 | 235 | 275 | 1322 | 1183 |
| D09 | 20 | 377 | 123 | 0 | 272 | 176 | 1212 |
| D10 | 17 | 135 | 26 | 17 | 262 | 1241 | 1171 |
| D11 | 33 | 571 | 543 | 355 | 343 | 1731 | 1143 |
| D12 | 8 | 26 | 648 | 1 | 259 | 957 | 1125 |
| D13 | 4 | 52 | 2254 | 0 | 279 | 816 | 890 |
| D14 | 2 | 19 | 1 | 625 | 280 | 853 | 738 |
| D15 | 2 | 1 | 2 | 0 | 278 | 176 | 650 |
| D16 | 2 | 3 | 1 | 0 | 330 | 176 | 634 |
| D17 | 4 | 112 | 755 | 18 | 281 | 1105 | 1098 |
| D18 | 2 | 15 | 1 | 0 | 269 | 176 | 628 |
| D19 | 1 | 78 | 3 | 0 | 358 | 176 | 446 |
| D20 | 8 | 25 | 152 | 0 | 362 | 1463 | 1217 |
| D21 | 2 | 285 | 8278 | 0 | 281 | 888 | 698 |
| D22 | 8 | 190 | 143 | 89 | 287 | 719 | 1254 |
| D23 | 1 | 33 | 206 | 0 | 275 | 762 | 465 |
| D24 | 8 | 205 | 5 | 0 | 290 | 707 | 1195 |

The last column measures compact JSON produced by an external illustrative serializer over the exact captured MyDisease rows. It is not a product output or a schema acceptance. It retains native IDs, labels, cross-reference namespaces, total, source terms reference and exact next IDs. It does not silently select a concept. It lacks the final shared envelope, fully reviewed component terms and final section availability, so it cannot establish the implementation's ceiling. Its maximum is 1,254 bytes. Where it exceeds the baseline, the additional facts are preserved source candidates, native totals, exact namespaces and provenance; several baseline replies are 176-byte empty objects despite positive direct-source candidates. That is a fact difference, not a token-saving success. An identified tokenizer was unavailable in the job; actual token counts remain unmeasured. The runner's byte-derived `max_tokens` estimate is not a tokenizer measurement. Byte ceilings remain the enforceable design proposal until actual product envelopes exist.

Repeated D01/D08/D21 baseline searches used the same isolated cache and returned identical stdout hashes in 19/9/7 ms, versus 459/275/281 ms on their first invocations. No HTTP trace was collected, so their actual HTTP request counts remain unknown; a fast repeated call alone does not prove zero requests. The one default exact breast get took 2,202 ms and 8,462 bytes. It returned identity/definition/parents/synonyms/xrefs, Open Targets gene scores, a treatment landscape and recruiting-trial count. Its native outcomes show MyChem and ClinicalTrials.gov data despite no explicit section. This is the concrete reason to keep the new default identity card free of hidden enrichment.

The baseline completed 28 product invocations plus version/help checks before the coordinator imposed an offline-only rule for further comparisons. Four planned get/genes/phenotypes commands did not execute. No further live baseline call was made. The PM subsequently required a separate ticket and verified network containment for any future 0.9 execution. The stop was an experiment classifier error: a raw-byte `429` search matched digits inside a decimal gene score. Inspection of structured source outcomes found no 429 or provider failure. The executed script was overwritten before preservation; the evidence store labels its reconstructed original separately from the amended, unexecuted proposal. A retained-body comparison must match actual request identity through the existing TLS recipe. The new direct literal requests and old rewritten requests are different identities; this design does not relabel one as proof of the other. No matched offline section comparison or full per-section token measurement is claimed.

## Identity and section witnesses

D08 preserves the exact melanoma concept at fourth place in MyDisease's first five candidates. D01's broad breast concept is absent from that provider's first five literal hits, while OLS returns MONDO:0007254 first. That is a provider ranking difference, never permission to rerank locally. D02's literal MyDisease query is a genuine zero; the exact annotation endpoint returns the explicit identity. The command must distinguish search syntax from get identity instead of silently repairing the query.

D21 returns MyDisease MONDO:0020481, `myotonia fluctuans`, and MONDO:0009691, `mycosis fungoides`. The published question's possible myelofibrosis expansion is not a label for MONDO:0020481. Do not suppress a returned candidate using a cancer keyword rule. Let the caller choose an explicit concept, or leave the question unresolved.

D15's exact OLS record for MONDO:0007607 has `is_obsolete: true`, two `consider` values (MONDO:0800444 and MONDO:0800445), and null `term_replaced_by`. The captured current concepts distinguish Birt-Hogg-Dube syndrome from Birt-Hogg-Dube syndrome 1. They are not an automatic one-to-one replacement. Preserve the source's actual exact-match, close-match and generic cross-reference predicates; do not flatten all three to equivalence.

Monarch's D13, D14 and D18 five-row gene pages have native totals 6, 5 and 8, while phenotype totals are 48, 60 and 100. These are association counts, not unique causal-gene counts or patient risks. D18's request for MONDO:0019003 returns a first gene edge whose actual object is MONDO:0008234 (MEN2A), with the requested concept in its closure. The adapter must expose that descendant scope and retain the actual edge object. It cannot relabel the edge as an exact direct association with the queried parent. Native `has_phenotype`, negation, original OMIM/Orphanet identities, evidence codes and optional qualifiers stay attached to their individual rows. A null qualifier remains absent. The primary knowledge source governs its own terms.

CIViC's disease-name search proves a discovery capability. Returned rows carry native DOID and molecular-profile IDs. A name query alone does not prove an exact join to a selected MONDO concept. A future get section must use a publisher-supported cross-reference and validate each returned source disease ID, or return source candidates and an explicit missing-join problem. It must not fabricate a disease ID selector that was not captured. Compound molecular profiles retain their full membership and native claim context.

D19's captured Stat Facts page states 47.2 new lung/bronchus cases per 100,000 men and women per year, age-adjusted, based on 2019–2023 cases. The same page's five-year relative survival is 29.5%, based on SEER 21 excluding Illinois for 2016–2022. D20's page states 91.9% five-year relative survival for female breast cancer in that survival dataset/period, and separately 132.5 new cases per 100,000 women per year for 2019–2023. A parser cannot substitute a similarly named ontology subtype, another trend statistic, projected cases or a different period. The source pages and hashes, not a copied constant, anchor future cases. A structured Explorer site request still needs its own exact native site selection and response validation.

HPO term metadata establishes exact term lookup, not a direct disease-to-phenotype endpoint or a phenotype-based diagnosis. MyDisease and Monarch can carry separately attributed HPO annotations. The design must not give their aggregate a blanket ontology-only license. No source was asked to classify a patient's signs.

## Structural risks and case harvest

Identity, terms and command facts each have one owner. The disease adapter must not restate shared source registry terms or maintain its own clinical synonym table. It must not copy consumer request types into neutral `biodata`, nor put provider/CLI rules into the neutral layer. The source-specific edge predicate and the source's subject/object remain the truth; deriving a concise header must not erase them.

Cases should follow the D01–D25 stories and retained capture identities. At minimum they cover every accepted command and option, exact identifier checks, namespaced cross-references, first/middle/last page and next actions, duplicate IDs, multiple candidate concepts, obsolete concepts with `consider`, graph expansion versus direct edges, source-native empty, disabled and missing-key skips before network, 429, 500, timeout, broken body and GraphQL data-plus-errors. These last failure cases are required recorded witnesses, not live failures claimed by this investigation. Each named filter needs a captured positive and a contrasting negative. Keep raw provider bodies outside Git where rights are unknown; publish only source-permitted facts or synthetic wire cases. No licensed clinical answer set enters the repository or an agent under test.

The statistics cases must assert measure, population, period, site and dataset together. The assertion that a number exists is insufficient. The ambiguous and obsolete cases must assert that no chosen identity or executable unsafe join is returned. Each association case must assert original/native endpoints and predicate, not only a gene symbol. Section budgets apply to the actual full envelope after implementation; illustrative JSON measurements cannot replace that proof.

## Questions for PM review

1. **Which provider should search use by default?** MyDisease is faster and supplies useful cross-reference namespaces, but its literal broad-cancer ordering can bury the broad concept. OLS supplies richer ontology metadata and puts several broad concepts first in these captures, at higher latency and response size. Explicit source selection costs one request and preserves each provider's semantics. Recommendation: default to OLS for ontology discovery, retain explicit MyDisease for exact annotations/resolution, and show all source-ranked candidates without automatic selection. This is a proposal; the illustrative MyDisease byte measurements do not establish an OLS product envelope budget. A multi-provider default would add requests and larger envelopes without proving a clinical identity choice.
2. **How should SEER statistics enter this slice?** Option A uses exact source site IDs and captured Stat Facts fields with a maintained structural HTML contract. Option B first proves a structured Explorer request after the caller selects a native site; a field missing from that public tier stays unsupported. Both require explicit population/period and independent source metadata. Recommendation: retain the explicit site command in design and require one structured site proof before implementation; do not revive the old name-to-site table. No whole dataset download or new paid service is proposed.
3. **Which association scope should Monarch use?** Its captured endpoint can return descendants through closure. One option preserves the native expanded scope and endpoints; another requires a separately proved direct-only provider selector. Recommendation: preserve and label the native scope now; do not locally promote child rows into parent claims or claim an unsupported direct-only filter.
4. **Should the trial surface accept `--condition-id`?** Keeping only literal `--condition` makes callers resolve and copy the source label themselves. A separate exact-ID option adds one explicit resolution request and preserves the chosen identity and NCIt cross-reference. Recommendation: accept the distinct option under the disease identity contract; ambiguity and missing NCIt stop before the trial source call. No automatic name recognition or trial eligibility judgment follows.
5. **Are the proposed drops acceptable?** Dropping Reactome, MedlinePlus, UMLS, models, funding, diagnostics, clinical phrase filters and automatic related-entity enrichment removes extra requests and unsupported joins. Keeping any of them needs a harvested cancer-care question, a public-tier positive capture and explicit source terms. Recommendation: accept the drops and retain explicit existing article/trial/drug routes for those separate tasks.
6. **How should the remaining measurement limits close?** The current investigation has matched search bytes/time and one default get, but no complete section/token comparison, no verified offline network-blocking baseline recipe, no live failure witness, no selected structured SEER statistics request, and incomplete direct HPO/Monarch upstream terms classification. Recommendation: review the identity/source design now; require these narrowly named proof gaps to close before claiming final section budgets or source acceptance. Any future 0.9 execution needs a separate PM ticket and verified network containment. Do not repeat live baseline fanout or label the unfinished measurements complete. HPO license resolution is research work, not a request for Ian to supply configuration.

## Evidence

All new captures and the released baseline are immutable external evidence on Yellow under `/home/ian/workspace/data/biomcp-redux/0117-disease-investigation/`. The source directory is `biomcp-redux-0117-disease-captures`; its `requests.json` SHA-256 is `f61f5c95d7c43d34d71d46cd75fe6e3f2abfb2f0022bb317c878535ec5d4d566`. Each row stores the exact URL/body, UTC retrieval time, HTTP status, elapsed time, body byte count and SHA-256. All 142 copied body hashes were independently checked on Yellow. The initial direct capture ran as `j11373`; the follow-up ran as `j11381`.

The baseline directory is `biomcp-redux-0117-disease-baseline`; its `manifest.json` SHA-256 is `a9da53b5d52861cf13cb7af9c460968e65d55e3c0ea20796b52f412b31d8d32b`. All 31 copied stdout hashes were checked on Yellow. It ran as `j11378`. HTTP request counts remain unknown. The parent directory preserves `capture.py`, both exact direct request plans, an explicitly reconstructed executed baseline script, the amended but unexecuted baseline proposal and the original bounded experiment plan. None contains credentials. The shared released executable and archive provenance remain owned by the parallel drug investigation until final cleanup.

The illustrative serializer and `biomcp-redux-0117-disease-analysis/` preserve each proposed JSON and the case matrix. Analysis job `j11396` made no provider requests. The matrix SHA-256 is `ec92e1271f0469c82a6b06918302366c261df6b0de1cef7b757d0ba502aa8a96`.

Retained trial evidence is reused without a refresh from `/home/ian/workspace/data/biomcp-redux/0079-trial-investigation/`, captured on 2026-10-24 in the original trial investigation. `mydisease-metadata.raw` SHA-256 is `0e25bc4ea24f7a2ae569dccca452004be2e8d8cf2974dedb414fa955e48c8a06`; `mondo-license.raw` SHA-256 is `f5b745ef98087f531e719ee8ca6a96809444573ecc7173c6fa68eaad39b3cc3f`. Reusing those files is not a newly observed provider response. No product code, schema, roadmap box or later ticket was changed by this design.
