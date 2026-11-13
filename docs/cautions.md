# Source and entity cautions

## Cache and store storage

Keep cache and store folders on a local disk, or a local Docker named volume. Sharing one cache between several processes through a Docker Desktop host bind mount or a network filesystem is unsupported and can lose cache writes. [SQLite WAL](https://www.sqlite.org/wal.html#overview) requires processes to share memory on the same host and does not support network filesystems. See [cache and store configuration](configuration.md#cache-and-store-folders) for folder selection and Docker storage.

## Article text and supplement saves

CLI saves require `--replace` for existing files. Stdio MCP allows new-file saves and refuses `--save` with `--replace` before provider requests or file changes. HTTP server calls refuse saves. CLI and stdio MCP saves belong to the caller and create no download directory or saved-file registry.

## Trial documents

The reply preserves the source URL, document metadata and license without granting reuse rights. Documents are downloaded as bounded PDF bytes and never read as text.

## FDA approval documents

`get diagnostic P170019 summary --source fda-pma` reads FDA's published SSED. `get drug NDA021588 approvals --source drugsfda --document-id 64255` reads the native submission's approval letter. The reader returns mechanically extracted embedded text, without summaries or clinical interpretation. It admits at most 25 MiB, 100 pages, and 4 MiB of expanded stream or total text. Each continuation returns at most 6144 escaped text bytes. `no-text` means no embedded text was found, including scanned documents; OCR is not run. `too-large` reports a reader bound. Native document URLs and submission ownership remain visible.

Do not put patient names, identifiers or other private information in query text; source queries may be sent to public providers.

## Variant profiles, normalization and search

In variant get and population replies, `query.resolver` names the identity resolver that answered. It is omitted when no named resolver answered. The source outcomes retain failures and identify direct identity services such as MyVariant. The executable request and its continuations retain the caller's resolver selection.

A checked member match preserves the full compound profile and its source claim context. Name matches remain candidates. Claims that the common model cannot represent retain their native facts. Rows marked `candidate` with `no_join_id` have no established transcript and consequence join to the requested variant. These residue counts do not state an allele frequency or clinical actionability. Cancer Hotspots uses ODbL with attribution and share-alike. VariantValidator and `all` require `--build GRCh37|GRCh38`. Each service keeps its own outcome, source mappings, messages and available version metadata. A provider validation warning can reject an HTTP 200 reply. `all` returns independent results and never chooses a transcript or consensus. Mapping and message pages use `--limit` and `--offset`; typed next commands keep the remaining rows reachable. Gene and protein values are escaped as source literals. The reply echoes the exact source query and assembly and keeps native candidate order and totals. Protein and raw searches send no assembly unless `--build` supplies one. Returned coordinates retain only the assembly stated by the source. Page actions retain every selector; a warm page uses the ordinary checked cache. Broad queries disclose the command's result-window bound. No candidate becomes a chosen allele.

## SEER statistics

SEER site names match the publisher's catalog labels. BioMCP selects a site for an exact label, or for words that appear whole in one label. Other matches return the matching sites and commands that preserve the selected statistic options. A name that matches no site returns no statistics and a command that lists the publisher's site names. The catalog also labels required dataset, incidence-period and survival-interval codes. Statistic rows retain separate requested and returned selectors, with labels supplied by that catalog.

The `value.unknown` membership and method problems identify facts the structured source reply does not state: dataset and registry membership, the incidence standard population and rate-adjustment method, and survival modeling, eligibility and age adjustment. A requested dataset label identifies the selection sent to SEER; it does not establish membership of the returned rows. These cautions remain alongside the numerical results and the source's connection outcome. Incidence rows retain rates per 100,000 people, and annual survival rows keep observed and modeled values separate.

## Cohort recurrence

The result counts distinct matched samples on that page. The default page of 5,000 source rows reads a common variant's whole cohort in one call. When `all_pages_read` is true, `observed_sample_count` beside `selected_sample_count` is the cohort mutation frequency. BioMCP computes no percentage, because `selected_sample_count` is the source sample-list size and that list is not always the set assayed for the gene. A larger cohort reports `more` and a next step; add the counts across pages yourself. A missing source count stays missing and is reported as a problem. Missing genome builds and uncertain insertion or deletion representations remain problems. The reply preserves study metadata and the reviewed ODbL terms, including the warning that individual studies may restrict commercial use.

## Historical COSMIC fragments

Each fragment names its MyVariant parent record, preserves the component's original spellings and reports its identity as unverified. The reply shows the snapshot version, source upload date, source warning and component terms. It does not establish that the fragment and parent describe the same allele. Source frequencies retain their published values without inferred units or denominators.

## Prediction, conservation and source assertions

Every component is on by default. See the [commercial source configuration](../README.md#configure-sources-and-keys) for source exclusions. Groups switch off matching component fields while other aggregator fields remain available. Naming `myvariant` switches off the entire service. These settings apply before requests and prevent disabled values from returning through cached replies or replay. Public API access grants no additional component rights. The retired `predictions` spelling names `predict` as its replacement. Select a native assertion ID and version to inspect its source details. Source-local absence does not establish allele absence. Selected historical records include an exact version link and compact native assessments. A conflict flag preserves disagreements between source outcome labels and identifiers; the original nested evidence remains behind the source link.


## AlphaGenome scores

The section requires the user's key, configured with `biomcp key set alphagenome` or `ALPHAGENOME_API_KEY`, and a source-resolved GRCh38 allele. Mitochondrial alleles return `unsupported` because this integration has not verified chromosome mapping and interval handling for them. Each scorer retains native numeric values, tensor coordinates and track and gene metadata. Finite values remain JSON numbers. Native nonfinite values carry `kind` as `nan`, `positive_infinity` or `negative_infinity`; Markdown shows `NaN`, `+Inf` or `-Inf`. These categories do not assert missingness. Section paging applies independently to every scorer. Scorer names, the interval and checkpoint selection describe the request. The source does not supply score units or a returned model version. Scores do not classify a variant. The publisher's [terms](https://alphagenome.google/terms) govern use; the service is labeled licensed.

## Population counts

The population join requires one source-given GRCh38 single-nucleotide allele. Missing or ambiguous join information produces a problem before gnomAD is called. Exome and genome records remain independent. The source selector does not establish a dataset release. Each request also asks for the source VA cohort release and counts. A typed population-frequency record requires that same response to supply a release for the matching total cohort and matching allele counts. Replies preserve native counts and report a problem when the release is missing or the counts disagree. A missing dataset stays absent; a reported zero stays zero. BioMCP does not compute a frequency from counts or sum overlapping ancestry and sex groups.

## Variant GWAS associations

An exact rsID needs no allele-resolution request. Association pages retain the source study, trait and effect allele with the source's effect estimates. An empty result means the catalog returned no associations for that request. It supplies no clinical classification. Named source sections use `--section-limit` and `--section-offset` independently of identity candidate paging.

## CPIC and FAERS searches

An explicit `--drug` value containing a standalone OR between names is refused regardless of letter case. Pass `--drug` once per name. Positional drug text is not read as a Boolean expression.

FAERS widens a drug name by default: it looks up the drug's other names through the drug-name resolver and searches them with the typed name in one native OR query. The reply carries one notice on FAERS that lists the names searched and each name left out with its reason. FAERS supplies the report total for that one query, never a sum of per-name counts, and the names found do not claim every report for the drug. Add `--exact-term` to search only the typed names. `--expand-drug-names` is still accepted for one release and changes nothing. Caller source/component exclusions and name/query limits still apply. A name lookup that does not finish is listed as skipped; the search then covers the typed names and stays complete. Returned pages carry `--name-snapshot` to keep their first page's names and evidence without repeating the lookup; keep the filters unchanged. Missing, expired or newly disallowed snapshots offer a restart. Reaction aggregates use the same names. MAUDE and VAERS search the text as typed: MAUDE has no drug field and VAERS takes fixed vaccine codes.

Named `recommendations` and `guidelines` sections preserve complete source rows and use `--section-limit` and `--section-offset`. An exact `--drug-id RxNorm:10324` on CYP2D6 recommendations selects the distinct native recommendation table. These commands preserve source labels and source-local empty replies. They calculate no phase, activity score, phenotype or treatment. Published website updates may precede the API tables; a retrieval date does not establish a dataset release. CPIC curated content is CC0 1.0. The reply retains the exact source expression, native order and report total. A report total is not incidence, risk or causation. Publisher dates and retrieval dates have separate roles.

## Vaccine identity

CVX vaccine identity lookup and sync are unavailable because no mapping to WONDER was proved. BioMCP does not translate vaccine identities or synchronize a CVX bundle.

## FAERS report sections

FAERS `search adverse-event --aggregate reactions` requests openFDA's exact reaction-term counts across matching reports. The reply separates these buckets from report headers and retains native terms, counts and order. A report may list several drugs and several reactions: buckets can overlap and do not identify a causal drug–reaction pair. BioMCP never sums them into a report total or derives rates, incidence, risk or comparative safety. The source provides no exposure denominator.

`--limit` bounds the returned reaction terms. `reaction_counts.limit_reached` means that the source filled that limit; it does not establish a total or prove that more terms exist. Count mode has no report offset or report total. Reaction terms require the MedDRA component and retain its terms of use. See [openFDA's count response](https://open.fda.gov/apis/drug/event/understanding-the-api-results/).

The drug index is its absolute position in the report. Missing retained reports offer an explicit restart. Drug roles belong to each original drug row, and reaction labels carry separate MedDRA terms. Report counts and identifiers do not establish causation or a marketed-product identity.

## Local bundles

HGNC and ClinGen reads can synchronize absent or stale bundles under `auto_sync_max_bytes`. Offline mode never synchronizes. EMA, GTR, WHO and cBioPortal studies read explicitly synchronized stored bundles and never synchronize on read. Missing stored bundles offer an explicit sync command; stale stored rows remain readable with a stale marker. A hosted server answers from its stored copy past the refresh date without the stale marker, and the reply states the date that copy was retrieved.

EMA's medicines file holds some `?` characters where a symbol such as `≥` was lost before publication, as in `?10 g/dl`. They cannot be told from a real question mark, so EMA indications show them as published.

## GTR search

Diagnostic reads use the stored pair and never synchronize it. Combine one relation selector with the native `--type` value. Search pages count distinct accession/version identities and preserve the relation that matched. Conflicting identities have no selected card.

Each resolved row carries `gene_count`, the distinct symbols in the test's native genes cell (1 for a single-gene test, hundreds for an exome-scale one), its native `method_categories`, and the laboratory's state, postcode and country. `url` is the test's GTR page. GTR's stored files hold no laboratory street, city, director or order link; the GTR page lists them. A page holds at most 50 rows, so a gene with a few hundred tests takes a few pages: `page.total` gives the count and the reply offers the next page.

`--name` matches literal text inside the native `lab_test_name`, ignoring letter case, across every stored version row, including tests with no gene or condition link. It counts current cards and conflicting identities before paging; a test whose only card is not current is left out. A name match has no matched relation unless a relation selector is also given. Tests whose name holds the text as whole words, with no letter or digit touching either end, come first, then tests where it sits only inside a longer word, each group in source order; `name_match` says `whole-word` or `partial-word` on each row. There is no fuzzy matching.

## GTR cards

The `genes`, `conditions` and `methods` sections page independent native rows with `--section-limit` and `--section-offset`. Relation pages distinguish native row totals from distinct identifier totals. A GTR listing states source membership and laboratory submission facts; it does not establish variant coverage, clinical availability, FDA authorization or NIH endorsement.

## ClinPGx and MAUDE sections

The observed total counts the complete received collection and does not claim a server-wide total. Reference rows do not contain full guidance. Fields absent from the min tier remain unsupported; absence does not state a negative clinical finding. ClinPGx accessions and numeric annotation IDs remain separate. ClinPGx uses the registry's noncommercial policy ruling. The `devices` and `narrative` sections retain whole source rows and use `--section-limit` and `--section-offset`. Section commands carry a retained snapshot so a walk cannot mix report versions. A missing snapshot offers an explicit restart. Native event, received and report dates remain distinct. Narrative text supplies no inferred diagnosis. `--event-type` matches the report's native `event_type` exactly (Malfunction, Injury, Death or Other); `--problem` matches the coded `product_problems` vocabulary, where malfunction is not a value. Reports with an empty event type or `No answer provided` match no `--event-type` value. A disabled source or component cannot return through cache or replay.

## WHO diagnostic cards

Each row retains its whole name, code, assay format, manufacturer, marker, regulatory-version text and prequalification year. Reads use the stored export and never synchronize it. WHO cards have no section command. WHO terms reserve commercial and substantial reproduction rights; `--disable-sources noncommercial` disables stored cards, searches and synchronization.

## PMA submissions and supplements

The `regulatory` section pages the selected row’s original statement fragments; the base-only `supplements` section returns five native headers per page. Missing supplements return source-local absence. A supplement’s decision belongs to that submission alone. Exact companion-table links remain separate from PMA approval records.

## FDA companion tables

Rows retain their whole cells and original links. Biomarker selection is unsupported for the group table. A companion row is not a treatment recommendation.

## FDA clearance records

It keeps the source decision fields and offers its exact companion-table selector. It has no sections or free-text search. PMA approval, 510(k) clearance, WHO prequalification and GTR listing remain independent source facts. A listing is not authorization. No source record establishes clinical suitability by itself.

## WHO finished products

Reads never download. Results preserve native combined INN, dosage-form and strength text, empty cells and original dates. WHO prequalification does not establish US or EU authorization. The export-specific reuse grant is unverified, so the source is noncommercial and links WHO's copyright policy.

## WHO diagnostic search

Both selectors match literal whole words in their native columns, ignore case and combine on the same source row. Search returns five product headers by default and groups repeated code rows by exact WHO Product ID. Reads never download or synchronize. A WHO listing does not establish FDA authorization or clinical suitability.

## Study metadata

Text matches literally and case-insensitively in native study IDs, names and descriptions; cancer-type codes remain case-sensitive. Reads never synchronize. Missing or stale copies offer an explicit sync command. Returned page commands retain the catalog revision and every selector. cBioPortal uses ODbL with original-study attribution unless otherwise noted; individual studies may restrict commercial use. The catalog contains metadata only. File references preserve repository blob size and native LICENSE links without downloading data files. Source counts keep their native names. No frequency, denominator, survival or comparison is calculated. Gene-count rows and patient totals remain deferred because the source lacks a gene-row filter.

## Phenotype sections

Each section returns five rows by default; `--section-limit` and `--section-offset` select a local page. Missing source fields remain unavailable, and empty source arrays remain empty collections. A synonym, cross reference or hierarchy link supplies no clinical equivalence.

## Phenotype set rankings

`search phenotype HP:0001263,HP:0001252,HP:0001250` sends the HP IDs, in order and with repeats, to Monarch's semantic similarity search and returns its one ranked list of human genes, or diseases with `--group diseases`. Each row keeps Monarch's score, its two direction scores and each queried term's best match; the reply names Monarch's metric, directionality and search mode. Monarch returns at most 50 rows and no further pages, so a gene outside them is unranked here, not absent from Monarch. Monarch's default hybrid mode scores only a Jaccard-prefiltered set, so a richly annotated gene can be left out; each hybrid reply says so and offers the same ranking with `--search-mode full`, which scores every entity and is slower. Monarch ranks without an HP ID it does not know, so BioMCP refuses that set and offers the known IDs. Monarch ranks nothing when it knows none of the IDs, and that refusal says so. A rank orders candidates; it is no diagnosis.

## GWAS thresholds

The walk stops at the first above-threshold row or the source end and then states the qualifying total. The source total remains separate. Thresholds use scientific notation and must be positive; no sort option is available.

## GWAS records

Select `associations` for a native association page or `ancestries` for a local page of the complete received ancestry collection. The commands retain source sample prose and effect labels without reconciliation or clinical interpretation.

## Protein domains

Entries keep every location group and fragment. Families, sites and superfamilies retain their original types. The native count stays separate from the selected count.

## Protein complexes and pathway members

Each header keeps its predicted flag and actual participant count. Search text and descriptions establish no membership. Non-protein members remain source facts. Disabled sources supply neither retained rows nor card actions. Members preserve native xref namespaces, values and node identities. Exact approved HGNC mappings offer gene pivots; labels and aliases never resolve an identifier. Totals count data nodes rather than distinct genes, and source failures remain visible.

## Cell-line identity

Names remain unchanged and results do not claim a total. The card preserves entry dates, source disease/species references and complete stock-qualified warnings. Publisher release metadata and retrieval time remain separate. Cellosaurus uses CC BY 4.0 with attribution; source omissions do not establish a negative biological finding.

## ChEMBL cell metadata

The reply preserves native names, tissue, organism, taxon and optional EFO/CLO identifiers. Metadata never requests assays. The total counts assays only and supplies no activity, experiment or drug sensitivity estimate. ChEMBL uses CC BY-SA 3.0. Its independently cached release metadata and labelled retrieval time remain separate.

## Cell-line sections

Each source keeps its own outcome and release dates. Composite reads never request assay counts. PharmacoDB drug response remains deferred until a data reuse grant is published, and the HPA expression route remains deferred until an exact cell identity join exists.

## GenCC submissions

`get gene ODC1 gencc` reads the local GenCC bundle by HGNC identity. Run `sync gencc` to install or refresh it. Reads keep stale rows and mark the source stale. Health checks report publisher reachability and local bundle readiness separately.

Each record keeps GenCC's submission ID, version, submitter, disease, inheritance, classification and dates in file order. BioMCP neither merges submissions nor ranks classifications. An empty section means the stored export contains no matching rows.

GenCC documents its new-format CSV export, weekly publication cadence, conditional requests and download policy on its [download page](https://thegencc.org/download). Its [terms](https://thegencc.org/terms) identify CC0 1.0 and request attribution to GenCC and its contributors. The [GenCC publication](https://pmc.ncbi.nlm.nih.gov/articles/PMC7613247/) describes the submission fields.

ClinicalTrials.gov records keep valid fields when a status, date, phase or other nonidentity value cannot map. The reply reports a field problem and omits the rejected value. An invalid NCT ID excludes its study from a search page or fails that exact record request. Overall status values follow the [publisher enum metadata](https://clinicaltrials.gov/api/v2/studies/enums), including `TEMPORARILY_NOT_AVAILABLE`.

The trial page, trial card, trial header and trial document result schemas change major versions because rejected source fields can now be null. Existing valid source values keep their wire representation.

A rejected registry age value stays unknown. It does not become the source’s missing-age state. Nonblank native text keeps its original bytes; blank native text is omitted with a field problem.


## NCI organizations

`search organization --source nci` sends NCI's documented name, city, state or province, country, postal code, organization family and coordinate radius selectors. `--name` uses the publisher's partial match. Names are not unique, and NCI publishes no organization ID. Each search row includes the exact source name, location, trial count and optional published address, coordinates, family and public contacts. There is no organization get command.

The count covers trials of every status, including closed trials. It does not count recruiting trials alone. One bounded unsized request supplies the complete organization observation. Returned page commands retain that snapshot and page it locally. A too-broad refusal names the filters that can narrow the request. Source bucket failures remain failures and never establish zero matches.

NCI trial facility searches use `sites.org_name` for an exact case-insensitive organization name. Each organization row offers this search as its next command. Each trial header shows the first locally equal site in publisher order and the number of equal sites. Site name, city, state and recruitment status retain source values. A listed site is not necessarily enrolling. If no returned site equals the requested name, the trial keeps its position and reports a `sites.org_name` problem. NCI facility searches refuse radius filters because NCI does not document that these filters bind one site.

## NCI biomarker concepts

`search biomarker --source nci --name PD-L1` searches NCI's native concept names and synonyms. `get biomarker C96022` searches by an exact NCIt code. Both require the user's NCI key. The public tier returns codes, name, type, trial count, synonyms, parent and ancestor codes, and native `eligibility_criterion` values. A code can return several native rows; BioMCP then returns candidates without choosing one. Counts cover NCI trials across statuses.

These commands return terminology. `search diagnostic --biomarker` filters diagnostic tests, and `search trial --variant` searches literal variant text. Native inclusion and exclusion values remain source assertions and do not establish patient eligibility. Search pages retain one bounded observation. A broad request can return narrowing guidance; a source failure never establishes zero concepts.

## Drug names across sources

After an empty name lookup, BioMCP may consult existing MyChem, openFDA label or ChEMBL records for source-stated names. ChEMBL first tries its native exact synonym filter when no selector was supplied. Cross-source retries require one exact name-resolution candidate and preserve the selected source's filters, IDs and native counts. Ambiguous names offer a source search instead of selecting an alias. The reply identifies any source supplying names, shows the actual successful search spelling and preserves it in continuation commands. A matching record in another source does not establish presence in the selected source. Source exclusions and the command budget also govern alias lookups; failed or disabled reads remain visible.

## Medical device recalls

`search diagnostic --source fda-device-recall` covers every medical device. It searches the native recalling firm phrase, exact product code and inclusive initiation dates. The [FDA field mapping](https://github.com/FDA/openfda/blob/master/schemas/device_recall_mapping.json) defines `product_code` as a keyword field and `product_res_number.exact` as the exact recall identity. Totals count product recall records, including separate records under one event number. A count makes no claim about harm.

The card preserves the recall's reason, root cause, product description and distribution pattern. Its submission numbers come only from top-level `k_numbers` and `pma_numbers` in the [device recall fields](https://open.fda.gov/apis/device/recall/searchable-fields/). Harmonized `openfda` identifiers describe the product code and never populate the card. One exact [device enforcement](https://open.fda.gov/apis/device/enforcement/) recall-number lookup supplies classification when present; absence leaves classification absent. A failed lookup preserves the recall card and discloses a partial result.

Search rows carry the same classification. One exact enforcement request per page reads the classes of that page's recall numbers; a row without an enforcement record shows no classification and the source notice counts those rows. BioMCP never infers a class from the recall status. `--classification "Class I"` selects [device enforcement](https://open.fda.gov/apis/device/enforcement/) records first, because device recall records hold no class, then reads their recall records in one exact request. Under that filter a source notice says the total counts enforcement records, the firm phrase and initiation dates match the enforcement fields, and `--product-code` is refused because enforcement records carry no product code. A failed join keeps the rows and discloses a partial result.

The MAUDE next step selects the recall's product code across manufacturers. Recall records and adverse-event reports remain separate source facts. Neither establishes product-specific harm or clinical suitability.

openFDA label search first tries exact brand and generic fields, then word matches, then indication wording, then other label text. The first field query with native results supplies every page and its total. A source match need not be the complete product name; `matched_name` records complete name matches separately. Display names use the first brand or generic value, falling back to the native SPL set ID when neither is available. Product-data paragraphs remain in the original label sections.

`search drug --indication "pancreatic cancer" --source openfda-label` searches only the label's native `indications_and_usage` field. It preserves the phrase without disease synonym expansion or clinical inference. An optional name query further narrows that same search. Results remain SPL label identities in source order, with the source total; follow `get drug` and the label section to read the original wording. Drugs@FDA does not support this filter.

`search drug --source drugsfda --applicant Alnylam` returns the applications whose native `sponsor_name` starts with that text, with the native total. Drugs@FDA stores sponsor names in upper case and matches them only as complete values, so BioMCP upper-cases the text, escapes it and appends the source's own prefix wildcard; a complete name such as `ALNYLAM PHARMS INC` still matches. A source notice states the prefix searched. A prefix can match several unrelated sponsors, and BioMCP adds no company-family or subsidiary inference. An optional name narrows the same search.

`get drug BLA125514 approvals --source drugsfda --decision-from 2024-01-01` keeps only the submissions whose status date falls in the window, inclusive; undated submissions are left out, the page total counts only the window, and a notice states the window and how many of the application's submissions fall in it. The dates are full days, YYYY-MM-DD, and the window does not combine with `--document-id`. Drugs@FDA approvals take `--section-limit` up to 200, so a whole application history fits in one page; other sections and sources stay at 50. A page asked for with more than 50 rows reads no supplement letters, so its rows carry no `supplement_purpose`; ask for 50 rows or fewer, or one `--document-id`, to read them.

Each Drugs@FDA search row carries `latest_submission`: the application's submission with the latest status date, with its type, number, status and class. It also carries `original_approval`, the date and number of its earliest approved original (ORIG) submission, when the source lists one. `--sort newest` asks the source to order applications by that date, newest first, with ties broken by application number so pages stay stable; applications with no dated submission come last. Without `--sort` the rows keep the source's own order. The total is the source's total either way. The date is the submission's status date, so a recent labeling supplement can put an old application first; read the row's type and class before calling it a new approval.

Drugs@FDA, FDA drug recall and EMA searches also widen a drug name by default, and `--exact-term` searches only the typed name. Drugs@FDA takes the exact or word step the typed name reaches and searches all names at that step in one query; recalls search all names in one query; both report the source's own total for that query. EMA searches each name in its local medicines list and counts each medicine once. Later pages keep the first page's names. MAUDE and VAERS adverse-event searches stay literal.

Label search widens a drug name by default: `biomcp search drug apixaban --source openfda-label` searches the typed name and its bounded source-stated other names in one native brand/generic group, exact names in any letter case first and then word matches, as the typed name alone would be searched. The typed name picks the discovery step (names, then indication text, then label text), so the widened search never finds fewer label sets than the typed name alone; later pages keep the first page's names and step. Add `--exact-term` to search only the typed name; `--expand-drug-names` is still accepted for one release and changes nothing. Literal indication wording stays unchanged. The whole-query bound is BioMCP's own bound; one notice on the label source lists the names searched and each name left out with its reason. Counts are the native label total, never a sum across names; revision numbers do not rank different SPL sets. Follow returned page commands unchanged; restart the first page if its retained name selection is unavailable or disallowed.
