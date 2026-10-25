# Pharmacogenomics slice investigation and proposed design

This investigation starts from `d18bfbba511e3852c99228f1a5166a14633e5cee`. It covers steps 1 to 4 of [the slice method](../slices.md). It proposes source retrieval, typed disclosure and exact follow-up commands. It does not calculate a patient phenotype, select therapy, recommend a dose or implement product code.

## Evidence boundaries

Official documentation was read on 2026-10-25. Source response counts, field presence, latency and product budgets require the bounded capture below. A documented capability is not a successful public-tier request. The source captures must precede acceptance of shared types. Historical source adapters and public papers do not replace that proof.

The reviewed drug, variant and disease designs establish source-owned identities, explicit selectors, independent collection pages and absence distinct from zero. The shared model in [biodata variants and annotations](../biodata-variants-and-annotations.md) establishes `Statement`, `Subject`, source attribution and native fallback. Its proposed `Subject` is broader than the `VariantRef` currently implemented at the starting revision. This slice must not silently implement either expansion or a star-allele subject.

## 1. Questions and accepted answers

Clinical question rulings are delegated to a fresh clinical reviewer. The question bank below will name each primary passage and its accepted source-level answer. A public paper supports the question and clinical scope; captured replies establish which current API answers it. A failed API cannot invalidate a published recommendation, and a publication cannot prove an API field exists.

## 2. Source findings

### Official documentation

| Source | Primary reference and date | Finding relevant to the design |
| --- | --- | --- |
| CPIC | [Database documentation](https://github.com/cpicpgx/cpic-data/wiki), edited 2026-03-09; read 2026-10-25 | Documents a relational API, source gene and drug keys, named alleles, diplotypes, independent recommendation lookup methods, populations and native frequency records. A tracked pair need not have a guideline. API data can change between releases. |
| CPIC | [March migration announcement](https://blog.clinpgx.org/updates-to-the-cpic-database-and-api/), 2026-03-04, updated 2026-03-13 | API host remains `api.cpicpgx.org`. `clinpgxid` replaces former PharmGKB identifiers; guideline identity changes from an array of annotation IDs to one guideline entity ID. Pair annotation level becomes `clinpgxlevel`. Do not populate old fields by assumption. |
| CPIC | [Releases](https://github.com/cpicpgx/cpic-data/releases), read 2026-10-25 | Published application and database versions can differ. Retrieval date alone is not a dataset release. Preserve a release only when the response or retained export states it. |
| ClinPGx | [API introduction](https://api.clinpgx.org/), read 2026-10-25 | Public JSON API advertises annotations and reference objects; parameters and replies may change. Published request limit is two per second. Capture recipe uses serial requests with one second between admissions. |
| ClinPGx | [API documentation](https://api.clinpgx.org/swagger/), read 2026-10-25 | Documentation requires JavaScript. No current annotation field, key requirement or server pagination contract has yet been proved by a data capture. The old adapter's `view=min` is a hypothesis to test, not an accepted tier contract. |

### Use and terms

Keep API access, data reuse, website text, guideline manuscripts and third-party components separate. A keyless HTTP success proves access, not permission to redistribute. No curator answer or complete source body enters Git.

| Asset | Current evidence | Proposed use label and group | Unresolved scope |
| --- | --- | --- | --- |
| ClinPGx API data | API introduction explicitly binds API data to [the current data policy](https://www.clinpgx.org/page/dataUsagePolicy). The policy page exposes only a JavaScript shell through the text reader. Current [gene page](https://www.clinpgx.org/gene/PA142672008) and [chemical page](https://www.clinpgx.org/chemical/PA449211/prescribingInfo) footer names Creative Commons Attribution-NonCommercial-ShareAlike 4.0. | Propose `noncommercial`, group `noncommercial`, with the policy URL and exact publisher name. Do not reuse the legacy BY-SA label without verification. | Capture the actual current policy text and any API-specific terms. A footer alone does not prove every underlying asset's license. |
| CPIC database/API | Current database wiki links its license/citation section to the ClinPGx policy. | Provisional `restricted`, group `restricted`, with unresolved CPIC scope stated; PM must settle against captured policy. Source remains open by default under the standing product rule unless disabled or a required user key is missing. | Establish whether current policy distinguishes CPIC database facts, CPIC guideline text and other ClinPGx content. Do not infer CPIC manuscript terms from an API or website footer. |
| CPIC manuscripts | Each journal or source publication carries its own reuse terms. | Preserve native publication URL and actual manuscript terms when exposed. | No copied recommendation prose is licensed merely by a PMID. |

The [open-by-default decision](../../decisions/2026-10-24-sources-are-open-by-default.md) supplies the existing labels and disable groups. The [keyed-source decision](../../decisions/2026-10-24-keyed-sources-run-only-with-the-user-s-key.md) applies if current terms require a user's account or key. Such a source makes no request without that configured key. Do not borrow credentials, infer authorization from old code or fall back to an unofficial mirror. A 401 or 403 ends that source's capture allocation. No request for paid access or account creation is part of this investigation.

### Identity and joins

Source identities remain separate. A CPIC gene symbol is a source key; a returned HGNC, NCBI or Ensembl identifier can support a typed gene next step after equality checks. A CPIC drug key is its published source-qualified identifier. Preserve every supplied cross-reference and its namespace. A ClinPGx PA identifier is a ClinPGx object identity, not a CAid, an rsID or a universal drug identifier. A guideline ID, an annotation ID and a publication PMID refer to different objects.

Named alleles keep gene context, native allele ID, exact source spelling, optional definition ID, copy representation and source version. Diplotypes keep the entire source expression, native record identity, gene context, allele references and original lookup data. Preserve unknown, uncertain, duplicated and multiple returned mappings. Do not sort two allele strings, infer phase, strip copy suffixes, turn a star allele into one genomic variant, or calculate an activity score.

The source may return phenotype, activity score or allele status as different lookup methods. Keep the declared method and exact source value; never coerce them into a common phenotype string. Preserve the full multi-gene lookup key, drug, population, guideline and every matching recommendation. A TPMT row must not discard its NUDT15 context. A partial key is not a uniquely selected recommendation.

Cross-entity actions require explicit source links in the captured record. Emit `get gene` only with an identifier the gene slice accepts. Emit `get drug ID --source SOURCE` only with a source-supported identity relation proved in that slice. An RxNorm ID with no supported direct drug route remains a cross-reference. A single source-given reference-qualified HGVS or checked allele ID may supply a variant lookup; an rsID may return several alleles. No name comparison links drugs, genes, conditions, profiles or patients.

### Bounded native capture plan

The build coordinator gives this recipe to a separate build delegate. Run it through `pm job` from a pushed full revision. Use a new experiment with an OWNER naming this ticket. Keep all replies, headers, request bodies, actual recording identities, failure logs, recipes and manifests outside Git and archive them on Yellow. The investigator makes no source data API calls itself.

The risk is whether current public CPIC and ClinPGx tiers carry lossless identities, context, lookup mappings and bounded pages needed by this design. The recipe uses GET only, no cookies, no credentials unless the coordinator explicitly supplies an already authorized user key, no downloads of a whole database and no legacy executable. All query parameters are passed through a URL encoder, not interpolated as shell code.

**Proposed request ceiling: 38 physical data HTTP attempts: 28 CPIC and 10 ClinPGx.** Every attempt, including a failed connection, consumes its reserved row. No retries, redirects, refreshes, fallback hosts, altered queries or automatic continuation. A redirect is recorded and refused. A missing column or invalid filter is retained as a failed witness; it does not authorize a repair request. A 401/403 stops remaining rows for that source. A 429 stops that source with its headers. Continue independent rows on the other source. Timeout each request after 20 seconds; bound each body at 4 MiB. Space admissions by one second. These are investigation limits, not proposed product configuration.

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
| C27 | `pair_view?genesymbol=eq.TP53` | Public issue's wrong-gene routing regression; expected API result remains unmeasured. |
| C28 | `pair_view?genesymbol=eq.DPYD&cpiclevel=eq.A` | Known-positive native level filter witness before exposing the option. |

ClinPGx base is exactly `https://api.clinpgx.org/v1/`. Requests send `Accept: application/json`. Rows P01–P06 query `data/clinicalAnnotation?location.genes.symbol=GENE&view=min` in C01 gene order. P07 queries `data/guidelineAnnotation?relatedGenes.symbol=DPYD&view=min`. P08 queries `data/labelAnnotation?relatedChemicals.name=irinotecan&view=min`. P09 queries `data/clinicalAnnotation?location.genes.symbol=TP53&view=min`. P10 queries `data/clinicalAnnotation?location.genes.symbol=DPYD&view=base`. These are exact legacy request hypotheses for current-tier verification. Do not claim `view=base` provides full details unless that actual reply proves it. No client-side truncation proves source pagination. If this API returns an entire collection, record its body and count within the body bound, do not issue an unbounded follow-up, and keep a public product page unsupported until a bounded source or checked local collection recipe is reviewed.

Record each physical attempt in an append-only ledger with row ID, exact URL and method, safe headers, UTC start/end, elapsed milliseconds, response status/content type, source response request ID if present, byte length, SHA-256, retrieval date, redirects refused, complete/truncated/read-failed state, parse outcome, top-level count, collection totals and continuation fields. Record native identity paths, absent/null/empty distinctions, nested row multiplicities and all scopes. Never invent a request ID, hash, count or date. Do not store secrets in the ledger. The recorder must begin in an empty directory and capture actual requests, not reconstruct identities later.

The delegate reports raw compact JSON bytes and a consistently labeled approximate token count, separate from hypothetical 1.0 bytes. Do not compare complete upstream rows to compact command output as a product regression. No 1.0 PGx command exists at the starting revision, so its output, request cost and cold/warm timings remain unmeasured. Documentation/policy retrieval is a separate bounded allowance to be set by the coordinator; preserve the exact policy text or script asset and distinguish website/footer evidence from API data rights.

## 3. What would delight the agent

1. **First question.** “Which source guidance exists for this explicitly named gene and drug?” One `search pgx --gene SYMBOL --drug-id ID --source cpic` returns source pair candidates and native guideline presence. Do not guess whether a bare word is a gene or drug.
2. **Smallest useful header.** Native pair ID if present, exact gene and drug IDs, source labels, guideline reference, native evidence/testing labels, source terms and a section count when the provider reports it. A CPIC level is not a source recommendation strength or a ClinPGx annotation level.
3. **Next step.** Typed actions expose the exact selected guideline, a recommendation page, a source diplotype lookup, annotations and source frequency records. Each action keeps the source, gene, drug and all page selectors. Missing join IDs produce a named limitation rather than a name-based action.
4. **Identity.** Return exact gene IDs, drug key, PA identities, guideline IDs, recommendation IDs, alleles/diplotypes and PMID links. Names aid display; IDs permit chaining.
5. **Disagreement.** Display separate source records with their own evidence framework, population, drug, genotype/diplotype, guideline date and classification. Do not vote, blend levels or select the newest date as a clinical winner.
6. **Empty and failure.** Native successful zero, an unsupported public field, missing user key, disabled source, rejected identity, malformed JSON, timeout and a truncated collection have distinct outcomes. None means “safe dose” or “no clinically relevant association.”
7. **Speed.** Start with one selected source and one bounded page. Named sections call only their owning source. Cold/warm latency remains unmeasured until actual commands exist; no read synchronizes a database.
8. **What changes from 0.9.** Remove bare-text gene/drug routing, substring evidence filtering, hidden overfetch and automatic multi-source section expansion. Keep source facts, complete context, independent paging and useful failure distinctions.
9. **Stop early.** A pair header can answer whether a source tracks that exact pair and links a guideline. A source-reported mapping can answer how the source labels an explicitly supplied diplotype. Neither turns into a patient-specific treatment choice.
10. **Trust.** Every source claim retains its record URL, publication, framework, date/version when supplied, exact matching scope and terms. Source revisions and retrieval times stay separate.

## 4. Proposed types and command tree

All names below are proposals for PM review. Reuse shared strings, identifiers, dates, source outcomes, terms, pages and typed actions. Generate schema and help once from the accepted types. A capture's missing field remains optional; an entire tier missing the field makes the section unsupported. Do not add a field because old structs contain it.

### Domain records and Statement boundary

`PgxPair` contains source-owned pair identity, gene/drug references, optional guideline identity and separate native CPIC level, ClinPGx level and testing labels. It contains no inferred interaction severity. `PgxGuideline` contains logical guideline ID, source publication references and source update metadata. Publication date and API retrieval date remain distinct.

`PgxNamedAllele` contains source ID, gene reference, source name, optional definition ID, native copy value and source-defined sequence/location references. `PgxDiplotypeResult` contains the source diplotype record/expression, complete gene context, native allele members, lookup method, source lookup key, phenotype/activity/allele-status values when present and all returned matches. It does not implement allele calling, phasing or diplotype normalization.

`PgxRecommendation` preserves source row identity, whole multi-gene lookup key, drug, guideline, population, classification/strength, source recommendation, implications, comments and publication references. `PgxAnnotation` preserves native annotation kind, record identity/version, evidence framework, genes, chemicals, variant or haplotype members, phenotype or outcome scope and reference links. Clinical, guideline and label annotations are separate typed collections. `PgxFrequency` preserves named allele, source population/group, publication, native frequency value/label and source-provided counts or aggregation method. It computes no frequency or ancestry assignment.

A star allele or diplotype is not a `VariantRef::Allele`. A pair-level recommendation is not a claim about an arbitrarily chosen genomic member. The currently agreed shared `Subject` alternatives do not explicitly model a phased named-allele diplotype. Preserve such source records natively with `statement_unmapped: true`. The same applies when a source classification is outside approved propositions or carries no supported subject identity. Do not misuse a compound molecular profile to conceal phase, copy or gene-result semantics. A new proposition or subject kind requires both owning PMs' agreement under the shared model. An explicitly source-given genomic variant claim may use existing `Statement` only when subject, proposition, classification and context fit without interpretation; no clinical annotation level becomes therapeutic sensitivity or resistance by inference.

### Proposed consumer schemas

Propose `biomcp.pgx_page/1`, `biomcp.pgx_card/1`, `biomcp.pgx_guidelines/1`, `biomcp.pgx_recommendations/1`, `biomcp.pgx_diplotype/1`, `biomcp.pgx_annotations/1` and `biomcp.pgx_frequencies/1`. This investigation changes no schema. `biodata` owns domain records without I/O. `biomcp-types` owns request selectors, page limits, response headers, outcomes and next actions. Source adapters own checked native replies. Query composes no clinical conclusion.

### Command proposal

| Command | Header and disclosure | Exact selectors and next actions |
| --- | --- | --- |
| `search pgx --gene SYMBOL --source cpic` | One page of tracked gene/drug pairs with native guideline and level labels. | `--drug-id` narrows with an exact source key only after a positive native capture. `--cpic-level` and `--pgx-testing` wait for their own positive native selector proofs. `--limit`/`--offset` keep independent native pages. |
| `search pgx --drug TEXT --source cpic` | Source-native drug candidates, never an automatically chosen pair. | Native text selector must be proved; unchanged literal input is echoed. Candidate actions use exact returned IDs. No local substring filter or reranker. |
| `get pgx --gene SYMBOL --drug-id ID --source cpic` | Selected pair identity and concise guideline references only. | Reject missing/ambiguous pair scope. No default recommendation, automatic phenotype or multi-source fan-out. |
| `get pgx --gene SYMBOL --drug-id ID guidelines --source cpic` | One source guideline/publication page. | `--section-limit`/`--section-offset`. Every linked article uses the exact returned PMID. |
| `get pgx --gene SYMBOL --drug-id ID recommendations --source cpic` | Native recommendation rows with full lookup keys and populations, explicitly unselected if several apply. | `--population` and a source-owned `--lookup-key JSON` are proposed only after exact selector captures; do not accept a bare phenotype as a universal key. All matches stay reachable. |
| `pgx diplotype --gene SYMBOL --diplotype TEXT --source cpic` | Exact source lookup result(s), with source-reported result method and values. | No score calculation. Native zero and multiple rows differ. A recommendation action includes a complete source key, drug and population only when established. |
| `get pgx --gene SYMBOL annotations --source pharmgkb` | One explicitly selected annotation-kind page with exact source subjects and native evidence levels. | `--kind clinical|guideline|label` selects independent collections. Default kind proposed `clinical`. `--section-limit`/`--section-offset` need proved source paging or an approved local snapshot. |
| `get pgx --gene SYMBOL frequencies --source cpic` | Native named-allele population rows with study/group context. | Independent page selectors; no local ancestry inference, aggregation or patient risk calculation. |

The source registry can retain `pharmgkb` as its compatibility identifier while displaying publisher ClinPGx and current terms. A rename needs a PM ruling; do not add two registry sources for one API. Literal identifiers retain their published namespace; no invented `CPIC:GENE:DRUG` compound identifier is accepted. The explicit named options above remove the old gene/drug word-shape heuristic. Required source-specific identifiers are checked before I/O.

### Disclosure and measurement budget

Default search exposes one small pair page. The default card exposes pair identity and guideline links. Recommendation prose and complete multi-gene context belong in a named section; source annotation bodies and frequency studies lie deeper. Every row remains reachable through an exact next action. Start the default page proposal at five rows, then measure complete actual output on the harvested cases before accepting a byte budget. A smaller default changes page size, never removes fields from the returned row.

| Layer | Measurement at this design revision | Budget acceptance requirement |
| --- | --- | --- |
| Default pair search/card | Unmeasured; no PGx product command exists. | Measure CLI and MCP compact JSON on every representative source capture; keep full envelope and validated comparable body separately. |
| Guideline page | Unmeasured. | Preserve PMID/ID/date roles and every returned source link; set a per-case full-output ceiling from measured size plus reviewed headroom. |
| Recommendation page | Unmeasured. | Include all genes, population, guideline and source text. Do not meet a cap by deleting context. Reduce default row count if needed. |
| Diplotype lookup | Unmeasured. | Include every source mapping and copy/phase context. A zero/multiple result gets its own ceiling. |
| Annotation/frequency pages | Unmeasured. | Establish actual public fields and complete collection bounds first. Keep each annotation kind independent. |
| 0.9 comparison | Unmeasured for all PGx commands. | Verified network containment and an approved per-request recipe must precede execution. No old binary runs in this investigation recipe. |

No numeric product budget is presented as measured. Approximate upstream tokens do not establish a command ceiling. Routine implementation cases must enforce full-output byte ceilings, compare CLI and MCP and preserve source provenance. Any default larger than the same-question contained 0.9 output requires an explicit explanation of added facts. A missing contained 0.9 measurement remains a named comparison gap.

### Proposed outside-in case map

The final case bank maps the harvested questions to every proposed command and option. Shared failure cases include 429, 500, timeout, broken JSON, HTTP 200 error body, mismatched gene/drug/diplotype identity, missing field, missing key, disabled source and warm-cache exclusion. Failure cases are authored replay controls, not deliberately induced provider failures. Reject invalid limits, conflicting selectors, malformed explicitly typed identifiers and missing multi-gene context before provider work.

Use exact native source recordings for known-positive gene, drug, pair, level, testing, population, lookup-key and annotation-kind selectors. Add a second-page case that proves no duplicated or lost rows. Preserve duplicate labels with distinct native identities and all recommendations for different populations. Include gene-key presence with no guideline, literal missing diplotype, complete multi-gene response, copy-bearing allele and source classification with an unsupported Statement subject. Do not label a synthetic key or body as native clinical proof.

### Structural risks

Keep source identity and field mapping in one adapter. Generate source names, help and schema from the accepted contracts. Preserve independent annotation kinds instead of collecting a single flattened list that loses the collection total or provenance. Use the shared transport and cache admission checks; no custom client, fallback host, retry or source-specific credential store. A source response whose identity differs is rejected before caching. Isolate tests through the existing runner. Keep source recordings and raw process output outside temporary test lifetimes when a proof is retained. No general PGx inference engine or variant parser is justified here.

## 0.9 command, option and section accounting

Static inventory uses the legacy checkout at `4050fdc7dbdf4d713d94cf02a39a147b863665be`, especially `src/cli/pgx/mod.rs`, `src/cli/pgx/dispatch.rs`, `src/entities/pgx.rs`, source adapters and `docs/user-guide/pgx.md`. This is source inspection, not executed output, not evidence of a released 0.9 binary's behavior and not a timing or parity claim. Proposed dispositions await PM review.

| Legacy surface | Static behavior | Proposed disposition and case need |
| --- | --- | --- |
| `search pgx` | CPIC pair rows; gene/drug search required. | Keep with explicit source, native identities, faithful totals and direct source selectors. |
| `-g`, `--gene` | Gene text uppercased and used as primary query. | Keep explicit gene selector; use the source's supported key grammar, not free-text gene detection. |
| positional search `QUERY` | Alias for gene option; shared conflict checks. | Drop ambiguous positional alias; retirement names `--gene`. |
| `-d`, `--drug` | Source drug-name substring query; combined gene/drug filters locally. | Keep native candidate discovery after positive selector proof; add exact `--drug-id` for selection. No local name join. |
| `--cpic-level` | Local filter after a bounded overfetch. | Keep only after direct native positive filter capture C28. Separate from recommendation strength and ClinPGx level. |
| `--pgx-testing` | Local match over five accepted labels. | Undecided until direct native selector and current enum proof; no hard-coded carry-forward. |
| `--evidence` | Free-text substring of guideline name or CPIC level. | Drop under no-text-heuristics ruling. Return source labels and use exact proved native selectors. |
| search `-l`, `--limit` | Default 10, allowed 1–50; fetches up to five times requested rows, capped at 200; truncates after local filters. | Keep bounded native paging, remove hidden overfetch and source-total/locally-filtered-count ambiguity. Proposed default five needs measurements. |
| search `--offset` | Native offset precedes local filtering. | Keep only with a proved native stable page contract and all selectors retained in next. |
| `get pgx QUERY` | Word-shape helper chooses gene or drug; default interactions. | Replace with explicitly declared gene/drug identity selectors and pair card; old bare form retires. |
| `interactions` | Gene-drug tracked-pair rows, not drug-drug interactions. | Rename pair disclosure into search/card; retirement explains the selected explicit route. |
| `recommendations` | CPIC recommendations, with extra drug discovery and potentially multi-gene contexts. | Keep native independent page and complete lookup keys/populations; no hidden discovery fan-out. |
| `frequencies` | CPIC allele frequency records, including additive drug-to-gene requests. | Keep explicit gene page; drop automatic drug-name fan-out. Preserve source methodology and missing denominators. |
| `guidelines` | CPIC guideline summaries. | Keep source guideline/publication pages with exact ID/version distinction. |
| `annotations` | Three PharmGKB endpoints merged and deduplicated with a shared limit. | Keep separate clinical/guideline/label collection pages under `--kind`; do not silently merge totals. |
| `all` | Includes every section. | Drop automatic section fan-out; named sections remain one action away. |
| get `--full` | All sections, up to 50 rows each. | Drop bulk expansion. Explicit section pages retain all facts. |
| multiple `SECTION` arguments | Several sections allowed at offset zero. | Replace with one named section per request and exact next actions. |
| get `-l`, `--limit` | Bounds rows of each selected section, default 10, allowed 1–50. | Card/candidate paging uses `--limit`; named sections use shared `--section-limit`, with measured defaults. |
| get `--offset` | Nonzero only for one named section and without full. | Rename named-section paging to `--section-offset`; reject incompatible selectors before I/O. |
| `--json`, `-j`, section-position JSON shorthand | Shared CLI rendering selection and a legacy section-token extraction. | Global output follows current JSON-default/`--format md` contract. No PGx-specific renderer or section-token trick. |
| PGx helper family | No separate helper in legacy CLI. | Proposed `pgx diplotype` is new explicit source lookup; not a claim of legacy parity. |
| recommendation drug summary and annotation note | Legacy output fields, not commands/options. | Replace hidden fan-out summaries with source row contexts and ordinary problems; preserve any captured unique fact in its owning section. |

Shared transport/configuration options remain shared. Keys, disabled-source groups, replay/record, freshness, output format and help need no PGx-specific duplicate. Section failures keep current envelope outcomes. The old local CPIC-level rank and case-insensitive guideline filtering do not define a new clinical ranking. The comparison still owes actual contained old-binary outputs for the same questions; a legacy request plan alone does not prove the old runtime remained contained.

## Sources kept and deferred

Keep CPIC as the source candidate for pair/guideline/recommendation and source-reported diplotype/frequency records. Keep PharmGKB through the current ClinPGx API as the annotation candidate, with independent kinds and current terms. Neither is accepted as a complete public implementation until its exact captures prove fields and identities. Do not add PharmCAT allele calling, PharmVar downloads, DPWG scraping, a sequence interpreter or a patient genotype calculator. A ClinPGx annotation may link another guideline, which remains a source link without a silently added adapter. Defer a frequency page if no harvested question needs it after clinical review. Drop neither source merely because it is keyed or noncommercial; use the existing key and disable rules.

## Ian's questions

No clinical or configuration question goes to Ian. The PM must settle the proposed explicit command forms, source registry display name, current terms scopes, native fallback for star/diplotype claims and any unresolved token comparison before filing implementation tickets. A new shared Subject/proposition requires both owning PMs. A paid service, commercial reuse commitment or newly required account needs the approval route; this design makes none.
