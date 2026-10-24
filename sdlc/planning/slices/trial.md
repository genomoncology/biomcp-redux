# Trial slice

This design follows steps 1 through 4 of [the slice method](../slices.md). It proposes the trial slice for milestone 2. It changes no product code. The investigation starts from `7a7ce2a0f09b5981353753e2f1c94bb39484b06c`.

The central contract is direct retrieval. BioMCP echoes the exact source query, preserves the registry's order and total, returns trial headers, and offers bounded sections. A text match discovers a trial. It never proves that a person meets its eligibility criteria. No local rule classifies inclusion, exclusion, mutation relevance, treatment line, or suitability.

## Evidence and limits

ClinicalTrials.gov captures use its public v2 API without a key. Each request has a bounded timeout, runs serially, and leaves a raw response and a request manifest. Measurements identify the host, executable, source revision and capture date. These are observations of one source snapshot, not permanent counts or speed promises.

A fresh clinical reviewer checked the question bank below against primary publications. Its confirmed answers describe each publication's population, intervention, comparison or endpoint. Current recruitment, locations and search counts come from the dated registry captures. Differences between a paper and a later registration remain attributed to their respective sources and dates.

No dedicated clinical ruling file was named in the assignment or found in this repo. The review is retained here beside the questions it governs. The reviewer changed no product code. Neither the public question bank nor its answers use the curation team's licensed answer bank.

## Questions and accepted answers

Questions are adapted from publications. They are not quotations. Mechanical followups are marked. Each row names the exact trial to preserve through search, get and section requests. The acceptance target is retrieval of the stated source facts, not a patient eligibility judgment.

| Case | Adapted question | Accepted answer | Source |
| --- | --- | --- | --- |
| T01 | Which trial compared first-line osimertinib with gefitinib or erlotinib in advanced NSCLC with EGFR exon 19 deletion or L858R? | FLAURA, NCT02296125. Do not broaden the two named entry alterations to all EGFR alterations. | [Soria 2018, PMID 29151359](https://pubmed.ncbi.nlm.nih.gov/29151359/) |
| T02 | What did FLAURA require regarding prior treatment for advanced disease? | NCT02296125 studied previously untreated advanced NSCLC. Return the complete registry eligibility text, including exceptions. | [FLAURA](https://pubmed.ncbi.nlm.nih.gov/29151359/) |
| T03 | What phase and primary endpoint did FLAURA report? | Phase 3; investigator-assessed progression-free survival. Preserve each registered primary outcome and its time frame. | [FLAURA](https://pubmed.ncbi.nlm.nih.gov/29151359/) |
| T04 | Which trial tested pembrolizumab with pemetrexed and platinum for previously untreated metastatic nonsquamous NSCLC? | KEYNOTE-189, NCT02578680. A pembrolizumab monotherapy trial is a different comparison. | [Gandhi 2018, PMID 29658856](https://pubmed.ncbi.nlm.nih.gov/29658856/) |
| T05 | What EGFR and ALK restriction did KEYNOTE-189 report? | NCT02578680 enrolled a population without sensitizing EGFR or ALK mutations. A keyword occurrence does not establish a positive mutation requirement. | [KEYNOTE-189](https://pubmed.ncbi.nlm.nih.gov/29658856/) |
| T06 | What were KEYNOTE-189's primary outcomes? | Overall survival and progression-free survival. Preserve radiologic review wording where the registry supplies it. | [KEYNOTE-189](https://pubmed.ncbi.nlm.nih.gov/29658856/) |
| T07 | Which trial studied trastuzumab deruxtecan in HER2-low metastatic breast cancer, and how did its publication define HER2-low? | DESTINY-Breast04, NCT03734029; IHC 1+ or IHC 2+/ISH-negative. Do not equate this with any ERBB2 sequence alteration. | [Modi 2025, PMID 41062831](https://pubmed.ncbi.nlm.nih.gov/41062831/) |
| T08 | What was the comparator in DESTINY-Breast04? | Physician's choice of chemotherapy versus trastuzumab deruxtecan. Keep permitted comparator interventions distinct from a combination regimen. | [DESTINY-Breast04](https://pubmed.ncbi.nlm.nih.gov/41062831/) |
| T09 | Mechanical followup: which DESTINY-Breast04 sites does the registry list near New York City? | Use NCT03734029 and an explicit center and radius. Return source site coordinates and each site's status. Never substitute author affiliations, travel time, or a current recruitment claim. | [Publication](https://pubmed.ncbi.nlm.nih.gov/41062831/), [registry](https://clinicaltrials.gov/study/NCT03734029) |
| T10 | Which randomized trial compared sotorasib with docetaxel in KRAS G12C-mutated advanced NSCLC? | CodeBreaK 200, NCT04303780. Do not broaden G12C to all KRAS variants. | [de Langen 2023, PMID 36764316](https://pubmed.ncbi.nlm.nih.gov/36764316/) |
| T11 | What prior treatment did CodeBreaK 200 require in its published population? | Progression after platinum chemotherapy and a PD-1 or PD-L1 inhibitor. Return the registry's exact eligibility wording. | [CodeBreaK 200](https://pubmed.ncbi.nlm.nih.gov/36764316/) |
| T12 | Retrieve NCT04303780 and show its comparison arms. | Exact ID NCT04303780; sotorasib versus docetaxel. Do not substitute CodeBreaK 100 or 300. | [CodeBreaK 200](https://pubmed.ncbi.nlm.nih.gov/36764316/) |
| T13 | Which adjuvant trial studied olaparib in high-risk HER2-negative early breast cancer with germline BRCA1 or BRCA2 variants? | OlympiA, NCT02032823; pathogenic or likely pathogenic germline BRCA1/2 variants. This is not an unrestricted somatic-BRCA or metastatic population. | [Tutt 2021, PMID 34081848](https://pubmed.ncbi.nlm.nih.gov/34081848/) |
| T14 | What treatment duration and comparator did OlympiA report? | One year of oral olaparib versus placebo after its stated local treatment and chemotherapy. This is a study fact, not an individual treatment recommendation. | [OlympiA](https://pubmed.ncbi.nlm.nih.gov/34081848/) |
| T15 | What primary outcome did OlympiA use? | Invasive disease-free survival. Preserve its source definition and time frame. | [OlympiA](https://pubmed.ncbi.nlm.nih.gov/34081848/) |
| T16 | In PROfound, which alterations defined cohort A? | NCT02987543: BRCA1, BRCA2 or ATM. Other prespecified genes formed cohort B. | [Hussain 2020, PMID 32955174](https://pubmed.ncbi.nlm.nih.gov/32955174/) |
| T17 | What disease setting and prior progression did PROfound require? | Metastatic castration-resistant prostate cancer with qualifying HRR alterations and progression during prior next-generation hormonal treatment. A BRCA finding alone does not determine eligibility. | [PROfound](https://pubmed.ncbi.nlm.nih.gov/32955174/) |
| T18 | Mechanical followup: is PROfound recruiting, and what does its registry record say about sites? | Return the captured overall status and update date for NCT02987543. Return site statuses separately. A publication date or address cannot establish current recruitment. | [Publication](https://pubmed.ncbi.nlm.nih.gov/32955174/), [registry](https://clinicaltrials.gov/study/NCT02987543) |
| T19 | Which VISION population was selected using PSMA imaging? | NCT03511664: metastatic castration-resistant prostate cancer after the stated androgen-receptor-pathway inhibitor and taxane treatments, with qualifying gallium-68 PSMA-11 PET/CT scans. A generic PSMA mention does not establish that requirement. | [Sartor 2021, PMID 34161051](https://pubmed.ncbi.nlm.nih.gov/34161051/) |
| T20 | What did VISION compare? | Lutetium-177–PSMA-617 plus protocol-permitted standard care versus standard care alone. The comparator is not unrestricted treatment. | [VISION](https://pubmed.ncbi.nlm.nih.gov/34161051/) |
| T21 | Which primary publication and primary endpoints belong to VISION? | PMID 34161051; DOI 10.1056/NEJMoa2107322; imaging-based progression-free survival and overall survival. Keep registry reference types separate from publication links. | [VISION](https://pubmed.ncbi.nlm.nih.gov/34161051/) |
| T22 | Which first-line colorectal trial compared pembrolizumab with chemotherapy for MSI-H/dMMR metastatic disease? | KEYNOTE-177, NCT02563002; phase 3. Do not broaden its population to all colorectal cancer. | [André 2020, PMID 33264544](https://pubmed.ncbi.nlm.nih.gov/33264544/) |
| T23 | Mechanical followup: does exact NCT02563002 combined with a phase-1 restriction return studies? | Accept zero only after a successful response for the exact conjunction. Do not put a sole NCT ID into a query parameter that causes the source to ignore other filters. | [KEYNOTE-177](https://pubmed.ncbi.nlm.nih.gov/33264544/) |
| T24 | Mechanical failure variant: what should that lookup say if the registry fails? | The source could not answer and the match count is unknown. Never return zero or trial absence after a timeout, rate limit, server error or malformed body. This is a proposed retrieval contract, not an observed outage. | [Trial identity](https://pubmed.ncbi.nlm.nih.gov/33264544/) |

The clinical reviewer labels the publication facts confirmed with high confidence. T09 and T18 require dated registry captures. T23 is a query-semantic check. T24 is a proposed failure contract. A changed protocol or conflicting primary publication requires an attributed discrepancy; it does not authorize a silent correction.

## Source findings and query construction

ClinicalTrials.gov remains the default. [Its API](https://clinicaltrials.gov/data-api/api), [search areas](https://clinicaltrials.gov/api/v2/studies/search-areas), [metadata](https://clinicaltrials.gov/api/v2/studies/metadata) and [enums](https://clinicaltrials.gov/api/v2/studies/enums) are the source of field and enum names. Never invent a field because a CLI flag has a similar name.

Search uses source parameters for condition, intervention, text, location, overall status and geography. Explicit field restrictions use `filter.advanced`. Fixed construction is echoed verbatim. The source applies its own synonyms and relevance order. BioMCP never expands drugs with chemical-name guesses, rewrites disease abbreviations, infers treatment line, or removes exclusion-only text matches after retrieval.

`--condition` passes literal source text to `query.cond`. A checked disease identifier can be resolved explicitly through MyDisease; ambiguity returns candidates. That resolution must not silently replace an already supplied condition search with its highest-scoring ontology label. `--gene` uses the existing checked gene resolver and echoes the selected source-approved symbol as registry search text. It grants no molecular eligibility claim. A variant is text that the caller explicitly supplies through `--variant`; any shared typed variant normalization follows the variant slice's reviewed contract.

### Disclosure and commands

The proposed command tree keeps one path for each operation:

| Command | Default output | Next step |
| --- | --- | --- |
| `search trial [TEXT]` | One bounded page of headers, source total, exact request and source state | Opaque continuation; `get trial NCT…`; named sections |
| `search trial --count-only` | Source total and exact query, no rows | Search page with identical filters |
| `get trial NCT…` | Identity, title, overall status, phases, study type, conditions, intervention names, sponsor, dates and full brief summary | Available section selectors |
| `get trial NCT… eligibility` | Source age bounds, sex, healthy-volunteer flag and intact eligibility text | Bounded text continuation if necessary |
| `get trial NCT… locations` | Bounded site page, coordinates, site status and all contacts attached to each selected site | Site page continuation |
| `get trial NCT… contacts` | Central contacts plus bounded, explicitly linked site contacts | Locations |
| `get trial NCT… outcomes` | Registered primary, secondary and other outcomes with descriptions and time frames | Bounded outcome continuation |
| `get trial NCT… arms` | Exact arm labels and intervention relationships | Further arm/intervention pages |
| `get trial NCT… references` | Registry publication IDs, citation text and source reference type | `get article PMID` |
| `get trial NCT… documents` | Source document manifest, date, type, size if supplied and exact filename | Explicit file retrieval |
| `trial document NCT… FILE --save PATH` | Bounded bytes saved outside the response and a receipt | Caller chooses its own reader |
| `get trial NCT… all` | Explicitly requested sections with their states and caps | Continuations for every truncated section |

Document retrieval preserves the existing capability without turning the trial slice into a PDF reader. It follows the already reviewed shared binary transport, validates the advertised filename and fixed publisher CDN/redirect policy, and never invents a URL from a filename. Large posted results remain a separate proposed section if the source capture proves the fields and a harvested question needs them. Registered outcomes and posted results are different data.

### Proposed types and schemas

`NctId` is a checked identifier with the registry's exact grammar. Parse it once and compare the returned identity with the requested identity before admitting data to cache. A syntactically valid absent record is distinct from invalid input and from source failure.

Domain values belong in `biodata`: `TrialId`, `TrialHeader`, `TrialPhase`, `TrialStatus`, `StudyType`, `TrialAgeBound`, `TrialSex`, `TrialLocation`, `TrialContact`, `TrialOutcome`, `TrialArm`, `TrialIntervention`, `TrialReference` and `TrialDocument`. Reuse existing checked dates, URLs, coordinates, text, counts and pagination values. A source's missing age limit remains an explicit no-limit state; unparseable data preserves raw source text beside an unavailable normalized value. Do not assume a missing phase means phase 1 or a missing site status means recruiting.

Response types hold cards, page metadata, section states and typed next steps. Proposed schemas are `biomcp.trial_page/1`, `biomcp.trial_card/1`, `biomcp.trial_locations/1`, `biomcp.trial_outcomes/1`, `biomcp.trial_arms/1`, `biomcp.trial_references/1` and `biomcp.trial_documents/1`. Generate schemas from one set of types. Follow the existing shared enum macro; maintain no parallel hand-written enum or field registry.

An outcome's time frame stays attached to that outcome. An arm's intervention relationship stays attached to the source arm label or source identifier. A site contact stays attached to its source site. No derived clinical classification enters these types. Site geographic coordinates use an explicit latitude/longitude pair, not a genomic assembly type.

The adapter owns provider raw shapes and maps them once. The query layer owns commands and explicit source capabilities. Fetch owns deadlines, retries, pacing, decoding, cache admission, source policy, record/replay and binary limits. Store owns continuation state only where necessary. Views render the same envelope for CLI and MCP. There is no trial-only transport, retry loop, cache or global state.

### The agent's next questions

1. **What does the agent ask first?** “Find trials for this condition with this exact intervention or text.” A single `search trial` answers it with source totals and one page.
2. **What is the smallest useful header?** Exact NCT ID, complete brief title, overall status, phases, study type, conditions, intervention names, last registry update and a source URL. A header never claims that the trial accepts a patient's biomarker or location.
3. **Which drill-down comes next?** Eligibility, sites, arms and outcomes. The response names those sections as typed next steps and states whether data is present, absent or not established for this source.
4. **Which identities must survive?** NCT IDs, registry reference PMIDs, exact document filenames, arm labels and stable section offsets. Aliases never replace the returned canonical NCT ID.
5. **How are disagreements shown?** Keep source, timestamp and source-native field values. NCI and ClinicalTrials.gov statuses do not become an invented universal “active” state. Publication endpoints and later registered endpoints remain attributed separately.
6. **How does empty differ from failure?** A successful search with `totalCount: 0` is complete and empty. A 400, timeout, 429, 500, invalid body or unsupported filter has a source problem and no asserted count. A key-required NCI source is skipped before any request when its key is absent.
7. **What sets cost?** A search and a detail each need one projected registry request. Eligibility inspection is an explicit detail request, not an invisible per-result verification loop. Warm calls use the shared cache. Timings below describe the captured machine and cache state.
8. **What does 0.9 carry that these questions do not need?** Duplicate entity-specific pivots, chemical alias guessing, treatment-line phrase lists and post-retrieval inclusion/exclusion classification. Keep their useful retrieval questions through explicit source text and intact sections.
9. **When can an agent stop?** The search header answers identity/status discovery; the source's complete count answers a count question; an exact get plus one section answers a specific protocol question. More detail is never loaded merely because it exists.
10. **What supports trust?** An exact source query, registration update date, retrieval date, source URL, field provenance, current source state and an explicit warning that text matches are discovery only.

## Design decisions for review

These are product choices for PM review. None authorizes implementation before that review.

| Question | Options and trade-offs | Recommendation |
| --- | --- | --- |
| What should molecular text mean? | Keep distinct legacy biomarker/mutation heuristics; or expose explicit source text and eligibility-text search with no inference. Legacy behavior changes counts after retrieval and can mistake exclusions for requirements. | Keep `--variant` and `--gene` as explicitly labeled discovery inputs; `--criteria` searches source eligibility text. Echo the exact fields and never verify clinical inclusion locally. |
| Should `--age` remain? | Local conversion and post-filtering breaks source-count parity; native source range constraints retain parity but require proved no-limit semantics. | Keep only a native, captured source expression. Include source missing-limit handling explicitly and describe this as registry-bound filtering, not full eligibility. |
| Should facility and radius mean the same site? | Independent trial-wide terms may match different sites; a source-native nested location expression can bind them to one site. | Use the source's same-location grouping when both restrictions are requested. Refuse a combination the source cannot express honestly. |
| Should NCI be folded with ClinicalTrials.gov? | A fold requires additional identity/status/conflict rules; named sources preserve each source's own coverage and counts. | Default ClinicalTrials.gov; explicit `--source nci`, no automatic fallback or combined best order. |
| Should old therapy-line options survive? | Reproduce word lists; or remove semantic claims and offer literal eligibility text. | Drop `--line-of-therapy`, `--prior-therapies` and `--progression-on` as semantic filters. Preserve their questions with `--criteria` and full text; the agent inspects the passage. |
| Where should bulky detail live? | Full protocol in the default card; or bounded sections with honest continuations. | Header, card, then sections. Keep full criteria and descriptions recoverable without arbitrary sentence truncation. |
| Should documents be readable here? | Add another PDF parsing path; or list metadata and save exact advertised bytes. | List and save. Reuse binary transport and outbound policy; defer document interpretation. |

## Structural risks

The main risks are duplicated field mappings, hidden post-filter state, excessive requests and tests that prove a fake count. Define each source field mapping once; derive schema and help; capture the exact direct query beside every filter's known-positive case; compare the product total with that same captured `totalCount`. Keep source order and total before rendering. Use shared isolation for CLI/MCP cases and source record/replay. Cap sections and body sizes in enforcing code, with truthful continuations. Do not turn this design's illustrative budgets into a second configuration system.

The implementation sequence is types/schema, one source adapter, search/count, exact get, sections and document retrieval, optional NCI, then accounting/help and installed-command proof. Each stage gets outside-in cases from the bank; the PM chooses tickets after reviewing this design.
