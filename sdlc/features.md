gene-identity: A caller resolves approved symbols, aliases, previous symbols and identifiers without guessing between ambiguous candidates. Cases: `tests/cases/gene/*.yaml`.
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
article-search: A caller walks stable native source windows with totals, checked selectors, optional source keys and explicit failures. Cases: `tests/cases/article/search/*.yaml`, `tests/cases/article/fold/*.yaml`, `tests/cases/article/semantic-scholar/*.yaml`, `crates/biomcp/tests/runner/scholar.rs`.
article-filters: A caller narrows article results by author, journal, dates and publication type before paging. Cases: `tests/cases/article/filters/*.yaml`.
article-fold: A caller combines PubMed and Europe PMC results while retaining each source's native rank and missing-source outcomes. Cases: `tests/cases/article/fold/*.yaml`.
article-litvar: A caller selects an exact variant, receives source candidates for ambiguity and combines its publications with other article sources in a stable pool. Cases: `tests/cases/article/litvar/*.yaml`, `crates/biomcp/tests/runner/litvar_tables.rs`.
article-get: A caller retrieves article cards by PMID, PMCID or checked DOI with complete abstracts and retraction links. Cases: `tests/cases/article/get/*.yaml`.
article-sections: A caller requests indexing and normalized entity annotations while preserving the article card when a source fails. Cases: `tests/cases/article/sections/*.yaml`, `tests/cases/article/indexing/*.yaml`.
article-citations: A caller sorts by iCite citation counts and walks citing or referenced papers with explicit missing counts. Cases: `tests/cases/article/citations/*.yaml`.
article-fulltext: A caller reads available full text through semantic parts, tables, outlines and checked continuations. Cases: `tests/cases/article/fulltext/*.yaml`.
article-retired: A caller using a retired article spelling receives its fixed replacement or removal explanation. Cases: `tests/cases/article/retired.yaml`.
mcp-parity: A caller sends the CLI argument array through the MCP tool and receives the same command result. Cases: `tests/cases/mcp/*.yaml`.
schema-discovery: A caller retrieves versioned schemas and receives a fixed usage envelope for invalid requests. Cases: `tests/cases/schema/*.yaml`.
corporate-network: A caller adds corporate certificate authorities, follows standard proxy settings and receives private configuration guidance for network failures. Cases: `crates/biomcp/tests/runner/corporate.rs`.
markdown-view: A caller reads gene and article results as Markdown, copies next commands, and keeps JSON for MCP. Cases: `crates/biomcp/tests/runner/markdown_shapes.rs`.
article-semantic-sections: Read machine-written summaries and source-ordered citation edges with contexts from Semantic Scholar. Cases: `tests/cases/article/tldr/*.yaml`, `tests/cases/article/edges/*.yaml`.
source-controls: Source terms, disabled sources and bounded source health preserve policy skips before cached reads or requests. Cases: `crates/biomcp/tests/runner/source_controls.rs`.
live-source-checks: A separate keyless live diagnostic reports upstream drift without changing replay assertions or recordings. Cases: `crates/biomcp/tests/runner/live_proof.rs`, `crates/biomcp/tests/runner/live_follow_proof.rs`.
article-named-search: A caller searches sentence resemblance and PubTator entities by name while retaining native order, matched sentences and explicit unsupported filters. Cases: `tests/cases/article/litsense/*.yaml`, `tests/cases/article/pubtator/*.yaml`, `crates/biomcp/tests/runner/named_search.rs`.
article-supplements: List source-confirmed files and read bounded spreadsheet and text rows through CLI and MCP. Cases: `tests/cases/article/supplements/*.yaml`, `crates/biomcp/tests/runner/supplement_tables.rs`.
article-preprints: A caller retrieves preprint cards and licensed full text by Europe PMC ID. Cases: `tests/cases/article/preprint/*.yaml`.
article-help: A caller discovers article arguments, sections and executable examples through generated help. Cases: `tests/cases/article/help.yaml`, `crates/biomcp/tests/runner/help_examples.rs`.
mcp-http: A native client sends independent MCP requests over bounded HTTP and checks local storage health. Cases: `crates/biomcp/tests/runner/http.rs`, `crates/biomcp/tests/runner/http_limits.rs`.
article-evidence: Read all source citation contexts and explicit full-text reference paragraphs through checked continuations. Cases: `tests/cases/article/evidence/*.yaml`, `crates/biomcp/tests/runner/evidence.rs`.
article-recommendations: Read Semantic Scholar recommendations in source order with stable pages and explicit failures. Cases: `tests/cases/article/recommendations/*.yaml`.

article-pdf: Inspect publisher PDF links and download only to an explicit destination through CLI or stdio MCP. Cases: `tests/cases/article/pdf/*.yaml`, `crates/biomcp/tests/runner/pdf.rs`.
