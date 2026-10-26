# Disease slice investigation and implemented contract

The initial investigation starts from published revision `ecef1096239f8aeee8fcc6cbefc5720e89506035`. The clinical harvest and capture reviews establish the clinical limits below. Historical proposed JSON remains labeled as illustration. The implemented command, migration, measured budget and case tables identify the accepted product behavior and its evidence limits.

## Settled product contract

OLS with the MONDO ontology is the default discovery route. `search disease TEXT` sends positional text unchanged. The ontology selector is a closed set containing only `mondo`, `doid` and `ncit`; its default is `mondo`. Exactly one of those values is accepted per request. Other ontology names fail before I/O. There is no new `--query` option, local ranking, alias list or automatic identity choice. Explicit ontology/source selectors remain visible in the echoed request. MyDisease provides exact annotation after the caller selects an identifier. Get does not select a name match. Existing article, trial and drug commands serve their own tasks; the disease card makes no automatic related-entity fanout.

Reuse the shared `Curie` and `Concept`. A disease concept is not a new `DiseaseId` namespace or a second ontology identity type. MONDO, DOID, NCIt, OncoTree and each ICD edition remain distinct. `Concept` holds the source label and optional checked `Curie`; every executable identity action requires the ID. A label, scoped synonym, parent edge, cross-reference, close match and exact match are distinct facts. Keep each publisher predicate, version and provenance. Neither matching text nor a cross-reference alone establishes equivalence.

The neutral `biodata` layer owns neutral concepts, relation records, association rows and statistics facts. Disease gene and phenotype associations are ordinary source association records, not clinical `Statement` values. They retain source subject, object, predicate, original IDs, evidence and qualifiers without a common clinical interpretation. Reuse `Attribution`, text and checked numbers; add only a small neutral association record where the existing records cannot hold an observed field. The consumer `biomcp-types` layer owns query selectors, sources, cards, section/page containers, component `ComponentTerms` labels and typed next actions. The source registry remains the only owner of use categories, terms names, publisher URLs and disable policy.

Every association page puts the requested disease ID beside each actual returned row subject/object. A parent query may return child rows through source closure. Preserve both IDs and the native closure when needed to explain that result. An exact endpoint equality is a structural fact, not a claim that an association is clinically direct, causal or diagnostic. Counts remain association-row totals; they are not unique genes or patient risks.

Trial integration belongs to [trial-owned ticket 0137](../../tickets/0137-trial-condition-id.md). Existing `--condition TEXT` remains literal text. `--condition-id CURIE` uses the named exact MyDisease resolver and sends the resolved concept's primary label unchanged as ClinicalTrials.gov `query.cond`. Echo the input CURIE, `resolver:"mydisease"`, returned concept, primary label and resulting literal source request. A synonym, query spelling or locally chosen label is never substituted. ClinicalTrials.gov still applies its own text semantics; this does not prove trial eligibility. NCI receives only an explicit, source-provided NCIt code. If no unique NCIt mapping exists, return candidates or a missing-mapping problem before NCI. Never replace an intended NCI disease filter with keyword search. No disease-name recognition or silent ontology expansion occurs.

SEER uses its own catalog site IDs. Disease discovery may offer a separate catalog command; it never chooses a site by ontology label or a local name table. Statistics require an explicit selected site and native statistic selectors. Use structured JSON only. HTML Stat Facts captures remain external comparison evidence; the adapter must not parse them. Preserve requested settings beside returned settings, dates, missing values and native measures. A requested setting is never proof of a returned population, dataset or period.

## Explicit cross-reference mapping

| Source fact | Checked next action | Missing or multiple mappings |
|---|---|---|
| OLS selected MONDO CURIE | Exact MyDisease annotation of that same CURIE; returned `_id` and source cross-references validate the relation. | An unmatched returned identity is a problem, not a name fallback. |
| MyDisease `mondo.xrefs.efo` | Retain the native EFO cross-reference as a fact only. The Open Targets section is deferred and supplies no next action; the exact joined request returned null. | Return all source candidates; do not choose the first EFO or search by disease name. |
| MyDisease `mondo.xrefs.doid` | A selected DOID remains a source cross-reference. Any source needing DOID receives that exact code only after this explicit mapping is selected. | D13 has two DOIDs; no automatic first code. CIViC disease-context retrieval is dropped from this slice. |
| MyDisease `mondo.xrefs.ncit` | NCI trial condition receives the selected exact NCIt value with its namespace. | No NCIt means no NCI condition request; preserve literal ClinicalTrials.gov as a separate user choice. |
| MyDisease `mondo.xrefs.oncotree` or an ontology cross-reference | Return the source-stated code and predicate as a cross-reference. | Do not call or infer an OncoTree identity. Direct OncoTree is dropped. |
| ICD cross-reference | Preserve the actual namespace, code, source version and relation. | Never relabel ICD-10-AM as ICD-10-CM, or a skin-melanoma code as broad melanoma. |

The retained exact D05 lookup requested DOID:3910 and returned MONDO:0005061 with that explicit DOID cross-reference, plus EFO:0000571, NCIt C3512 and OncoTree LUAD. That is a source-proved alias mapping, not literal request/response ID equality. D08 MONDO:0005105 carries EFO:0000756, DOID:1909, NCIt C3224 and OncoTree MEL. D13 MONDO:0018875 carries DOID:3012 and DOID:0111503. Keep both, with their source relation. Do not invent a preferred mapping. The captured Open Targets native identifiers are source candidate IDs, not proof of a cross-provider join. The one later exact EFO request described below returned `disease:null`, so the Open Targets disease-genes section is deferred.

MyDisease relation predicates retain the exact component field path, such as `mydisease:mondo.xrefs.doid` and `mydisease:disease_ontology.parents`. Each relation uses that component's own subject ID and label. Target IDs have no invented labels. A bare cross-reference code receives the upper-case native field namespace; a value that already carries that same namespace stays unchanged. Thus `icd10`, `icd10cm` and `snomedct_us_2025_09_01` remain distinct, and `icd11` preserves its complete `foundation:…` local code. Synonym scopes likewise retain paths such as `mydisease:mondo.synonym.exact`. The card's version is the aggregate MyDisease `_version`; component ontology releases remain unknown. Its alias is the particular native cross-reference that proved the requested alternate ID.

## Commands, headers and next actions

Each section uses one named source. There is no merged gene list, blended score or automatic fallback. CLI and MCP share the same typed argument list. The table is the proposed command tree; defaults do not imply unproved public fields.

| Command and selectors | Default header and body | Next actions and cases |
|---|---|---|
| `search disease TEXT`; `--source ols`; `--ontology mondo` by default; explicit `ncit` or `doid`; `--limit`, `--offset` | Echo literal text, provider, ontology selectors and page. Candidate header: `Concept`, ontology, obsolete flag when supplied, exact native ID and typed get action. Source total is separate from displayed rows. | Exact get per row; next source page with every selector preserved. D01–D10, D18-subtypes, D21, D23–D25. |
| `get disease CURIE`; default `--source mydisease`; explicit `--source ols --ontology mondo\|doid\|ncit` for an OLS concept | Echo requested ID, returned ID and proved alias relation; source concept, complete optional definition, obsolete/consider/replacement facts, component terms and available sections. | `synonyms`, `xrefs`, `hierarchy`, `genes`, `phenotypes`; only supported actions. Non-MONDO/DOID MyDisease requests return unsupported usage rather than implicit routing. D02, D05, D08, D13, D15. |
| `get disease CURIE synonyms`; `--source mydisease` or explicit `ols`; `--section-limit`, `--section-offset` | Requested/returned identity, one source, synonym text, source scope/language and native total. | Next synonym page with source and ID fixed. D03, D04, D08, D12, D15. |
| `get disease CURIE xrefs`; `--source mydisease` or explicit `ols`; `--section-limit`, `--section-offset` | Native cross-reference ID, predicate, source ontology/version and exact target serialization. Multiple codes remain choices. | Exact target-source action only for a verified mapping; next cross-reference page. D02, D05, D08, D13, D25. |
| `get disease CURIE hierarchy`; `--source mydisease`; `--section-limit`, `--section-offset` | MyDisease native parents only, exact endpoint IDs, ontology and relation provenance. Children wait for a positive OLS child capture in the OLS implementation ticket; no child action is advertised. | Exact parent concept get and next parent page. D05, D06, D07, D15. D18 subtype discovery uses OLS search until child capture proof exists. |
| `get disease CURIE genes --source monarch`; `--section-limit`, `--section-offset` | Requested ID beside actual native subject/object IDs, predicate, original IDs, evidence, component terms and association-row total. | Next native page and existing explicit gene command when a checked gene identity exists. D12–D18. |
| `get disease CURIE phenotypes --source monarch`; `--section-limit`, `--section-offset` | Requested ID beside actual subject/object IDs, native HP labels, negation, evidence and frequency/onset qualifiers. No direct HPO call. | Next native page; exact source links. D13–D18. |
| `search disease --source seer`; `--limit`, `--offset` | Native site code, exact label, active flag and source catalog version/date. No disease-to-site assertion. | Explicit statistics commands with a caller-selected site, next catalog page. D19, D20. |
| `get disease <site ID> incidence\|survival --source seer --sex ID --race ID --age-range ID --dataset ID`; incidence-only `--period ID`; survival `--interval ID`; `--section-limit`, `--section-offset` | Echo every requested selector; return structured native selector keys/labels, returned period and fields, source counts and explicit unverified dimensions. Incidence and survival remain distinct. | Next source row page; catalog action; no recent aggregate survival action; that unsupported form is dropped. D19, D20. |
| `search trial --condition-id CURIE` | Separate trial-owned ticket: exact MyDisease concept and unchanged primary-label ClinicalTrials.gov condition text, or explicit NCIt for NCI; echo each mapping and request. Existing trial filters remain owned by trial. | Existing trial get/paging; ambiguity or missing mapping stops before the requested trial source. D05, D08, D13, D21. |

For `--source seer`, search takes no text or ontology selector and returns only the publisher site catalog. Get takes the publisher site ID in the ID position and a named `incidence` or `survival` section. It accepts no `--site` or `--measure` flags and refuses an ontology CURIE. These explicit provider forms are ordinary search/get requests, not name-based routing. Other search/get forms retain their ontology contracts.

Search pages use `--limit` and `--offset`. All get pages use `--section-limit` and `--section-offset`, including SEER. No `-l` alias is accepted. Source cursors remain native cursors if the source requires them; offset conversion cannot fabricate a result window. `all` is retired from the disease proposal: explicit one-source sections provide bounded work and clear provenance. Shared `--format`, `--disable-sources` and ordinary checked cache/replay behavior remain the shared product contract. Every advertised command gets generated help from that typed tree.

## What the agent needs

1. A positional name search returns a few source-ranked headers and exact actions. It does not select an identity by rank or matching text.
2. The header needs exact ID, label, ontology/source and obsolete state; source total and next page explain what is outside the current window.
3. The default exact card supplies identity and a complete source definition when returned. Large synonym, relation and association lists stay in one-source pages.
4. Downstream calls need exact identifiers and publisher predicates. Multiple mappings are visible choices, not a preferred code inferred from names.
5. A disagreement remains distinct attributed facts. Native association scores and predicates are not a common clinical interpretation.
6. A valid empty source reply, disabled/missing-key skip, failed request and malformed body remain distinct. HTTP 200 with GraphQL errors is not an empty success.
7. The actual live baseline below measures loaded-M5 search and one card. Warm speed, HTTP counts, final product section sizes and actual tokenizer counts are not invented.
8. A clinician needs source/version/date, relation scope and evidence. A phenotype association supplies no diagnosis, patient probability or treatment advice.

## Concrete proposed JSON

These are design examples using retained source facts. They are not product executions or new provider captures. The one-row pages illustrate a requested page size of one; the actual source captures used five. Full five-row serialized examples and their byte measurements remain external. Schema names below are proposed version-one consumer schemas, generated from the eventual response types. Shared `Envelope`, `Section`, `PageInfo`, `Step`, `SourceOutcome` and `ComponentTerms` remain the framing types.

This proposed single-ontology header uses the native MONDO concept from retained D02 data. The earlier capture queried three ontologies; this illustration is not a measured MONDO-only request. It omits native paging totals rather than inventing that unmeasured count. Actual implementation cases must supply source-proved paging metadata:

```json
{"schema":"biomcp.disease-search/1","status":"complete","query":{"text":"MONDO:0007254","source":"ols","limit":5,"offset":0,"ontology":"mondo"},"result":{"items":[{"concept":{"id":"MONDO:0007254","label":"breast cancer"},"ontology":"mondo","next":[{"kind":"disambiguate","args":["get","disease","MONDO:0007254"]}]}],"components":[{"name":"mondo","terms":{"usage":"attribution","name":"MONDO CC BY 4.0","url":"https://mondo.monarchinitiative.org/pages/download/"}}]},"sources":[{"source":"ols","use":"attribution","terms":"EMBL-EBI terms; ontology-specific rights","terms_url":"https://www.ebi.ac.uk/about/terms-of-use/","state":"ok"}]}
```

A default card can have no definition when the exact source concept has none. It does not borrow one from a similarly named concept:

```json
{"schema":"biomcp.disease-card/1","status":"complete","query":{"id":"MONDO:0800444","source":"mydisease"},"result":{"requested_id":"MONDO:0800444","returned_id":"MONDO:0800444","concept":{"id":"MONDO:0800444","label":"Birt-Hogg-Dube syndrome"},"sections":["synonyms","xrefs","hierarchy"],"components":[{"name":"mondo","terms":{"usage":"attribution","name":"MONDO CC BY 4.0","url":"https://mondo.monarchinitiative.org/pages/download/"}}]},"sources":[{"source":"mydisease","use":"attribution","terms":"MyDisease source-specific terms","terms_url":"https://docs.mydisease.info/en/latest/doc/data.html","state":"ok"}],"next":[{"kind":"section","args":["get","disease","MONDO:0800444","synonyms"]},{"kind":"section","args":["get","disease","MONDO:0800444","xrefs"]},{"kind":"section","args":["get","disease","MONDO:0800444","hierarchy"]}]}
```

A source association page puts the parent request next to the actual returned child object. The row remains a native association record. The captured row is an HPO-published annotation. Its Orphanet provenance remains attached without treating that identifier as a separate licensed payload:

```json
{"schema":"biomcp.disease-associations/1","status":"complete","query":{"id":"MONDO:0019003","source":"monarch","section":"genes","section_limit":1,"section_offset":0},"result":{"requested_id":"MONDO:0019003","items":[{"requested_id":"MONDO:0019003","subject":{"id":"HGNC:9967","label":"RET"},"predicate":"biolink:gene_associated_with_condition","object":{"id":"MONDO:0008234","label":"multiple endocrine neoplasia type 2A"},"original_subject":"NCBIGene:5979","original_object":"Orphanet:247698","primary_knowledge_source":"infores:orphanet","provided_by":"hpoa_gene_to_disease_edges","components":[{"name":"hpo","terms":{"usage":"attribution","name":"HPO custom attribution, version and integrity terms","url":"https://human-phenotype-ontology.github.io/license.html"}}]}]},"page":{"offset":0,"returned":1,"total":8},"sources":[{"source":"monarch","use":"attribution","terms":"Monarch source-specific licensing","terms_url":"https://monarch-app.monarchinitiative.org/Licensing/","state":"ok"}],"next":[{"kind":"page","args":["get","disease","MONDO:0019003","genes","--source","monarch","--section-limit","1","--section-offset","1"]}]}
```

A structured statistics page retains native dimensions. Each unverified dimension appears as a structured source problem; an absent catalog publisher date and version each receive their own problem. The documented incidence units are a separate publisher-method fact, not a field invented in the returned body:

```json
{"schema":"biomcp.disease-statistics/1","status":"complete","query":{"source":"seer","sex":"1","race":"1","age_range":"1","period":"5","dataset":"4","id":"47","section":"incidence","section_limit":1,"section_offset":0},"result":{"items":[{"requested_site":"47","returned_selectors":{"sex":"1","race":"1","age_range":"1","stage":"101","subtype":"47","site":"47"},"returned_year_range":5,"native_values":{"rate":"47.172044","rate_se":"0.072713","rate_lower_ci":"47.029634","rate_upper_ci":"47.314797","count":434448}}],"components":[]},"problems":[{"source":"seer","field":"dataset_membership","code":"value.unknown"},{"source":"seer","field":"registry_membership","code":"value.unknown"},{"source":"seer","field":"standard_population","code":"value.unknown"},{"source":"seer","field":"observed_or_delay_adjusted_rate","code":"value.unknown"}],"page":{"offset":0,"returned":1,"total":5},"sources":[{"source":"seer","use":"open","terms":"SEER/NCI Using This Website","terms_url":"https://seer.cancer.gov/about/using-website.html","state":"ok"}],"next":[{"kind":"page","args":["get","disease","47","incidence","--source","seer","--sex","1","--race","1","--age-range","1","--period","5","--dataset","4","--section-limit","1","--section-offset","1"]}]}
```

The survival command has no `--period` option. It returns the provider's annual series and native `returned_year_range:31`, with observed and modeled fields separate. The earlier capture's requested 9/returned 31 mismatch is historical evidence that the old selector was not honored; it is not a selector the new command promises to support. Missing observed values stay null/absent, never zero. The unsupported recent five-year aggregate is dropped by PM ruling; the annual series is not its replacement answer.

## Progressive disclosure and measured byte budgets

The default search/card answer identity only. Synonyms, cross-references and hierarchy default to five-row pages. Monarch genes default to two association rows and phenotypes to one; explicit valid limits remain unchanged. Statistics and the SEER catalog are separate explicit commands. Full definitions stay complete when present; if a source definition makes the default exceed its budget, the implementation needs a named complete-definition drill-down instead of silent truncation. Source claims, predicates and qualifiers cannot be dropped to meet a ceiling.

The table below uses actual compact CLI output and CLI/MCP parity cases from the accepted adapters. Full bytes include the whole envelope. Comparable bytes remove only the validated `use`, `terms` and `terms_url` fields on source outcomes under the shared accounting rule; component terms, native closures, evidence and qualifiers remain included. Each existing fixed case ceiling remains unchanged. Token estimates use `ceil(comparable bytes/4)` and are not tokenizer measurements. Earlier illustrative projections remain external investigation evidence and no longer supply product allocations.

| Implemented surface | Full JSON bytes | Comparable bytes | Comparable token estimate | Existing comparable case ceiling | Qualified 0.9.3 comparison or reason unavailable |
| --- | ---: | ---: | ---: | ---: | --- |
| OLS discovery, eight five-row or shorter pages | 816–2,147 | 682–2,013 | 171–504 | 785–2,315 by exact case | Contained D01–D21 discovery: 176–1,731 bytes. Older MyDisease requests differ from OLS; no matched-request reduction or complete answer parity. |
| Default MyDisease card, D08 melanoma | 1,783 | 1,655 | 414 | 2,024 | No qualified same-case get: the contained source allowance ended before exact gets. Historical D02 get 8,462 bytes has a different identity and hidden enrichment. |
| Other measured exact MyDisease cards, D02/D03/D04/D05/D06/D07/D12/D13/D15 | 847–1,888 | 719–1,760 | 180–440 | 2,024 | No qualified same-case get: contained source allowance exhausted. Exact OLS cards separately measure 839–1,223 full / 705–1,089 comparable bytes with caps 811–1,253. |
| Synonyms, native MyDisease default five-row D05 page | 1,770 | 1,642 | 411 | 1,889 | No qualified section output: the contained allowance ended before sections. OLS D05 three-row page: 1,003 full / 869 comparable, cap 1,000. |
| Cross-references, native MyDisease default five-row D05 page | 2,041 | 1,913 | 479 | 2,200 | No qualified section output: contained allowance exhausted. OLS D05 five-row page: 1,829 full / 1,695 comparable, cap 1,950. |
| Hierarchy, native MyDisease default D05 parent page | 1,688 | 1,560 | 390 | 1,794 | No qualified section output: contained allowance exhausted. Parents retain native source endpoints; no child route. |
| Monarch genes, explicit limit 5, D12/D13/D14/D18 native pages | 796–9,658 | 665–9,527 | 167–2,382 | 765–10,957 by exact page | No qualified gene output: the contained allowance ended before gene sections. Totals count source association rows. |
| Open Targets disease genes | Not implemented | Not implemented | — | None | Deferred: exact source-provided EFO join returned null. Historical source-native pages do not prove the join or an executable product route. |
| Monarch phenotypes, explicit limit 5, D13/D14/D15/D18 native pages | 13,255–16,400 | 13,124–16,269 | 3,281–4,068 | 15,093–18,710 by exact page | No qualified phenotype output: contained allowance exhausted. Full ordered native closures, evidence and qualifiers explain the cost above the original illustrative allocation. |
| Monarch genes, new default two rows, derived local deliveries | 3,686–4,496 | 3,555–4,365 | 889–1,092 | 4,089–5,020 by exact page | No qualified 0.9 section. Native ordered prefixes of retained five-row bodies; no actual provider request at limit 2 is claimed. |
| Monarch phenotypes, new default one row, derived local deliveries | 3,312–4,016 | 3,181–3,885 | 796–972 | 3,659–4,468 by exact page | No qualified 0.9 section. Native first rows of retained five-row bodies; no actual provider request at limit 1 is claimed. |
| SEER incidence, D19 five stage rows | 4,462 | 4,348 | 1,087 | 5,001 | No qualified 0.9 statistic output. Old disease-name routing and Stat Facts HTML do not measure this explicit structured site/statistic command. |
| SEER annual survival, D20 ten pages | 2,971–5,087 | 2,857–4,973 | 715–1,244 | 5,719 maximum | No qualified 0.9 annual-series output. The dropped recent aggregate is a different question; annual rows do not substitute for it. |
| SEER sites, all fifteen native pages | 690–994 | 576–880 | 144–220 | 1,012 maximum | No released same-command catalog measurement; no old executable rerun was authorized. |

MyDisease two-row page maxima are synonyms 1,203 full / 1,075 comparable bytes (cap 1,237), xrefs 1,322 / 1,194 (cap 1,374), and hierarchy 1,332 / 1,204 (cap 1,385). Explicit limit-50 pages measure synonyms 2,067 / 1,939 (cap 2,230), xrefs 5,014 / 4,886 (cap 5,619), and hierarchy 1,689 / 1,561 (cap 1,796). These are separately measured commands, not a multiplication of a default-page allowance. The OLS fifty-candidate boundary retains all fifty get actions and the page action: 14,160 full / 14,026 comparable bytes, cap 16,130.

The native measurements come from OLS `j11984`/`j12001`, MyDisease `j12142`, SEER `j12155`, and Monarch `j12295`. MyDisease's original source captures, assertions and measurement logs remain, but its first product run deleted derived recordings and JSON outputs. The retained measurements establish the recorded size; they cannot provide a replay of those deleted artifacts. No output was reconstructed and no established provider proof was repeated. The later retention regression protects future product proof runs.

PM ruling m4768 keeps the original default association allocations by reducing default row counts, without selecting or dropping fields. Presence-aware decoding distinguishes omission from an explicit limit: omitted Monarch genes resolves to 2, omitted phenotypes to 1, and every other source/section to 5. Explicit null, zero and out-of-range limits still fail; explicit 5 stays 5. CLI and MCP omit absent flags until this shared check, and every checked request serializes the effective numeric limit. Page actions retain that limit and native row order. Existing explicit-limit measurements and case ceilings remain unchanged.

The new-default proof `j12430` uses eight labeled local projections of unchanged source bodies: only the native ordered prefix and fixture page limit differ. It preserves every field in each retained row. The archive on Yellow is `data/biomcp-redux/0145-disease-docs-proof/j12430-monarch-defaults.tar.gz`, SHA-256 `62e66f8da0c1ac929d5ce42099031cdc292e9ec6ba69e25ea1cf1dbd7790e31c`. It retains original request URLs, body hashes and retrieval dates separately from the local delivery times, projection manifest, actual local recordings and CLI JSON output. The job checked CLI/MCP equality, but raw MCP protocol stdout was not retained. It proves the product's smaller-page behavior over those facts; it is not native capture proof of limit 2 or 1, a continuous cross-date source walk, or a new clinical answer. A synthetic complete walk separately checks that omitted and explicit limits keep all six source rows reachable without duplicates.

The broader same-question clinical comparison remains owned by [0098](../../tickets/0098-compare-with-0-9-on-outside-question-sets.md). These rows account for product bytes and explicitly unavailable old measurements. They do not establish complete clinical parity or a clinical answer for every harvested question.


### Implemented SEER command measurements

The SEER implementation ran retained bodies through the real CLI recorder, then checked CLI/MCP replay and offline pages in `j12155`. This job made no provider requests. It verified the original body hashes and retained the catalog/incidence capture times and the later correct survival request time. The later survival request used native year range 31; it did not relabel the historical request for 9. Publisher catalog date/version remain absent and produce separate source problems. The optional `fetched` field is the original response retrieval timestamp, never a publisher revision or a current provider fetch claim.

| Implemented surface | Full JSON bytes | Comparable JSON bytes | Comparable `ceil(bytes/4)` estimate | Measured case ceiling |
|---|---:|---:|---:|---:|
| Native catalog, all 15 pages at five rows per page | 690–994 | 576–880 | 144–220 | 1,012 |
| Native site 47 incidence, all five separate stage rows | 4,462 | 4,348 | 1,087 | 5,001 |
| Native site 55 annual survival, all ten five-row pages | 2,971–5,087 | 2,857–4,973 | 715–1,244 | 5,719 |

Comparable size removes only the 114 exact validated bytes of the required source use/terms fields. Full sizes remain reported. Each case ceiling is the measured maximum for its page profile plus 15 percent, rounded up. The statistics exceed the historical illustrations because the checked rows retain each requested and returned selector, native stage or annual context, uncertainty, count and attribution. Their estimates are not measured tokenizer counts.

No same-surface 0.9 statistic/catalog ratio is available from the bounded baseline. Its retained D19 discovery output is 176 bytes and its D20 discovery output is 1,463 bytes; these are different query surfaces and do not measure the native statistics commands. The annual view supplies no recent aggregate survival answer. Full clinical parity remains a separate review.

## Complete command and option case map

The D identifiers below are harvested stories. The executed witness table below distinguishes implemented cases, external source evidence and deferred answers. Suffixes describe the behavior to trace within its owning case. Direct capture identities and remaining exact-selector gaps stay explicit. Each retained option gets a positive source fact and a contrast; each dropped spelling gets a typed retirement/usage case. No unsupported filter is quietly accepted.

| Surface | Command/option/section accounting | Harvested case witnesses and required assertions |
|---|---|---|
| Search input | Positional `TEXT` retained; old positional alias retained as the sole form; `-q`/`--query` retired | D01-name, D02-id-text, D21-ambiguity; D01-retired-query asserts recovery to positional syntax. Source literal text is echoed unchanged. |
| Search provider/ontology | New provider `--source ols`; `--ontology mondo` default; explicit `ncit` or `doid`; old `--source mondo/doid/mesh` retired with guidance | D02-mondo, D05-doid, D08-ncit, D05-retired-multi-ontology rejects a comma list before I/O; exact default narrowing needs its own captured positive. D21 preserves source order and never selects a candidate. |
| Search page | `--limit`, `--offset` | D01-first, D01-next, D01-terminal, D01-retired-short-limit; exact totals, no duplicate IDs, all selectors on next, no next past terminal. No local reranking. |
| Retired clinical filters | `--inheritance`, `--phenotype`, `--onset`, `--no-fallback` | D13-retired-inheritance, D14-retired-phenotype, D18-retired-onset, D23-retired-fallback. No clinical phrase expansion, wrong-field mapping or automatic query recovery. |
| Get identity | `CURIE`; default MyDisease; explicit `--source ols --ontology mondo\|doid\|ncit`; old free-text get and `--name` retired | D02-exact, D05-explicit-DOID-alias, D15-obsolete/consider; echo requested and returned IDs, prove alias via native cross-reference, reject wrong identity. D21-name-get returns search guidance, not a guess. |
| Card/definition | No implicit section; complete optional definition and version/provenance | D02-card, D08-card, D15-parent-no-definition. No hidden Open Targets/MyChem/ClinicalTrials.gov request. |
| Synonyms | `synonyms`; one explicit source; `--section-limit`, `--section-offset` | D03/D04-separate-concepts, D08-synonyms-first/terminal, D12-historical-scope, D15-synonym-types. Preserve source scope and language. |
| Cross-references | `xrefs`; source and section paging | D02-ICD-namespace, D05-EFO/DOID/NCIt/OncoTree, D08-crossrefs, D13-multiple-DOID, D25-conflict. No preferred code or loose-to-exact promotion. |
| Hierarchy | `hierarchy`; source and section paging | D05-parents, D06-broad-parent, D07-parent, D15-consider-not-parent. MyDisease parents only. D18-subtypes uses OLS search; hierarchy children remain unsupported until an OLS positive capture. |
| Gene associations | `genes --source monarch`; Open Targets disease genes deferred; independent section paging | D12-source-context, D13-associated-CHEK2, D14-row-count-not-genes and D18-parent-request-child-object. D11/D22 Open Targets gene/score cases are deferred with that section; an explicit unsupported source request makes no call. No `Statement` or blended score. |
| Phenotype associations | `phenotypes --source monarch`; independent section paging; `clinical_features` renamed with recovery | D13-evidence, D14-phenotypes, D15-syndrome, D18-negation/frequency. Requested and actual IDs, native evidence/qualifiers and nulls remain intact. No direct HPO lookup. |
| Retired get sections | `pathways`, `diagnostics`, `variants`, `models`, `prevalence`, `funding`, `civic`, `disgenet`, `all`; legacy name-driven `survival` | D05-retired-pathways, D12-retired-diagnostics, D22-retired-variants, D14-retired-models, D19-retired-prevalence, D13-retired-funding, D08-retired-civic/disgenet/all, D20-explicit-site-recovery. Each gives a concrete supported route or an honest dropped message. |
| SEER site discovery | `search disease --source seer`; `--limit`, `--offset` | D19-sites-first/next, D20-sites-selection. Code47/55 comes from the publisher catalog; never from ontology-name matching. |
| SEER statistic identity | `get disease <site ID> incidence --source seer`; `get disease <site ID> survival --source seer` | D19-incidence and D20-survival; distinguish native stage rates from annual survival, preserve exact returned site and requested selector. Unsupported recent aggregate is not a zero. |
| SEER population/time | `--sex`, `--race`, `--age-range`, `--dataset`, incidence-only `--period`, survival `--interval` | D19-both-sexes/all-races/all-ages/year5; D20-female/interval5/native-annual-period31; survival --period is rejected before I/O. The historical request9/return31 witness explains that retirement. Dataset4 is request/catalog metadata only unless echoed/proved in the response. Do not label an unverified dimension as confirmed. Selector availability must not be inferred from catalog existence. |
| SEER row pages | `--section-limit`, `--section-offset` | D19-stage-page/terminal; D20-annual-page/model-only. Five stage rows versus48annual rows; null observed values stay null. Paging the same retained payload must not make an extra provider request. |
| Trial integration | `search trial --condition-id`; existing literal `--condition` and trial `--source`, `--limit`, `--offset` remain trial-owned | D05-NCIt, D08-exact-label-ctgov, D13-multiple-mapping, D21-ambiguity. ClinicalTrials.gov gets the exact resolved primary label with MyDisease named as resolver; NCI gets NCIt only; both requests echo the mapping. Existing trial cases remain the owner of unrelated filters. |
| Retired convenience commands | `disease trials`, `disease articles`, `disease drugs`; each old `--limit`/`--offset`, trial `--source` | D08-trials-recovery, D24-articles-recovery, D11-drugs-recovery. Give explicit supported entity commands; no hidden resolution, fanout or source fallback. |
| Retired phenotype ranking | `search phenotype TERMS`, `--limit`, `--offset` | D13/D14-retired-ranking. Do not produce a patient diagnosis from symptom text or introduce a scored disease ranker. |
| Help and shared output | `list disease` retired to generated help; `-h`/`--help`; `-j`/`--json` recover to `--format json`; shared `--format md/json` | D02-help and D02-format-parity; generated command examples and typed MCP args match the command tree. |
| Source policy/cache | `--disable-sources` names/groups and config union; existing cache/replay controls; old `--no-cache` follows shared retirement policy | D08-disable-ols, D13-disable-monarch/hpo/omim/orphanet/restricted, D02-disable-mondo/doid/ncit, D19-disable-seer. Assert zero prohibited requests and no cache/replay leakage; allowed independent sources remain available. |
| Empty and source failures | No new user flags | D02-literal-zero versus exact-get; D23-CIViC-source-local-zero stays investigation evidence for a dropped route. D24-source-failed, D24-timeout, D24-broken-body and D24-GraphQL-data-errors use recorded failure controls; no live failure is fabricated. |

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

D20's requested recent aggregate is dropped from the initial product by explicit PM ruling. Its clinical answer remains source-comparison evidence. The annual survival series is a distinct native view and does not satisfy that aggregate question by substitution.

The harvest clinical reviewer confirmed the answers with high confidence and preserved the cited editions' limits. A separate fresh clinical reviewer then hash-checked eleven selected captured bodies and accepted the identity, association-scope and statistics contract without a blocking clinical finding. Exact API crosswalks still need captured publisher fields. A source-supplied ICD-10-GM skin-melanoma code must not become a broad melanoma code or an ICD-10-CM code; the [BfArM 2025 C43 page](https://klassifikationen.bfarm.de/icd-10-gm/kode-suche/htmlgm2025/block-c43-c44.htm) makes its skin scope explicit.

## Released 0.9 accounting

The shared baseline download tried the official v0.9.4 release, received 404, and used explicitly labeled released **0.9.3**. The binary SHA-256 is `54f66769d9193c569d1b65756ad766eafd1a2c5e92d08f8c855cc8964e9a7145`; archive SHA-256 is `e637062c18c6692f3ec49c5847e69b29f6feee7f7909345970bd504d72174994`. The installed 0.9.1 executable was not used. Actual released help confirms the search/get options below. Repository source inspection identifies their implementation contracts; it is not a claim that every optional source was measured live.

| 0.9 surface | Proposed disposition | Reason and proof requirement |
|---|---|---|
| `search disease QUERY`; `-q`, `--query` | Keep positional text; retire `-q`/`--query` with explicit guidance | The source applies its own matching. No colon-shape routing, hand-made boosts or silent query rewriting. D01–D24 cover literal search. |
| `search disease --source mondo\|doid\|mesh` | Change to distinct provider and ontology selectors | A provider is not an ontology. Old MyDisease source selection checks field presence. A positive direct-source case must prove each retained selector. |
| `--inheritance` | Do not port its hand-written clinical phrase vocabulary | A future exact HPO ID filter needs a positive captured field and scope. No inherited syndrome diagnosis from a returned association. |
| `--phenotype` | Defer an explicit HPO-ID selector until positive filter proof | The old request searches `hpo.phenotype_related_to_disease.hpo_id`, not phenotype label text. No unsupported label-to-ID guessing. |
| `--onset` | Drop old field mapping; reconsider only a proved native onset filter | Old request targets `hpo.clinical_course.hpo_name`. Record the field mismatch; do not rename clinical-course facts into onset facts. A positive source witness is required before adding an onset option. |
| `--no-fallback` | Retire | The new search has no discover fallback. A valid zero remains zero. |
| Search `--limit`, `--offset`; old `-l` | Keep bounded pagination; retire the short alias | Native totals and source order remain separate from displayed rows. Native cursors cannot masquerade as offsets. |
| `get disease NAME_OR_ID`; `--name` | Change to explicit ID get; names use search | Ambiguity returns candidates. No best-match selection or free-text semantic parser. Typed IDs retain exact namespaces. |
| Get positional section words | Keep proved named sections | One section cannot consume a disease name accidentally. Explicit ID syntax resolves the old multi-word ambiguity. |
| `genes` | Keep attributed native associations | D11–D18 and D22 need context, relation, original subject/object and evidence. No merged clinical score or local causal label. Open Targets disease genes are deferred after the exact EFO lookup below. |
| `phenotypes` | Keep source-qualified rows | D13–D18 need phenotype, evidence, negation and native frequency qualifiers where present. |
| `clinical_features` | Rename to `phenotypes` | It is an alternate presentation of the same Monarch/HPO rows, not an independent clinical interpretation. |
| `pathways` | Drop | Old route derives Reactome pathways from associated genes. No harvested care question establishes a needed disease-to-pathway join. |
| `diagnostics` | Drop from this slice | It fans out to GTR/WHO IVD. The disease examples need no diagnostic catalog. The disease card does not imply a test recommendation. |
| `variants` | Drop disease-context variant fanout | No indispensable disease question requires it. Exact variant/profile work stays on the accepted variant surface. |
| `civic` | Drop disease-context retrieval | No harvested disease question requires an additional direct CIViC call. Existing variant CIViC commands remain owned by variant; no disease-name join is added. |
| `models` | Drop | Model-organism evidence does not serve the cancer-care identity questions. |
| `prevalence` | Drop | No harvested question needs this source-specific prevalence-like view. The new explicit SEER incidence/survival forms are distinct measures, not a rename. |
| `survival` | Replace inferred mapping with explicit SEER site/statistic query | D19/D20 require exact native population and period. The old name-to-site table is prohibited. |
| `funding` | Drop | NIH grant amounts do not answer the harvested care questions. |
| `disgenet` | Keep dropped | The gene slice already dropped it; this investigation found no indispensable new question that changes that ruling. No keyed request was made. |
| `all` | Drop; use explicit one-source sections | The default identity card makes no enrichment fanout. Every requested section keeps its own status and page. |
| Hidden default `treatments`/`recruiting_trials` enrichment | Drop from default; use explicit existing drug/trial commands | The measured breast get calls MyChem indication search and ClinicalTrials.gov without a named section. Identity lookup must not silently perform those tasks. |
| `disease trials NAME`; `--source`, `--limit`, `--offset` | Map to the accepted trial surface | Literal `search trial --condition`; explicit NCIt route for NCI. The settled explicit `--condition-id` contract preserves the source mapping and literal request. |
| `disease articles NAME`; `--limit`, `--offset` | Map to explicit article disease/text search | No ontology resolution or broadening by default. Article behavior remains owned by that slice. |
| `disease drugs NAME`; `--limit`, `--offset` | Retire convenience command; use the explicit drug-owned indication surface | No implicit drug source fallback or disease synonym expansion. The drug design owns supported filters. |
| `list disease` | Replace with generated disease help | Generate command/section/option accounting from the implemented request tree. No second hand-written clinical vocabulary. |
| Adjacent `search phenotype` and its text/ID/limit/offset inputs | Drop the whole ranking convenience command from this slice | D13–D18 ask for known-syndrome facts, not a scored patient diagnosis. A source-native similarity route would need its own harvested question and explicit semantics. |
| Shared `-j`, `--json` | Map to existing `--format json` | Preserve the common envelope and CLI/MCP parity. |
| Shared `--no-cache` | Use the existing explicit transport cache policy | No new disease-only cache control. Capture and replay use the shared transport. |
| Shared `--disable-sources`, help | Keep existing shared settings/help | Enforce before cache/network access; report skipped/disabled with registry terms. |

This table applies the settled product dispositions. The implemented spelling and owning evidence are recorded below. Each retained selector has its captured positive and command case; unsupported filters remain explicit retirements. The 0.9 code's `icd10cm` crosswalk querying `umls.icd10am` is a namespace mismatch; the new identity model must preserve those coding systems separately. No interpretation of their clinical interchangeability is made.

### Implemented migration ledger

The rows below close every surface in the accounting table above. Shared aliases and positional section words remain listed here even though the historical command-count inventory excludes aliases, section values and global flags. Trial-owned `disease trials` and its option slots remain counted once in [trial accounting](trial.md#09-command-option-and-section-accounting); article-owned `disease articles` remains counted once in [article accounting](article.md#1-how-biomcp-093-does-articles).

| Released surface | Implemented answer and owner | Owning case or limit |
| --- | --- | --- |
| `search disease QUERY`, positional text | `search disease TEXT`, OLS | `disease_ols.rs`: native order and literal selectors; external OLS product proof. |
| `-q`, `--query` | Retired to positional text | `tests/cases/disease/retired.yaml`. |
| `--source mondo/doid/mesh` | `--source ols --ontology mondo/doid/ncit`; mesh unsupported | OLS native selectors and retired source guidance. |
| `--inheritance` | Dropped clinical phrase expansion | Retired case; no request. |
| `--phenotype` | Deferred exact HPO-ID filter | Retired case; no request. |
| `--onset` | Dropped wrong-field mapping | Retired case; no request. |
| `--no-fallback` | Retired; valid zero stays zero | Retired case and OLS literal-zero regression. |
| `--limit`, `--offset`, `-l` | Long paging options retained; short alias retired | OLS native first/next/terminal pages and fifty-row boundary. |
| `get disease NAME_OR_ID`, `--name` | Exact CURIE get; names use discovery | MyDisease native identity/alias checks and retired name guidance. |
| Get positional section words | One named source section, or identity card | Generated get help and independently paged sections. |
| `genes` | `get disease CURIE genes --source monarch` | Native association endpoint/predicate cases; Open Targets stays deferred. |
| `phenotypes` | `get disease CURIE phenotypes --source monarch` | Native evidence, negation and qualifier cases. |
| `clinical_features` | Retired with `phenotypes` recovery | Retired case. |
| `pathways` | Dropped disease-to-gene-to-pathway fanout | Retired case. |
| `diagnostics` | Dropped from disease slice | Retired case. |
| `variants` | Dropped disease-context fanout; explicit variant commands | Retired case. |
| `civic` | Dropped disease context; variant CIViC unchanged | Retired case; no disease-name join. |
| `models` | Dropped outside care-question scope | Retired case. |
| `prevalence` | Dropped; incidence and survival are distinct measures | Retired case. |
| Legacy name-driven `survival` | Explicit SEER catalog site and survival section | Retired recovery and SEER native annual-series cases. |
| `funding` | Dropped | Retired case. |
| `disgenet` | Remains dropped | Retired case. |
| `all` | Dropped; request one source section | Retired case. |
| Hidden `treatments` and `recruiting_trials` | Removed from identity lookup; explicit drug/trial routes | MyDisease request-count and identity cases. |
| `disease trials`, `--source`, `--limit`, `--offset` | `search trial --condition` or trial-owned exact `--condition-id` | Trial condition-ID native proof and trial retirement case; owned by 0137. |
| `disease articles`, `--limit`, `--offset` | `search article --disease` | Existing article accounting and disease retirement case. |
| `disease drugs`, `--limit`, `--offset` | Dropped convenience route; consult explicit drug help | Retired case. No indication search is asserted. |
| `list disease` | Generated `help search disease` and `help get disease` | Disease help and retired list case. |
| `search phenotype TERMS`, `--limit`, `--offset` | Dropped symptom-to-diagnosis ranking | Retired case; no patient ranker. |
| Shared `-j`, `--json` | Retired to `--format json` | Retired case and CLI/MCP envelope parity. |
| Shared `--no-cache` | Retired to `--fresh` | Retired case; shared transport owns cache behavior. |
| Shared `--disable-sources`, help | Retained; source/component policy before I/O/cache/replay | Disease registry admission, source cases and generated help. |

<!-- parity-accounting -->
| 0.9 leaf command | Command outcome | Ported or renamed options | Dropped options | Deferred options | Undecided options | Ruling |
| --- | --- | --- | --- | --- | --- | --- |
| `search disease` | Ported or renamed | `--source`, `--limit`, `--offset` | `--query`, `--inheritance`, `--phenotype`, `--onset`, `--no-fallback` | — | — | Literal OLS text and explicit ontology selection replace rewritten MyDisease discovery; unsupported clinical filters and fallback remain retired. |
| `get disease` | Ported or renamed | — | `--name` | — | — | Exact CURIE cards and explicit one-source sections replace name selection and hidden enrichment. Section values and shared flags are accounted above, outside option-slot counts. |
| `disease drugs` | Dropped by ruling | — | `--limit`, `--offset` | — | — | Drop convenience indication search; explicit drug commands retain their own source contracts and no reverse indication join is fabricated. |
<!-- /parity-accounting -->

### Executed harvested-case map

Product proof uses retained native bodies through the actual CLI recorder, CLI/MCP replay and schema validation. Raw bodies with uncertain storage rights stay outside Git. OLS product proof is on Yellow in `data/biomcp-redux/0140-ols-product-proof/`; MyDisease measurements and assertions are in `0141-mydisease-product-proof/`; SEER product proof is in `0143-seer-product-proof/`; Monarch product proof is in `0130-0142-native-proof/0142-command-proof/`. The external investigation matrix in `0117-disease-investigation/biomcp-redux-0117-disease-analysis/` remains the outside-repo case for questions without a product answer. It supplies source facts, not a substitute executable product case or complete clinical parity.

| Design ID | Actual case or external case | Proven scope or explicit remaining limit |
| --- | --- | --- |
| D01 | `disease_ols/native.rs`, native breast discovery and three pages | Literal text, native order/totals, terminal continuation. |
| D02 | OLS exact MONDO card/search; `disease_mydisease/native.rs` breast card; disease help | Identity, source/component terms, no hidden enrichment. |
| D03 | MyDisease native MONDO:0005494 card; external D03 matrix | Distinct source identity; no clinical subtype classifier. |
| D04 | MyDisease native MONDO:0004984 card; external D04 matrix | Separate source concept; no TNBC equivalence inference. |
| D05 | OLS DOID search/card/sections; MyDisease DOID:3910 card/sections | Proven alternate-ID relation and native coding namespaces; trial mapping remains trial-owned. |
| D06 | MyDisease native MONDO:0005233 card; external D06 matrix | Source identity; no local broad-to-narrow equivalence. |
| D07 | MyDisease native MONDO:0018905 card; external D07 matrix | Source identity; no new classification rule. |
| D08 | MyDisease native melanoma card/sections; registry admission cases | Scoped synonyms, native cross-references, independent pages; exact trial label route owned by 0137. |
| D09 | External D09 capture/matrix and contained old source-local zero | Investigation-only site/scope comparison; no complete product clinical answer claimed. |
| D10 | External D10 capture/matrix | Investigation-only uveal identity comparison. |
| D11 | External D11 capture/matrix; disease-drugs retirement case | Open Targets score/gene answer deferred; no Monarch substitution. |
| D12 | MyDisease native MONDO:0005835 card; Monarch native genes proof | Native endpoints/evidence; no causal or germline-panel interpretation. |
| D13 | MyDisease MONDO:0018875 card/sections; Monarch genes/phenotypes first/next pages | Multiple mappings remain distinct; duplicate closure IDs and provenance retained. |
| D14 | Monarch MONDO:0008667 genes/phenotypes first/next pages | Association-row count; native empty terminal gene page; no unique-gene total inference. |
| D15 | OLS obsolete MONDO:0007607; MyDisease obsolete/parent cards; Monarch MONDO:0800444 phenotypes | Consider choices are not replacement; returned MONDO:0800445 subjects keep requested parent and native closure. |
| D16 | External D16 capture/matrix | Source-local empty comparison only; no clinical absence claim. |
| D17 | External D17 capture/matrix | Investigation-only association context; no complete product clinical answer claimed. |
| D18 | OLS MEN2 discovery; Monarch MONDO:0019003 genes/phenotypes pages | Parent request beside actual child endpoints, negation, qualifiers and complete ordered closures. |
| D19 | `disease_seer/native.rs`, catalog and incidence pages | Publisher site selection and returned stage/statistic context; unverified dimensions remain problems. |
| D20 | SEER native catalog/annual survival pages, model-only tail | Annual series only; recent aggregate answer dropped and not substituted. |
| D21 | OLS native MF ambiguity; retired name-get case | Every candidate remains a choice; no abbreviation expansion selected. |
| D22 | External D22 capture/matrix; retired variant fanout | Open Targets score/gene answer deferred. |
| D23 | Retired no-fallback case; external CIViC source-local zero | No recovery rewrite; dropped CIViC disease route remains investigation evidence. |
| D24 | OLS/MyDisease/Monarch/SEER failure cases and `retired.yaml` | Synthetic timeout, HTTP failure and malformed body preserve failed/absent distinction; no observed live outage claim. |
| D25 | D05 native namespace/xref cases plus external dataset-conflict case | Preserves native codes; does not resolve the dataset's clinical-field conflict. |

All native command cases enforce fixed measured byte ceilings; ordinary cases keep their existing source-specific caps. The generated examples are selected verbatim from `tests/cases/disease/examples.yaml` and execute against synthetic local SEER replay. They demonstrate source catalog selection and explicit statistic selectors without moving native source bodies into Git. `help_examples.rs` checks every printed example against its owning case and runs CLI/MCP parity. Disease help covers literal ontology discovery, the catalog form, all section words and all typed selectors. MCP discovery uses that same generated tree.

## Source and component use policy

These are proposed registry declarations for the kept disease adapters. `use` is an operational category, not a blanket license grant. Each output source outcome and `health` entry derives its category, terms name and publisher page from the same registry. Each carried component uses the existing `ComponentTerms` shape (`usage`, `name`, `url`) on its result rows or section. No disease adapter keeps a second table in code. This prose records the review target only.

| Provider or carried component | Use | Terms name and publisher page | Disable behavior |
|---|---|---|---|
| OLS service | `attribution` | [EMBL-EBI Terms of Use; ontology-specific rights](https://www.ebi.ac.uk/about/terms-of-use/) | `ols` disables its network/cache path. Host terms never replace component terms. |
| OLS MONDO / MyDisease MONDO | `attribution` | [MONDO CC BY 4.0](https://mondo.monarchinitiative.org/pages/download/) | Component `mondo`; no matching ontology query/projection when disabled. Default discovery reports skipped rather than choosing another ontology. |
| OLS DOID / MyDisease Disease Ontology | `open` | [Disease Ontology CC0 1.0](https://www.disease-ontology.org/resources/do-resources/) | Component `doid`; exclude its ontology/projection before the request. |
| OLS NCIt | `attribution` | [NCI Thesaurus CC BY 4.0](https://evs.nci.nih.gov/ftp1/NCI_Thesaurus/ThesaurusTermsofUse.pdf) | Component `ncit`; exclude its ontology before the request. The captured OBO edition is experimental and versioned, not a silently substituted current NCI service. |
| MyDisease service | `attribution` | [MyDisease source-specific terms](https://docs.mydisease.info/en/latest/doc/data.html) | `mydisease` disables the whole aggregator. Ordinary identity requests project only the chosen ontology fields. No broad `fields=all`. |
| Monarch service | `attribution` | [Monarch source-specific licensing](https://monarch-app.monarchinitiative.org/Licensing/) | `monarch` disables the whole source. Native primary and aggregator knowledge sources remain attached to each association. |
| Carried HPO annotations | `attribution` | [HPO project custom license: attribution, version and integrity](https://human-phenotype-ontology.github.io/license.html) | Component `hpo`; an explicit `hpo` disable excludes it. The captured publisher grant allows commercial use and requires attribution, displayed dates/version, preserved content/relationships and product acknowledgement. The `restricted` group does not disable these annotations. No direct HPO adapter is added. |
| Separately licensed OMIM payloads, if later added | `restricted` | [OMIM Use Agreement](https://omim.org/help/agreement) | Component `omim`; `restricted` disables it before a component-bearing request. This category requires an actual separately licensed payload; an OMIM identifier or provenance label in an HPO-published annotation does not establish one. No direct OMIM request or licensed OMIM export enters Git. |
| Verified Orphanet Science annotations | `attribution` | [Orphadata Science CC BY 4.0 and citation/version conditions](https://sciences.orphadata.com/faq/) | Component `orphanet`; preserve source dataset/version and attribution. A separately supplied Orphanet product needs its own publisher grant; a provenance label alone does not establish such a payload. |
| HPO-published annotations with OMIM/Orphanet provenance | `attribution` | [HPO annotation grant and conditions](https://human-phenotype-ontology.github.io/license.html) | Preserve original IDs and primary-source provenance. They remain HPO-published annotations; no separate proprietary payload is inferred from `infores` or an identifier. |
| Separately supplied upstream content outside the reviewed annotation grant | `restricted` until reviewed | Actual publisher terms URL with an unverified flag | This requires evidence of a distinct payload. Do not assign it to the captured HPO annotations merely because they preserve external provenance. |
| Investigated Open Targets disease associations, deferred from this build | `open` | [Open Targets CC0 1.0, including its documented integrated-source arrangements](https://platform-docs.opentargets.org/licence) | No disease command or new health entry is added for this deferred section; any existing `opentargets` policy remains source-owned; no second application of a different service's raw-source terms to the platform's documented licensed output. Preserve datasource score identities. |
| SEER Explorer catalog and structured statistics | `open` | [SEER/NCI Using This Website](https://seer.cancer.gov/about/using-website.html) | `seer` disables catalog/statistic network and cache reads. Preserve attribution, retrieval date and native unverified dimensions. No license claim for third-party materials. |

The `noncommercial` and `restricted` groups use the shared meanings. All sources/components are enabled by default. Commercial users can disable both groups through the existing settings. No kept component is presently labeled `noncommercial`; the group still handles a later source-reviewed component without a new disease-specific flag. Naming a provider disables that whole provider. Naming a carried component disables that component. Group and explicit settings apply before requests, cached data and replay, not as a cosmetic output filter.

OLS ontology selection and MyDisease field projection can omit disabled components before I/O. An OMIM identifier merely cited in a MONDO cross-reference remains a MONDO-published cross-reference; it is not an OMIM assertion payload. The same distinction applies to HPO-published Monarch annotations: OMIM/Orphanet original identifiers and primary-source labels preserve provenance, while the reviewed HPO annotation grant supplies their use category. A separate upstream prose or data payload would require its own terms. The captured annotations establish no such separate payload.

No verified Monarch server filter in this investigation excludes every disabled upstream component. An explicit `hpo` disable therefore means a mixed Monarch section must return `skipped/disabled` before I/O unless the implementation has a source-proved narrowing request. It must not fetch forbidden rows and discard them afterward, guess an API filter, or silently call a different provider. Disabling the `restricted` group alone leaves the reviewed Monarch genes and phenotypes available. Independent allowed MyDisease/OLS sections remain available; no deferred Open Targets action is advertised. Health exposes component disabled states without pretending a mixed data query is safe. Closing a missing upstream license or filter proof is source research, not a question for Ian to configure a key.

The dropped disease routes retain these recorded categories but do not add new implemented health entries: direct OncoTree `restricted` pending publisher-specific reuse proof; direct HPO `attribution` under the reviewed grant; CIViC `open` under [CC0](https://civicdb.org/releases/licensing); Reactome annotation data `open` under [CC0](https://reactome.org/license); MedlinePlus public-domain topic summaries `open` and separately licensed materials `restricted` under its [content policy](https://medlineplus.gov/about/using/usingcontent/); UMLS `restricted` under the keyed [UMLS license](https://www.nlm.nih.gov/research/umls/license.html); DisGeNET `restricted` under its source terms; NIH RePORTER public grant records `open` under [NIH API documentation](https://api.reporter.nih.gov/). These are final drops from this slice, not deferred adapter promises. Existing adapters owned by earlier slices keep their existing declarations.

## Direct source findings and measured comparison

The first source capture jobs made **142 direct requests** from `imaurer-m5` on 2026-10-24, using one attempt, no local response cache, a 20-second request deadline and serial admission. The six-request repair and final four-request pass bring the direct total to 152, inside the 180-request cap. Every clinical query returned HTTP 200. The HPO repository `LICENSE` URL returned 404; that is a failed terms lookup, not an HPO disease absence. No 429, 5xx or timeout occurred, so no live outage example was established. Failure, broken-body and timeout acceptance cases must use the existing recorded transport mechanism; they must not be called newly observed provider failures.

MyDisease, OLS, Open Targets and CIViC received every literal D01–D24 query. The other sources received only applicable typed identity, phenotype or site questions. No identifier was guessed to make an unrelated source answer every question. D25 reuses D05 identity requests. Source totals count different things: ontology hits, Open Targets search hits and CIViC evidence items are not directly comparable measures of clinical completeness.

| Source | Direct requests, including terms | Observed latency min/median/max, ms | Public-tier fields proved and limit | Proposed choice |
|---|---:|---:|---|---|
| MyDisease.info | 33 | 259/283/378 | Candidate IDs, MONDO label, typed synonym groups, parents, xrefs; exact annotation includes native HPO fields where present. Literal ID query can return zero while exact annotation succeeds. | Keep exact annotation. Captured discovery differences remain investigation evidence; OLS owns the proposed search. No automatic first hit. |
| OLS | 34 | 413/566/4468 | Namespaced concepts, labels, definitions, scoped synonyms, xrefs, obsolete flag, `consider` and replacement metadata. | Keep as explicit ontology source and exact metadata drill-down. No automatic fallback after MyDisease failure. |
| Monarch | 7 | 125/158/243 | Native subject/object, predicates, association IDs, original IDs, primary/aggregator sources, evidence, negation and optional qualifiers. Returned edges can match descendants through closure. | Keep native association sections; put the requested ID beside the returned row IDs and preserve actual edge endpoints. |
| HPO | 3 | 127/202/214 | Exact HP term identity, name and definition on two term requests. The attempted raw license path was absent. | Drop direct HPO requests: the harvested syndrome questions have native HP labels in Monarch/MyDisease annotations. Carry HPO component terms on those annotations. |
| Open Targets | 32 | 216/238/971 | Source-ranked disease candidates, source IDs, association count, target IDs/symbols, native overall and datasource scores. | Defer the disease-genes section: the single exact retained EFO cross-reference lookup returned null. Do not claim gene absence or advertise an action. |
| CIViC | 25 | 190/223/351 | Accepted evidence totals, native disease name/DOID, evidence/profile IDs, type, level, direction, significance and publication identity. | Drop disease-context calls: no harvested question needs this additional source after the kept identity/association routes. Existing variant CIViC remains unchanged. |
| SEER | 3 | 134/199/220 | Two Stat Facts HTML pages carry incidence/survival context; Explorer catalog carries native site vocabulary. The initial round did not query a selected structured site; the six-request repair below supplies that proof for two native forms. | Keep the two structured native forms under the fresh clinical contract below; no HTML parsing or recent aggregate survival claim. |
| Reactome | 2 | 190/220/251 | Disease search response and publisher terms page. A broad result is not an established disease-to-pathway association. | Drop from this slice: no harvested question needs the old gene-to-pathway fanout. |
| MedlinePlus | 2 | 219/413/608 | Health-topic XML for Li-Fraumeni query and terms page. Topic discovery is not a disease-identifier equivalence service. | Drop from this slice: reference links can remain source links; no extra patient-education section is required by the cases. |
| OncoTree | 1 | 1169/1169/1169 | Native tumor tree with code, name and source cross-references. | Drop direct OncoTree: D25 needs preservation of conflicting source codes, not a new classification service. Retain source-provided OncoTree cross-references inside the kept ontologies. |
| UMLS | 0 | Unmeasured | No key request or key inspection. | Drop from the initial slice; no indispensable question justifies a keyed dependency. A future ordinary configured-key smoke needs its own approved recipe. |
| DisGeNET / NIH RePORTER | 0 | Unmeasured | Standing gene-slice drops; no new indispensable clinical question. | Keep dropped. |

Terms remain source and component facts. [MONDO's publisher](https://mondo.monarchinitiative.org/pages/download/) names CC BY 4.0; [OLS's host terms](https://www.ebi.ac.uk/about/terms-of-use/) do not replace ontology-specific rights. [Open Targets](https://platform-docs.opentargets.org/licence) marks its platform data CC0 and documents its integrated source arrangements. [CIViC](https://civicdb.org/releases/licensing) dedicates its data under CC0. [Reactome](https://reactome.org/license) distinguishes data from illustrations and other materials. [MedlinePlus](https://medlineplus.gov/about/using/usingcontent/) distinguishes public-domain summaries from separately licensed material. [Monarch's current licensing guidance](https://monarch-app.monarchinitiative.org/Licensing/) recommends licenses and explicitly leaves repository-specific terms authoritative; it is not a blanket relicensing of every upstream association. The [HPO repository](https://github.com/obophenotype/human-phenotype-ontology) directs readers to its [publisher license page](https://hpo.jax.org/app/license). The current publisher license source and rendered page were captured successfully in the final bounded pass below; those actual terms determine the HPO component label. Open Targets' integration does not override them. Raw unknown-rights replies stay outside Git. The existing registry must carry each finally reviewed source/component use label, terms name and publisher page; no open endpoint grants additional rights.

The following historical table records native source totals and an earlier released baseline's time and stdout bytes. That run had unknown HTTP request counts; it does not supply the contained comparison below. The baseline command is `search disease --query TERM --limit 5 --no-fallback --json`, using an isolated home/config/cache with keys absent. All 24 searches exited zero. Times describe this loaded M5 run, not an isolated speed benchmark. Direct HTTP timings above and end-to-end product timings below measure different work.

| Case | MyDisease hits | OLS hits | Open Targets hits | CIViC evidence items | 0.9.3 ms | 0.9.3 bytes |
|---|---:|---:|---:|---:|---:|---:|
| D01 | 60 | 1414 | 2901 | 317 | 459 | 1356 |
| D02 | 0 | 1 | 1 | 0 | 275 | 678 |
| D03 | 1 | 17 | 1 | 8 | 324 | 723 |
| D04 | 1 | 5 | 1 | 0 | 329 | 176 |
| D05 | 14 | 176 | 186 | 126 | 279 | 1418 |
| D06 | 2 | 141 | 3 | 0 | 278 | 934 |
| D07 | 28 | 394 | 2 | 19 | 293 | 1173 |
| D08 | 126 | 1825 | 2322 | 235 | 275 | 1322 |
| D09 | 20 | 377 | 123 | 0 | 272 | 176 |
| D10 | 17 | 135 | 26 | 17 | 262 | 1241 |
| D11 | 33 | 571 | 543 | 355 | 343 | 1731 |
| D12 | 8 | 26 | 648 | 1 | 259 | 957 |
| D13 | 4 | 52 | 2254 | 0 | 279 | 816 |
| D14 | 2 | 19 | 1 | 625 | 280 | 853 |
| D15 | 2 | 1 | 2 | 0 | 278 | 176 |
| D16 | 2 | 3 | 1 | 0 | 330 | 176 |
| D17 | 4 | 112 | 755 | 18 | 281 | 1105 |
| D18 | 2 | 15 | 1 | 0 | 269 | 176 |
| D19 | 1 | 78 | 3 | 0 | 358 | 176 |
| D20 | 8 | 25 | 152 | 0 | 362 | 1463 |
| D21 | 2 | 285 | 8278 | 0 | 281 | 888 |
| D22 | 8 | 190 | 143 | 89 | 287 | 719 |
| D23 | 1 | 33 | 206 | 0 | 275 | 762 |
| D24 | 8 | 205 | 5 | 0 | 290 | 707 |


The repair budget table above replaces the initial MyDisease-only illustrative search format. Those earlier illustrative files remain external historical evidence; they are not the settled OLS/MONDO default or an implemented schema. The current byte ceilings use the repaired full-envelope illustrations.

Repeated D01/D08/D21 baseline searches used the same isolated cache and returned identical stdout hashes in 19/9/7 ms, versus 459/275/281 ms on their first invocations. No HTTP trace was collected, so their actual HTTP request counts remain unknown; a fast repeated call alone does not prove zero requests. The one default exact breast get took 2,202 ms and 8,462 bytes. It returned identity/definition/parents/synonyms/xrefs, Open Targets gene scores, a treatment landscape and recruiting-trial count. Its native outcomes show MyChem and ClinicalTrials.gov data despite no explicit section. This is the concrete reason to keep the new default identity card free of hidden enrichment.

The baseline completed 28 product invocations plus version/help checks before the coordinator imposed an offline-only rule for further comparisons. Four planned get/genes/phenotypes commands did not execute. No further live baseline call was made. The PM subsequently required a separate ticket and verified network containment for any future 0.9 execution. The stop was an experiment classifier error: a raw-byte `429` search matched digits inside a decimal gene score. Inspection of structured source outcomes found no 429 or provider failure. The executed script was overwritten before preservation; the evidence store labels its reconstructed original separately from the amended, unexecuted proposal. A retained-body comparison must match actual request identity through the existing TLS recipe. The new direct literal requests and old rewritten requests are different identities; this design does not relabel one as proof of the other. No matched offline section comparison or full per-section token measurement is claimed.

## Contained 0.9.3 search measurements

The separate contained pass `j11622` used the verified official 0.9.3 executable and published main `d45ef78f6e1bd9e99215810079061150a74e575d`. The OS allowed the old process only its local recorder port. Every forwarded request reserved its source and total allowance before network access. Disease requests made exactly 28 new MyDisease calls, bringing this investigation's known direct-source total from 152 to 180. No other disease source received a new request. The source kept no ontology release in these request identities; its release remains unknown.

Fresh clinical review in `j11646` and `j11647` accepts these as bounded historical output measurements and candidate-fidelity witnesses. It does not accept them as complete clinical answers. Commands used `search disease --query TERM --limit 5 --no-fallback --json`. The old resolver sometimes also sent a rewritten cancer-to-carcinoma query; both actual identities are recorded. The table gives whole stdout bytes and an explicit `ceil(bytes/4)` estimate, not tokenizer counts. Each first invocation's local HTTP count equals its new provider count.

| Case | Whole stdout bytes | `ceil(bytes/4)` proxy | Recorded HTTP / new provider requests | Accepted scope |
| --- | ---: | ---: | ---: | --- |
| D01 | 1,356 | 339 | 2 / 2 | Candidate output only. Total 49 is a local union over delivered pages with native totals 36 and 41, not a complete publisher total. |
| D02 | 678 | 170 | 1 / 1 | Identity candidates only; no selected identity or complete clinical answer. |
| D03 | 723 | 181 | 2 / 2 | Candidate output only; no subtype or treatment conclusion. |
| D04 | 176 | 44 | 2 / 2 | Source-local empty result; no disease absence claim. |
| D05 | 1,418 | 355 | 1 / 1 | Candidate output only. D25 reuses this output but its dataset-field conflict remains unresolved. |
| D06 | 934 | 234 | 1 / 1 | Candidate output only; preserve source label conflicts without resolving them. |
| D07 | 1,173 | 294 | 1 / 1 | Candidate output only; no hierarchy or clinical conclusion. |
| D08 | 1,322 | 331 | 1 / 1 | Five displayed candidates from 25 delivered native hits and native total 90. Rank does not select an identity. |
| D09 | 176 | 44 | 1 / 1 | Source-local empty for this term. It does not negate cutaneous melanoma; D08's retained body includes MONDO:0005012. |
| D10 | 1,241 | 311 | 1 / 1 | Candidate output only; no hierarchy or clinical conclusion. |
| D11 | 1,731 | 433 | 2 / 2 | Candidate output only. Total seven combines native totals six and one; it is not one publisher's complete total. |
| D12 | 957 | 240 | 1 / 1 | Candidate output only; no genes or syndrome conclusion. |
| D13 | 816 | 204 | 1 / 1 | Preserve MONDO:0018875 and DOID:0111504 as returned candidates without asserting equivalence. |
| D14 | 853 | 214 | 1 / 1 | Candidate output only; no gene association answer. |
| D15 | 176 | 44 | 1 / 1 | Source-local empty; no disease absence or replacement conclusion. |
| D16 | 176 | 44 | 2 / 2 | Source-local empty; no disease absence or gene association conclusion. |
| D17 | 1,105 | 277 | 1 / 1 | Candidate output only; no gene association answer. |
| D18 | 176 | 44 | 1 / 1 | Source-local empty; no answer about MEN subtypes. |
| D19 | 176 | 44 | 2 / 2 | Ontology-search empty; no incidence measurement. |
| D20 | 1,463 | 366 | 2 / 2 | Identity candidates only; no survival measurement. |
| D21 | 888 | 222 | 1 / 1 | Two MF candidates demonstrate nonexhaustive ambiguity; no selected expansion. |

Capture 0052 preserves a source conflict for MONDO:0056806: the Disease Ontology name says lung nonsquamous non-small-cell lung carcinoma, while the MONDO label says squamous. Neither the output nor a local mapping may resolve that disagreement. Positive identity searches do not answer the bank's gene, phenotype, hierarchy, incidence or survival questions. An empty source result never proves clinical absence.

Warm commands 061–063 repeated D01, D08 and D21 with stdout hashes identical to 037, 044 and 057. They made two, one and one local HTTP deliveries respectively, all from unchanged bodies already captured in this pass, and zero new provider calls. Their local replay speed is neither fresh provider latency nor independent clinical confirmation.

D22–D24 searches and every exact get, gene or phenotype command in this pass remain unmeasured because the source allowance was exhausted before their requests could be forwarded. D25 adds no request and does not resolve the dataset's conflicting clinical fields. Across both slices, 27 output files passed mechanical checks and fresh review only for their stated bounded scope; the other 41 outputs remain excluded. No complete token-budget or same-question 1.0 parity claim follows.

Evidence is on Yellow under `data/biomcp-redux/0136-contained-baseline/`. Archive `stage2.tar.gz` SHA-256 is `9878d81ff99e4cdc940e034c683a0eb7658ee6b50e4b42bf104e214d6d3da578`; extracted `stage2/stage2-manifest.json` SHA-256 is `18b342c3e46ac750ea3bd2dd7ab8c7a424eaf51487d5d5a9c8bfaef04ca385f0`. All 455 files were checked on Yellow. `stage2/stage2/qualified-output-index.json` links each output to exact provider captures and original retrieval dates. Original disease captures and the earlier baseline remain immutable. Actual tokenizer counts, complete section comparisons and final 1.0 CLI/MCP output remain unmeasured.

The one later openFDA rerun, `j11763`, changed no disease measurement and made zero new disease-source requests. Its 11 new openFDA calls bring the two contained passes to 82 provider requests while disease's cumulative direct-source total remains 180. Seven additional drug outputs were accepted only for bounded historical byte measurements after fresh clinical review; their clinical omissions and evidence hashes are recorded in the [drug comparison](drug.md#one-bounded-openfda-rerun-with-the-shared-classifier). None supplies a missing disease gene, phenotype, hierarchy or statistics answer. The original disease captures and all excluded stage2 outputs remain unchanged, and complete token-budget parity remains unmeasured.

## Structured SEER repair and fresh clinical ruling

The six authorized repair requests ran as `j11421`, bringing direct source requests from 142 to 148. There was one attempt per URL with the same 20-second deadline and source stop rule. The final four-request pass below is the only later source research; no old-binary call followed. The retained official catalog supplied every selected code; no disease-name table supplied a site.

| Request | Native selectors and response | What is proved |
|---|---|---|
| D19 structured incidence | Site 47, data_type 1, graph_type 10, sex 1, race 1, age_range 1, year_range 5, data_model 4; HTTP 200 `application/json`,872 bytes,204 ms | A JSON string containing one structured object. Five stage rows carry native rate/SE/CI/count fields; returned year_range 5. |
| D20 structured survival | Site 55, data_type 4, graph_type 1, sex 3, race 1, age_range 1, interval 5, requested year_range 9, data_model 4; HTTP 200 `application/json`,4,648 bytes,545 ms | A JSON string containing a structured annual trend object. Returned year_range 31 contradicts the requested period. Forty-eight annual rows keep observed and modeled values separate. No recent aggregate survival is proved. |
| OLS ontology metadata | Exact `mondo`, `doid`, `ncit` metadata endpoints; HTTP 200 | Versions 2026-10-06,2026-09-30 and 26.02d respectively. These responses contain no license grant. Publisher terms pages, not absence of a metadata field, determine use labels. |
| Current HPO publisher license URL | HTTP 404, HTML | Failed document lookup only. It proves neither absence of HPO terms nor permission to redistribute its annotations. No direct HPO route is retained. |

A fresh clinical reviewer hash-checked both structured bodies and the retained selector catalog. The confirmed incidence row key decodes as Both Sexes, All Races/Ethnicities, All Ages, stage 101 All Stages, subtype 47 Lung and Bronchus(All Types), site 47 Lung and Bronchus, returned year_range 5=2019–2023. Native rate 47.172044 has SE 0.072713, CI 47.029634–47.314797 and count 434448. Other returned stage codes 104/105/106/107 mean Localized/Regional/Distant/Unstaged. These are stage-stratified rates, not stage percentages or a trend. The [publisher Help](https://seer.cancer.gov/statistics-network/explorer/help.html) defines Recent Rates as a five-year age-adjusted incidence rate per 100,000. That method meaning is documented separately; it is not an echoed response field. The structured body does not establish the exact standard population, observed-versus-delay-adjusted rate, registry membership, submission or dataset. Request `data_model 4` and its catalog label cannot fill those gaps. The native count must not become a US-wide population count.

The confirmed survival key means five-year relative survival, Female, All Races/Ethnicities, All Ages and Breast. Returned period 31=1975–2022 differs from requested 9=2016–2022. Rows 1975–2018 contain both observed `rel_rate` and modeled `modeled_rel_rate`; 2019–2022 contain modeled values with null observed rate/SE/CI/count. For example, 2018 observed 93.862708% and modeled 93.171232% are separate source values, and 2022 modeled 93.690345% has no observed estimate. No 91.9% recent aggregate appears. Never average annual rows or borrow the Stat Facts value. All Ages does not establish age adjustment. Exact survival dataset, eligibility rules, registry membership and modeling method remain unverified.

The survival `trend_data` uses native `apc` fields and catalog trend_type 2=Annual Percent Change, while Help describes recent survival trend summaries using average absolute change. Preserve those native fields with an unresolved semantic warning if exposed, or omit trend interpretation; do not rename them as percentage-point changes. The accepted proposal exposes annual source rows and no derived trend interpretation.

The adapter contract decodes the provider's one JSON-string wrapper, then validates the structured object, its declared key order and field order. It preserves requested selectors beside actual returned selectors and adds a problem for a contradiction. It never parses HTML, averages annual statistics, fills missing dimensions from the request or chooses a site from a MONDO label. The [publisher site definitions](https://seer.cancer.gov/statistics-network/explorer/cancer-sites.html) explain that site recodes have histology/year exceptions; a catalog label is not an ontology equivalence assertion. The [revision page](https://seer.cancer.gov/statistics-network/explorer/revisions.html) describes updates but does not bind either captured response to a particular dataset release.

## Identity and section witnesses

D08 preserves the exact melanoma concept at fourth place in MyDisease's first five candidates. D01's broad breast concept is absent from that provider's first five literal hits, while OLS returns MONDO:0007254 first. That is a provider ranking difference, never permission to rerank locally. D02's literal MyDisease query is a genuine zero; the exact annotation endpoint returns the explicit identity. The command must distinguish search syntax from get identity instead of silently repairing the query.

D21 returns MyDisease MONDO:0020481, `myotonia fluctuans`, and MONDO:0009691, `mycosis fungoides`. The published question's possible myelofibrosis expansion is not a label for MONDO:0020481. Do not suppress a returned candidate using a cancer keyword rule. Let the caller choose an explicit concept, or leave the question unresolved.

D15's exact OLS record for MONDO:0007607 has `is_obsolete: true`, two `consider` values (MONDO:0800444 and MONDO:0800445), and null `term_replaced_by`. The captured current concepts distinguish Birt-Hogg-Dube syndrome from Birt-Hogg-Dube syndrome 1. They are not an automatic one-to-one replacement. The [publisher split issue](https://github.com/monarch-initiative/mondo/issues/6518) and [MONDO curation guidance](https://mondo.readthedocs.io/en/latest/editors-guide/using-mondo-for-curation/) support preserving both `consider` relations. Preserve the source's actual exact-match, close-match and generic cross-reference predicates; do not flatten all three to equivalence.

Monarch's D13, D14 and D18 five-row gene pages have native totals 6, 5 and 8, while phenotype totals are 48, 60 and 100. These are association counts, not unique causal-gene counts or patient risks. D18's request for MONDO:0019003 returns a first gene edge whose actual object is MONDO:0008234 (MEN2A), with the requested concept in its closure. The adapter must retain the requested ID beside the actual edge object and its native closure where needed. The [Monarch data model](https://monarch-initiative.github.io/monarch-py/Data-Model/) defines closure as the object plus its ancestor identifiers. It cannot relabel the edge as an exact direct association with the queried parent. The final contract puts requested ID beside native row IDs. An exact endpoint match is a structural identity statement, not a clinical-directness label. D14's five gene rows contain only CCND1 and VHL, with differing predicates and primary sources; D18's five rows all name RET but target MEN2A, familial medullary thyroid carcinoma or MEN2B. D13's captured CHEK2 associated-with row must not be upgraded into TP53-equivalent syndrome causation. Native `has_phenotype`, negation, original OMIM/Orphanet identities, evidence codes and optional qualifiers stay attached to their individual rows. A null qualifier remains absent. The primary knowledge source remains provenance; the HPO-published annotation grant governs the captured annotation. Separately licensed upstream content needs independent payload evidence.

CIViC's disease-name search proves a discovery capability. Returned rows carry native DOID and molecular-profile IDs. A name query alone does not prove an exact join to a selected MONDO concept. Disease-context CIViC retrieval is dropped from this slice because no harvested question needs the extra direct source. Existing variant commands retain their independently accepted profile/claim contract.

D19's captured Stat Facts page states 47.2 new lung/bronchus cases per 100,000 men and women per year, age-adjusted, based on 2019–2023 cases. The same page's five-year relative survival is 29.5%, based on SEER 21 excluding Illinois for 2016–2022. D20's page states 91.9% five-year relative survival for female breast cancer in that survival dataset/period, and separately 132.5 new cases per 100,000 women per year for 2019–2023. A parser cannot substitute a similarly named ontology subtype, another trend statistic, projected cases or a different period. The source pages and hashes, not a copied constant, anchor future cases. The structured repair above establishes two native result forms; it does not establish the recent aggregate survival answer or an ontology-to-site mapping.

HPO term metadata establishes exact term lookup, not a direct disease-to-phenotype endpoint or a phenotype-based diagnosis. MyDisease and Monarch can carry separately attributed HPO annotations. The design must not give their aggregate a blanket ontology-only license. No source was asked to classify a patient's signs.

## Structural risks and case harvest

Identity, terms and command facts each have one owner. The disease adapter must not restate shared source registry terms or maintain its own clinical synonym table. It must not copy consumer request types into neutral `biodata`, nor put provider/CLI rules into the neutral layer. The source-specific edge predicate and the source's subject/object remain the truth; deriving a concise header must not erase them.

Cases should follow the D01–D25 stories and retained capture identities. At minimum they cover every accepted command and option, exact identifier checks, namespaced cross-references, first/middle/last page and next actions, duplicate IDs, multiple candidate concepts, obsolete concepts with `consider`, graph expansion versus direct edges, source-native empty, disabled and missing-key skips before network, 429, 500, timeout, broken body and GraphQL data-plus-errors. These last failure cases are required recorded witnesses, not live failures claimed by this investigation. Each named filter needs a captured positive and a contrasting negative. Keep raw provider bodies outside Git where rights are unknown; publish only source-permitted facts or synthetic wire cases. No licensed clinical answer set enters the repository or an agent under test.

The statistics cases must assert measure, population, period, site and dataset together. The assertion that a number exists is insufficient. The ambiguous and obsolete cases must assert that no chosen identity or executable unsafe join is returned. Each association case must assert original/native endpoints and predicate, not only a gene symbol. Section budgets apply to the actual full envelope after implementation; illustrative JSON measurements cannot replace that proof.

## Final bounded source and catalog findings

Job `j11468` made exactly four additional direct requests, bringing the direct total to **152 of 180**. Each request had one attempt and the existing 20-second deadline. It made no name search, alternate-ID retry or old-binary execution. The same job measured the retained SEER catalog offline without a provider request.

The retained D05 exact MyDisease record establishes MONDO:0005061 with native `mondo.xrefs.efo` value EFO:0000571. The one Open Targets request sent native `efoId=EFO_0000571`, requested its disease ID/name and a five-row `associatedTargets` page, and returned HTTP 200 with `{"data":{"disease":null}}`. No positive disease-gene record or count came back. This establishes only the result of that exact selector, not gene absence or a universal failure of EFO mappings. The Open Targets disease-genes section is therefore **deferred** under the PM's permitted outcome. Earlier positive Open Targets pages used source-native candidate IDs and do not substitute for the missing cross-provider join. No Open Targets section, automatic fallback, next action or accepted byte ceiling is advertised for the initial disease build. D11/D22 retain their discovery/cross-reference stories, but their Open Targets gene/score answer cases are deferred with this join. A Monarch predicate is not substituted for an Open Targets score.

The current HPO ontology repository `LICENSE.md` returned 200 and points to its publisher website. The current publisher repository's `license.md` and its rendered license page also returned 200. The source file retains a 2015 last-modified header; this is a currently served publisher grant, not a newly issued 2026 license. Its stated coverage includes HPO vocabularies, annotations, tools and documentation. It permits use subject to acknowledgement/citation, date or version on public file displays, preservation of content and logical relationships, and a product acknowledgement. The declared use is `attribution`: the grant allows commercial use, and no publisher clause requires a restricted or noncommercial category. Citation, version, integrity and acknowledgement conditions remain attached. This follows the shared use-category ruling and supersedes the earlier failed-lookup classification. The adapter must preserve source facts and source dates/versions where supplied; absent component version remains an explicit metadata problem, not a guessed version or a claim that the grant covers a separately licensed upstream payload. The [current repository license pointer](https://github.com/obophenotype/human-phenotype-ontology/blob/master/LICENSE.md), [publisher license source](https://github.com/Human-Phenotype-Ontology/Human-Phenotype-Ontology.github.io/blob/master/license.md) and [rendered publisher terms](https://human-phenotype-ontology.github.io/license.html) establish this reviewed label. Direct HPO remains dropped because no harvested question needs the extra call.

The retained SEER catalog has 73 native active-state records. Offline formatting serialized all 15 pages at five rows per page, preserving native order, code, label, active flag, source outcome and a next action only before the terminal page. Whole envelopes measure 490–808 bytes, with explicit bytes/4 proxies 123–202. The proposed 950-byte ceiling is close to that measured maximum. These are source-fact serialization measurements; actual CLI/MCP output remains **UNMEASURED**; the unmeasured 0.9 catalog comparison belongs to [0136](../../tickets/0136-contained-0-9-drug-disease-baseline.md).

The final capture manifest is `biomcp-redux-0117-final-source-captures/requests.json`, SHA-256 `dd209e16f4842cc9af38cbf0c840a5b8a8762344bcfb105275cdb016e935b8ea`. Its four body hashes were verified on Yellow. The null Open Targets body SHA-256 is `24f2b330e1138cd1fe3e9ff3e599ab45df608e110092cc204dbc4b2dd9e7bc29`; HPO repository pointer `534f0ba30b74d13590247ddb5e24d749f5733b41cf19a730010b55edf4266336`; publisher license source `9f7c918c3ac9b6eab76eb21fb624a595fa9d306017a5b5aa03e305b86183a7a2`; rendered license `d63799ff8d381891114b22e78c8164f198a9642f9ed0ca8b53bc73238c318b39`. The catalog-format manifest is `biomcp-redux-0117-catalog-format/measurements.json`, SHA-256 `553528dfbf78dafa210f63c04292009f925955e00f330f2f06f4623851251918`; all 15 serialized page hashes were verified. Exact request plans and the executed offline formatter are retained with this external evidence. No licensed source body enters Git.

## Ian's questions

None.

## Evidence

All new captures and the released baseline are immutable external evidence on Yellow under `/home/ian/workspace/data/biomcp-redux/0117-disease-investigation/`. The source directory is `biomcp-redux-0117-disease-captures`; its `requests.json` SHA-256 is `f61f5c95d7c43d34d71d46cd75fe6e3f2abfb2f0022bb317c878535ec5d4d566`. Each row stores the exact URL/body, UTC retrieval time, HTTP status, elapsed time, body byte count and SHA-256. All 142 copied body hashes were independently checked on Yellow. The initial direct capture ran as `j11373`; the follow-up ran as `j11381`.

The baseline directory is `biomcp-redux-0117-disease-baseline`; its `manifest.json` SHA-256 is `a9da53b5d52861cf13cb7af9c460968e65d55e3c0ea20796b52f412b31d8d32b`. All 31 copied stdout hashes were checked on Yellow. It ran as `j11378`. HTTP request counts remain unknown. The parent directory preserves `capture.py`, both exact direct request plans, an explicitly reconstructed executed baseline script, the amended but unexecuted baseline proposal and the original bounded experiment plan. None contains credentials. The shared released executable and archive provenance remain owned by the parallel drug investigation until final cleanup.

The illustrative serializer and `biomcp-redux-0117-disease-analysis/` preserve each proposed JSON and the case matrix. Analysis job `j11396` made no provider requests. The matrix SHA-256 is `ec92e1271f0469c82a6b06918302366c261df6b0de1cef7b757d0ba502aa8a96`.

Retained trial evidence is reused without a refresh from `/home/ian/workspace/data/biomcp-redux/0079-trial-investigation/`, captured on 2026-10-24 in the original trial investigation. `mydisease-metadata.raw` SHA-256 is `0e25bc4ea24f7a2ae569dccca452004be2e8d8cf2974dedb414fa955e48c8a06`; `mondo-license.raw` SHA-256 is `f5b745ef98087f531e719ee8ca6a96809444573ecc7173c6fa68eaad39b3cc3f`. Reusing those files is not a newly observed provider response. No product code, schema, roadmap box or later ticket was changed by this design.

The repair capture directory is `biomcp-redux-0117-disease-repair-captures`; its `requests.json` SHA-256 is `8cc788bb74db36532fbf713fce98fbfe48d54c6582fb10c5e87de952802c300d`. All six copied body hashes were verified on Yellow. The two structured SEER body hashes are `cfc6e2d3339f378349fe1b5de64a29e247acfd1279c4e3537a6a2bdbaf0d41c1` and `d0133734879bb7f014eb507727d0de750080fddfc8c16e4be05e3ec534ae3eb4`. The exact six-request plan is retained alongside them. A fresh clinical reviewer independently checked those bodies plus the earlier selector catalog; the ruling is recorded in the structured repair section above.

Formatting-only jobs `j11428` and `j11436` read retained bodies and made no requests or product executions. The initial repair illustrations remain separate from the final examples. `biomcp-redux-0117-disease-repair-views-final/measurements.json` SHA-256 is `8717dd166377498876a73b9c8dc5887b7d691bdcd559936c50ea4b339753381e`. All 43 copied final illustrative envelope hashes were verified on Yellow. The executed formatting scripts are retained separately. These proposal measurements never substitute for generated-schema CLI/MCP acceptance or future verified source request identities.


## Exact OLS discovery and child witness

The bounded native capture `j11900` sent the default search to one ontology with literal text, `q=breast+cancer&rows=5&start=0&ontology=mondo&groupField=iri`. OLS returned 76 candidates. Offsets 0, 5 and 71 each returned five native rows. Separate DOID lung-adenocarcinoma and NCIt melanoma searches, literal CURIE discovery, the ambiguous `MF` query and the multiple-endocrine-neoplasia subtype search all returned positive pages. The immutable request manifest SHA-256 is `7cd1db0335a1cb96203260f4ea7887b54ebdc9e94df9f3ee8c3014f063f106ba`. These source facts do not establish a completed product comparison with 0.9.

The same capture requested `https://www.ebi.ac.uk/ols4/api/ontologies/mondo/terms/http%253A%252F%252Fpurl.obolibrary.org%252Fobo%252FMONDO_0019003/children` on 2026-10-25 at 01:37:08 UTC. Its HTTP 200 body contains three native children: MONDO:0007958, MONDO:0008082 and MONDO:0008234. The body SHA-256 is `3c6533a9e3dc11f2bc79a460c3e97c541c1bef48c712b408c7bcaf4e7e05c985`. This positive witness supports a later child-relation ticket. The current hierarchy remains parents only, and the OLS adapter does not implement hierarchy in this ticket.

The exact DOID:3910 record contains synonym strings and a null `obo_synonym` field. Its synonym rows therefore omit scope. A missing source label on a cross-reference endpoint remains absent. Native cross-reference descriptions remain unchanged beside `oboInOwl:hasDbXref`; arbitrary description text does not become a label or an equivalence predicate. The obsolete MONDO:0007607 card retains both native `consider` IDs and a null replacement. No alternative is selected automatically.


### Executed OLS command budgets

The OLS product proof `j11984` used eleven hash-checked retained responses through the real transport recorder and then offline CLI/MCP cases against the generated schemas. It made no provider requests. These executed values supersede the earlier OLS unmeasured notes for the specific identities below; the earlier tables remain historical illustrations. Retrieval dates in recordings remain the original capture dates. The actual transport URL can encode a query space as `%20` where the source capture used `+`; the decoded parameters and literal text are compared exactly, and the recorder creates each request identity itself.

| Exact command witness | Full JSON bytes | Comparable bytes | Comparable case ceiling |
|---|---:|---:|---:|
| MONDO breast cancer, offset 0 | 2,056 | 1,922 | 2,211 |
| DOID lung adenocarcinoma | 2,112 | 1,978 | 2,275 |
| NCIt melanoma | 2,147 | 2,013 | 2,315 |
| Literal MONDO:0007254 discovery | 816 | 682 | 785 |
| Ambiguous MF discovery | 2,105 | 1,971 | 2,267 |
| Multiple endocrine neoplasia type 2 discovery | 2,021 | 1,887 | 2,171 |
| MONDO breast cancer, offset 5 | 2,104 | 1,970 | 2,266 |
| MONDO breast cancer, offset 71 | 1,948 | 1,814 | 2,087 |
| Exact MONDO:0007254 card | 1,223 | 1,089 | 1,253 |
| Exact DOID:3910 card | 1,097 | 963 | 1,108 |
| Exact obsolete MONDO:0007607 card | 839 | 705 | 811 |

The comparable calculation removes exactly 134 validated bytes for the OLS outcome's canonical `use`, `terms` and `terms_url` fields. Component terms remain included. Each fixed case ceiling is the measured comparable size plus 15 percent, rounded up. Full sizes remain visible. Token figures remain byte-count proxies; no tokenizer measurement is claimed.

The synthetic maximum-page boundary returns fifty source-ordered candidates and all fifty exact get actions plus one continuation. It measures 14,160 full bytes and 14,026 comparable bytes, with a fixed 16,130-byte case ceiling. The default remains five candidates. A limit of 51 fails before I/O. Other entities keep their existing next-action bound.

The contained 0.9.3 discovery measurements below use MyDisease's older rewritten requests, so they do not provide matched OLS request ratios. The historical D02 default get measured 8,462 bytes and included hidden enrichment. The exact OLS card now exposes only its requested source facts and named section actions. Those different command scopes do not establish complete clinical parity. The broader comparison remains owned by the comparison work after the source adapters integrate.


The final section measurement `j12001` reused the exact DOID:3910 response. Its three unscoped synonyms measure 1,003 full bytes and 869 comparable bytes, with a 1,000-byte comparable ceiling. Five cross-references measure 1,829 full bytes and 1,695 comparable bytes, with a 1,950-byte ceiling. The two-row synonym pages at offsets 0 and 2 measure 881 and 585 comparable bytes, with ceilings 1,014 and 673. The two-row cross-reference pages at offsets 0 and 2 measure 1,038 and 1,043 comparable bytes, with ceilings 1,194 and 1,200. These actual DOID command measurements differ from the earlier illustrative D08 allocations. Native endpoints, predicates, descriptions and per-row attribution remain included. Every ceiling uses the measured comparable output plus 15 percent, rounded up. No same-case old synonym or cross-reference command measurement exists, so no reduction ratio is claimed.

The external product proof is preserved on Yellow at `data/biomcp-redux/0140-ols-product-proof/j12001.tar.gz`, SHA-256 `b64dc7a38c101250c1b0cb34fdaf874dd21842c03fe07f0daa8b4faa4a9d57d0`. Its 36-file manifest SHA-256 is `666fbab85dc675197eed992e4ab7042c5f0b53c9368895c9183f80bc823f9edd`. Every stored file was independently checked after transfer. Source bodies and recordings remain outside Git.
