# Source and entity cautions

## Cache and store storage

Keep cache and store folders on a local disk, or a local Docker named volume. Sharing one cache between several processes through a Docker Desktop host bind mount or a network filesystem is unsupported and can lose cache writes. [SQLite WAL](https://www.sqlite.org/wal.html#overview) requires processes to share memory on the same host and does not support network filesystems. See [cache and store configuration](configuration.md#cache-and-store-folders) for folder selection and Docker storage.

## Article text and supplement saves

CLI saves require `--replace` for existing files. Stdio MCP allows new-file saves and refuses `--save` with `--replace` before provider requests or file changes. HTTP server calls refuse saves. CLI and stdio MCP saves belong to the caller and create no download directory or saved-file registry.

## Article open-access flags

A `search article` row's `open_access` comes from the source named in `open_access_source`. By default it is `pubmed`: BioMCP sends one NCBI request per page with the filter that, in [PubMed Central's words](https://pmc.ncbi.nlm.nih.gov/tools/openftlist/), finds "all Open Access Subset articles" in PubMed, and the subset "includes journal articles and preprints that are made available under Creative Commons or similar licenses that allow more liberal redistribution and reuse than a traditionally copyrighted work." With `--license-lookup`, or on a row Europe PMC itself returned, it is `europepmc`: Europe PMC's own `isOpenAccess` flag, which follows Europe PMC's definition, not PubMed Central's. On 440 papers from 44 searches on 2026-11-19, the two disagreed on 92: NCBI listed 90 as open access that Europe PMC did not, mostly recent deposits Europe PMC had not yet indexed, and Europe PMC flagged 2 that NCBI did not. BioMCP does not decide which is right. Only the Europe PMC lookup reads licences, so a row without one says `license_not_checked`; "not checked" never means "no licence". License terms vary: read the article's own license statement before reuse.

## Partial replies and drug-name lookups

An answered reply is `partial` when a requested section is `skipped` or `unavailable`, including when its source is disabled, a key is missing, or a join ID could not be established. A `not-applicable` section or an empty answer from an available source can coexist with `complete` status. A failed, stale or syncing source also makes an answered reply `partial`. The `incomplete` list gives each gap's source, reason or problem code, and a pointer to its detail in the reply. A get that finds nothing while a lookup that could have resolved the name failed says `failed`, not `absent`. When an article search's open-access or licence check fails, the search's own source row stays as the search left it and carries a notice naming the check, and the reply is `partial` with an entry at the first unchecked row.

If optional drug-name expansion cannot finish, `search article` and `search trial` can still return a complete search for the typed term. The source notices say that records under other drug names may be missing. `--exact-term` skips the expansion.

## Trial documents

The reply preserves the source URL, document metadata and license without granting reuse rights. `trial document` reads embedded text from up to 100 physical pages per call and can find a phrase regardless of case. Follow `next` to continue, or use `--page-start N` to start at a specific page. Part and match offsets apply within those pages. If text extraction is refused, use the CLI save command when offered or open the source URL.

## FDA safety signals and withdrawn approvals

FDA's quarterly `fda-safety-signals` search uses a shipped snapshot of current and available archived reports for first and offline reads. Each result page carries FDA's wording on causality. It reports the snapshot date when used and the count of unread quarters, including [archived reports](https://www.fda.gov/drugs/fda-adverse-event-monitoring-system-aems/archived-quarterly-reports-new-safety-information-or-potential-signals-serious-risks-identified-fda) unavailable for collection. Use `biomcp search drug ocrelizumab --source fda-safety-signals`.

The `fda-withdrawn-approvals` source returns one row per withdrawn cancer indication with FDA's wording and dates. A query can use drug or indication text, or a withdrawal date in FDA's printed form or ISO year, month or day format. The shipped table also works offline. Use `biomcp search drug tazemetostat --source fda-withdrawn-approvals`.

## FDA approval documents

`get diagnostic P170019 summary --source fda-pma` reads FDA's published SSED. `get drug NDA021588 approvals --source drugsfda --document-id 64255` reads the native submission's approval letter. The reader returns mechanically extracted embedded text, without summaries or clinical interpretation. It admits at most 25 MiB, 100 pages, and 4 MiB of expanded stream or total text. Each continuation returns at most 6144 escaped text bytes. It reads text that a page draws through embedded form objects. `no-text` means no embedded text was found, including scanned documents; OCR is not run. When a font's character map has a header the PDF library refuses, the reader reads the map's own code ranges and mappings again. Text shown in a composite font with no character map, or one over the reader's bounds, gives glyph numbers, not characters; when that text reads as gibberish it is cut out. `garbled` means gibberish was most of the document, and no text is returned. `partial` means text was returned, but gibberish was cut out or a part of a page could not be read, so some text may be missing, including characters a font's character map does not map. `too-large` reports a reader bound, including a document that holds over a million objects or whose cross-reference data is unusually large. `damaged` means the file is broken or irregular, for example cut short or with objects that overlap, so no text was read; BioMCP does not repair damaged PDFs, so fetch the document another way. An encrypted PDF that keeps its objects in object streams is also refused as `damaged`, because its object streams cannot be measured before they are decrypted. Each PDF is read in a separate process that is stopped at the call's time limit and, on Linux, at a memory cap (`pdf_memory_limit_mib`); `refused` means reading was stopped there, so the document could not be read safely and no text was returned. On macOS the wall-clock and CPU-time limits apply. A hosted server that is already reading as many PDFs as it allows tells the caller to retry. A document whose text state is anything but `text` makes the reply `partial`, and `incomplete` lists its text. Native document URLs and submission ownership remain visible.

Do not put patient names, identifiers or other private information in query text; source queries may be sent to public providers.

## Label section history

`get drug NDA208065 label-history --source drugsfda --label-section warnings_and_cautions --find vasculitis` says in which archived label version text first appears in one section of a drug's US prescribing information. It reads the label PDFs Drugs@FDA lists with submissions, fetched only from FDA's own site, with the same PDF reader and limits as approval letters, except that a label may have up to 400 pages. The same PDF listed under http and https, or with a `#page=` suffix, counts once. If several submissions list that PDF, the history uses the earliest dated approval when available. A label listed without a link remains unread. Matching is literal text, ignoring letter case, line breaks and hyphenation; `--find` may repeat, and any of its values counts.

The default search starts from the oldest and newest versions. When the earliest readable label lacks the text and the newest has it,
BioMCP halves the range, probes toward the matching end when it cannot read the midpoint, and uses any remaining reads to look for earlier matches. Halving assumes that the text stays once added.
The reply's `method` states the assumption when it guided the search. When both ends lack the text, the search samples versions between
them and halves toward an earlier match if it finds one. When the earliest has the text and the newest does not, it reads in order.
`--every-version` reads in order, and `--section-offset` continues it from a later version. Each page draws its answer from its own reads.
A call opens at most 10 PDFs and stops with less than a second left. `not_opened` counts versions it did not open.
A call that stopped is `partial`, and its `next` step offers a continuation when more versions remain.

Sections are found from the label's own contents list ("FULL PRESCRIBING INFORMATION: CONTENTS"): a heading in the body counts
only when its number and title match a listed entry, in the list's order, so table rows that look like subsection numbers are
never headings, and a section runs to the next listed heading only when no numbered heading of another section stands between
them. A numbered section missing from the contents list is unread unless the neighboring body sections verify the omission.
A boxed warning can be marked absent when the label's opening
is bounded and shows no box. When the list is missing, the parser needs a marker that identifies the body, then it reads from
the body's numbered headings: a section starts at its one heading and must end at the heading of
the section that always follows it (5 WARNINGS AND PRECAUTIONS ends at 6 ADVERSE REACTIONS). Labels in the format used before 2006
are read the same way from their unnumbered titles: WARNINGS must be followed by PRECAUTIONS and then ADVERSE REACTIONS, and a
title after "see", inside parentheses or before a comma is a cross-reference, not a heading. A missed or repeated heading makes
the section `section-not-bounded` (or `older-format` when an older label has no heading for it), never absent and never run into
the next section. When a boxed warning's own text is not in the PDF, as when its page is an image, the contents list's title for
the box is searched: a match there counts, a miss leaves the box unread. A PDF whose only text is FDA's stamp is `no-text`. A
document is `not-prescribing-information` only when it has no prescribing-information structure and its first lines name another
kind of document, such as Instructions for Use or a Medication Guide. A version that could not be read, or whose section holds
private-use characters (symbols or digits the PDF's fonts do not name) and no match, is unread.

The earliest version marked `match` supplies the date. If any older version was unreadable or unopened, the reply says "on or
before" and is `partial`. A matching version without a dated approval leaves the first appearance uncertain. When none of the versions read matches and some could not be read or were not opened, the reply says
BioMCP could not tell. Sentences are given as the PDF's text gives them, at most two and at most 300 characters each. BioMCP
reports text and dates only; it does not judge what a label change means.

## Variant profiles, normalization and search

In variant get and population replies, `query.resolver` names the identity resolver that answered. It is omitted when no named resolver answered. The source outcomes retain failures and identify direct identity services such as MyVariant. The executable request and its continuations retain the caller's resolver selection.

A checked member match preserves the full compound profile and its source claim context. Name matches remain candidates. Claims that the common model cannot represent retain their native facts. Rows marked `candidate` with `no_join_id` have no established transcript and consequence join to the requested variant. These residue counts do not state an allele frequency or clinical actionability. Cancer Hotspots uses ODbL with attribution and share-alike. VariantValidator and `all` require `--build GRCh37|GRCh38`. Each service keeps its own outcome, source mappings, messages and available version metadata. A provider validation warning can reject an HTTP 200 reply. `all` returns independent results and never chooses a transcript or consensus. Mapping and message pages use `--limit` and `--offset`; typed next commands keep the remaining rows reachable. Gene and protein values are escaped as source literals. The reply echoes the exact source query and assembly and keeps native candidate order and totals. Protein and raw searches send no assembly unless `--build` supplies one. Returned coordinates retain only the assembly stated by the source. Page actions retain every selector; a warm page uses the ordinary checked cache. Broad queries disclose the command's result-window bound. No candidate becomes a chosen allele.

## SEER statistics

SEER site names match the publisher's catalog labels. BioMCP selects a site for an exact label, or for words that appear whole in one label. Other matches return the matching sites and commands that preserve the selected statistic options. A name that matches no site returns no statistics and a command that lists the publisher's site names. The catalog also labels required dataset, incidence-period and survival-interval codes. Statistic rows retain separate requested and returned selectors, with labels supplied by that catalog.

The `value.unknown` membership and method problems identify facts the structured source reply does not state: dataset and registry membership, the incidence standard population and rate-adjustment method, and survival modeling, eligibility and age adjustment. A requested dataset label identifies the selection sent to SEER; it does not establish membership of the returned rows. These cautions remain alongside the numerical results and the source's connection outcome. Incidence rows retain rates per 100,000 people, and annual survival rows keep observed and modeled values separate.

## Disease cross-reference links

`get disease MONDO:0016512 xrefs --source ols --section-limit 50` shows publisher-supplied HTTP(S) links in `target_url`. The row's `attribution` identifies the MONDO record that states the cross-reference. BioMCP copies the target without opening it. The NORD link for `NORD:1318` leads to the Kabuki syndrome page, which lists All Things Kabuki. The old GARD query link on the same record does not establish a link to GARD's Kabuki syndrome page. [OLS MONDO record](https://www.ebi.ac.uk/ols4/api/ontologies/mondo/terms?obo_id=MONDO:0016512) · [NORD Kabuki syndrome page](https://rarediseases.org/rare-diseases/kabuki-syndrome/)

## Disease IDs read through MONDO

`get disease ORPHA:115` and `get disease OMIM:121050` read the MONDO record whose cross-references state that code, and `alias` names the cross-reference row used. Gene and phenotype sections read Monarch for that MONDO record, and a MyDisease notice names the mapping. The mapping is MONDO's cross-reference, not Orphanet's own mapping relation, so the MONDO record may be broader than the code asked for, such as a group for a subtype code. A MONDO record that MONDO marks obsolete is still returned, with a notice naming any replacement MONDO gives.

## Orphanet prevalence

`get disease ORPHA:699029` uses the Orphadata disorder catalog when MyDisease finds no MONDO mapping. The card carries Orphadata's name, disorder type and release. If MyDisease returns an unusable record, the Orphadata card is partial and lists the MyDisease failure. Run `sync orphadata --xrefs` to install or refresh the catalog when the first read reaches the automatic download limit. Read the prevalence section directly with `get disease ORPHA:699029 prevalence`.

`get disease MONDO:0016512 prevalence --source orphadata` reads Orphanet's prevalence estimates, average age of onset and type of inheritance from a local copy of two Orphadata files, `en_product9_prev.xml` and `en_product9_ages.xml`. Run `sync orphadata` to install or refresh them; a live first read fetches a missing copy under `auto_sync_max_bytes`, reads never refresh a stored copy, and offline and replay never fetch. A MONDO or DOID concept reaches Orphanet only through the Orphanet cross-references MONDO states for it, which the reply lists under `links`; BioMCP adds no name or ontology matching. A MONDO Orphanet link can name a related or broader Orphanet entity, such as a clinical subtype, so each item carries Orphanet's own label and disorder type. A concept without an Orphanet link returns no rows and marks `orphadata` skipped with `no-join-id`. `Orphanet:2322` reads one disorder directly.

Each estimate keeps Orphanet's type, qualification, class, mean value, geography, source text and validation status as published, with PubMed IDs taken from the source text. The mean value is per 100,000 people for a value-and-class estimate and a count for cases or families; a class-only estimate has no value. BioMCP neither pools nor ranks estimates. The reply records the file release date and version.

Orphadata Science data is licensed under [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/); every file states the licence, and sync refuses a file that states another. When you reuse it, cite it as Orphadata's [FAQ](https://sciences.orphadata.com/faq/) asks: "Orphadata Science: Free access data from Orphanet. © INSERM 1999. Available on https://sciences.orphadata.com/ . Data version [date of data version]", using the release date in the reply. Orphadata's [legal notice](https://www.orphadata.com/legal-notice/) also asks that an AI tool or service offered to others state that it uses Orphanet/Orphadata data, that its results may not truly reflect that data, and that Orphanet is not responsible for their quality.

## Cohort recurrence

The result counts distinct matched samples on that page. The default page of 5,000 source rows reads a common variant's whole cohort in one call. When `all_pages_read` is true, `observed_sample_count` beside `selected_sample_count` is the cohort mutation frequency. BioMCP computes no percentage, because `selected_sample_count` is the source sample-list size and that list is not always the set assayed for the gene. A larger cohort reports `more` and a next step; add the counts across pages yourself. A missing source count stays missing and is reported as a problem. Missing genome builds and uncertain insertion or deletion representations remain problems. The reply preserves study metadata and the reviewed ODbL terms, including the warning that individual studies may restrict commercial use.

## Historical COSMIC fragments

For a GRCh38 allele, `get variant` reads COSMIC and CADD predictions from MyVariant's GRCh37 index using the same allele's source-qualified GRCh37 representation. A missing representation gives `no-join-id`, and a missing annotation gives `not-found`. Prediction and conservation read dbNSFP from the allele's own index. When prediction also requests CADD, it keeps the dbNSFP answer and gives the reason for missing CADD in a source notice.

Each fragment names its MyVariant parent record, preserves the component's original spellings and reports its identity as unverified. The reply shows the snapshot version, source upload date, source warning and component terms. It does not establish that the fragment and parent describe the same allele. Source frequencies retain their published values without inferred units or denominators.

## Prediction, conservation and source assertions

Every component is on by default. See the [commercial source configuration](../README.md#configure-sources-and-keys) for source exclusions. Groups switch off matching component fields while other aggregator fields remain available. Naming `myvariant` switches off the entire service. These settings apply before requests and prevent disabled values from returning through cached replies or replay. Public API access grants no additional component rights. The retired `predictions` spelling names `predict` as its replacement. ClinVar pages list the variant record (VCV), then its per-condition records (RCV), then each submitter's record (SCV); `--kind scv` on `variant assertions`, or `--clinvar-kind scv` on the ClinVar section, keeps one kind and counts only it. Select a native assertion ID and version to inspect its source details. Source-local absence does not establish allele absence. Selected historical records include an exact version link and compact native assessments. A conflict flag preserves disagreements between source outcome labels and identifiers; the original nested evidence remains behind the source link.


## AlphaGenome scores

The section requires the user's key, configured with `biomcp key set alphagenome` or `ALPHAGENOME_API_KEY`, and a source-resolved GRCh38 allele. Mitochondrial alleles return `unsupported` because this integration has not verified chromosome mapping and interval handling for them. Each scorer retains native numeric values, tensor coordinates and track and gene metadata. Finite values remain JSON numbers. Native nonfinite values carry `kind` as `nan`, `positive_infinity` or `negative_infinity`; Markdown shows `NaN`, `+Inf` or `-Inf`. These categories do not assert missingness. Section paging applies independently to every scorer. Scorer names, the interval and checkpoint selection describe the request. The source does not supply score units or a returned model version. Scores do not classify a variant. The publisher's [terms](https://alphagenome.google/terms) govern use; the service is labeled licensed.

## Population counts

When gnomAD reports that it found no variant record, the population section has `state: empty` and a `record` naming the queried dataset selector (`gnomad_r4`) and exact GRCh38 allele key. The JSON source notice names the lookup and says it has no measured allele frequency. A missing variant identity or failed lookup produces an unavailable section with a reason. The reply’s `query.dataset` identifies the dataset selector and short-variant release as `gnomad_r4/4.1.0` (GRCh38). Mitochondrial, structural, copy-number and tandem-repeat variants are outside this lookup’s scope. Identifiable variants in these classes yield `unavailable` with reason `out-of-scope`.

The population join requires one exact GRCh38 short-variant allele key. Missing or ambiguous join information produces a problem before gnomAD is called. Exome and genome records remain independent. The lookup release does not establish the release of any individual source count. Each request also asks for the source VA cohort release and counts. A typed population-frequency record requires that same response to supply a release for the matching total cohort and matching allele counts. Replies preserve native counts and report a problem when the release is missing or the counts disagree. A missing dataset stays absent; a reported zero stays zero. BioMCP does not compute a frequency from counts or sum overlapping ancestry and sex groups.

## Variant GWAS associations

An exact rsID needs no allele-resolution request. Association pages retain the source study, trait and effect allele with the source's effect estimates. An empty result means the catalog returned no associations for that request. It supplies no clinical classification. Named source sections use `--section-limit` and `--section-offset` independently of identity candidate paging.

## CPIC and FAERS searches

An explicit `--drug` value containing a standalone OR between names is refused regardless of letter case. Pass `--drug` once per name. Positional drug text is not read as a Boolean expression.

FAERS widens a drug name by default: it looks up the drug's other names through the drug-name resolver and searches them with the typed name in one native OR query. The reply carries one notice on FAERS that lists the names searched and each name left out with its reason. FAERS supplies the report total for that one query, never a sum of per-name counts, and the names found do not claim every report for the drug. Add `--exact-term` to search only the typed names. `--expand-drug-names` is still accepted for one release and changes nothing. Caller source/component exclusions and name/query limits still apply. A name lookup that does not finish is listed as skipped; the search then covers the typed names and stays complete. Returned pages carry `--name-snapshot` to keep their first page's names and evidence without repeating the lookup; keep the filters unchanged. Missing, expired or newly disallowed snapshots offer a restart. Reaction aggregates use the same names. MAUDE and VAERS search the text as typed: MAUDE has no drug field and VAERS takes fixed vaccine codes.

Named `recommendations` and `guidelines` sections preserve complete source rows and use `--section-limit` and `--section-offset`. An exact `--drug-id RxNorm:10324` on CYP2D6 recommendations selects the distinct native recommendation table; its first page lists every phenotype CPIC holds for that gene and drug with its row count, and `--phenotype` keeps only rows with that exact CPIC phenotype text, such as `Poor Metabolizer`. These commands preserve source labels and source-local empty replies. They calculate no phase, activity score, phenotype or treatment. Published website updates may precede the API tables; a retrieval date does not establish a dataset release. CPIC curated content is CC0 1.0. The reply retains the exact source expression, native order and report total. A report total is not incidence, risk or causation. Publisher dates and retrieval dates have separate roles.

## Vaccine identity

CVX vaccine identity lookup and sync are unavailable because no mapping to WONDER was proved. BioMCP does not translate vaccine identities or synchronize a CVX bundle.

## VAERS symptom order

CDC WONDER returns a VAERS table in its own order, which for symptoms is alphabetical, and its request form offers no sort. The COVID-19 symptom table has about 13,000 rows. `search adverse-event COVID19 --source vaers --sort reports` orders the whole table BioMCP already received by WONDER's own Events Reported count, largest first, before it pages; equal counts and rows without a count keep the source's order. Later pages and the other tables keep the order, and following the sort step reuses the table already held, with no new WONDER request. Each row's count and percent, the footer and the report total are WONDER's. A report may list several symptoms, so the percents do not add to 100.

## VAERS rate limit

CDC WONDER refuses a table request that comes less than 15 seconds after the last one it accepted from the same address, and its refusal names no wait. BioMCP then waits 15 seconds and asks once more, within the VAERS command's 60-second limit. If WONDER refuses again, the source fails as `rate-limited` and its notice gives `retry_after_seconds: 15`.

## FAERS drug roles

FAERS drug searches default to `--drug-role any`. The count includes reports listing the drug in any role, including concomitant, and the reply states the role in `query.drug_role`.

`--drug-role suspect` counts a report when a searched name matches the whole generic, brand or medicinal-product name on a drug entry marked suspect, ignoring letter case. BioMCP checks these entries because openFDA indexes names and roles across the whole report. If the candidate reports exceed the maximum search page size, BioMCP asks you to narrow the receipt dates before giving a total. The suspect filter applies to report searches. Aggregations use the any-role count.

## FAERS report sections

FAERS `search adverse-event --aggregate reactions` requests openFDA's exact reaction-term counts across matching reports. The reply separates these buckets from report headers and retains native terms, counts and order. A report may list several drugs and several reactions: buckets can overlap and do not identify a causal drug–reaction pair. BioMCP never sums them into a report total or derives rates, incidence, risk or comparative safety. The source provides no exposure denominator.

`--limit` bounds the returned reaction terms. `reaction_counts.limit_reached` means that the source filled that limit; it does not establish a total or prove that more terms exist. Count mode has no report offset or report total. Reaction terms require the MedDRA component and retain its terms of use. See [openFDA's count response](https://open.fda.gov/apis/drug/event/understanding-the-api-results/).

FAERS or MAUDE `search adverse-event --aggregate received-year` (or `received-month`) with `--received-from` and `--received-to` counts matching reports by FDA receipt date, for example `search adverse-event pembrolizumab --reaction myocarditis --aggregate received-year --received-from 2021 --received-to 2025`. It uses the same widened drug names and filters as the search: BioMCP asks openFDA once for its daily receipt-date counts over the whole range and sums them, so each year equals the report total of the same search dated to that year. `received_counts` lists every year or month of the range up to the source's last update, oldest first, including zeros. A period that the range only partly covers, or that runs past the source's last update, is marked `partial`. Counts are reports, not people or rates; a rising count can reflect reporting rather than risk.

The drug index is its absolute position in the report. Missing retained reports offer an explicit restart. Drug roles belong to each original drug row, and reaction labels carry separate MedDRA terms. Report counts and identifiers do not establish causation or a marketed-product identity.

A FAERS card, drug row or reaction row keeps every source code and puts the word from [openFDA's published field list](https://open.fda.gov/fields/drugevent.yaml) beside it: sex, age unit (800 to 805), reporter qualification, drug role, action taken and reaction outcome. openFDA's role list holds only 1 to 3, so roles `4` and `5` count as off-list; ICH E2B(R3) defines role 4 as "drug not administered". A present code that is not on that list has no word and is named in `unmapped_codes` with its code; a field the source left out is never listed there. An off-list age unit also drops the age, because a number without its unit cannot be read, and the reply carries a problem for it. openFDA publishes no code list for the six seriousness flags: it documents each as `1` when the event had that result "and absent otherwise". The data also carries `2`, which openFDA does not document. A `1` carries openFDA's own wording, such as "The adverse event resulted in death"; a `2` carries a label that says only that the report did not record that result, such as "The report did not record that the adverse event resulted in death". The report-level `serious` flag carries `serious_label` beside it in the header of every FAERS search row, card and section: `serious` for `1` and `not serious` for `2`. openFDA defines `1` as "The adverse event resulted in death, a life threatening condition, hospitalization, disability, congenital anomaly, or other serious condition" and `2` as "The adverse event did not result in any of the above"; that definition sits once in the field's schema description, not on every row. MAUDE rows carry neither field. Any other seriousness code has no label, and the card names it in `unmapped_codes`. openFDA's public FAERS data carries no case narrative: `patient.summary.narrativeincludeclinical` holds only the case event date, which the card shows unchanged as `case_event_date`.

## ClinGen clinical actionability

`get gene FGF23 clingen` reads the adult and pediatric actionability summaries alongside validity and dosage. Each context reports its read status and the source rows for the resolved gene. A successful read with `status.count` of zero means the published summary has no matching rows. An unscored curation still links to its report and gives the early rule-out status. Scores, evidence grades, and retraction labels come from ClinGen. Actionability is included for single-gene requests. If a first read runs out of time, repeating the command reuses any bundles it already downloaded. [ClinGen clinical actionability reports](https://search.clinicalgenome.org/kb/actionability/report-index)

## Local bundles

HGNC and ClinGen reads can synchronize absent or stale bundles under `auto_sync_max_bytes`. Other stored sources, such as EMA, WHO, FDA companion diagnostics, Orphadata and cBioPortal studies, fetch a missing bundle on a live first read under the same limit and never refresh a stored one on read. Offline mode and replay never download. A missing bundle over the limit, such as GTR, offers an explicit sync command; stale stored rows remain readable with a stale marker. A hosted server answers from its stored copy past the refresh date without the stale marker, and the reply states the date that copy was retrieved.

EMA's medicines file holds some `?` characters where a symbol such as `≥` was lost before publication, as in `?10 g/dl`. They cannot be told from a real question mark, so EMA indications show them as published.

## GTR search

Diagnostic reads use the stored pair and never refresh it. Combine one relation selector with the native `--type` value. Search pages count distinct accession/version identities and preserve the relation that matched. Conflicting identities have no selected card.

Each resolved row carries `gene_count`, the distinct symbols in the test's native genes cell (1 for a single-gene test, hundreds for an exome-scale one), its native `method_categories`, and the laboratory's state, postcode and country. `url` is the test's GTR page. Search rows do not carry the laboratory's street, city, directors or order link; the test's card does. A page holds at most 50 rows, so a gene with a few hundred tests takes a few pages: `page.total` gives the count and the reply offers the next page.

`--name` matches literal text inside the native `lab_test_name`, ignoring letter case, across every stored version row, including tests with no gene or condition link. It counts current cards and conflicting identities before paging; a test whose only card is not current is left out. A name match has no matched relation unless a relation selector is also given. Tests whose name holds the text as whole words, with no letter or digit touching either end, come first, then tests where it sits only inside a longer word, each group in source order; `name_match` says `whole-word` or `partial-word` on each row. There is no fuzzy matching.

## GTR cards

A card carries, from GTR's XML release, the laboratory's mailing address (`laboratory_address`), its directors by GTR's display name (`laboratory_directors`) and the laboratory's how-to-order and test page links (`order_url`, `test_url`) as published; each is absent when GTR lists none, and the links are the laboratory's own pages, not checked by BioMCP.

The `genes`, `conditions` and `methods` sections page independent native rows with `--section-limit` and `--section-offset`. Relation pages distinguish native row totals from distinct identifier totals. A GTR listing states source membership and laboratory submission facts; it does not establish variant coverage, clinical availability, FDA authorization or NIH endorsement.

## ClinPGx and MAUDE sections

The observed total counts the complete received collection and does not claim a server-wide total. Reference rows do not contain full guidance. Fields absent from the min tier remain unsupported; absence does not state a negative clinical finding. ClinPGx accessions and numeric annotation IDs remain separate. ClinPGx uses the registry's noncommercial policy ruling. The `devices` and `narrative` sections retain whole source rows and use `--section-limit` and `--section-offset`. Section commands carry a retained snapshot so a walk cannot mix report versions. A missing snapshot offers an explicit restart. Native event, received and report dates remain distinct. Narrative text supplies no inferred diagnosis. `--event-type` matches the report's native `event_type` exactly (Malfunction, Injury, Death or Other); `--problem` matches the coded `product_problems` vocabulary, where malfunction is not a value. Reports with an empty event type or `No answer provided` match no `--event-type` value. A disabled source or component cannot return through cache or replay.

## WHO diagnostic cards

Each row retains its whole name, code, assay format, manufacturer, marker, regulatory-version text and prequalification year. Reads use the stored export: a live first read fetches a missing copy under `auto_sync_max_bytes`, reads never refresh a stored copy, and offline and replay never fetch. WHO cards have no section command. WHO terms reserve commercial and substantial reproduction rights; `--disable-sources noncommercial` disables stored cards, searches and synchronization.

## PMA submissions and supplements

The `regulatory` section pages the selected row’s original statement fragments; the base-only `supplements` section returns five native headers per page. Missing supplements return source-local absence. A supplement’s decision belongs to that submission alone. Exact companion-table links remain separate from PMA approval records.

`search diagnostic --source fda-pma --intended-use "cell-free"` matches the words of each submission's approval-order statement (`ao_statement`), the intended-use text FDA publishes, as one phrase; `--name` still matches only `trade_name`. Supplements carry their own statements, so one device can match more than once. The reply's `source_query` shows the exact openFDA expression and so the field each filter matched. openFDA publishes no intended-use text for 510(k) records, so `--intended-use` is refused for `--source fda-510k`. The phrase follows the same openFDA rules as a device recall name: no plurals or synonyms, refused punctuation spaced, an accented letter sent as its base letter, and other non-ASCII characters spaced.

`search diagnostic --source fda-510k --classification-name "cell-free"` matches FDA's device classification name, the name of the device type its product code stands for, which openFDA adds to each 510(k) and De Novo record as `openfda.device_name`. It is not intended-use text: every device under one product code shares the name, so it finds devices of a kind, such as DEN230046 under "High Throughput Sequencing Based Tumor Profiling Test Of Circulating Cell-Free Nucleic Acids", while `--name` matches only the submitted `device_name`. The reply's `source_query` names the field. The phrase follows the same openFDA rules as `--intended-use`. PMA classification names are not searched.

`search diagnostic --source fda-510k --aggregate product-code` also counts every matching 510(k) record by FDA product code with openFDA's count facet, for example `--decision-from 2024-01-01 --decision-to 2024-12-31 --aggregate product-code`. Sent with no limit, that facet returns only the 100 largest codes; BioMCP asks for openFDA's most, 1,000. `product_code_counts` lists the codes most records first; records openFDA counts under an empty code are in `without_product_code`, and `sum` adds both; `sum_matches_total` says whether that sum equals the page total, so every matching record is counted under a listed code. When `limit_reached` is true, more codes may exist: narrow the decision dates, for example one year at a time. The reply still holds the first page of records, and its next page drops the count.

## FDA companion tables

Rows retain their whole cells and original links. Both tables also give `diagnostic`, `indication`, `device` and `drug` as cell text under one name. A row's `row_date` is the newest date printed in its own device cell, which may be a supplement's date rather than the first approval; a row whose cell prints no valid date has none and is named in `problems`, never guessed. `--sort date` and `--date-from`/`--date-to` read every matching row of the stored table before paging. `--biomarker` matches a whole biomarker name in the individual table's biomarker cell, in any case and ignoring surrounding spaces, so `PIK3CA` matches `PIK3CA, AKT1, and PTEN` but not `PIK3CA2`. The group table has no biomarker cell and is not searched; the reply names it in `problems` as `filter.unsupported`. A companion row is not a treatment recommendation.

## FDA clearance records

It keeps the source decision fields and offers its exact companion-table selector. It has no sections or free-text search. PMA approval, 510(k) clearance, WHO prequalification and GTR listing remain independent source facts. A listing is not authorization. No source record establishes clinical suitability by itself.

## openFDA text filters

openFDA answers HTTP 400 to `! # $ ; ? ^ | ~`, the backtick and every character outside ASCII, even inside a quoted phrase. Every openFDA text filter (Drugs@FDA, label, drug and device recall, FAERS, MAUDE, 510(k) and PMA) therefore sends an accented Latin letter as its base letter, so `--applicant Nestlé` searches `NESTLE`, drops a separate combining accent mark (as macOS pastes `ü`), so the word stays whole, and replaces those characters and other non-ASCII characters, such as ®, with a space, which splits words as openFDA's own text search does. A value left with no letter or digit, such as `®` alone, is refused before the request for a device recall name, `--intended-use` and `--classification-name`; an applicant like that matches no sponsor. Inside an OR group, such as a second `--product-code` or `--drug`, only that value matches nothing, and the notice says so. When a value is sent as other words, the source carries a notice such as `--firm was searched as "Pfizer"`; adverse event and diagnostic replies also show the whole expression in `source_query`, while drug replies have none. Quotes and backslashes stay escaped, so no operator, wildcard or field in a value reaches openFDA as syntax.

## WHO finished products

Reads use the stored export: a live first read fetches a missing copy under `auto_sync_max_bytes`, reads never refresh a stored copy, and offline and replay never fetch. Results preserve native combined INN, dosage-form and strength text, empty cells and original dates. WHO prequalification does not establish US or EU authorization. The export-specific reuse grant is unverified, so the source is noncommercial and links WHO's copyright policy.

## WHO essential medicines

`get drug NAME essential-medicines` reads the WHO eEML live. WHO says the PDF Model Lists (EML and EMLc) are the binding versions and the eEML may differ from them or miss medicines once listed and later removed. The eEML names a medicine by its own name, such as dabigatran, not a salt such as dabigatran etexilate or a brand such as Pradaxa; `search drug NAME --source who-eml` lists the eEML's own matches. Only Model List listings are shown: the eEML also publishes applications the Expert Committee rejected, which are left out, and the total counts listings only. Each listing's status is on its own page, so a medicine with many applications takes a second per page. A changed eEML page fails the call rather than answering absent. Its terms forbid using it to advertise or promote commercial products or to suggest WHO endorsement.

## FDA drug safety communications

`search drug NAME --source fda-dsc` reads FDA's [index of Drug Safety Communications](https://www.fda.gov/drugs/drug-safety-and-availability/drug-safety-communications) and joins it with the list BioMCP ships, so a first search answers at once. The index lists communications from 2020-01-14; for earlier ones FDA links an archived copy, which BioMCP does not read. A row can match its title, drug names linked to Drugs@FDA or available text, after the usual drug-name widening. The row's `matched_in` lists where the name appeared. The reply counts all communications searched and how many had page text, including when some pages are unread. Zero matches does not establish that FDA issued no communication about this drug. `date` is the date FDA's index lists, and only a full month, day and four-digit year is read, so FDA's `9-1-21` stays unread and is named as a problem; `issued` is the date the page states for itself, and a disagreement between the two is named, never resolved. `updated` is the page's "Content current as of" date. A communication FDA added after the shipped list is read from its page on a later call when FDA's spacing allows: BioMCP waits 30 seconds between www.fda.gov page reads, as [FDA's robots.txt](https://www.fda.gov/robots.txt) asks, and until then the row shows FDA's title, date and address with `page_read: false`. When the index cannot be read, the shipped list answers alone and the source is marked stale. `get drug UUID --source fda-dsc` uses a communication identifier from search to read one page live and returns its text as FDA wrote it. FDA's [website policies](https://www.fda.gov/about-fda/about-website/website-policies): "Unless otherwise noted, the contents of the FDA website (www.fda.gov) — both text and graphics — are not copyrighted. They are in the public domain and may be republished, reprinted and otherwise used freely by anyone without the need to obtain permission from FDA." `scripts/fda_dsc_seed.py` remakes the shipped list with this reader.

## WHO diagnostic search

Both selectors match literal whole words in their native columns, ignore case and combine on the same source row. Search returns five product headers by default and groups repeated code rows by exact WHO Product ID. Reads use the stored export: a live first read fetches a missing copy under `auto_sync_max_bytes`, reads never refresh a stored copy, and offline and replay never fetch. A WHO listing does not establish FDA authorization or clinical suitability.

## Study metadata

Text matches literally and case-insensitively in native study IDs, names and descriptions; cancer-type codes remain case-sensitive. A live first read fetches a missing catalog; reads never refresh a stored one, and stale copies offer an explicit sync command. Returned page commands retain the catalog revision and every selector. cBioPortal uses ODbL with original-study attribution unless otherwise noted; individual studies may restrict commercial use. The catalog contains metadata only. File references preserve repository blob size and native LICENSE links without downloading data files. Source counts keep their native names. No frequency, denominator, survival or comparison is calculated. Gene-count rows and patient totals remain deferred because the source lacks a gene-row filter.

## Phenotype sections

Each section returns five rows by default, and `ancestors` returns 50; `--section-limit` and `--section-offset` select a local page. Missing source fields remain unavailable, and empty source arrays remain empty collections. A synonym, cross reference or hierarchy link supplies no clinical equivalence. The `ancestors` section comes from OLS, not Monarch, so it answers when Monarch is down: it lists every term the queried term sits under, up to the HP root, without the path between them. If OLS lists an ancestor without its ID, the reply is partial, never a shorter complete list. An obsolete term says so, lists no ancestors and names the term OLS gives as its replacement.

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

## STRING interaction partners

`get protein Q14191 interactions` reads STRING 12.5's functional association partners for the protein's canonical UniProt accession; STRING keeps one protein per gene, so an isoform reads its canonical entry, and distinct products of one gene share one entry and one partner list: CDKN2A's P42771 (p16) and Q8N726 (p14ARF) both map to `9606.ENSP00000418915`. The reply's `string_id` names the entry, so a partner list is never evidence about one specific product. A partner is listed when its combined score is 0.4 or above, STRING's medium confidence, and rows keep STRING's order, highest score first. A score is STRING's confidence that two proteins are functionally associated, which includes physical binding but also shared pathways, co-expression and co-mention in papers; it is not a binding affinity. Each row names the evidence channels that scored above zero. `total` counts every listed partner, and the card shows the first `--section-limit` rows. An empty section means STRING does not know the protein or lists no partner at 0.4 or above. STRING data is licensed under [CC BY 4.0](https://string-db.org/cgi/access); cite STRING and its version when you reuse it.

## Cell-line identity

Names remain unchanged. A full search page's total is Cellosaurus's own match count, read with one small extra request; a shorter page's total is the rows read to its end. The card preserves entry dates, source disease/species references and complete stock-qualified warnings. Publisher release metadata and retrieval time remain separate. Cellosaurus uses CC BY 4.0 with attribution; source omissions do not establish a negative biological finding.

## ChEMBL cell metadata

The reply preserves native names, tissue, organism, taxon and optional EFO/CLO identifiers. Metadata never requests assays. The total counts assays only and supplies no activity, experiment or drug sensitivity estimate. ChEMBL uses CC BY-SA 3.0. Its independently cached release metadata and labelled retrieval time remain separate.

## Protein ChEMBL targets

`get protein P09874 chembl` asks ChEMBL for the single-protein targets whose component is the protein's canonical UniProt accession; an isoform asks for its canonical accession, since ChEMBL names canonical accessions. The section's `accession`, or an empty section's record, names the accession ChEMBL was asked about. ChEMBL also files the protein under complexes, families and protein-protein interaction targets; those are not shown or counted. Each target's `assay_count` and `activity_count` are ChEMBL's own totals (`page_meta.total_count` of its assay and activity lists for that target), not rows BioMCP read or filtered, and they count every assay type and every activity record, active or not, including records ChEMBL flags as doubtful. They measure how much a target has been tested, not whether any compound works. An empty section means ChEMBL lists no single-protein target for the accession. ChEMBL uses CC BY-SA 3.0.

## Cell-line sections

Each source keeps its own outcome and release dates. Composite reads never request assay counts. PharmacoDB drug response remains deferred until a data reuse grant is published, and the HPA expression route remains deferred until an exact cell identity join exists.

## GenCC submissions

`get gene ODC1 gencc` reads the local GenCC bundle by HGNC identity. A live first read fetches a missing bundle under `auto_sync_max_bytes`; run `sync gencc` to refresh it. Reads never refresh a stored copy, offline and replay never fetch, and reads keep stale rows and mark the source stale. Health checks report publisher reachability and local bundle readiness separately.

Each record keeps GenCC's submission ID, version, submitter, disease, inheritance, classification and dates in file order. BioMCP neither merges submissions nor ranks classifications. An empty section means the stored export contains no matching rows.

GenCC documents its new-format CSV export, weekly publication cadence, conditional requests and download policy on its [download page](https://thegencc.org/download). Its [terms](https://thegencc.org/terms) identify CC0 1.0 and request attribution to GenCC and its contributors. The [GenCC publication](https://pmc.ncbi.nlm.nih.gov/articles/PMC7613247/) describes the submission fields.

## ClinVar gene counts

`get gene BRCA1 clinvar` counts ClinVar variation records (VCV) that list the gene's NCBI Gene ID, each record once whatever its assemblies. Records that also list other genes count too: most BRCA1 records that list a second gene name a small variant that ClinVar also places in an overlapping regulatory element, not a large deletion. Classification buckets follow the terms in each record's aggregate germline label, which ClinVar joins with `/` and `;`: "Pathogenic; risk factor" counts as pathogenic, "Conflicting classifications of pathogenicity; risk factor" as conflicting, and a label with no classification term, such as "risk factor", "Pathogenic, low penetrance" or a bare "VUS-high", as other. The buckets do not overlap except for the rare label that lists terms from two of them; somatic clinical impact and oncogenicity are not counted. `pathogenic_or_likely_grch38` keeps the pathogenic or likely pathogenic records that have a GRCh38 position, which is what a GRCh38 row count of ClinVar's `variant_summary.txt.gz` gives; the rest are mostly exon-level deletions and duplications placed only on GRCh37.

The counts come from E-utilities searches with `retmax=0`. ClinVar's review status search also matches somatic review statuses, so the pathogenic or likely pathogenic review buckets come from Variation ID lists, and the few records listed under more than one status are summarized to read their germline review status. `build` and `build_updated` name the ClinVar release that answered. Counts can differ by a few from a summary file of another release.

ClinicalTrials.gov records keep valid fields when a status, date, phase or other nonidentity value cannot map. The reply reports a field problem and omits the rejected value. An invalid NCT ID excludes its study from a search page or fails that exact record request. Overall status values follow the [publisher enum metadata](https://clinicaltrials.gov/api/v2/studies/enums), including `TEMPORARILY_NOT_AVAILABLE`.

The trial page, trial card, trial header and trial document result schemas change major versions because rejected source fields can now be null. Existing valid source values keep their wire representation.

A rejected registry age value stays unknown. It does not become the source’s missing-age state. Nonblank native text keeps its original bytes; blank native text is omitted with a field problem.


## NCI organizations

`search organization --source nci` sends NCI's documented name, city, state or province, country, postal code, organization family and coordinate radius selectors. `--name` uses the publisher's partial match. Names are not unique, and NCI publishes no organization ID. Each search row includes the exact source name, location, trial count and optional published address, coordinates, family and public contacts. There is no organization get command.

The count covers trials of every status, including closed trials. It does not count recruiting trials alone. One bounded unsized request supplies the complete organization observation. Returned page commands retain that snapshot and page it locally. A too-broad refusal names the filters that can narrow the request. Source bucket failures remain failures and never establish zero matches.

NCI trial facility searches use `sites.org_name` for an exact case-insensitive organization name. Each organization row offers this search as its next command. Each trial header shows the first locally equal site in publisher order and the number of equal sites. Site name, city, state and recruitment status retain source values. A listed site is not necessarily enrolling. If no returned site equals the requested name, the trial keeps its position and reports a `sites.org_name` problem. NCI facility searches refuse radius filters because NCI does not document that these filters bind one site.

## NCI biomarker concepts

`search biomarker --source nci --name PD-L1` searches NCI's native concept names and synonyms. `get biomarker C96022` searches by an exact NCIt code, here C96022 (CD274 Gene, PD-L1). Both require the user's NCI key. The public tier returns codes, name, type, trial count, synonyms, parent and ancestor codes, and native `eligibility_criterion` values. A code can return several native rows; BioMCP then returns candidates without choosing one. Counts cover NCI trials across statuses.

These commands return terminology. `search diagnostic --biomarker` filters diagnostic tests, and `search trial --variant` searches literal variant text. Native inclusion and exclusion values remain source assertions and do not establish patient eligibility. Search pages retain one bounded observation. A broad request can return narrowing guidance; a source failure never establishes zero concepts.

## Drug names across sources

After an empty name lookup, BioMCP may consult existing MyChem, openFDA label or ChEMBL records for source-stated names. It reads ChEMBL's records from MyChem's copy of them, so a slow ChEMBL API does not hold the lookup. ChEMBL first tries its native exact synonym filter when no selector was supplied. Cross-source retries require one exact name-resolution candidate and preserve the selected source's filters, IDs and native counts. Ambiguous names offer a source search instead of selecting an alias. The reply identifies any source supplying names, shows the actual successful search spelling and preserves it in continuation commands. A matching record in another source does not establish presence in the selected source. Source exclusions and the command budget also govern alias lookups; failed or disabled reads remain visible.

## Medical device recalls

`search diagnostic --source fda-device-recall` covers every medical device. It searches the native recalling firm phrase (`--manufacturer`, or `--firm` as in the drug recall search), a device name phrase, exact product code and inclusive initiation dates. The [FDA field mapping](https://github.com/FDA/openfda/blob/master/schemas/device_recall_mapping.json) defines `product_code` as a keyword field and `product_res_number.exact` as the exact recall identity. Totals count product recall records, including separate records under one event number. A count makes no claim about harm.

`--name "infusion pump"` matches the words of the recall's native `product_description` as one phrase, ignoring case and punctuation, as openFDA's text search does. It adds no plurals, synonyms or product codes: `infusion pumps` finds fewer records than `infusion pump`, and a name total counts a different set from a `--product-code` total, because a description can name a device whose category code differs, and a product code covers devices whose description never uses the phrase. The reply's `source_query` shows the exact openFDA expression, and a source notice says which field the phrase matched. openFDA answers HTTP 400 to `! # $ ; ? ^ | ~` and the backtick even inside a quoted phrase, so BioMCP replaces them with a space, which splits words the same way. openFDA answers HTTP 400 to every character outside ASCII too: a symbol such as ® or ™ becomes a space, so `Alaris® pump` searches `Alaris pump`, and the notice names the phrase searched; an accented letter such as é is sent as its base letter, so `infusion pümp` searches `infusion pump`. The micro sign µ counts as a symbol. A name with no ASCII letter or digit, such as `®` alone, is refused. Each search row carries the recall's product description and reason beside its firm and classification.

The card preserves the recall's reason, root cause, product description and distribution pattern. Its submission numbers come only from top-level `k_numbers` and `pma_numbers` in the [device recall fields](https://open.fda.gov/apis/device/recall/searchable-fields/). Harmonized `openfda` identifiers describe the product code and never populate the card. One exact [device enforcement](https://open.fda.gov/apis/device/enforcement/) recall-number lookup supplies classification when present; absence leaves classification absent. A failed lookup preserves the recall card and discloses a partial result.

Search rows carry the same classification. One exact enforcement request per page reads the classes of that page's recall numbers; a row without an enforcement record shows no classification and the source notice counts those rows. BioMCP never infers a class from the recall status. `--classification "Class I"` selects [device enforcement](https://open.fda.gov/apis/device/enforcement/) records first, because device recall records hold no class, then reads their recall records in one exact request. An enforcement record without a recall record still shows its own `reason_for_recall`, which both datasets publish. Under that filter a source notice says the total counts enforcement records, the firm phrase and initiation dates match the enforcement fields, and `--product-code` is refused because enforcement records carry no product code. A failed join keeps the rows and discloses a partial result.

The MAUDE next step selects the recall's product code across manufacturers. Recall records and adverse-event reports remain separate source facts. Neither establishes product-specific harm or clinical suitability.

openFDA label search first tries exact brand and generic fields, then word matches, then indication wording, then other label text. The first field query with native results supplies every page and its total. A source match need not be the complete product name; `matched_name` records complete name matches separately. Display names use the first brand or generic value. A label without either, which openFDA leaves for some labels, takes the leading words of its own product data only where a display panel text prints them whole, ended by an opening parenthesis with at most one ® or ™ directly before it, or as the whole of that text. The indications section is never read, because it names the drug family, such as ZYPREXA for ZYPREXA Zydis. A plain-text (R) or (TM) reads as ® or ™, not as an opening parenthesis. Known limits: a name can drop a salt the panel prints after the parenthesis, such as Terbinafine for Terbinafine Hydrochloride from "terbinafine (equivalent to ...", and can drop a trailing bracketed word, such as Kogenate FS Antihemophilic Factor without (Recombinant). Otherwise, as for a brand printed alone before more product words, the row has no `name`, never its set ID or a cut-short name, and a search page notice lists those rows and how to read them. Product-data paragraphs remain in the original label sections.

`search drug --indication "pancreatic cancer" --source openfda-label` searches only the label's native `indications_and_usage` field. It preserves the phrase without disease synonym expansion or clinical inference. An optional name query further narrows that same search. Results remain SPL label identities in source order, with the source total; follow `get drug` and the label section to read the original wording. Drugs@FDA does not support this filter.

`search drug --mechanism-of-action "Janus kinase" --source openfda-label` searches only the label's native `mechanism_of_action` field for those words, as openFDA matches a phrase, in any letter case. BioMCP adds no synonyms and does not decide which drugs belong to a class: a label that words the mechanism differently, such as "Janus Associated Kinases", is not returned, and a label that mentions the words in passing, such as an interferon label naming Janus kinase 1 in its signaling cascade, is. Each row's `matched_sentence` quotes the first sentence of that field holding the words, so a reader can judge each match; it is absent when openFDA matched the words across a sentence end. `--application-type nda,bla` keeps labels whose openFDA `application_number` starts with NDA or BLA, leaving out generics (ANDA) and labels with no application; it narrows any openFDA label search. openFDA fills its own pharmacologic class fields (`pharm_class_epc`, `pharm_class_moa`) on fewer than half of NDA and BLA labels, so BioMCP does not search them.

`search drug --source drugsfda --applicant Alnylam` returns the applications whose native `sponsor_name` starts with that text, with the native total. Drugs@FDA stores sponsor names in upper case and matches them only as complete values, so BioMCP upper-cases the text, escapes it and appends the source's own prefix wildcard; a complete name such as `ALNYLAM PHARMS INC` still matches. A source notice states the prefix searched. A prefix can match several unrelated sponsors, and BioMCP adds no company-family or subsidiary inference. An optional name narrows the same search.

`get drug BLA125514 approvals --source drugsfda --decision-from 2024-01-01` keeps only the submissions whose status date falls in the window, inclusive; undated submissions are left out, the page total counts only the window, and a notice states the window and how many of the application's submissions fall in it. The dates are full days, YYYY-MM-DD, and the window does not combine with `--document-id`. Drugs@FDA approvals take `--section-limit` up to 200, so a whole application history fits in one page; other sections and sources stay at 50. A page asked for with more than 50 rows reads no supplement letters, so its rows carry no `supplement_purpose`; ask for 50 rows or fewer, or one `--document-id`, to read them.

Use `--fields=application_docs.id,application_docs.type` to list document IDs and types in an approval history. Pass a Letter ID to `--document-id` to read it. Use `--fields=application_docs` for full link metadata.

Each Drugs@FDA search row carries `latest_submission`: the application's submission with the latest status date, with its type, number, status and class. It also carries `original_approval`, the date and number of its earliest approved original (ORIG) submission, when the source lists one. `--sort newest` asks the source to order applications by that date, newest first, with ties broken by application number so pages stay stable; applications with no dated submission come last. Without `--sort` the rows keep the source's own order. The total is the source's total either way. The date is the submission's status date, so a recent labeling supplement can put an old application first; read the row's type and class before calling it a new approval.

`--sort original-approval` orders applications by `original_approval` date, newest first, ties by application number, with applications that list no approved original submission last. openFDA sorts and filters an application by any of its submissions, never by one chosen submission, so BioMCP reads every matching application (500 per request, at most 3,000) and orders them itself; a source notice says so and gives the count read. `--approved-from` and `--approved-to` (YYYY, YYYY-MM or YYYY-MM-DD, inclusive) keep only applications whose original approval falls in the window, read the same way; the page total then counts the kept applications, and the notice gives both counts. Pages stay at most 50 rows. When more than 3,000 applications match, BioMCP returns no rows, marks the source failed as `too-large` and states the match count; narrow the search with a drug name or a longer `--applicant`.

Drugs@FDA, FDA drug recall and EMA searches also widen a drug name by default, and `--exact-term` searches only the typed name. Drugs@FDA takes the exact or word step the typed name reaches and searches all names at that step in one query; recalls search all names in one query; both report the source's own total for that query. EMA searches each name in its local medicines list and counts each medicine once. Later pages keep the first page's names. MAUDE and VAERS adverse-event searches stay literal.

Label search widens a drug name by default: `biomcp search drug apixaban --source openfda-label` searches the typed name and its bounded source-stated other names in one native brand/generic group, exact names in any letter case first and then word matches, as the typed name alone would be searched. The typed name picks the discovery step (names, then indication text, then label text), so the widened search never finds fewer label sets than the typed name alone; later pages keep the first page's names and step. Add `--exact-term` to search only the typed name; `--expand-drug-names` is still accepted for one release and changes nothing. Literal indication wording stays unchanged. The whole-query bound is BioMCP's own bound; one notice on the label source lists the names searched and each name left out with its reason. Counts are the native label total, never a sum across names; revision numbers do not rank different SPL sets. Follow returned page commands unchanged; restart the first page if its retained name selection is unavailable or disallowed.

### FDA Purple Book

`biomcp search drug adalimumab --source purple-book` returns the full biosimilar product roster for a reference name or BLA on one page. Each row counts the presentations under each FDA designation and shows the earliest reported approval and interchangeability dates, plus differing dates where supplied. Use `--presentations`, `--strength`, or `--device` for FDA presentation rows. The presentation product number distinguishes rows with the same strength and device. When FDA's reference names match multiple BLAs, the reply gives the candidate BLAs.

FDA updates the list monthly. BioMCP ships a snapshot and shows the release month. Download a newer complete monthly CSV from [FDA's Purple Book downloads](https://purplebooksearch.fda.gov/downloads) and import it with `biomcp sync purple-book --file PATH`. A failed import preserves the active release.
