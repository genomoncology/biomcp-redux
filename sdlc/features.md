gene-identity: A caller resolves approved symbols, aliases, previous symbols and identifiers without guessing between ambiguous candidates. Cases: `tests/cases/gene/*.yaml`, `crates/biomcp/tests/runner/replay.rs`.
gene-search: A caller searches genes by words, chromosome band and locus group with totals and bounded pages. Cases: `tests/cases/gene/search*.yaml`, `tests/cases/gene/band.yaml`.
gene-bundle: A caller synchronizes the HGNC bundle and reads gene identity offline. Cases: `tests/cases/gene/hgnc-sync.yaml`, `tests/cases/gene/offline.yaml`.
gene-help: A caller discovers gene arguments, section names and executable examples through help. Cases: `tests/cases/gene/help.yaml`.
gene-sections: A caller requests named gene sections separately or together and sees each section's source and availability. Cases: `tests/cases/gene/sections/*.yaml`.
gene-protein: A caller sees every reviewed human protein linked by the gene's identifiers. Cases: `tests/cases/gene/sections/protein-*.yaml`.
gene-pathways: A caller sees Reactome pathways linked to the gene. Cases: `tests/cases/gene/sections/pathways-*.yaml`.
gene-expression: A caller sees source-stated tissue expression and subcellular locations for the gene. Cases: `tests/cases/gene/sections/expression-*.yaml`.
gene-civic: A caller sees CIViC gene descriptions and counts with checked gene identity. Cases: `tests/cases/gene/sections/civic-*.yaml`.
gene-clingen: A caller sees ClinGen validity and dosage records from a synchronized local bundle. Cases: `tests/cases/gene/sections/clingen-*.yaml`.
gene-constraint: A caller sees population constraint metrics with the source version. Cases: `tests/cases/gene/sections/constraint.yaml`.
gene-oncokb: A caller sees OncoKB gene roles with their own key or a skipped-source explanation without one. Cases: `tests/cases/gene/sections/oncokb-*.yaml`.
gene-retired: A caller using a retired gene spelling receives its fixed replacement or removal explanation. Cases: `tests/cases/gene/retired.yaml`.
article-search: A caller walks stable native source windows with totals, checked selectors, optional source keys and explicit failures, and chooses explicit term drops for a short complete pool. Cases: `tests/cases/article/search/*.yaml`, `tests/cases/article/fold/*.yaml`, `tests/cases/article/semantic-scholar/*.yaml`, `crates/biomcp/tests/runner/scholar.rs`.
article-filters: A caller narrows article results by author, journal, dates and publication type before paging, and sees a distinct electronic publication date. Cases: `tests/cases/article/filters/*.yaml`.
article-fold: A caller combines PubMed and Europe PMC results while retaining each source's native rank and missing-source outcomes. Cases: `tests/cases/article/fold/*.yaml`.
article-litvar: A caller selects an exact variant, receives source candidates for ambiguity and combines its publications with other article sources in a stable pool. Cases: `tests/cases/article/litvar/*.yaml`, `crates/biomcp/tests/runner/litvar_tables.rs`.
article-get: A caller retrieves article cards by PMID, PMCID or checked DOI with complete abstracts and retraction links. Cases: `tests/cases/article/get/*.yaml`.
article-sections: A caller requests indexing and normalized entity annotations while preserving the article card when a source fails. Cases: `tests/cases/article/sections/*.yaml`, `tests/cases/article/indexing/*.yaml`.
article-citations: A caller sorts by iCite citation counts and walks citing or referenced papers with explicit missing counts. Cases: `tests/cases/article/citations/*.yaml`.
article-fulltext: A caller reads available full text through semantic parts, tables, outlines and checked continuations. Cases: `tests/cases/article/fulltext/*.yaml`.
article-retired: A caller using a retired article spelling receives its fixed replacement or removal explanation. Cases: `tests/cases/article/retired.yaml`.
mcp-parity: A caller sends the CLI argument array through the MCP tool, receives the same command result and validates structured replies against each advertised schema. Cases: `tests/cases/mcp/*.yaml`, `crates/biomcp/tests/runner/protocol_schemas.rs`, `crates/biomcp/tests/runner/research.rs`.
schema-discovery: A caller lists versioned schemas with descriptions, discovers resolvable command reply contracts and scoped help aliases, and follows safe guidance for invalid schema requests and unknown commands. Cases: `tests/cases/schema/*.yaml`, `crates/biomcp/tests/runner/schema_proof.rs`, `crates/biomcp/tests/runner/help_entry.rs`.
corporate-network: A caller adds corporate certificate authorities, follows standard proxy settings and receives private configuration guidance for network failures. Cases: `crates/biomcp/tests/runner/corporate.rs`.
markdown-view: A caller reads gene and article results as Markdown, copies next commands, and keeps JSON for MCP. Cases: `crates/biomcp/tests/runner/markdown_shapes.rs`.
article-semantic-sections: Read machine-written summaries and source-ordered citation edges with contexts from Semantic Scholar. Cases: `tests/cases/article/tldr/*.yaml`, `tests/cases/article/edges/*.yaml`.
source-controls: Source terms, disabled sources and bounded source health preserve policy skips before cached reads or requests. Cases: `crates/biomcp/tests/runner/source_controls.rs`, `crates/biomcp/tests/runner/health_bundles.rs`, `crates/biomcp/tests/runner/store_sizes.rs`, `crates/biomcp/tests/runner/drug_registry.rs`, `crates/biomcp/tests/runner/adverse_event_registry.rs`, `crates/biomcp/tests/runner/diagnostic_registry.rs`, `crates/biomcp/tests/runner/pgx_registry.rs`.
live-source-checks: A separate keyless live diagnostic reports upstream drift without changing replay assertions or recordings. Cases: `crates/biomcp/tests/runner/live_proof.rs`, `crates/biomcp/tests/runner/live_follow_proof.rs`, `crates/biomcp/tests/runner/live_witness_proof.rs`.
article-named-search: A caller searches sentence resemblance and PubTator entities by name while retaining native order, matched sentences and explicit unsupported filters. Cases: `tests/cases/article/litsense/*.yaml`, `tests/cases/article/pubtator/*.yaml`, `crates/biomcp/tests/runner/named_search.rs`.
article-supplements: List source-confirmed files and read bounded spreadsheet and text rows through CLI and MCP. Cases: `tests/cases/article/supplements/*.yaml`, `crates/biomcp/tests/runner/supplement_tables.rs`.
article-preprints: A caller retrieves preprint cards and licensed full text by Europe PMC ID. Cases: `tests/cases/article/preprint/*.yaml`.
article-help: A caller discovers article arguments, sections and executable examples through generated help. Cases: `tests/cases/article/help.yaml`, `crates/biomcp/tests/runner/help_examples.rs`.
mcp-http: A native client sends independent MCP requests over bounded HTTP and checks local storage health. Cases: `crates/biomcp/tests/runner/http.rs`, `crates/biomcp/tests/runner/http_limits.rs`, `crates/biomcp/tests/runner/mcp_boundaries.rs`.
article-evidence: Read all source citation contexts and explicit full-text reference paragraphs through checked continuations. Cases: `tests/cases/article/evidence/*.yaml`, `crates/biomcp/tests/runner/evidence.rs`.
article-recommendations: Read Semantic Scholar recommendations in source order with stable pages and explicit failures. Cases: `tests/cases/article/recommendations/*.yaml`.

article-saves: Save whole article Markdown or original supplement bytes only to an explicit caller destination. Cases: `crates/biomcp/tests/runner/article_save.rs`, `crates/biomcp/tests/runner/mcp_boundaries.rs`.
article-pdf: Inspect publisher PDF links and download only to an explicit destination through CLI or stdio MCP. Cases: `tests/cases/article/pdf/*.yaml`, `crates/biomcp/tests/runner/pdf.rs`.
key-registration: Register, replace, list and remove local source credentials without exposing values, and reload them on the next server call. The private settings cases in `crates/biomcp/src/config/deferred_keys_tests.rs` execute parsed commands with observed environment reads and remain in the excluded binary-test collection. Cases: `crates/biomcp/tests/runner/key_registration.rs`, `crates/biomcp/tests/runner/key_reload.rs`, `crates/biomcp/tests/runner/openfda_key.rs`, `crates/biomcp/tests/runner/openfda_website.rs`, `crates/biomcp/tests/runner/key_help.rs`.

variant-identity: A caller resolves published variant identifiers, keeps every source candidate, and sees explicit source rejections without local allele repair. Cases: `tests/cases/variant/*.yaml`, `crates/biomcp/tests/runner/variant.rs`, `crates/biomcp/tests/runner/variant_transcripts.rs`, `crates/biomcp/tests/runner/next_navigation.rs`.
trial-search: Search the registry with repeated condition, intervention and keyword alternatives joined with OR, explicit native filters and exact MyDisease condition resolution, exact totals, opaque pages, factual condition-word mismatch notices, explicit registered-condition phrase restrictions and caller-controlled wider alternatives for short searches. Cases: `tests/cases/trial/*.yaml`, `crates/biomcp/tests/runner/trial_bad_values.rs`, `crates/biomcp/tests/runner/trial_tables.rs`, `crates/biomcp/tests/runner/trial_wider.rs`, `crates/biomcp/tests/runner/trial_condition.rs`, `crates/biomcp/tests/runner/trial_condition/alternatives.rs`.
trial-get: Retrieve exact registry cards and recover intact eligibility, outcome and arm records through bounded continuations. Cases: `tests/cases/trial/get/*.yaml`, `crates/biomcp/tests/runner/trial_get.rs`, `crates/biomcp/tests/runner/trial_bad_values.rs`.

trial-nci: Explicit NCI trial search and exact retrieval require the user key and preserve native fields without registry fallback; exact facility search returns one matched site and its match count; exact biomarker code search retains native criterion and inclusion_indicator entries and reports same-entry mismatches. Cases: `tests/cases/trial/nci-*.yaml`, `crates/biomcp/tests/runner/nci.rs`, `crates/biomcp/tests/runner/nci/facility.rs`, `crates/biomcp/tests/runner/nci/trial_biomarker.rs`, `crates/biomcp/tests/runner/nci_walk.rs`, `crates/biomcp/tests/runner/trial_condition/alternatives.rs`.
trial-publications: Retrieve native registry references and document manifests and save a listed document to an explicit caller path. Cases: `tests/cases/trial/references.yaml`, `tests/cases/trial/documents.yaml`, `crates/biomcp/tests/runner/trial_documents.rs`.
trial-sites: Page original registry sites and their attached contacts, and retrieve requested-trial central contacts and officials. Cases: `tests/cases/trial/sites/*.yaml`, `crates/biomcp/tests/runner/trial_sites.rs`.

variant-population: A caller reads source-given population counts and current catalog associations, keeps missing datasets distinct from zero, and pages ancestry without inferred frequencies or clinical meaning. Cases: `tests/cases/variant/population/*.yaml`, `crates/biomcp/tests/runner/variant_population.rs`.
variant-search: Search explicit gene/protein, assembly-qualified regions or raw MyVariant queries with proved literal source filters, preserve native candidates, source transcript notation and matches, and pages, and honor component terms switches. Cases: `tests/cases/variant/search*.yaml`, `tests/cases/variant/transcripts.yaml`, `tests/cases/variant/filters/*.yaml`, `crates/biomcp/tests/runner/variant_search.rs`, `crates/biomcp/tests/runner/variant_transcripts.rs`, `crates/biomcp/tests/runner/variant_filters.rs`, `crates/biomcp/tests/runner/next_navigation.rs`.
trial-help: Discover trial commands through schema-generated help and executable case examples. Cases: `tests/cases/trial/help.yaml`.

variant-recurrence: Retrieve bounded exact-allele sample observations in an explicitly selected cBioPortal cohort without inferring a denominator. Cases: `tests/cases/variant/recurrence.yaml`, `crates/biomcp/tests/runner/recurrence.rs`.
variant-normalization: A caller sends unchanged HGVS to named services and receives independent source mappings, warnings and versions. Cases: `tests/cases/variant/normalization.yaml`, `crates/biomcp/tests/runner/normalization.rs`.

variant-assertions: Retrieve source-native ClinVar and ClinGen assertions without merging classification domains or mistaking source absence for unknown allele identity. Cases: `crates/biomcp/tests/runner/variant.rs`, `crates/biomcp/tests/runner/variant_assertions.rs`, `crates/biomcp/tests/runner/clinvar_key.rs`.
variant-civic: Preserve CIViC exact profile membership and separate native claim pages. Cases: `crates/biomcp/tests/runner/variant_civic.rs`.
variant-hotspots: Retain native transcript and residue counts without an inferred variant join. Cases: `crates/biomcp/tests/runner/variant_hotspots.rs`.

variant-oncokb: Retrieve only user-keyed OncoKB annotations with explicit assembly and tumor context, preserving source protein claims and native implication groups. Cases: `crates/biomcp/tests/runner/oncokb.rs`.

variant-help: Variant help gives executable examples and explicit migration steps without clinical interpretation. Cases: `tests/cases/variant/help*.yaml`, `tests/cases/variant/retired.yaml`.

drug-types: A public API caller validates published drug identifiers, selected-source requests, neutral records and source-preserving continuations before command adapters exist. Contracts: `biomcp-types/drug_contracts` (features: `schema`).

drug-native-sources: Retrieve native ChEMBL and Open Targets drug candidates, identities, mechanisms and indications without inferring authorization. Cases: `crates/biomcp/tests/runner/drug_native.rs`, `crates/biomcp/tests/runner/drug_native_identity.rs`, `tests/cases/drug/native-policy.yaml`.
drug-ema: Explicitly synchronize EMA medicines and read separate native candidates, exact identity cards and source-scoped regulatory dates without implicit synchronization. Cases: `tests/cases/drug/ema*.yaml`, `crates/biomcp/tests/runner/drug_ema.rs`, `crates/biomcp/tests/runner/drug_ema/navigation.rs`.
drug-civic: Search literal CIViC therapy candidates and retrieve independent native evidence and assertion pages with their complete profile context. Cases: `crates/biomcp/tests/runner/drug_civic.rs`.
drug-mychem: Search literal MyChem candidate pages and retrieve exact native document identities with attributed cross-references. Cases: `tests/cases/drug/disabled.yaml`, `crates/biomcp/tests/runner/drug_mychem.rs`, `crates/biomcp/tests/runner/next_navigation.rs`.
disease-types: A public API caller validates disease identifiers, source-selected requests, native association provenance and statistics without hidden enrichment before command adapters exist. Contracts: `biomcp-types/disease_contracts` (features: `schema`).

drug-label: Find complete brand or generic names with an explicit match basis and read exact SPL identities and native label fields with complete bounded text continuation, preserving revision, original text and source outcomes. Cases: `crates/biomcp/tests/runner/drug_label.rs`.
drug-approvals: Read source-native application candidates, independent product status and application submission history with explicit product selection. Cases: `crates/biomcp/tests/runner/drug_fda.rs`, `crates/biomcp/tests/runner/drug_fda/errors.rs`, `crates/biomcp/tests/runner/drug_fda/capture.rs`.
disease-ols: Discover source-ordered ontology candidates, read exact OLS facts and refuse retired disease inputs before source access. Cases: `tests/cases/disease/*.yaml`, `crates/biomcp/tests/runner/disease_ols.rs`, `crates/biomcp/tests/runner/disease_ols/native.rs`.
disease-mydisease: Read an exact selected annotation and independently page native synonyms, cross references and parents without hidden enrichment. Cases: `crates/biomcp/tests/runner/disease_mydisease.rs`, `crates/biomcp/tests/runner/disease_mydisease/cases.rs`, `crates/biomcp/tests/runner/disease_mydisease/native.rs`.

disease-seer: Select native SEER catalog sites and page stage incidence or annual observed and modeled survival without inferred population or dataset facts. Cases: `crates/biomcp/tests/runner/disease_seer.rs`, `crates/biomcp/tests/runner/disease_seer/failures.rs`, `crates/biomcp/tests/runner/disease_seer/native.rs`.

drug-ddinter: Explicit synchronization replaces all eight native pair files atomically, and stored-only search and interaction pages retain native identity, level, order, provenance and content revision. Cases: `tests/cases/drug/ddinter.yaml`, `crates/biomcp/tests/runner/drug_ddinter.rs`.
disease-monarch: Read native gene and phenotype association rows with their endpoints, evidence, provenance and component terms, without causal inference. Cases: `crates/biomcp/tests/runner/disease_monarch.rs`.

drug-next-steps: Follow source-stated variant, build and tumor context from drug claims to keyed variant annotation, with explicit missing context and no drug-name inference. Cases: `crates/biomcp/tests/runner/drug_next.rs`, `crates/biomcp/tests/runner/drug_next/native.rs`.
drug-retirements: Retired and unproved drug commands return fixed migration guidance before requests. Cases: `crates/biomcp/tests/runner/drug_retirements.rs`.
drug-failures: Every kept drug source reports synthetic transport and broken-body failures with publisher terms. Cases: `crates/biomcp/tests/runner/drug_failures.rs`.
drug-identity: Drug sources reject mismatched, conflicting and absent identity joins before cache admission or product selection. Cases: `crates/biomcp/tests/runner/drug_identity.rs`, `crates/biomcp/tests/runner/drug_mychem.rs`, `crates/biomcp/tests/runner/drug_native_identity.rs`, `crates/biomcp/tests/runner/drug_fda/errors.rs`, `crates/biomcp/tests/runner/drug_label.rs`.

drug-help: Discover source-scoped drug sections, selectors and explicit bundle synchronization through generated help and executable case examples. Cases: `tests/cases/drug/help.yaml`.

adverse-event-types: A public API caller validates source-specific report requests and preserves independent native arrays, code strings and bounded consumer schemas before adapters exist. Contracts: `biomcp-types/adverse_event_contracts` (features: `schema`).

pgx-types: A public API caller preserves source-native PGx context and validates explicit source and section requests before adapters exist. Contracts: `biomcp-types/pgx_contracts` (features: `schema`).
diagnostic-types: A public API caller validates source-owned diagnostic identities, neutral records and standalone consumer schemas before adapters exist. Contracts: `biomcp-types/diagnostic_contracts` (features: `schema`).

diagnostic-gtr: Explicitly synchronize the complete GTR pair and read exact native test candidates, cards and relation pages without implicit synchronization or clinical inference. Cases: `crates/biomcp/tests/runner/diagnostic_gtr.rs`, `crates/biomcp/tests/runner/diagnostic_gtr/mechanics.rs`, `tests/cases/diagnostic/gtr.yaml`.
adverse-event-search: Search literal FAERS report mentions with typed phrase, seriousness and receipt selectors, native report pages and exact next requests. Cases: `tests/cases/adverse-event/help.yaml`, `tests/cases/adverse-event/examples.yaml`, `tests/cases/adverse-event/search-policy.yaml`, `tests/cases/adverse-event/serious-any.yaml`, `tests/cases/adverse-event/serious-flags.yaml`, `crates/biomcp/tests/runner/faers_search.rs`.

cpic-pgx: Read explicit CPIC gene pairs, identity, native recommendations, guideline references and declared diplotype mappings without clinical inference. Cases: `tests/cases/pgx/*.yaml`, `crates/biomcp/tests/runner/cpic.rs`.
maude-reports: Search native device phrases, manufacturer phrases, exact product codes and product problem phrases and read exact native cards, device rows and complete narratives with source terms and independent count units. Cases: `crates/biomcp/tests/runner/maude.rs`, `crates/biomcp/tests/runner/maude/filters.rs`, `tests/cases/adverse-event/maude-filters.yaml`.
adverse-event-get: Read one exact FAERS report and independently page retained drug, reaction and original identifier arrays without causal or product identity inference. Cases: `crates/biomcp/tests/runner/faers_get.rs`, `tests/cases/adverse-event/get-examples.yaml`.

diagnostic-cdx: Explicitly synchronize both FDA companion tables and search whole physical association rows using their own checked identifiers. Cases: `crates/biomcp/tests/runner/diagnostic_cdx.rs`, `tests/cases/diagnostic/fda-cdx.yaml`, `tests/cases/diagnostic/help.yaml`.
adverse-event-retirements: Retired adverse-event inputs return fixed guidance before requests. Cases: `crates/biomcp/tests/runner/adverse_event_retirements.rs`.
adverse-event-failures: FAERS and MAUDE failures preserve source identity and terms instead of inventing empty reports. Cases: `crates/biomcp/tests/runner/adverse_event_failures.rs`.

clinpgx-annotations: Read independent bounded ClinPGx annotation collections and exact base records without clinical inference. Cases: `crates/biomcp/tests/runner/clinpgx.rs`.
diagnostic-who: Explicitly synchronize the unfiltered WHO export and read every native row for an exact known product without implicit synchronization or inferred authorization. Cases: `crates/biomcp/tests/runner/diagnostic_who.rs`, `crates/biomcp/tests/runner/diagnostic_who/mechanics.rs`, `tests/cases/diagnostic/who.yaml`, `tests/cases/diagnostic/who-help.yaml`.

diagnostic-pma: Read an exact native FDA base or supplement and page its own regulatory statement and supplement headers. Cases: `tests/cases/diagnostic/pma.yaml`, `tests/cases/diagnostic/pma-failures.yaml`, `crates/biomcp/tests/runner/diagnostic_pma.rs`, `crates/biomcp/tests/runner/diagnostic_pma/walks.rs`, `crates/biomcp/tests/runner/diagnostic_pma/metadata.rs`.

pgx-retirements: Refuse retired PGx spellings and unsupported source pairings before requests without rewriting queries or inferring phase. Cases: `crates/biomcp/tests/runner/pgx_retirements.rs`.
diagnostic-510k: Retrieve one exact native FDA clearance with source-owned identity, native decision fields and companion-table navigation. Cases: `crates/biomcp/tests/runner/diagnostic_510k.rs`, `tests/cases/diagnostic/510k-failures.yaml`.

diagnostic-retirements: Retired diagnostic spellings return fixed supported routes before any source request. Cases: `crates/biomcp/tests/runner/diagnostic_retirements.rs`, `crates/biomcp/tests/runner/diagnostic_bundles.rs`.

drug-who-medicines: Explicit WHO finished-product sync publishes a validated local bundle; explicit source reads preserve native prequalification rows without downloading. Cases: `tests/cases/drug/who-missing.yaml`, `tests/cases/drug/who-sync.yaml`, `crates/biomcp/tests/runner/drug_who.rs`.

vaers-summary: Read explicit supported vaccine aggregate tables without inferring individual risks or silently changing sources. Cases: `crates/biomcp/tests/runner/vaers.rs`.
diagnostic-who-search: Search stored WHO product headers by literal marker and manufacturer words. Cases: `tests/cases/diagnostic/who-search.yaml`, `crates/biomcp/tests/runner/diagnostic_who/search.rs`.

mcp-research: Search citable source headers and fetch the same CLI record through research compatibility tools. Cases: `crates/biomcp/tests/runner/research.rs`.

study-types: A public API caller preserves exact study identities, source-scoped count metadata and independent acquisition times through bounded standalone study contracts. Contracts: `biomcp-types/study_contracts` (features: `schema`).
store-clean: Publishing removes bounded expired pools and snapshots; ordinary reply writes prune expired replies; explicit cleanup removes expired state and obsolete owned database files with their sidecars while preserving supported generations, bundle tables in obsolete files, fresh state and foreign tables. Cases: `crates/biomcp/tests/runner/store_clean.rs`, `crates/biomcp/tests/runner/store_coexistence.rs`, `tests/cases/store/clean.yaml`.
gwas-types: A public API caller validates Catalog identities, exact scientific probabilities, native association context and standalone source-scoped consumer contracts before adapters exist. Contracts: `biomcp-types/gwas_contracts` (features: `schema`).

study-catalog: Explicit catalog sync preserves checked metadata atomically; literal stored study searches retain selectors and source order. Cases: `crates/biomcp/tests/runner/study_catalog.rs`, `tests/cases/study/*.yaml`.
phenotype-types: A public API caller preserves native phenotype fields and validates HP identities and source section pairs before adapters exist. Contracts: `biomcp-types/phenotype_contracts` (features: `schema`).
pathway-types: A public API caller validates source-owned pathway identities and selectors, preserves native references and distinguishes source totals from observed relation counts before adapters exist. Contracts: `biomcp-types/pathway_contracts` (features: `schema`).
protein-types: Neutral protein identities, source records and checked standalone requests preserve source scope before adapters exist. Contracts: `biomcp-types/protein_contracts` (features: `schema`).

study-get: Exact stored study cards expose bounded independent profile and sample-list metadata without participant identifiers. Cases: `crates/biomcp/tests/runner/study_get.rs`.
gwas-search: Search source-native association selectors in native p-value order, with exact positive thresholds and retained page walks that state a qualifying total only at the end. Cases: `tests/cases/gwas/examples.yaml`, `tests/cases/gwas/help.yaml`, `crates/biomcp/tests/runner/gwas_search.rs`.
protein-search: A caller searches reviewed human proteins and walks retained source windows without guessing identities. Cases: `tests/cases/protein/*.yaml`, `crates/biomcp/tests/runner/protein_commands.rs`.

phenotype-identity: A caller searches literal HP term candidates and reads independent exact HPO or Monarch cards without guessing or merging. Cases: `tests/cases/phenotype/*.yaml`, `crates/biomcp/tests/runner/phenotype/mod.rs`, `crates/biomcp/tests/runner/phenotype/faults.rs`, `crates/biomcp/tests/runner/phenotype/states.rs`.
pathway-reactome: Discover native human pathway headers and exact Reactome cards without inferred membership or replacement identities. Cases: `tests/cases/pathway-sections/*.yaml`, `crates/biomcp/tests/runner/pathway_sections.rs`, `tests/cases/pathway/card-R-HSA-9652817.yaml`, `tests/cases/pathway/card-R-HSA-6802952.yaml`, `tests/cases/pathway/card-R-HSA-177929.yaml`, `tests/cases/pathway/card-R-HSA-2028269.yaml`, `tests/cases/pathway/card-R-HSA-201556.yaml`, `tests/cases/pathway/card-R-HSA-187037.yaml`, `tests/cases/pathway/card-R-HSA-73894.yaml`, `tests/cases/pathway/card-R-HSA-5685942.yaml`, `tests/cases/pathway/card-R-HSA-5358508.yaml`, `tests/cases/pathway/card-R-HSA-6803211.yaml`, `tests/cases/pathway/card-R-HSA-109581.yaml`, `tests/cases/pathway/card-R-HSA-1640170.yaml`, `tests/cases/pathway/card-R-HSA-195721.yaml`, `tests/cases/pathway/card-R-HSA-157118.yaml`, `tests/cases/pathway/card-R-HSA-5358351.yaml`, `tests/cases/pathway/card-R-HSA-8950505.yaml`, `tests/cases/pathway/card-R-HSA-9612973.yaml`, `tests/cases/pathway/card-R-HSA-74751.yaml`, `tests/cases/pathway/card-R-HSA-913531.yaml`, `tests/cases/pathway/card-R-HSA-5674400.yaml`, `tests/cases/pathway/card-R-HSA-2219528.yaml`, `tests/cases/pathway/card-R-HSA-1227990.yaml`, `tests/cases/pathway/card-R-HSA-9755088.yaml`, `tests/cases/pathway/cards.yaml`, `tests/cases/pathway/search-0.yaml`, `tests/cases/pathway/search-1.yaml`, `tests/cases/pathway/search-2.yaml`, `tests/cases/pathway/examples.yaml`, `tests/cases/pathway/failure-429.yaml`, `tests/cases/pathway/failure-500.yaml`, `tests/cases/pathway/failure-timeout.yaml`, `tests/cases/pathway/failure-broken.yaml`, `tests/cases/pathway/failure-generic404.yaml`, `tests/cases/pathway/failure-foreign404.yaml`, `tests/cases/pathway/failure-mismatch.yaml`, `tests/cases/pathway/failure-mixed.yaml`, `tests/cases/pathway/top-level.yaml`, `tests/cases/pathway/gene.yaml`, `tests/cases/pathway/summary.yaml`, `tests/cases/pathway/native-page.yaml`, `tests/cases/pathway/validation.yaml`, `crates/biomcp/tests/runner/pathway_commands.rs`, `tests/cases/pathway/failure-zero200.yaml`, `tests/cases/pathway/failure-admission-empty.yaml`.

pathway-wikipathways: Explicitly synchronize both metadata catalogs atomically and read source-owned human pathway identities and literal-title matches without network access. Cases: `tests/cases/pathway/wikipathways-missing.yaml`, `tests/cases/pathway/sync-examples.yaml`, `crates/biomcp/tests/runner/wikipathways.rs`.

study-files: Read bounded native DataHub file references and study license links without fetching data files. Cases: `crates/biomcp/tests/runner/study_files.rs`.
phenotype-sections: A caller pages native source synonyms, cross references and direct hierarchy links from one retained term reply. Cases: `crates/biomcp/tests/runner/phenotype/sections.rs`, `crates/biomcp/tests/runner/phenotype/section_failures.rs`.

pathway-wikipathways-genes: Read exact human pathway GeneProduct data nodes from bounded publisher GPML and page the retained relation without name-based identifier inference. Cases: `tests/cases/pathway/wikipathways-genes-missing.yaml`, `crates/biomcp/tests/runner/wikipathways_genes.rs`.
phenotype-associations: A caller pages exact direct Monarch disease and human gene association rows without clinical inference or identity repair. Cases: `crates/biomcp/tests/runner/phenotype/associations.rs`.
gwas-get: Read exact Catalog association and study records and their independent native context sections. Cases: `tests/cases/gwas/get-examples.yaml`, `crates/biomcp/tests/runner/gwas_get.rs`.
pathway-retirements: Retired pathway spellings return fixed supported routes before any source request. Cases: `crates/biomcp/tests/runner/pathway_retirements.rs`.

pathway-failures: Source faults and foreign identities retain failed or rejected outcomes without cache admission. Cases: `crates/biomcp/tests/runner/pathway_failures.rs`.
study-retirements: Legacy study calculations, charts and local directories return fixed supported routes before source access. Cases: `crates/biomcp/tests/runner/study_retirements.rs`.
study-failures: Synthetic catalog and study section faults fail explicitly and preserve accepted stored metadata. Cases: `crates/biomcp/tests/runner/study_failures.rs`.
protein-sections: A caller reads native protein features, alternative products, references, complete sequences and qualified function text without transferring coordinates. Cases: `tests/cases/protein/sections/*.yaml`, `crates/biomcp/tests/runner/protein_sections.rs`.


protein-domains-complexes: Read source-native InterPro matches and exact-participant Complex Portal pages without inferred membership. Cases: `crates/biomcp/tests/runner/protein_domains.rs`.

pathway-help: Discover every pathway source, selector, section and paging option through generated help and parsed case examples. Cases: `tests/cases/pathway/help.yaml`.

protein-docs-accounting: Discover every protein command, native section and selector through generated help and parsed CLI/MCP examples. Cases: `tests/cases/protein/help.yaml`, `tests/cases/protein/examples.yaml`.

cell-line-types: A public API caller validates exact cell identities and standalone requests while preserving native variation members, provenance and independently scoped collections before I/O. Contracts: `biomcp-types/cell_line_contracts` (features: `schema`).

cell-line-identity: A caller searches literal Cellosaurus headers and retrieves exact accession or verified source-qualified cell identities. Cases: `tests/cases/cell-line/*.yaml`, `crates/biomcp/tests/runner/cell_line/*.rs`.

cell-line-chembl: A caller reads exact accession-linked ChEMBL metadata and requests native assay totals separately, with source terms and independent release provenance. Cases: `crates/biomcp/tests/runner/cell_line/chembl.rs`, `crates/biomcp/tests/runner/cell_line/chembl/*.rs`.

cell-line-sections: A caller pages complete native Cellosaurus names, identities, cross-references and variation rows without inference. Cases: `crates/biomcp/tests/runner/cell_line/sections.rs`.

author-types: A public API caller checks source-qualified author records and native paper and work-group contracts without inferring person identity. Contracts: `biomcp-types/author_contracts` (features: `schema`).

cell-line-all: A caller receives every bounded first section page with independent source failures, skips and provenance, while deferred routes return fixed guidance before provider requests. Cases: `crates/biomcp/tests/runner/cell_line/all.rs`, `crates/biomcp/tests/runner/cell_line/all/*.rs`.

author-search: Literal Semantic Scholar author candidates retain provider identities and native order. Cases: `tests/cases/author/*.yaml`, `crates/biomcp/tests/runner/author/*.rs`.
author-orcid: Read exact public ORCID names and locally page whole public work groups without identity joins. Cases: `tests/cases/orcid/*.yaml`, `crates/biomcp/tests/runner/orcid.rs`.

author-papers: Exact Semantic Scholar author papers preserve native order, IDs and details. Cases: `tests/cases/author/papers*.yaml`.
cache-resilience: Checked provider replies and absences survive cache admission failures, and busy reads leave time for the source. Cases: `crates/biomcp/tests/runner/cache_contention.rs`.

housekeeping: Print build provenance and ready-to-paste local MCP configuration without reading keys or contacting sources. Cases: `crates/biomcp/tests/runner/housekeeping.rs`.
store-coexistence: Allowed bundle reads import only names missing from the current generation through a verified atomic transfer, preserve both files, and retain current published bundles on conflicts. Cases: `crates/biomcp/tests/runner/store_coexistence.rs`.

agent-skill: Agents read case-backed guidance and install managed guidance only inside an explicit directory. Cases: `crates/biomcp/tests/runner/skill.rs`.

stream-tensors: Decode serialized provider tensor payloads within explicit wire and expansion bounds. Contracts: `biomcp-adapters/tensors`.

stream-worker: Blocking source legs make one bounded streaming RPC over their checked TLS and proxy route within the shared quota and absolute deadline. Contracts: `biomcp-fetch/grpc_worker`.
stream-persistence: Complete checked RPC streams enter the ordinary cache and replay from bounded recordings without connections. Contracts: `biomcp-fetch/grpc_persistence`.

alphagenome-protocol: Encode the reviewed human DNASE request and admit only complete bounded reference and alternate envelopes. Contracts: `biomcp-adapters/alphagenome`.

alphagenome-registration: Discover the restricted AlphaGenome service and require its own user key before source admission. Contracts: `biomcp-adapters/alphagenome_registration`.

alphagenome-score-protocol: Encode the ruled human scoring request and admit exactly the requested complete scorer groups by request position. Contracts: `biomcp-adapters/score_variant`.

variant-alphagenome: Read independently paged native scoring groups for one source-resolved GRCh38 variant with the caller's required key. Cases: `crates/biomcp/tests/runner/alphagenome.rs`, `crates/biomcp/tests/runner/alphagenome_tls.rs`, `crates/biomcp/tests/runner/alphagenome_capture.rs`.

query-validation: A caller sees argument names for typed input failures, conflicting fields and invalid ranges without rejected values. Cases: `tests/cases/grammar/semantic-arguments.yaml`.
outbound-hosts: Refuse undeclared source request origins before cache, replay or network admission. Contracts: `biomcp-fetch/paths`.

local-file-persistence: Create private Unix cache files and update symlinked credential configuration targets while retaining field order. Cases: `crates/biomcp/tests/runner/local_files.rs`.

diagnostic-device-recall: Search native device recall records, inspect exact recall numbers and follow product codes to MAUDE without inferring harm. Cases: `crates/biomcp/tests/runner/diagnostic_recall.rs`, `crates/biomcp/tests/runner/diagnostic_recall_selectors.rs`, `crates/biomcp/tests/runner/diagnostic_recall_boundaries.rs`, `tests/cases/diagnostic/recall.yaml`.

organization-nci: Find source-native NCI organizations with bounded snapshots and local paging. Cases: `tests/cases/organization/*.yaml`, `crates/biomcp/tests/runner/nci/organization.rs`.

biomarker-nci: Find NCI biomarker concepts by name or synonym and retrieve native concept rows by NCIt code. Cases: `tests/cases/biomarker/*.yaml`, `crates/biomcp/tests/runner/nci/biomarker.rs`.
drug-recall: Search published FDA drug recall records by name, firm, classification, status, reason and initiation dates; page and fetch exact native recall numbers with source field problems. Cases: `crates/biomcp/tests/runner/drug_recall.rs`.
