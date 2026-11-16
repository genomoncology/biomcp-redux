# BioMCP 1.0

BioMCP 1.0 retrieves source records for cancer care and human health research through the CLI and MCP. Upgrade existing `biomcp-cli` installations with their package manager. FHIR patient reading is planned for 1.1 and is unavailable in 1.0.

## Breaking changes for 0.9 users

JSON replaces Markdown as the default CLI output. Select `--format md` or set `"format": "md"` in configuration for Markdown. MCP always returns JSON. Scripts must read the new response envelopes and entity schemas instead of parsing 0.9 output.

MCP uses `single` mode by default: the `biomcp` tool accepts a CLI argument array, alongside the research `search` and `fetch` tools. Opt into typed tools with `"mcp_tool_mode": "typed"` in configuration or `biomcp serve --tool-mode typed`. The command-line setting overrides configuration. Typed mode offers 18 tools: 13 entity tools (`gene`, `variant`, `disease`, `drug`, `trial`, `article`, `protein`, `pathway`, `phenotype`, `adverse_event`, `diagnostic`, `cell_line` and `study`) plus `biomcp`, `help`, `search`, `fetch` and `saved`. A hosted server offers `saved` only when started with `--saved-folder`. An entity tool takes `action` and one object named after the action, for example `gene` with `{"action":"get","get":{"id":"BRAF"}}`. Every search takes its text as `query`, every get takes its record ID as `id`, and sections go in `section`; an ID, a section or any list argument may be one string or a list. The server also reads `input`, `text`, `name`, `keyword`, `keywords` and `sections` as the argument they mean, and section words are plain text it checks. Every registry command is reachable through a typed schema. Available tools and source choices follow the server's policy; use its `tools/list` response for the current catalog.

Command help now includes summaries from shared command metadata, and typed tool descriptions use those summaries. `help` accepts a command path and returns the existing command help. For `get` errors, follow the returned `next` action: an identifier with the accepted shape but no source record keeps its existing help advice.

Run `biomcp mcp-config` to generate stdio wiring for the installed executable. The client starts `biomcp serve` with the selected tool mode. HTTP uses `biomcp serve --http [ADDR]` with `/mcp`; it refuses browser origins and requires allowed hosts for non-loopback listeners. Hosted deployments need caller credential isolation and source pacing configuration. See the [MCP setup](https://github.com/genomoncology/biomcp/blob/main/README.md#connect-an-mcp-client).

The [upgrade table](https://github.com/genomoncology/biomcp/blob/main/sdlc/planning/upgrade.md#commands) lists every 0.9 leaf command and each removed or deferred option, with its replacement or retirement ruling. Read the linked entity sections for positional sections and aliases. The [1.0 command reference](https://github.com/genomoncology/biomcp/blob/main/docs/reference/commands.md) gives accepted arguments. Major changes include:

- Replace global `-j` and `--json` with `--format json`. Replace `--no-cache` with `--fresh`; this requests fresh source replies under the 1.0 store rules.
- Run `mcp-config` without `--absolute-path`; it always prints the executable's absolute path. Keep `--client CLIENT` for client-specific configuration.
- Use positional text for `search author` and `search cell-line`; their `-q` and `--query` options retire.
- Use `get ENTITY` with repeated IDs instead of batch wrappers. Use trailing section words instead of batch `--sections`.
- Use `sync SOURCE` for stored bundles and `store clean` for expired replies and collections. `cache clean --dry-run`, `--max-age` and `--max-size` have no maintenance-policy replacement.
- Use returned `--next-page` and `--pool` commands for continuation. Keep the returned argument arrays and policy settings together. Trial `--offset` is continuation bookkeeping and requires `--next-page` when nonzero. Hidden sessions do not identify the new collections.
- Use explicit entity search and get commands. Discover, chart generation, local study analysis, self-update and automatic text-based classification or identity merging retire. Patient reading moves to 1.1.
- Trial discovery's `--variant` and `--criteria` search other spellings of a protein change, or only the typed text with `--exact-term`; mutation and biomarker aliases and semantic therapy-line filters retire. Discovery never determines eligibility.
- Article downloads use exact file selections and `--save PATH`. Text readers, PDFs and supplements have separate commands. HTTP MCP refuses file saves. Plain `serve --http` refuses `article pdf` and `trial document`, whose save is required, with a `usage.arguments` error naming `--save` before it asks any source.
- Variant normalization uses `--source`, `--hgvs` and explicit builds. Variant source assertions, population and recurrence use their named commands. No source score becomes a clinical classification.

## HGNC gene name search

Gene search matches words in HGNC current names, previous names and alias names, while exact symbols rank first. Pre-release 1.0 HGNC stores need their index rebuilt to include the added name fields. A sync that downloads the complete HGNC file rebuilds the index; an HTTP 304 response reuses the old index. Use a fresh store when the publisher keeps returning 304. No released 1.0 store requires migration.

## Added population fields

Variant population replies add source-published `homozygote_count` for exome and genome totals and `joint` AC, AN and homozygote totals. A joint section with `state: empty` means gnomAD supplied no joint data; BioMCP never adds exome and genome counts. `dataset_version` names a release only when matching same-response exome and genome totals independently report the same version. Joint counts have no invented independent release or frequency. Existing counts, filters and ancestry pagination retain their meanings.

## Saved work

BioMCP keeps saved searches, lists of records and notes in `saved.sqlite` in the data folder. A cache clean or an upgrade keeps them.

- Add `--save-as NAME` to a first-page search to save it. `saved list add NAME --entity ENTITY ID...` keeps IDs that `get ENTITY` accepts in a named list, and `saved list remove NAME ID...` takes them out.
- `saved note TARGET TEXT` attaches a note to a saved search or list, to a list item with `--item ID`, or, with `saved note --project default TEXT`, to the project. Notes come back labelled as user data, never as instructions. A name holds up to 80 characters and a note up to 2,000.
- `saved show [NAME]` lists what is saved, `saved export` returns all of it as typed JSON, and `saved delete NAME` removes a search or list with its notes; `--note ID` removes one note and `--all` removes everything.
- `saved changed [NAME]` reads each saved search again, at most once an hour, and reports the IDs it gained, lost or changed since its last check. Each search's state is `first`, `unchanged`, `changed`, `not_due`, `rebaselined`, `incomplete` or `failed`. On a hosted server a re-check follows the same source policy as any hosted call.
- In typed MCP mode the `saved` tool offers the actions `show`, `save_search`, `changed`, `list_add`, `list_remove`, `note`, `export` and `delete`, and a first search page with results offers a `save` next step.

## Drug names and variant spellings widen by default

Drug-name searches also search the drug's other names by default: trial and article search as before, and now FAERS, openFDA labels, Drugs@FDA, EMA and FDA drug recalls. Trial search's `--variant` and `--criteria` also search a protein change's other spellings, such as V600E, Val600Glu and p.V600E, and the codon when `--gene` is given. The reply says which names and spellings it searched, and which names it left out and why, such as code-like synonyms or names the source cannot take. `--exact-term` searches only the text as typed. Other drug sources, MAUDE and VAERS search the text as typed. `--expand-drug-names` is accepted in 1.0 and changes nothing. The name lookup reads ChEMBL records from MyChem instead of waiting on ChEMBL's own service. When the lookup is slow or incomplete, trial and article search use the typed name, say so, and stay complete.

## Added commands, filters and sections

Drugs and regulators:

- `get drug NAME markets` gives one row per market, Drugs@FDA, EMA and WHO prequalification, each with its record number, holder, status, first approval date and URL. EMA picks the exact brand, then the earliest authorisation, and says so.
- Drugs@FDA search takes `--applicant` (sponsor names starting with the text), `--approved-from` and `--approved-to` (original approval window), and `--sort newest` or `--sort original-approval`; a sorted or windowed search says how many applications it read. Rows carry `latest_submission` and `original_approval`. `get drug ID approvals` takes `--decision-from` and `--decision-to`, and `--section-limit` reaches 200, so a long application history fits one page.
- EMA: `get drug ID regulatory --source ema` no longer labels the medicines file's latest-procedure date as the Commission decision; that date's role is now `latest_procedure_affecting_product_information_date`. For a referral, `european_commission_decision_date` comes from EMA's referrals file, which `sync ema` now downloads, so sync EMA again.
- `get drug NAME interactions --source ddinter --partner NAME` keeps one drug pair; DDInter's files carry a level per pair and no management text. `sync ddinter` now reads all of DDInter's published files. WHO PQ search takes `--therapeutic-area`, and its rows carry `prequalified_on` as a date.
- Label reads: `get drug NAME label` without `--label-section` lists the section names in `sections` instead of refusing. A generic name picks the brand-name NDA or BLA label, then the label with the most indications, and says why. Exact name matching ignores letter case, and the total counts exact and word matches together. Several label sections come in one reply, each reply fills the `drug_label_bytes` budget, `remaining` counts the fragments still to read, and a section the label lacks is named in `hint`. Many-label generic names, such as metformin, no longer fail as too large or out of time.
- openFDA label replies give the label's `effective_date`, openFDA's `effective_time`, in `identity` beside its SPL set ID and `version`, so a reader can tell a current label from an old one.
- Protein domains and complexes, alone or as card sections, add `total`, the rows in the whole selected set, like every other section. `native_total` keeps its meaning: InterPro's count before `--type` selection, or Complex Portal's text-search count before exact participant admission. Report `total`.
- FDA drug recalls: `--status` and `--reason` searches return full pages, and older recall numbers such as `D-502-2013` appear and can be read. A recall page names each record it drops with the new `record.declined` problem.
- openFDA text filters accept punctuation and accented letters: accents become base letters, other marks become spaces, and a notice shows what was searched.

Devices and diagnostics:

- FDA PMA: `search diagnostic --source fda-pma --intended-use TEXT` matches the approval order's intended-use statement. `get diagnostic P... summary --find TEXT` returns the matching passages of FDA's safety and effectiveness summary with their fragment numbers, and summary text keeps its words whole. PMA and 510(k) pages carry `source_query`, the openFDA expression that matched. FDA PDF downloads ask for a PDF in the form FDA's servers accept, and a refusal names the document's URL and status.
- FDA 510(k): `--classification-name TEXT` matches FDA's device classification name. `get diagnostic DEN... --source fda-510k` reads a De Novo record, and rows show `decision_reading`, such as `de-novo-granted`, after `k_number`.
- FDA companion diagnostics: rows of both FDA tables start with `row_date`, `diagnostic`, `indication`, `device` and `drug`, beside FDA's own `cells`. `diagnostic` and `indication` read as one line, so a test listed in both tables has one name. `--date-from`, `--date-to` and `--sort date` filter and order the whole list. A search by drug application includes the group-labeling rows.
- GTR: `--name` matches test names and `--condition` matches condition names, ignoring letter case and hyphens, with whole-word matches first and each row's `name_match` saying which. `--gene` with `--condition-id` returns tests that list both. Cards and rows carry `specimens`, `gene_count`, `method_categories` and the lab's state and postcode; cards add `laboratory_certifications`, `laboratory_address`, `laboratory_directors`, `order_url` and `test_url`. `sync gtr` now also downloads NCBI's GTR XML release, about 245 MB in all.
- FDA device recalls: `--name TEXT` matches the recall's product description, `--classification` selects a recall class, and `--firm` is another name for `--manufacturer`. Rows carry `product_description` and `reason`; cards add `voluntary_mandated`, `initial_firm_notification`, `product_quantity`, `classified` and `reported`. Older recall numbers such as `Z-1192-03` appear and can be read, pages name each record they drop, and recall text joins line breaks into spaces.
- WHO IVD: `--prequalified-from` and `--prequalified-to` take the prequalification year, and rows carry `prequalification_year`.

Adverse events:

- FAERS cards label the age unit and every seriousness flag beside its code, and `unmapped_codes` names a code missing from openFDA's published list. Search rows and the `drugs`, `reactions` and identifier sections carry `serious_label`. `case_event_date` holds the case event date; openFDA's public FAERS data has no case narrative.
- `--aggregate received-year` or `received-month`, with `--received-from` and `--received-to`, counts FAERS or MAUDE reports by FDA receipt period.
- MAUDE takes `--event-type` (malfunction, injury, death or other). Reports with no event type fall outside every event-type count.
- VAERS takes `--sort reports` to order the whole table largest first. VAERS tables get a 60-second budget, and a rate-limited table is tried once more before the reply says when to retry.

Trials:

- `get trial NCT...` shows `enrollment`, its count with `ACTUAL` or `ESTIMATED`, and a stopped trial's `why_stopped` text.
- `get trial ID flow` and `get trial ID baseline` return participant flow and baseline characteristics without the posted outcomes.
- `results` pages hold up to 40,000 bytes of whole posted records, so a trial's outcome measures usually come in one call instead of one record a page.
- `locations` pages hold up to 100 sites, state the site total and offer the next page. Search rows carry `has_results`. A batched trial get names each trial it could not read, and each failed item's `failure` gives the source, the reason and the problem in plain words. A batch of up to 20 trials gets one ClinicalTrials.gov pacing interval per trial after the first (19 seconds at the default pacing, at most 60), so a cold batch no longer runs out of time partway.
- `search trial --fields` refuses a field the source's rows lack and names that source's fields.
- A ClinicalTrials.gov continuation is tied to its offset, filters and matching mode. Changing any of them is refused before the registry is asked, with a fresh first page offered, and a registry rejection of `--next-page` says so.

Genes, variants, diseases and phenotypes:

- `get gene GENE clinvar` counts the gene's ClinVar variants by germline classification and by review status, each variant once, with the ClinVar release. It works without an NCBI key, and it gets 30 seconds, its command too, so large genes finish under keyless NCBI pacing.
- Gene search by `--band` or a ClinGen ISCA ID also returns ClinGen dosage regions under `result.regions`, with their haploinsufficiency and triplosensitivity scores, and `result.clingen_regions_matched` says how many matched. `--region` takes a GRCh38 interval and selects overlapping regions only; unlike the 0.9 option of that name, it adds no genes. `--regions-only` returns only the regions. `sync clingen` also fetches the region file.
- `get phenotype HP:ID ancestors` returns every ancestor term from OLS, and works when Monarch is down.
- `search phenotype` given comma-separated HP IDs returns Monarch's one ranked list of similar genes, or diseases with `--group diseases`, with `--metric` and `--search-mode`. The ranking gets a 30-second budget.
- `get pgx --gene GENE --source cpic recommendations --drug-id ID --phenotype TEXT` keeps one CPIC phenotype, such as Poor Metabolizer, and the first page lists the phenotypes present. A plain drug name in `--drug-id` is refused with the accepted form.
- `get variant ID civic` joins a CIViC variant that states the same protein change, honours `--section-limit`, and takes `--disease` and `--drug`. `get variant CAID hotspots` accepts a ClinGen CAid. `get variant clinvar:ID population` falls back to the ClinGen Allele Registry when MyVariant lacks the record.
- `variant assertions ID --source clinvar --kind scv` keeps only ClinVar's per-submitter records, and `--kind vcv` or `rcv` the aggregate records; the page total counts only that kind. `get variant ID clinvar` takes the same filter as `--clinvar-kind`.
- When a transcript HGVS fails `variant normalize` on its reference base, the reply lists the gene's transcripts where the change fits and offers the fitting one; the reply is `partial` rather than `failed`. `search variant --variant` given a full HGVS name offers the runnable `--gene ... --coding ...` call.
- `get disease ID prevalence --source orphadata` returns Orphanet prevalence estimates, age of onset and inheritance after `sync orphadata`. Hierarchy rows carry parent labels, and each synonym appears once with `also_stated_by` naming the other sources that state it.
- `pathway genes` and `pathway events` with an ID the source does not hold answer `absent`, not `failed`.

Articles:

- `search article` rows, and citation, reference and recommendation pages, carry `open_access` when Europe PMC states it.
- `article pdf` for a Europe PMC preprint says at once that Europe PMC refuses automated PDF downloads and offers the full text. A publisher refusal names the host, URL and status, and a failed save gives `result.file.reason`, `not-pdf` or `not-open`.
- A single-source search reports the source's own total. A drug-name search on Semantic Scholar or LitSense says the per-name counts overlap and are not added, and a cut-short search says later rows cannot be paged. `get article` states its limit of 20 IDs.

Other commands:

- Protein features, domains and cards carry `coordinate_note`, which explains that spans are zero-based and half-open, unlike UniProt's one-based coordinates and the one-based `--position`.
- `search gwas --p-value` leaves out `qualifying_total`, and is not `complete`, when an association cannot be compared with the threshold.
- `variant recurrence` fails, with no counts, when `--sample-set` is unknown or belongs to another study or list.
- `search cell-line` reports Cellosaurus's own match count as its total.
- `biomcp health` honours `command_timeout_seconds` and each source's deadline instead of a 3-second cap.
- `help questions` lists every command by the question it answers, and `help examples` gives one worked call per use case in CLI and typed form. The MCP `help` tool takes the same words.

## Agent skill

`biomcp skill install` and the Claude plugin now ship only `SKILL.md`. The worked examples stay in the binary: read them with `biomcp skill list` and `biomcp skill NAME`.

## Hosted use and source terms

See [hosted server configuration](../docs/configuration.md#hosted-servers) for source modes, caller credentials and deployment limits.

Hosted mode runs only over HTTP behind a hosting front end. Start it with `biomcp serve --hosted --http`; `serve --hosted` without `--http` now exits with a `usage.arguments` error before serving, because source use, source exclusions, caller keys and accounts are all per-request HTTP headers that stdio cannot carry.

The published `restricted` source label is replaced by the actual `open`, `attribution`, `noncommercial`, `licensed` or `per-record` category. The retired `restricted` disable group is no longer accepted; use `noncommercial` and `licensed` to exclude commercially restricted sources and components. Current named groups follow corrected labels:

- `open` gains `car`, `clinvar`, `erepo`, `gwas`, `litsense`, `mutalyzer`, `mygene`, `nci`, `pubmed`, `pubtator`, `variantvalidator` and `cpic`, plus the MyVariant `clinvar` and `snpeff` components. The MyVariant service is open while each carried component retains its own terms.
- `noncommercial` loses `cpic` and gains `who-pq` and the MyVariant `snpedia` component. Its existing DDInter, ClinPGx, Semantic Scholar API, ORCID, WHO IVD, CADD and academic dbNSFP exclusions remain.
- The `licensed` label applies to `alphagenome`, `oncokb`, the MyVariant `cosmic` component and the MyDisease `omim` component.
- The `per-record` label applies to PMC OA and Europe PMC full text. Publisher PDFs obtained through Semantic Scholar also retain their own record licenses, separately from the API's noncommercial terms.

ClinicalTrials.gov, cBioPortal, Cancer Hotspots, VAERS, FAERS MedDRA terms and MAUDE GMDN terms carry attribution labels and remain available in restricted use. CPIC has its own terms independently of ClinPGx. CADD and COSMIC terms links are corrected, and SnpEff is identified by its MIT license. NCBI and openFDA keys continue to raise the limits of free sources. Previously stored continuations resolve old labels through the current source registry when read. The new `orphadata` source, Orphanet's prevalence, onset and inheritance files read through `sync orphadata`, carries the `attribution` label.

Hosted servers gain these settings and limits:

- Set the same `BIOMCP_TRIAL_CONTINUATION_KEY` in every process of a multi-task deployment, so a ClinicalTrials.gov continuation from one task works on another. Without it, continuations stay with the task that issued them.
- A call to a slow source, a VAERS table, a Monarch similarity ranking or ClinVar gene counts, gets that source's longer budget instead of ending at `command_timeout_seconds`.
- Saved work needs `serve --saved-folder DIR` and an `x-biomcp-account` header that the front end sets from the verified user. The [hosted server configuration](../docs/configuration.md#hosted-servers) gives the front end's whole contract.
- `reply_cache_max_bytes` caps the stored source replies, 2 GiB by default: about 560 typical 3.6 MiB openFDA label pages, a tenth of a 20 GiB container disk. When a new reply would pass it, expired replies are removed first, then the oldest, so a server that cannot run `store clean` keeps its stored replies under the cap.
- A running hosted call stops when its client closes the connection: its source requests close and its slot frees at once, not at the call's deadline. Over HTTP a client cancels by closing the connection; `notifications/cancelled` is ignored. The [hosted server configuration](../docs/configuration.md#hosted-servers) states the timeout and cancellation contract.

## Configuration migration

Create a JSON configuration file at the platform path described in the [upgrade guide](https://github.com/genomoncology/biomcp/blob/main/sdlc/planning/upgrade.md#configuration-and-keys). Unknown fields fail. Replace the old `cache.toml` fields `[cache].dir`, `[cache].max_size`, `[cache].min_disk_free` and `[cache].max_age_secs`. The old cache mode, size and disk policies no longer apply. The following 0.9 operator variables no longer configure 1.0:

- `BIOMCP_CACHE_DIR`, `BIOMCP_CACHE_MODE`, `BIOMCP_CACHE_MAX_AGE`, `BIOMCP_CACHE_MAX_SIZE`, `BIOMCP_CACHE_MIN_DISK_FREE`: use the platform store and explicit `store clean`; there is no user cache eviction policy.
- `BIOMCP_DISABLE_SOURCES` and `BIOMCP_DISABLE_KEGG`: move exclusions to JSON `disable_sources` or repeat `--disable-sources SOURCE`. The old environment variables no longer exclude sources. Replace the old `non-commercial`, `license-required` and `commercial-restricted` group names with the current `noncommercial` and `licensed` groups as appropriate. To exclude both groups, set `{"disable_sources": ["noncommercial", "licensed"]}`. Review source terms again before upgrading because group membership follows the current source registry.
- `BIOMCP_DEFAULT_ASSEMBLY`: pass the command's explicit `--build` when required.
- `BIOMCP_STUDY_DIR`, `BIOMCP_DDINTER_DIR`, `BIOMCP_EMA_DIR`, `BIOMCP_WHO_DIR`, `BIOMCP_CVX_DIR`, `BIOMCP_GTR_DIR`, `BIOMCP_GENCC_DIR`, `BIOMCP_WHO_IVD_DIR`: use supported `sync SOURCE` bundles. Unsupported bundles and local study analysis have no 1.0 replacement.
- `BIOMCP_FHIR_BASE`: patient reading is planned for 1.1.
- `BIOMCP_CA_BUNDLE` and its `SSL_CERT_FILE` fallback: set `ca_bundle` to add PEM roots to platform trust.
- `BIOMCP_VARIANT_QUERY_GENE_ROUTING`: use explicit variant selectors.
- `DISGENET_API_KEY`: the DisGeNET gene section retires.
- `RUST_LOG`: the 0.9 tracing filter has no 1.0 logging configuration equivalent.

The source credential names `ONCOKB_TOKEN`, `NCBI_API_KEY`, `S2_API_KEY`, `NCI_API_KEY`, `OPENFDA_API_KEY`, `ORCID_ACCESS_TOKEN` and `ALPHAGENOME_API_KEY` remain recognized. Use `biomcp key set SOURCE` to keep values out of shell history. Stored keys override environment values. Use JSON `format`, `disable_sources`, `keys`, `ncbi_email`, `ca_bundle`, `proxy` and `http.allowed_hosts` for the corresponding settings. Proxy environment variables remain supported. Every source starts enabled; commercial users must disable the `noncommercial` and `licensed` groups and review publisher terms.

[Configuration](../docs/configuration.md) also takes these keys and defaults:

- `sync_timeout_seconds`: how long one `sync` may take, 120 seconds by default. A sync that runs out of time names the limit and this key.
- `drug_label_bytes`: how much label text one `--label-section` reply returns, from 6144 through 32768 bytes.
- `transcript_fit_seconds`: how long `variant normalize` waits for each check of which transcript a mismatched reference base fits.
- `mcp_queue_wait_seconds` and `mcp_queue_max_calls`: how long, and on stdio how many, MCP calls over the call limit wait for a turn, instead of being refused at once. `0` turns waiting off. Plain `serve --http` now queues too, and its `http_max_connections` defaults to twice its call limit. A call that waits too long gets `error.data.retry_after_seconds`.
- `mcp_reply_form` accepts `once`, the new default for local servers: each reply carries its JSON one time, and a typed reply has no summary line. Hosted servers keep `both`.
- Source pacing defaults: openFDA allows 4 requests in flight at its unchanged rate, and keyless NCBI E-utilities calls pace at 5 requests per 2 seconds.

## Output migration

Every JSON reply advertises its schema identity. Read `result` for records, `sources` for each source's outcome and use (`help sources` lists each source's publisher terms and link), `problems` for typed failures, `status` for completeness, `page` for paging and `next` for argument arrays. A `next` step may also carry `options`, the flags its call also takes, each with a placeholder to fill in; a refusal's first step is the corrected call when one exists. Commands have their own typed result shapes; 0.9 fields are not a compatibility interface. Missing optional values stay absent. Source-native identifiers and assertions remain separate when sources disagree. Counts preserve their source scope and never establish clinical eligibility or an assayed denominator.

The 0.9 metadata and error fields change as follows:

| 0.9 field | 1.0 migration |
| --- | --- |
| Flattened entity fields at the reply root | Read the command's `result` and its typed item or section shape. Gene cards, for example, use `result.items[].answer.gene`; article and trial cards have different schemas. |
| `_meta.evidence_urls[].label`, `_meta.evidence_urls[].url` | Read record URLs and source provenance from the result's schema. There is no universal evidence URL list. |
| `_meta.next_commands[]` | Read `next[].args` arrays with typed actions; do not split shell command strings. |
| `_meta.suggestions[]` | Read explicit next actions and source problems. No generated suggestion text is a compatibility field. |
| `_meta.section_sources[]` | Read `sources[]` and the source-native identities in each result. |
| `_meta.source_status[]`, `_meta.warnings[]`, `_meta.notes[]` | Read typed `sources[]`, `problems[]` and `status` for source failures, warnings and stale data. The old message strings are not compatibility fields. |
| `_meta.workflow`, `_meta.workflow_rationale`, `_meta.workflow_playbook` | Removed with automatic workflow selection. |
| `_meta.discovery_sources[]` | Removed with discover. Use explicit entity and source queries. |
| `_meta.not_found` | Read the typed problem and status for the requested command. |
| `_meta.alias_resolution.kind`, `requested_entity`, `query`, `canonical`, `canonical_id`, `confidence`, `match_tier`, `sources`, `candidates` | Read each command's explicit resolution and candidates. The old confidence tiers and cross-entity alias inference retire. |
| `error.code`, `error.message`, `error.source`, `error.recovery`, `error.limit_bytes`, `error.limit`, `error.unit` | Read `problems[]` and returned next actions. Error codes and limit fields use the advertised command schema. |
| Variant `filter_evaluation`, `requested_variant`, `resolution` at the reply root | Read the variant result's checked request, source outcomes and native candidates. Do not reuse old top-level projections. |
| Preview `values`, `returned`, `total`, `has_more`, `omitted_oversized` | Read the section or page schema and follow its continuation. A preview count never supplies a population denominator. |

The seven 0.9 public entity schemas also change their entire root contract: gene, variant, article, trial, drug, disease and pathway records now sit inside command-specific envelopes. Their [1.0 schemas](https://github.com/genomoncology/biomcp/tree/main/crates/biomcp-types/schemas) define every field. Source-specific sections no longer flatten into those seven shared records. Rebuild integrations against their command schema rather than treating a field of the same name as the same contract.

The following 0.9 entity fields leave the reply root. Every nested path beneath these roots also changes. Read each command's `result` schema for the new nesting and source-specific values. Derived clinical summaries and local analysis fields have no replacement unless their source record appears in that schema.

| Entity | 0.9 root fields replaced |
| --- | --- |
| gene | `symbol`, `name`, `entrez_id`, `ensembl_id`, `location`, `genomic_coordinates`, `omim_id`, `uniprot_id`, `summary`, `gene_type`, `aliases`, `clinical_diseases`, `clinical_drugs`, `clingen`, `gencc` |
| variant | `id`, `genome_build`, `genome_build_provenance`, `gene`, `hgvs_p`, `legacy_name`, `hgvs_c`, `rsid`, `cosmic_id`, `significance`, `significance_source`, `significance_evaluated`, `significance_note`, `clinvar_id`, `clinvar_review_status`, `clinvar_review_stars`, `conditions`, `consequence`, `cadd_score`, `polyphen_pred`, `sift_pred`, `conservation`, `expanded_predictions`, `population`, `cosmic_context`, `cgi_associations`, `civic`, `clinvar_conditions`, `clinvar_condition_reports`, `top_disease`, `cancer_frequencies`, `cancer_frequency_source`, `supporting_pmids`, `prediction` |
| article | `pmid`, `pmcid`, `doi`, `title`, `authors`, `journal`, `date`, `citation_count`, `publication_type`, `open_access`, `abstract_text`, `annotations`, `pubtator_fallback`, `source`, `score`, `is_retracted` |
| trial | `nct_id`, `source`, `title`, `status`, `phase`, `study_type`, `conditions`, `interventions`, `arms`, `arm_intervention_assignments`, `sponsor`, `enrollment`, `summary`, `start_date`, `completion_date`, `eligibility` |
| drug | `name`, `drugbank_id`, `chembl_id`, `unii`, `drug_type`, `mechanism`, `mechanisms`, `approval_date`, `approval_date_raw`, `approval_date_display`, `approval_summary`, `brand_names`, `route`, `targets`, `target`, `indications`, `pharm_classes`, `top_adverse_events` |
| disease | `id`, `name`, `definition`, `synonyms`, `parents`, `top_genes`, `associated_genes`, `treatment_landscape`, `recruiting_trial_count`, `top_variant`, `xrefs` |
| pathway | `id`, `name`, `species`, `summary`, `genes`, `events`, `enrichment` |

Cache failures now distinguish `fetch.cache_encoding`, `fetch.cache_filesystem`, `fetch.cache_internal` and `fetch.cache_limit`. Their `CacheProblem` wire values are `encoding`, `filesystem`, `internal` and `limit`. Genuine SQLite failures keep `fetch.cache_database`; unreadable stored snapshots keep `fetch.cache_snapshot`. The former use of the database code for these other failures has no compatibility alias. Handle each typed code instead of treating every local failure as a database failure.

Identifier schemas now enforce the same bounds as their checked values. These exact restrictions also apply wherever a command embeds the identifier, including `VariantInput`.

| Identifier | Accepted numeric range |
| --- | --- |
| VCV version | 1 through 4,294,967,295 |
| RCV version | 1 through 4,294,967,295 |
| SCV version | 1 through 4,294,967,295 |
| COSMIC number | 1 through 18,446,744,073,709,551,615 |
| ClinVar variation number | 1 through 4,294,967,295 |
| ClinVar allele number | 1 through 4,294,967,295 |
| ClinVar branch of `VariantId` | 1 through 4,294,967,295 |
| OMIM number | Six digits with a first digit from 1 through 6 |

`VariantInput` applies the ClinVar variation bound to both bare numbers and `clinvar:` IDs. Other identifier namespaces keep their own bounds. Author IDs require `semantic-scholar:ID` or a checked `orcid:ID`; `semanticscholar:ID` is removed from `AuthorId`, embedded author records and selector schemas. Author errors now describe only the canonical ID spellings and pattern. Their error code stays the same. The `--source semanticscholar` source selector remains accepted.

ClinVar VCV, RCV and SCV equality and hashing include the version. Rust callers can use `same_accession` to compare accessions across versions. Variable-width accession output uses its canonical spelling, so `LRG_0001t01` becomes `LRG_1t1`. Fixed-width accessions such as `NM_000546` retain their zero padding.

The schema identity changes below compare this release's schemas with the preserved `release/migration-schema-baseline.tar.gz` history. The [schema directory](https://github.com/genomoncology/biomcp/tree/main/crates/biomcp-types/schemas) defines output fields and required values. In particular, adverse-event snapshot fields become optional when a cache write fails; callers must handle their absence. A major bump permits an intentional breaking schema change. The schema check does not prohibit all breaking changes.

Posted trial results are an explicit exception to the 1.0 command freeze. `get trial ID results` adds the ClinicalTrials.gov results section, including native group values, units, parameter and dispersion labels, analyses and adverse-event tables. The trial-cards reply schema adds `card.results_first_posted` and `sections.results`; the trial-get schema adds the `results` section. The card date is the first public posting date, not submission or last update. Values are reported source values, with no clinical calculations.

Replies also change in these ways:

- `sources[]` entries no longer carry `terms` and `terms_url`; `help sources` lists each source's terms and link. Reply schemas took a new major version for this.
- `--fields a,b` on an entity search, or a `fields` list beside `action` on a typed entity tool, keeps only those top-level fields of each row, plus the row's required and identifier fields and the whole envelope. `help search ENTITY` lists the names on a `Fields:` line, and the research `search` and `fetch` tools refuse it. Trial rows' `conditions`, `interventions` and `url` became optional so trials can narrow.
- One get returns several sections, and one drug read several label sections, in one reply; each part keeps its own paging and names its section. A several-sections drug reply, `biomcp.drug-sections/1`, states the drug's identity once.
- Every search and section page reports the source's total or says the source gives none, and a page cut short by a limit says so and offers the next page. Totals from several sources are never added together. JSON replies write the envelope's `page`, and each page's total and offset, before the rows.
- A section that is empty for a record the source holds is `complete` with an empty list; `absent` means the source does not hold the record.
- An empty, partial or failed reply says why, and a failed reply names its cause in `problems`. A source reply over the size limit fails as `too-large`, not `bad-reply`. An unknown pool or snapshot name is `usage.arguments`, no longer `pool.expired`. Offline, an expired cached not-found says current absence is unknown.
- MCP replies list each object's fields in the same order as CLI JSON.
- Markdown shows source text's full stops, colons and @ signs as plain characters, and a source value cannot become a list, code block or link. Source line breaks print as spaces, never as a literal `\u{000a}`; Markdown text fields keep real line breaks.
- `version --verbose` no longer lists schema versions; `biomcp schema` does.
- A refusal's first `next` step is the corrected call when one exists. Common mistyped forms, such as `-q`, `-c` or `-i`, `--section` where the section is a word after the ID, or a retired option, fail with an error naming the option and the working call. A typed call refused for an unknown argument gets the same fixes as the CLI. When several sources could fix a missing input, the refusal offers help instead of guessing a source.
- `get variant ID oncokb` no longer refuses the `oncokb` section word when `--build` or `--tumor-type` is missing. Without an OncoKB key the section is skipped as `no-key` with `help key oncokb` as the next step; with a key, missing context skips it with `help get variant` as the next step, as `all` already did.
- Help answers the underscore entity names that `help questions` prints, such as `help adverse_event`. `help get drug approvals` shows one section's help with only the options that section accepts. Generated command help uses the same plain words as the option details below it, "source" and "exact" rather than "native" and "literal". An option limited to some sources says `Only with --source X`, and each command-wide line appears once.

## Typed tool names for 0.x and hosted users

The typed tools of earlier builds, including the hosted server's tools, are gone. The reference tool is removed, the help tool is now `help`, and every other typed tool name is replaced by an entity tool and an action. Move a call's arguments into the object named after the action. A reference call that named the protein-get command with its arguments now calls `protein` with `{"action":"get","get":{"id":"P04637"}}`. A `biomcp`, `search` or `fetch` call is unchanged. Agents that cache tool names must refresh their tool list. This table maps each old name to its new tool and action. Where one old command appears twice, both tools carry it.

<!-- retired-tools:start -->
| Old tool or reference command | New tool | Action |
| --- | --- | --- |
| `biomcp_reference` | the entity tool for the thing | the action in the rows below |
| `biomcp_help` | `help` | none |
| `search_gene` | `gene` | `search` |
| `get_gene` | `gene` | `get` |
| `search_pgx` | `gene` | `pgx` |
| `get_pgx` | `gene` | `pgx_get` |
| `search_gwas` | `gene` | `gwas` |
| `search_variant` | `variant` | `search` |
| `get_variant` | `variant` | `get` |
| `variant_normalize` | `variant` | `normalize` |
| `variant_assertions` | `variant` | `assertions` |
| `variant_population` | `variant` | `population` |
| `variant_recurrence` | `variant` | `recurrence` |
| `search_gwas` | `variant` | `gwas` |
| `get_gwas` | `variant` | `gwas_get` |
| `search_disease` | `disease` | `search` |
| `get_disease` | `disease` | `get` |
| `search_gwas` | `disease` | `gwas` |
| `search_drug` | `drug` | `search` |
| `get_drug` | `drug` | `get` |
| `get_pgx` | `drug` | `pgx_get` |
| `search_trial` | `trial` | `search` |
| `get_trial` | `trial` | `get` |
| `trial_document` | `trial` | `document` |
| `search_organization` | `trial` | `organization_search` |
| `search_biomarker` | `trial` | `biomarker_search` |
| `get_biomarker` | `trial` | `biomarker_get` |
| `search_article` | `article` | `search` |
| `get_article` | `article` | `get` |
| `article_text` | `article` | `text` |
| `article_pdf` | `article` | `pdf` |
| `article_supplement` | `article` | `supplement` |
| `article_citations` | `article` | `citations` |
| `article_references` | `article` | `references` |
| `article_citation_evidence` | `article` | `citation_evidence` |
| `article_recommendations` | `article` | `recommendations` |
| `search_author` | `article` | `author_search` |
| `get_author` | `article` | `author_get` |
| `author_papers` | `article` | `author_papers` |
| `search_protein` | `protein` | `search` |
| `get_protein` | `protein` | `get` |
| `protein_features` | `protein` | `features` |
| `protein_isoforms` | `protein` | `isoforms` |
| `protein_domains` | `protein` | `domains` |
| `protein_complexes` | `protein` | `complexes` |
| `protein_complex_members` | `protein` | `complex_members` |
| `protein_structures` | `protein` | `structures` |
| `protein_sequence` | `protein` | `sequence` |
| `search_pathway` | `pathway` | `search` |
| `get_pathway` | `pathway` | `get` |
| `pathway_genes` | `pathway` | `genes` |
| `pathway_events` | `pathway` | `events` |
| `search_phenotype` | `phenotype` | `search` |
| `get_phenotype` | `phenotype` | `get` |
| `search_adverse_event` | `adverse_event` | `search` |
| `get_adverse_event` | `adverse_event` | `get` |
| `search_diagnostic` | `diagnostic` | `search` |
| `get_diagnostic` | `diagnostic` | `get` |
| `search_cell_line` | `cell_line` | `search` |
| `get_cell_line` | `cell_line` | `get` |
| `cell_line_assay_count` | `cell_line` | `assays` |
| `search_study` | `study` | `search` |
| `get_study` | `study` | `get` |
| `search_gwas` | `study` | `gwas` |
| `get_gwas` | `study` | `gwas_get` |
<!-- retired-tools:end -->
