# Command reference

Generated from the command grammar and schema help. Run `make docs-generate` to update.

Options shown on the root command apply to its subcommands. Query sections and source rules follow each command.

Listed choices accept any letter case; replies use their listed spelling.

## biomcp

```text
Retrieve source records for biomedical evidence questions.

Usage: biomcp [OPTIONS] <COMMAND>

Commands:
  search      Search source records by entity and checked filters.
  get         Retrieve one entity and its source sections.
  author      Read papers attributed to a source author record.
  cell-line   Read source cell line assay counts.
  protein     Read protein sequences, features and source relationships.
  pathway     Read pathway genes and source events.
  article     Read article text, references, citations and files.
  trial       Read trial groups and public documents.
  variant     Read source variant assertions, populations and recurrence.
  sync        Synchronize a local source bundle.
  skill       Read embedded guidance and worked examples. No source requests. Overview and catalog are bounded by the embedded payload; install and status return one summary. Install writes only inside DIR and refuses symbolic links and parent traversal.
  source      List source names as JSON. No source requests or key reads.
  version     Print version; --verbose adds build and path provenance. No source requests or key reads.
  mcp-config  Print MCP configuration with this executable’s absolute path. JSON formats name their top-level object and retain released client aliases. The codex client prints its shell registration command. No source requests or key reads.
  health      Report source terms and bounded reachability.
  store       Manage local cached replies and snapshots.
  key         Register and manage source credentials without showing values.
  serve       Serve MCP over standard input or HTTP.
  schema      List every schema identity and description with biomcp schema, or print one schema artifact with biomcp schema <id>.
  help        Describe commands and their arguments.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

-h, --help: Print scoped command help without source requests.
--version: Print the installed version without source requests.
Entity-first order also runs on the command line: gene get BRAF runs get gene BRAF.

```

search: Search source records by entity and checked filters.
get: Retrieve one entity and its source sections.
author: Read papers attributed to a source author record.
cell-line: Read source cell line assay counts.
protein: Read protein sequences, features and source relationships.
pathway: Read pathway genes and source events.
article: Read article text, references, citations and files.
trial: Read trial groups and public documents.
variant: Read source variant assertions, populations and recurrence.
sync: Synchronize a local source bundle.
skill: Read embedded guidance and worked examples
source: List source names as JSON
version: Print version; --verbose adds build and path provenance
mcp-config: Print MCP configuration with this executable’s absolute path
health: Report source terms and bounded reachability.
store: Manage local cached replies and snapshots.
key: Register and manage source credentials without showing values.
serve: Serve MCP over standard input or HTTP.
schema: List every schema identity and description with biomcp schema, or print one schema artifact with biomcp schema <id>.
help: Describe commands and their arguments.
Listed choices accept any letter case; replies use their listed spelling.
Options are local to the command and independent of source selection.

- `--format`: Choose JSON or Markdown output; configuration can change the default. Value form: string; example: --format json.
- `--disable-sources`: Exclude source names or source terms groups. Value form: string; example: --disable-sources civic.
- `--source_pacing`: Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Value form: string; example: --source_pacing '{}'.
- `--config`: Read configuration from this file. Value form: string; example: --config config.json.
- `--offline`: Read local data without source requests. Value form: boolean; example: --offline.
- `--fresh`: Request fresh source data instead of cached replies. Value form: boolean; example: --fresh.
- `--help`: Print scoped command help without source requests. Value form: boolean; example: --help.
- `--version`: Print the installed version without source requests. Value form: boolean; example: --version.

## biomcp search

```text
Search source records by entity and checked filters.

Usage: biomcp search [OPTIONS] <COMMAND>

Commands:
  biomarker      Find NCI biomarker concepts by name or synonym. The public tier returns codes, name, type, trial count, synonyms, parent_ids, ancestor_ids and eligibility_criterion. These concepts differ from diagnostic --biomarker filters and literal trial --variant text. Use search trial --source nci --biomarker-code CODE for trials that list an exact concept. Native criteria do not establish patient eligibility.
  author         Literal author name query and native window; positional text maps to query.
  organization   Find NCI organizations. Names are not unique; trial counts cover every status, including closed trials. No organization ID or get is available.
  cell-line      One native cell line search with optional text, species, disease, variation and derived-line filters.
  gwas           Source associations in default p-value-ascending order. Reports no personal risk, causal variant or gene, or clinical interpretation of a statistic.
  protein        Search five reviewed human UniProt headers by literal text or exact gene.
                 No variant effect, structure, interaction or clinical meaning is inferred. No bioinformatics tool runs.
  phenotype      Search literal human HP term candidates through Monarch with bounded native pages, or give only HP IDs, comma-separated, to get Monarch's one ranked list of genes or diseases most similar to that set. A term lookup answers terminology and a ranking orders source candidates; neither establishes a diagnosis.
  pathway        Search Reactome (default) or WikiPathways human pathways. Membership names a source and release; it implies no therapy, response or actionability. No enrichment is calculated.
  study          Literal stored metadata search; text is never parsed as a query language. No frequency, denominator, survival or comparison is calculated. Source counts keep their native names.
  pgx            Checked gene-keyed CPIC pair search. No phenotype, phase or dose is calculated.
  diagnostic     Search five source records by default. FDA device recalls cover every medical device and show class; totals count recall records, without harm inference.
  adverse-event  Search five report headers by default. MAUDE accepts device text or a manufacturer, product-code, problem or event-type filter. One report may list several devices; filters may match different devices. VAERS accepts MMR, COVID19, COVID-19 vaccine or one exact qualified native influenza code, matched without case; use influenza to list codes and labels. Tables remain separate and are never summed. One report may list several vaccines. VAERS returns five reported symptom rows by default; seriousness and age select independent tables. Counts are reports, not people. Reports do not show that a vaccine caused an event and cannot measure risk or compare vaccine safety.
  disease        Literal discovery text or an explicit SEER catalog page.
  drug           Search one drug source. FDA drug recalls accept a name or native firm, classification, status, reason and initiation dates; counts are recall records. WHO prequalification accepts a name, area or date range. openFDA labels also accept literal indication wording. Drugs@FDA accepts a name or an applicant. Other sources require a name.
  gene           Search genes by text, cytoband, or their intersection, with ClinGen dosage regions that match.
  article        Checked search selectors and pool cursor.
  trial          Search ClinicalTrials.gov or keyed NCI with native counts. NCI --facility matches exact organization names; a listed site is not necessarily enrolling.
  variant        Discover source candidates using one declared selector, without interpreting free text.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Search source records by entity and checked filters.


## biomcp search biomarker

```text
Find NCI biomarker concepts by name or synonym. The public tier returns codes, name, type, trial count, synonyms, parent_ids, ancestor_ids and eligibility_criterion. These concepts differ from diagnostic --biomarker filters and literal trial --variant text. Use search trial --source nci --biomarker-code CODE for trials that list an exact concept. Native criteria do not establish patient eligibility.

Usage: biomcp search biomarker [OPTIONS] --source <source>

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --limit <limit>
          Maximum headers per local page.

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --name <name>
          Native partial name or synonym search.

      --offset <offset>
          Offset within the complete retained observation.

      --pool <pool>
          Snapshot identity retained by continuation commands.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --source <source>
          Explicit keyed NCI source; help search biomarker
          Explicit keyed NCI source.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Search NCI biomarker concepts by name or synonym. Sources: nci.
Listed choices accept any letter case; replies use their listed spelling.
Options apply to every listed source unless their description restricts them.

- `--limit`: Maximum headers per local page. Value form: integer; example: --limit 5.
- `--name`: Source partial name or synonym search. Value form: string; example: --name PD-L1.
- `--offset`: Offset within the complete retained observation. Value form: integer; example: --offset 0.
- `--pool`: Snapshot identity retained by continuation commands. Value form: string; example: --pool <pool> (syntax; replace the placeholder with your retained value).
- `--source`: Explicit keyed NCI source. Value form: string; example: --source nci.

## biomcp search author

```text
Literal author name query and native window; positional text maps to query.

Usage: biomcp search author [OPTIONS] <query>

Arguments:
  <query>
          Literal query from the positional argument.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

  -l, --limit <limit>
          Bounded source page size.

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --offset <offset>
          Native offset.

      --pool <pool>
          Retained page identity.

      --source <source>
          Source; auto uses PubMed if Semantic Scholar is off; help search author
          Source; auto uses PubMed if Semantic Scholar is off.
          PubMed accepts names or topics and returns unresolved names on papers.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Search research paper authors. Sources: pubmed, semantic-scholar.
Listed choices accept any letter case; replies use their listed spelling.
Options apply to every listed source unless their description restricts them.

- `--limit`: Bounded source page size. Value form: integer; example: --limit 5.
- `--offset`: Source offset. Value form: integer; example: --offset 0.
- `--pool`: Retained page identity. Value form: string; example: --pool <pool> (syntax; replace the placeholder with your retained value).
- `query`: Exact query from the positional argument.
Form: search author QUERY; not --query.
- `--source`: Source; auto uses PubMed if Semantic Scholar is off.
PubMed accepts names or topics and returns unresolved names on papers. Value form: string; example: --source semanticscholar.

## biomcp search organization

```text
Find NCI organizations. Names are not unique; trial counts cover every status, including closed trials. No organization ID or get is available.

Usage: biomcp search organization [OPTIONS] --source <source>

Options:
      --city <city>
          Native organization city.

      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --country <country>
          Native organization country.

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --distance <distance>
          Positive radius with explicit mi or km units.

      --family <family>
          Native organization family.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --lat <lat>
          Radius center latitude in degrees; requires lon and distance.

      --limit <limit>
          Five headers by default, at most fifty.

      --lon <lon>
          Radius center longitude in degrees; requires lat and distance.

      --name <name>
          NCI partial organization name; names are not unique.

      --offset <offset>
          Offset within the complete retained organization snapshot.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --pool <pool>
          Snapshot identity retained by continuation commands.

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

      --postal-code <postal_code>
          Native postal code.

      --source <source>
          Explicit keyed NCI source; help search organization
          Explicit keyed NCI source.

      --state <state>
          Native state or province.

```

Search clinical research organizations. Sources: nci.
Listed choices accept any letter case; replies use their listed spelling.
Options apply to every listed source unless their description restricts them.

- `--city`: Source organization city. Value form: string; example: --city Boston.
- `--country`: Source organization country. Value form: string; example: --country 'United States'.
- `--distance`: Positive radius with explicit mi or km units. Value form: string; example: --distance 10mi.
- `--family`: Source organization family. Value form: string; example: --family 'Dana-Farber/Harvard Cancer Center'.
- `--lat`: Radius center latitude in degrees; requires lon and distance. Value form: string; example: --lat 42.3469.
- `--limit`: Five headers by default, at most fifty. Value form: integer; example: --limit 5.
- `--lon`: Radius center longitude in degrees; requires lat and distance. Value form: string; example: --lon -71.1025.
- `--name`: NCI partial organization name; names are not unique. Value form: string; example: --name Dana-Farber.
- `--offset`: Offset within the complete retained organization snapshot. Value form: integer; example: --offset 0.
- `--pool`: Snapshot identity retained by continuation commands. Value form: string; example: --pool <pool> (syntax; replace the placeholder with your retained value).
- `--postal-code`: Source postal code. Value form: string; example: --postal-code 02215.
- `--source`: Explicit keyed NCI source. Value form: string; example: --source nci.
- `--state`: Source state or province. Value form: string; example: --state MA.

## biomcp search cell-line

```text
One native cell line search with optional text, species, disease, variation and derived-line filters.

Usage: biomcp search cell-line [OPTIONS] [query]

Arguments:
  [query]
          Literal source query from the positional argument.

Options:
      --disease <disease>
          Cellosaurus disease: NCIt name or ID (for example Melanoma or C3224).

      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --exclude-derived[=<exclude_derived>]
          Leave out lines derived from a parent line; needs another search term.

          [default: false]
          [possible values: true, false]

      --gene <gene>
          Cellosaurus variation gene, as the source writes it (for example KRAS).

  -l, --limit <limit>
          Native page size.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --mutation <mutation>
          Cellosaurus variation text, as the source writes it (for example p.Gly12Asp).

      --offset <offset>
          Native start offset.

      --protein <protein>
          Protein change, sent in the source's spelling (G12D is sent as p.Gly12Asp).

      --species <species>
          Cellosaurus species: NCBI Taxonomy name or ID (for example Homo sapiens or 9606).

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Search cell lines by name or identifier. Sources: cellosaurus.
Options apply to every listed source unless their description restricts them.

- `--disease`: Cellosaurus disease: NCIt name or ID (for example Melanoma or C3224). Value form: string; example: --disease Melanoma.
- `--exclude-derived`: Leave out lines derived from a parent line; needs another search term. Value form: boolean; example: --exclude-derived.
- `--gene`: Cellosaurus variation gene, as the source writes it (for example KRAS). Value form: string; example: --gene KRAS.
- `--limit`: Source page size. Value form: integer; example: --limit 25.
- `--mutation`: Cellosaurus variation text, as the source writes it (for example p.Gly12Asp). Value form: string; example: --mutation p.Gly12Asp.
- `--offset`: Source start offset. Value form: integer; example: --offset 0.
- `--protein`: Protein change, sent in the source's spelling (G12D is sent as p.Gly12Asp). Value form: string; example: --protein G12D.
- `query`: Exact source query from the positional argument.
Form: search cell-line QUERY; not --query.
- `--species`: Cellosaurus species: NCBI Taxonomy name or ID (for example Homo sapiens or 9606). Value form: string; example: --species 'Homo sapiens'.

## biomcp search gwas

```text
Source associations in default p-value-ascending order. Reports no personal risk, causal variant or gene, or clinical interpretation of a statistic.

Usage: biomcp search gwas [OPTIONS] [query]

Arguments:
  [query]
          Exact native mapped gene; legacy positional query aliases this field.

Options:
      --extended-geneset[=<extended_geneset>]
          Source extended gene set; requires gene.

          [default: false]
          [possible values: true, false]

      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

  -g, --gene <gene>
          Exact native mapped gene; legacy positional query aliases this field.

      --include-child-traits[=<include_child_traits>]
          Source child-trait expansion; requires trait_id.

          [default: false]
          [possible values: true, false]

  -l, --limit <limit>
          Maximum association rows.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offset <offset>
          Native association offset.

      --p-value <p_value>
          Inclusive positive probability in scientific notation. Requires the default p-value-ascending order; no sort option is available. The qualifying total stays unknown until the ordered walk reaches its end.

      --pool <pool>
          Retained threshold walk identity; requires a threshold.

      --pubmed <pubmed>
          Exact source publication identity.

      --studies[=<studies>]
          Return study records for the selected publication instead of associations.

          [default: false]
          [possible values: true, false]

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --study <study>
          Exact source study accession.

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

      --trait <trait>
          Literal native efo_trait label; never searches reported_trait prose.

      --trait-id <trait_id>
          Exact native ontology identity.

      --variant <variant>
          Exact source rsID.

```

Search genetic associations in GWAS Catalog. Sources: gwas.
Options apply to every listed source unless their description restricts them.

- `--extended-geneset`: Source extended gene set; requires gene. Value form: boolean; example: --extended-geneset.
- `--gene`: Exact source mapped gene; legacy positional query aliases this field. Value form: string; example: --gene FGFR2.
- `--include-child-traits`: Source child-trait expansion; requires trait_id. Value form: boolean; example: --include-child-traits.
- `--limit`: Maximum association rows. Value form: integer; example: --limit 3.
- `--offset`: Source association offset. Value form: integer; example: --offset 3.
- `--p-value`: Inclusive positive probability in scientific notation. Requires the default p-value-ascending order; no sort option is available. The qualifying total stays unknown until the ordered walk reaches its end. Value form: string; example: --p-value 4e-254.
- `--pool`: Retained threshold walk identity; requires a threshold. Value form: string; example: --pool <pool> (syntax; replace the placeholder with your retained value).
- `--pubmed`: Exact source publication identity. Value form: string; example: --pubmed 17529967.
- `query`: Exact source mapped gene; legacy positional query aliases this field.
Form: search gwas QUERY; not --query.
- `--studies`: Return study records for the selected publication instead of associations. Value form: boolean; example: --studies.
- `--study`: Exact source study accession. Value form: string; example: --study GCST000035.
- `--trait`: Exact source efo_trait label; never searches reported_trait prose. Value form: string; example: --trait 'breast carcinoma'.
- `--trait-id`: Exact source ontology identity. Value form: string; example: --trait-id MONDO_0004989.
- `--variant`: Exact source rsID. Value form: string; example: --variant rs2981582.

## biomcp search protein

```text
Search five reviewed human UniProt headers by literal text or exact gene.
No variant effect, structure, interaction or clinical meaning is inferred. No bioinformatics tool runs.

Usage: biomcp search protein [OPTIONS] [query]

Arguments:
  [query]
          Literal text, exclusive with gene.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --gene <gene>
          Exact native gene field, without identifier inference.

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --limit <limit>
          Five rows by default; no more than 25.

      --offset <offset>
          Source/retained result offset.

      --pool <pool>
          Exact retained collection identity.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --reviewed[=<reviewed>]
          Reviewed human scope stays enabled; unreviewed search is deferred.

          [default: false]
          [possible values: true, false]

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Search reviewed human proteins in UniProt. Sources: uniprot.
Options apply to every listed source unless their description restricts them.

- `--gene`: Exact source gene field, without identifier inference. Value form: string; example: --gene BRAF.
- `--limit`: Five rows by default; no more than 25. Value form: integer; example: --limit 25.
- `--offset`: Source/retained result offset. Value form: integer; example: --offset 0.
- `--pool`: Exact retained collection identity. Value form: string; example: --pool <pool> (syntax; replace the placeholder with your retained value).
- `query`: Exact text, exclusive with gene.
Form: search protein QUERY; not --query.
- `--reviewed`: Reviewed human scope stays enabled; unreviewed search is deferred. Value form: boolean; example: --reviewed.

## biomcp search phenotype

```text
Search literal human HP term candidates through Monarch with bounded native pages, or give only HP IDs, comma-separated, to get Monarch's one ranked list of genes or diseases most similar to that set. A term lookup answers terminology and a ranking orders source candidates; neither establishes a diagnosis.

Usage: biomcp search phenotype [OPTIONS] <text>

Arguments:
  <text>
          Unchanged literal term search text, or HP IDs separated by commas to rank genes or diseases for the set.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --group <group>
          HP IDs only: rank genes, the default, or diseases; help search phenotype
          HP IDs only: rank genes, the default, or diseases.

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

  -l, --limit <limit>
          Maximum native headers, default five.

      --metric <metric>
          HP IDs only: the Monarch similarity metric to rank by; help search phenotype
          HP IDs only: the Monarch similarity metric to rank by.

      --offset <offset>
          Native source row offset; a ranked HP ID set has no further pages.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --search-mode <search_mode>
          HP IDs only: hybrid (default) or slower full; help search phenotype
          HP IDs only: hybrid (default) or slower full.

      --source <source>
          Monarch only; HPO search is unsupported; help search phenotype
          Monarch only; HPO search is unsupported.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Search human phenotypes in HPO. Sources: hpo, monarch, ols.
Listed choices accept any letter case; replies use their listed spelling.
Options apply to every listed source unless their description restricts them.

- `--group`: HP IDs only: rank genes, the default, or diseases. Value form: string; example: --group diseases.
- `--limit`: Maximum source headers, default five. Value form: integer; example: --limit 3.
- `--metric`: HP IDs only: the Monarch similarity metric to rank by. Value form: string; example: --metric phenodigm-score.
- `--offset`: Source source row offset; a ranked HP ID set has no further pages. Value form: integer; example: --offset 0.
- `--search-mode`: HP IDs only: hybrid (default) or slower full. Value form: string; example: --search-mode full.
- `--source`: Monarch only; HPO search is unsupported. Value form: string; example: --source monarch.
- `text`: Unchanged exact term search text, or HP IDs separated by commas to rank genes or diseases for the set.
Form: search phenotype TEXT; not --text.

## biomcp search pathway

```text
Search Reactome (default) or WikiPathways human pathways. Membership names a source and release; it implies no therapy, response or actionability. No enrichment is calculated.

Usage: biomcp search pathway [OPTIONS] [query]

Arguments:
  [query]
          Literal source query; exclusive with gene and top-level discovery.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --fresh[=<fresh>]
          Request a new retained observation.

          [default: false]
          [possible values: true, false]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --gene <gene>
          Human gene input for exact resolution and UniProt mapping; Reactome only.

  -l, --limit <limit>
          Five rows by default, one to fifty. Reactome rows equals this limit without refill.

      --offset <offset>
          Native Reactome Start row or retained WikiPathways catalog offset.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --pool <pool>
          Retained snapshot identity for deterministic continuation.

      --source <source>
          Sources in caller order; repeated sources take turns; help search pathway
          Sources in caller order; repeated sources take turns without title joins.

      --top-level[=<top_level>]
          Native human Reactome top-level discovery, exclusive with text and gene.

          [default: false]
          [possible values: true, false]

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

```

Search human biological pathways. Sources: reactome, wikipathways.
Listed choices accept any letter case; replies use their listed spelling.
Options apply to every listed source unless their description restricts them.

- `--fresh`: Request a new retained observation. Value form: boolean; example: --fresh.
- `--gene`: Human gene input for exact resolution and UniProt mapping; Reactome only. Value form: string; example: --gene HGNC:1097.
- `--limit`: Five rows by default, one to fifty. Reactome rows equals this limit without refill. Value form: integer; example: --limit 5.
- `--offset`: Source Reactome Start row or retained WikiPathways catalog offset. Value form: integer; example: --offset 0.
- `--pool`: Retained snapshot identity for deterministic continuation. Value form: string; example: --pool <pool> (syntax; replace the placeholder with your retained value).
- `query`: Exact source query; exclusive with gene and top-level discovery.
Form: search pathway QUERY; not --query.
- `--source`: Sources in caller order; repeated sources take turns without title joins. Value form: repeatable values (repeat the flag); example: --source reactome.
- `--top-level`: Source human Reactome top-level discovery, exclusive with text and gene. Value form: boolean; example: --top-level.

## biomcp search study

```text
Literal stored metadata search; text is never parsed as a query language. No frequency, denominator, survival or comparison is calculated. Source counts keep their native names.

Usage: biomcp search study [OPTIONS] [text]

Arguments:
  [text]
          Literal study ID, name or description text, without synonyms or typo repair.

Options:
      --cancer-type <cancer_type>
          Exact cBioPortal cancer-type code; no OncoTree conversion.

      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --limit <limit>
          Maximum headers per page.

      --offset <offset>
          Zero-based offset in the checked snapshot.

      --pool <pool>
          Opaque continuation retaining the catalog revision and selectors.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Search cancer genomics studies in cBioPortal. Sources: cbioportal.
Options apply to every listed source unless their description restricts them.

- `--cancer-type`: Exact cBioPortal cancer-type code; no OncoTree conversion. Value form: string; example: --cancer-type mixed.
- `--limit`: Maximum headers per page. Value form: integer; example: --limit 1.
- `--offset`: Zero-based offset in the checked snapshot. Value form: integer; example: --offset 0.
- `--pool`: Opaque continuation retaining the catalog revision and selectors. Value form: string; example: --pool <pool> (syntax; replace the placeholder with your retained value).
- `text`: Exact study ID, name or description text, without synonyms or typo repair.
Form: search study TEXT; not --text.

## biomcp search pgx

```text
Checked gene-keyed CPIC pair search. No phenotype, phase or dose is calculated.

Usage: biomcp search pgx [OPTIONS] --gene <gene> --source <source>

Options:
      --cpic-level <cpic_level>
          Literal native level, including compound levels such as B/C.

      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --gene <gene>
          Explicit source gene symbol.

      --limit <limit>
          Rows per native page.

      --offset <offset>
          Native source offset.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --source <source>
          Required CPIC source; ClinPGx is not supported for search; help search pgx
          Required CPIC source; ClinPGx is not supported for search.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Search pharmacogenomic gene and drug pairs. Sources: cpic.
Listed choices accept any letter case; replies use their listed spelling.
Options apply to every listed source unless their description restricts them.

- `--cpic-level`: Exact source level, including compound levels such as B/C. Value form: string; example: --cpic-level B/C.
- `--gene`: Explicit source gene symbol. Value form: string; example: --gene DPYD.
- `--limit`: Rows per source page. Value form: integer; example: --limit 1.
- `--offset`: Source source offset. Value form: integer; example: --offset 0.
- `--source`: Required CPIC source; ClinPGx is not supported for search. Value form: string; example: --source cpic.

## biomcp search diagnostic

```text
Search five source records by default. FDA device recalls cover every medical device and show class; totals count recall records, without harm inference.

Usage: biomcp search diagnostic [OPTIONS] --source <source>

Options:
      --biomarker <biomarker>
          Exact table-0 cell.
          Exact table-0 biomarker cell; unsupported in the group table.

      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --bundle-revision <bundle_revision>
          Stored bundle revision.
          Immutable stored bundle revision retained by a continuation.

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --classification <classification>
          FDA recall class; help search diagnostic
          FDA recall class.
          Native enforcement classification; FDA device recalls only. Selects enforcement records, which carry no product code.

      --condition-id <condition_id>
          Exact MedGen:CUI.
          Exact MedGen:CUI identifier only.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --decision-from <decision_from>
          First date, YYYY-MM-DD.
          Inclusive native decision date, YYYY-MM-DD; PMA and 510(k) only.

      --decision-to <decision_to>
          Last date, YYYY-MM-DD.
          Inclusive native decision date, YYYY-MM-DD; PMA and 510(k) only.

      --device-id <device_id>
          PMA or K identifier.
          Exact PMA application, application/supplement or native K identity.

      --drug-application <drug_application>
          NDA/BLA application.
          Cell-owned typed NDA/BLA application.

      --gene <gene>
          Exact gene symbol.
          Exact source gene_symbol field, without alias expansion.

      --gene-id <gene_id>
          Exact NCBI GeneID.
          Exact source GeneID, exclusive with gene symbol.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

      --initiated-from <initiated_from>
          First date, YYYY-MM-DD.
          Inclusive initiation date, YYYY-MM-DD; FDA device recalls only.

      --initiated-to <initiated_to>
          Last date, YYYY-MM-DD.
          Inclusive initiation date, YYYY-MM-DD; FDA device recalls only.

      --intended-use <intended_use>
          Intended-use phrase.
          Phrase in FDA PMA ao_statement, the approval order's intended use; PMA only, as openFDA 510(k) has no intended-use text.

      --limit <limit>
          Five results by default; at most fifty source rows or product headers.

      --manufacturer <manufacturer>
          Manufacturer phrase.
          WHO manufacturer words, native FDA applicant (PMA/510(k)) or recalling_firm phrase.

      --marker <marker>
          WHO marker phrase.
          Literal whole-word phrase in WHO's Pathogen/Disease/Marker cell. WHO rows carry prequalification_year.

      --name <name>
          Native test or device name.
          Phrase in PMA trade_name, 510(k) device_name or recall product_description; text in GTR lab_test_name.

      --offset <offset>
          Zero-based source result offset.

      --prequalified-from <prequalified_from>
          WHO IVD year.
          First prequalification year included, YYYY; WHO publishes only the year.

      --prequalified-to <prequalified_to>
          WHO IVD year.
          Last prequalification year included, YYYY; WHO publishes only the year.

      --product-code <product_code>
          FDA product code.
          Exact native FDA product code; PMA, 510(k) and device recalls only.

      --source <source>
          Native diagnostic source; help search diagnostic
          Native diagnostic source.
          Explicit native GTR, WHO IVD, FDA PMA, 510(k), companion or medical device recall discovery.

      --type <type>
          Native test_type; help search diagnostic
          Native test_type.
          Exact native test_type value.

```

Search diagnostic laboratory tests and devices. Sources: fda-510k, fda-cdx, fda-device-recall, fda-pma, gtr, who-ivd.
Listed choices accept any letter case; replies use their listed spelling.
Options apply to every listed source unless their description restricts them.

- `--biomarker`: Exact table-0 cell.
Exact table-0 biomarker cell; unsupported in the group table. Value form: string; example: --biomarker KRAS; only with --source fda-cdx.
- `--bundle-revision`: Stored bundle revision.
Immutable stored bundle revision retained by a continuation. Value form: string; example: --bundle-revision sha256:3eb93d3ba1a08b508955b0699d32042584fd8e0c805c72f7eeb470ef04ba959a.
- `--classification`: FDA recall class.
Source enforcement classification; FDA device recalls only. Selects enforcement records, which carry no product code. Value form: string; example: --classification 'Class I'; only with --source fda-device-recall.
- `--condition-id`: Exact MedGen:CUI.
Exact MedGen:CUI identifier only. Value form: string; example: --condition-id MedGen:C2676676; only with --source gtr.
- `--decision-from`: First date, YYYY-MM-DD.
Inclusive source decision date, YYYY-MM-DD; PMA and 510(k) only. Value form: string; example: --decision-from 2017-11-30; only with --source fda-pma or fda-510k.
- `--decision-to`: Last date, YYYY-MM-DD.
Inclusive source decision date, YYYY-MM-DD; PMA and 510(k) only. Value form: string; example: --decision-to 2017-11-30; only with --source fda-pma or fda-510k.
- `--device-id`: PMA or K identifier.
Exact PMA application, application/supplement or source K identity. Value form: string; example: --device-id P200010/S026; only with --source fda-cdx.
- `--drug-application`: NDA/BLA application.
Cell-owned typed NDA/BLA application. Value form: string; example: --drug-application NDA208065; only with --source fda-cdx.
- `--gene`: Exact gene symbol.
Exact source gene_symbol field, without alias expansion. Value form: string; example: --gene BRCA1; only with --source gtr.
- `--gene-id`: Exact NCBI GeneID.
Exact source GeneID, exclusive with gene symbol. Value form: string; example: --gene-id 672; only with --source gtr.
- `--initiated-from`: First date, YYYY-MM-DD.
Inclusive initiation date, YYYY-MM-DD; FDA device recalls only. Value form: string; example: --initiated-from 2024-01-01; only with --source fda-device-recall.
- `--initiated-to`: Last date, YYYY-MM-DD.
Inclusive initiation date, YYYY-MM-DD; FDA device recalls only. Value form: string; example: --initiated-to 2024-12-31; only with --source fda-device-recall.
- `--intended-use`: Intended-use phrase.
Phrase in FDA PMA ao_statement, the approval order's intended use; PMA only, as openFDA 510(k) has no intended-use text. Value form: string; example: --intended-use cell-free; only with --source fda-pma.
- `--limit`: Five results by default; at most fifty source rows or product headers. Value form: integer; example: --limit 50.
- `--manufacturer`: Manufacturer phrase.
WHO manufacturer words, source FDA applicant (PMA/510(k)) or recalling_firm phrase. Value form: string; example: --manufacturer 'Synthetic maker'; only with --source who-ivd, fda-pma, fda-510k or fda-device-recall.
- `--marker`: WHO marker phrase.
Exact whole-word phrase in WHO's Pathogen/Disease/Marker cell. WHO rows carry prequalification_year. Value form: string; example: --marker HIV; only with --source who-ivd.
- `--name`: Source test or device name.
Phrase in PMA trade_name, 510(k) device_name or recall product_description; text in GTR lab_test_name. Value form: string; example: --name 'FoundationOne CDx'; only with --source gtr, fda-pma, fda-510k or fda-device-recall.
- `--offset`: Zero-based source result offset. Value form: integer; example: --offset 5.
- `--prequalified-from`: WHO IVD year.
First prequalification year included, YYYY; WHO publishes only the year. Value form: string; example: --prequalified-from 2023; only with --source who-ivd.
- `--prequalified-to`: WHO IVD year.
Last prequalification year included, YYYY; WHO publishes only the year. Value form: string; example: --prequalified-to 2023; only with --source who-ivd.
- `--product-code`: FDA product code.
Exact source FDA product code; PMA, 510(k) and device recalls only. Value form: string; example: --product-code ABC; only with --source fda-pma, fda-510k or fda-device-recall.
- `--source`: Source diagnostic source.
Explicit source GTR, WHO IVD, FDA PMA, 510(k), companion or medical device recall discovery. Value form: string; example: --source fda-device-recall.
- `--type`: Source test_type.
Exact source test_type value. Value form: string; example: --type Research; only with --source gtr.

## biomcp search adverse-event

```text
Search five report headers by default. MAUDE accepts device text or a manufacturer, product-code, problem or event-type filter. One report may list several devices; filters may match different devices. VAERS accepts MMR, COVID19, COVID-19 vaccine or one exact qualified native influenza code, matched without case; use influenza to list codes and labels. Tables remain separate and are never summed. One report may list several vaccines. VAERS returns five reported symptom rows by default; seriousness and age select independent tables. Counts are reports, not people. Reports do not show that a vaccine caused an event and cannot measure risk or compare vaccine safety.

Usage: biomcp search adverse-event [OPTIONS] [text] [section]

Arguments:
  [text]
          Drug, device or vaccine.
          FAERS drug name, searched with the drug's other names unless --exact-term; MAUDE device phrase or exact supported VAERS vaccine, both searched as typed.

  [section]
          VAERS table; help search adverse-event

Options:
      --aggregate <aggregate>
          Report counts; help search adverse-event
          Report counts.
          reactions: FAERS reaction-term buckets; needs MedDRA. received-year or received-month: FAERS or MAUDE reports per FDA receipt period between --received-from and --received-to, same names and filters as the search; partly covered periods are marked partial. Counts are reports, not rates or causes. Requires offset zero.

      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --drug <drug>...
          FAERS drug names; OR.
          FAERS drug name, widened like the main text; repeat --drug for OR. A standalone OR joining names inside one --drug value is refused. Other filters AND with the names. BioMCP caps the escaped native query, including filters, at FAERS_QUERY_MAX_BYTES (4096 bytes).

      --event-type <event_type>
          MAUDE event type; help search adverse-event
          MAUDE event type.
          MAUDE report event type on native event_type: malfunction, injury, death or other.

      --exact-term[=<exact_term>]
          Search typed drug names only.
          FAERS searches each typed drug name together with the drug's other names by default; this searches only the names as typed. MAUDE and VAERS always search the text as typed.

          [default: false]
          [possible values: true, false]

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --expand-drug-names[=<expand_drug_names>]
          FAERS no-op.
          FAERS: accepted for one release and changes nothing; FAERS widens drug names by default.

          [default: false]
          [possible values: true, false]

      --limit <limit>
          Report headers, reaction terms or VAERS rows; default five.

      --manufacturer <manufacturer>
          MAUDE manufacturer.
          MAUDE manufacturer phrase on device.manufacturer_d_name. A report may list several devices; filters may match different devices.

      --name-snapshot <name_snapshot>
          FAERS name snapshot.
          Retained FAERS name resolution; follow the returned command without changing filters.

      --offset <offset>
          Report row offset; counts need zero. VAERS needs its snapshot.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --problem <problem>
          MAUDE product problem.
          MAUDE phrase on native product_problems.

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

      --product-code <product_code>...
          MAUDE product codes; OR.
          MAUDE exact native device.device_report_product_code; repeated codes join with OR.

      --reaction <reaction>
          FAERS MedDRA phrase.
          FAERS MedDRA term phrase match, not exact equality.

      --received-from <received_from>
          First date, YYYY-MM[-DD].
          Inclusive FAERS or MAUDE FDA receipt lower bound, YYYY-MM-DD, YYYY-MM or YYYY; a month or year starts on its first day.

      --received-to <received_to>
          Last date, YYYY-MM[-DD].
          Inclusive FAERS or MAUDE FDA receipt upper bound, YYYY-MM-DD, YYYY-MM or YYYY; a month or year ends on its last day.

      --serious <serious>
          FAERS seriousness flag; help search adverse-event
          FAERS seriousness flag.
          FAERS report flag: any uses the source seriousness flag; the specific flags can overlap. Never a reaction outcome.

      --snapshot <snapshot>
          VAERS snapshot.
          Retained VAERS table for provider-free continuation.

      --sort <sort>
          VAERS order; help search adverse-event
          VAERS order.
          reports: BioMCP orders the whole CDC WONDER table, not just the page, by its Events Reported count, largest first; equal counts keep the source's order. Counts, percents and totals stay the source's. Omit it for the source's own order, alphabetical for symptoms.

      --source <source>
          Source; default FAERS; help search adverse-event
          Source; default FAERS.
          FAERS by default; VAERS accepts only exact supported vaccine names.

```

Search drug, device and vaccine adverse events. Sources: faers, maude, vaers.
Listed choices accept any letter case; replies use their listed spelling.
Options apply to every listed source unless their description restricts them.

- `--aggregate`: Report counts.
reactions: FAERS reaction-term buckets; needs MedDRA. received-year or received-month: FAERS or MAUDE reports per FDA receipt period between --received-from and --received-to, same names and filters as the search; partly covered periods are marked partial. Counts are reports, not rates or causes. Requires offset zero. Value form: string; example: --aggregate reactions.
- `--drug`: FAERS drug names; OR.
FAERS drug name, widened like the main text; repeat --drug for OR. A standalone OR joining names inside one --drug value is refused. Other filters AND with the names. BioMCP caps the escaped source query, including filters, at FAERS_QUERY_MAX_BYTES (4096 bytes). Value form: repeatable values (repeat the flag); example: --drug "synthetic\\B".
- `--event-type`: MAUDE event type.
MAUDE report event type on source event_type: malfunction, injury, death or other. Value form: string; example: --event-type malfunction.
- `--exact-term`: Search typed drug names only.
FAERS searches each typed drug name together with the drug's other names by default; this searches only the names as typed. MAUDE and VAERS always search the text as typed. Value form: boolean; example: --exact-term.
- `--expand-drug-names`: FAERS no-op.
FAERS: accepted for one release and changes nothing; FAERS widens drug names by default. Value form: boolean; example: --expand-drug-names.
- `--limit`: Report headers, reaction terms or VAERS rows; default five. Value form: integer; example: --limit 1.
- `--manufacturer`: MAUDE manufacturer.
MAUDE manufacturer phrase on device.manufacturer_d_name. A report may list several devices; filters may match different devices. Value form: string; example: --manufacturer 'Synthetic maker'.
- `--name-snapshot`: FAERS name snapshot.
Retained FAERS name resolution; follow the returned command without changing filters. Value form: string; example: --name-snapshot <name-snapshot> (syntax; replace the placeholder with your retained value).
- `--offset`: Report row offset; counts need zero. VAERS needs its snapshot. Value form: integer; example: --offset 0.
- `--problem`: MAUDE product problem.
MAUDE phrase on source product_problems. Value form: string; example: --problem 'Synthetic problem'.
- `--product-code`: MAUDE product codes; OR.
MAUDE exact source device.device_report_product_code; repeated codes join with OR. Value form: repeatable values (repeat the flag); example: --product-code ABC.
- `--reaction`: FAERS MedDRA phrase.
FAERS MedDRA term phrase match, not exact equality. Value form: string; example: --reaction 'synthetic reaction'.
- `--received-from`: First date, YYYY-MM[-DD].
Inclusive FAERS or MAUDE FDA receipt lower bound, YYYY-MM-DD, YYYY-MM or YYYY; a month or year starts on its first day. Value form: string; example: --received-from 2024-01-01.
- `--received-to`: Last date, YYYY-MM[-DD].
Inclusive FAERS or MAUDE FDA receipt upper bound, YYYY-MM-DD, YYYY-MM or YYYY; a month or year ends on its last day. Value form: string; example: --received-to 2024-12-31.
- `section`: VAERS table.
Form: search adverse-event TEXT SECTION; not --section.
- `--serious`: FAERS seriousness flag.
FAERS report flag: any uses the source seriousness flag; the specific flags can overlap. Never a reaction outcome. Value form: string; example: --serious death.
- `--snapshot`: VAERS snapshot.
Retained VAERS table for source-free continuation. Value form: string; example: --snapshot <snapshot> (syntax; replace the placeholder with your retained value).
- `--sort`: VAERS order.
reports: BioMCP orders the whole CDC WONDER table, not just the page, by its Events Reported count, largest first; equal counts keep the source's order. Counts, percents and totals stay the source's. Omit it for the source's own order, alphabetical for symptoms. Value form: string; example: --sort reports; only with --source vaers.
- `--source`: Source; default FAERS.
FAERS by default; VAERS accepts only exact supported vaccine names. Value form: string; example: --source maude.
- `text`: Drug, device or vaccine.
FAERS drug name, searched with the drug's other names unless --exact-term; MAUDE device phrase or exact supported VAERS vaccine, both searched as typed.
Form: search adverse-event TEXT SECTION; not --text.

Independent VAERS table; omitted means reported symptoms.

- `symptoms`: Source reported symptom rows.
- `seriousness`: Reports classified by source seriousness.
- `age`: Source age groups.

## biomcp search disease

```text
Literal discovery text or an explicit SEER catalog page.

Usage: biomcp search disease [OPTIONS] [text]

Arguments:
  [text]
          Literal source search text; optional site-name filter for SEER.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --limit <limit>
          Maximum source rows on one page.

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --offset <offset>
          Native row offset.

      --ontology <ontology>
          Exactly one ontology, default MONDO; help search disease
          Exactly one ontology, default MONDO.

      --source <source>
          One explicit source, default OLS; help search disease
          One explicit source, default OLS.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Search diseases and their identifiers. Sources: monarch, mydisease, ols, seer.
Listed choices accept any letter case; replies use their listed spelling.
Options apply to every listed source unless their description restricts them.

- `--limit`: Maximum source rows on one page. Value form: integer; example: --limit 1.
- `--offset`: Source row offset. Value form: integer; example: --offset 1.
- `--ontology`: Exactly one ontology, default MONDO. Value form: string; example: --ontology mondo.
- `--source`: One explicit source, default OLS. Value form: string; example: --source seer.
- `text`: Exact source search text; optional site-name filter for SEER.
Form: search disease TEXT; not --text.

## biomcp search drug

```text
Search one drug source. FDA drug recalls accept a name or native firm, classification, status, reason and initiation dates; counts are recall records. WHO prequalification accepts a name, area or date range. openFDA labels also accept literal indication wording. Drugs@FDA accepts a name or an applicant. Other sources require a name.

Usage: biomcp search drug [OPTIONS] [query]

Arguments:
  [query]
          Drug name phrase.
          FDA drug recalls, WHO date ranges, label indication and Drugs@FDA applicant searches allow no name.
          Drugs@FDA searches brands and ingredients,
          exact names before word matches. openFDA labels search exact then word brand/generic names,
          then label text (including indications) when no names match; source matches need not be exact names.
          Harmonized recall names match but never become display names.

Options:
      --applicant <applicant>
          Applicant name start; Drugs@FDA only.
          Matches native sponsor_name values starting with this text, ignoring letter case;
          no company-family or subsidiary inference. An optional name narrows the same search.

      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --approved-from <approved_from>
          Original approval from, inclusive; Drugs@FDA only.
          Accepts YYYY, YYYY-MM or YYYY-MM-DD. Keeps applications whose original_approval date
          falls in the window; BioMCP reads every match and counts the kept ones.

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --approved-to <approved_to>
          Original approval to, inclusive; Drugs@FDA only.
          Accepts YYYY, YYYY-MM or YYYY-MM-DD.

      --classification <classification>
          Native FDA classification; drug recalls only; help search drug
          Native FDA classification; drug recalls only.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --exact-term[=<exact_term>]
          Search the typed name only.
          openFDA labels, Drugs@FDA, FDA drug recalls and EMA search the typed name together with
          the drug's other names by default; this searches only the name as typed. Other sources search the name as typed.

          [default: false]
          [possible values: true, false]

      --expand-drug-names[=<expand_drug_names>]
          Accepted for one release; changes nothing.
          Sources that widen drug names do so by default.

          [default: false]
          [possible values: true, false]

      --firm <firm>
          Recall firm phrase.
          Matches the published firm literally.

      --indication <indication>
          Literal label indication; openFDA labels only.
          Search the native Indications and Usage field without disease synonym expansion.
          An optional name query further narrows the same source search.

      --initiated-from <initiated_from>
          Inclusive initiation day, YYYY-MM-DD; drug recalls only.

      --initiated-to <initiated_to>
          Inclusive initiation day, YYYY-MM-DD; drug recalls only.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

      --limit <limit>
          Native candidate rows per page.

      --name-selector <name_selector>
          ChEMBL name selector; help search drug
          ChEMBL name selector.
          Omit to search the preferred name.

      --name-snapshot <name_snapshot>
          Label name page token.
          Follow the returned command with unchanged filters to keep the first page's names and step.

      --offset <offset>
          Native rows to skip.

      --prequalified-from <prequalified_from>
          WHO PQ start date, inclusive.
          Accepts YYYY, YYYY-MM or YYYY-MM-DD.

      --prequalified-to <prequalified_to>
          WHO PQ end date, inclusive.
          Accepts YYYY, YYYY-MM or YYYY-MM-DD.

      --reason <reason>
          Recall reason phrase.

      --sort <sort>
          Drugs@FDA order; help search drug
          Drugs@FDA order.
          newest: each application's latest submission status date first, ties by application number;
          applications with no dated submission come last. That latest date is often a labeling or
          efficacy supplement, not the original approval; each row's original_approval gives that.
          original-approval: each row's original_approval date first, ties by application number;
          rows without one come last. BioMCP reads every match and orders them.
          Omit it for the source's own order.

      --source <source>
          Selected source; help search drug
          Selected source.

          After an empty name lookup, existing sources may supply aliases.

      --status <status>
          Literal recall status.

      --therapeutic-area <therapeutic_area>
          WHO PQ therapeutic area.
          Whole words of WHO's Therapeutic Area column, any letter case, such as Oncology.

```

Search drugs and medicines by name or source label indication. Sources: chembl, civic, ddinter, drugsfda, ema, fda-drug-recall, mychem, openfda-label, opentargets, who-pq.
Listed choices accept any letter case; replies use their listed spelling.
Options apply to every listed source unless their description restricts them.

- `--applicant`: Applicant name start; Drugs@FDA only.
Matches source sponsor_name values starting with this text, ignoring letter case;
no company-family or subsidiary inference. An optional name narrows the same search. Value form: string; example: --applicant 'ALNYLAM PHARMS INC'; only with --source drugsfda.
- `--approved-from`: Original approval from, inclusive; Drugs@FDA only.
Accepts YYYY, YYYY-MM or YYYY-MM-DD. Keeps applications whose original_approval date
falls in the window; BioMCP reads every match and counts the kept ones. Value form: string; example: --approved-from 2024-01-01; only with --source drugsfda.
- `--approved-to`: Original approval to, inclusive; Drugs@FDA only.
Accepts YYYY, YYYY-MM or YYYY-MM-DD. Value form: string; example: --approved-to 2023; only with --source drugsfda.
- `--classification`: Source FDA classification; drug recalls only. Value form: string; example: --classification 'Class II'; only with --source fda-drug-recall.
- `--exact-term`: Search the typed name only.
openFDA labels, Drugs@FDA, FDA drug recalls and EMA search the typed name together with
the drug's other names by default; this searches only the name as typed. Other sources search the name as typed. Value form: boolean; example: --exact-term.
- `--expand-drug-names`: Accepted for one release; changes nothing.
Sources that widen drug names do so by default. Value form: boolean; example: --expand-drug-names.
- `--firm`: Recall firm phrase.
Matches the published firm literally. Value form: string; example: --firm Apotex; only with --source fda-drug-recall.
- `--indication`: Exact label indication; openFDA labels only.
Search the source Indications and Usage field without disease synonym expansion.
An optional name query further narrows the same source search. Value form: string; example: --indication melanoma; only with --source openfda-label.
- `--initiated-from`: Inclusive initiation day, YYYY-MM-DD; drug recalls only. Value form: string; example: --initiated-from 2024-01-01; only with --source fda-drug-recall.
- `--initiated-to`: Inclusive initiation day, YYYY-MM-DD; drug recalls only. Value form: string; example: --initiated-to 2024-12-31; only with --source fda-drug-recall.
- `--limit`: Source candidate rows per page. Value form: integer; example: --limit 1.
- `--name-selector`: ChEMBL name selector.
Omit to search the preferred name. Value form: string; example: --name-selector preferred; only with --source chembl.
- `--name-snapshot`: Label name page token.
Follow the returned command with unchanged filters to keep the first page's names and step. Value form: string; example: --name-snapshot <name-snapshot> (syntax; replace the placeholder with your retained value).
- `--offset`: Source rows to skip. Value form: integer; example: --offset 1.
- `--prequalified-from`: WHO PQ start date, inclusive.
Accepts YYYY, YYYY-MM or YYYY-MM-DD. Value form: string; example: --prequalified-from 2020; only with --source who-pq.
- `--prequalified-to`: WHO PQ end date, inclusive.
Accepts YYYY, YYYY-MM or YYYY-MM-DD. Value form: string; example: --prequalified-to 2020; only with --source who-pq.
- `query`: Drug name phrase.
FDA drug recalls, WHO date ranges, label indication and Drugs@FDA applicant searches allow no name.
Drugs@FDA searches brands and ingredients,
exact names before word matches. openFDA labels search exact then word brand/generic names,
then label text (including indications) when no names match; source matches need not be exact names.
Harmonized recall names match but never become display names.
Form: search drug QUERY; not --query.
- `--reason`: Recall reason phrase. Value form: string; example: --reason 'Lack of Sterility Assurance'; only with --source fda-drug-recall.
- `--sort`: Drugs@FDA order.
newest: each application's latest submission status date first, ties by application number;
applications with no dated submission come last. That latest date is often a labeling or
efficacy supplement, not the original approval; each row's original_approval gives that.
original-approval: each row's original_approval date first, ties by application number;
rows without one come last. BioMCP reads every match and orders them.
Omit it for the source's own order. Value form: string; example: --sort newest; only with --source drugsfda.
- `--source`: Selected source.

After an empty name lookup, existing sources may supply aliases. Value form: string; example: --source ema.
- `--status`: Exact recall status. Value form: string; example: --status Ongoing; only with --source fda-drug-recall.
- `--therapeutic-area`: WHO PQ therapeutic area.
Whole words of WHO's Therapeutic Area column, any letter case, such as Oncology. Value form: string; example: --therapeutic-area oncology; only with --source who-pq.

## biomcp search gene

```text
Search genes by text, cytoband, or their intersection, with ClinGen dosage regions that match.

Usage: biomcp search gene [OPTIONS] [query]

Arguments:
  [query]
          Words matched against symbols, previous symbols, aliases and names; a ClinGen ISCA ID, such as ISCA-37400, returns that region.

Options:
      --band <band>
          A cytoband prefix. Also lists ClinGen dosage regions whose band span contains it, such as 15q12 in 15q11.2-q13.1.

      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --limit <limit>
          Number of results per page.

      --locus-group <locus_group>
          Optional HGNC locus group intersected with the selector; help search gene
          Optional HGNC locus group intersected with the selector.

      --offset <offset>
          Number of results to skip.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --region <region>
          A GRCh38 interval, such as chr16:29600000-30200000. Selects overlapping ClinGen dosage regions; it adds no genes.

      --regions-only[=<regions_only>]
          Return only the ClinGen dosage regions that the band, interval or ISCA ID matches, without gene rows.

          [default: false]
          [possible values: true, false]

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Search human genes by name, identifier or cytoband, and ClinGen dosage regions by cytoband or GRCh38 interval. Sources: clingen, hgnc.
Listed choices accept any letter case; replies use their listed spelling.
Options apply to every listed source unless their description restricts them.

- `--band`: A cytoband prefix. Also lists ClinGen dosage regions whose band span contains it, such as 15q12 in 15q11.2-q13.1. Value form: string; example: --band 9p21.3.
- `--limit`: Number of results per page. Value form: integer; example: --limit 50.
- `--locus-group`: Optional HGNC locus group intersected with the selector. Value form: string; example: --locus-group pseudogene.
- `--offset`: Number of results to skip. Value form: integer; example: --offset 24.
- `query`: Words matched against symbols, previous symbols, aliases and names; a ClinGen ISCA ID, such as ISCA-37400, returns that region.
Form: search gene QUERY; not --query.
- `--region`: A GRCh38 interval, such as chr16:29600000-30200000. Selects overlapping ClinGen dosage regions; it adds no genes. Value form: string; example: --region chr16:29600000-29700000.
- `--regions-only`: Return only the ClinGen dosage regions that the band, interval or ISCA ID matches, without gene rows. Value form: boolean; example: --regions-only.

## biomcp search article

```text
Checked search selectors and pool cursor.

Usage: biomcp search article [OPTIONS] [text]

Arguments:
  [text]
          Source search words.
          Words sent to each named source in its default fields; unsupported by LitVar, which requires a variant selector.

Options:
      --author <author>
          PubMed/Europe PMC author words.
          PubMed and Europe PMC only: author words sent to the source's author field.

      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --disease <disease>
          Disease words.
          Available for command sources except LitVar.

      --drug <drug>
          Drug words.
          Available for command sources except LitVar.

      --exact-term[=<exact_term>]
          Search the typed drug term without resolving its other names.

          [default: false]
          [possible values: true, false]

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --gene <gene>
          Gene symbol or ID.
          Gene input; PubMed resolves it through HGNC.

      --journal <journal>
          PubMed/Europe PMC journal words.
          PubMed and Europe PMC only: journal words sent to the source's journal field.

      --limit <limit>
          Maximum page length.

      --no-preprints[=<no_preprints>]
          Exclude preprints.
          PubMed and Europe PMC only: exclude preprints using the source's native filter.

          [default: false]
          [possible values: true, false]

      --offset <offset>
          Number of pool items to skip.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --open-access[=<open_access>]
          Native open access.
          PubMed, Europe PMC and Semantic Scholar only: require native open access; Semantic Scholar means a public PDF.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

      --pool <pool>
          Stable result snapshot.
          Stable snapshot from an earlier response.

      --exclude-retracted


      --include-retracted


      --since <since>
          First publication date.
          PubMed, Europe PMC and Semantic Scholar only: inclusive lower publication-date bound; PubMed matches electronic or print date.

      --sort <sort>
          rank, date or citations; help search article
          rank, date or citations.
          rank (default): native provider order; several sources take turns; native search cost.
          date: native search cost; PubMed means date added and Europe PMC means newest first; other sources refuse it.
          citations: most cited first across the whole pool; adds iCite calls, about 1 s per 1,000 PMIDs.
          All sources support citations.

      --source <source>
          Sources; default pubmed; help search article
          Sources; default pubmed.
          Sources to search in caller order, comma separated. Default: pubmed.
          Several sources take turns, remove duplicates and show every native rank.
          pubmed: keywords with PubMed MeSH mapping; dropped words appear as notices.
          europepmc: words in open full text and preprints; incidental mentions are possible.
          litvar: exact variant publications; requires --variant and a protein change needs --gene; unresolved inputs return candidates.
          semantic-scholar: broad discovery; a shared keyless pool may return 429; an optional free key raises the limit.
          litsense: sentence resemblance, not a match; unrelated hits and timeouts are possible.
          pubtator: entity and word search including full text; needs the right entity ID; --gene uses HGNC Entrez identity and cannot run without that join.

      --type <type>
          Publication type; help search article
          Publication type.
          PubMed and Europe PMC only: native publication-type filter.

      --until <until>
          Last publication date.
          PubMed, Europe PMC and Semantic Scholar only: inclusive upper publication-date bound; PubMed matches electronic or print date.

      --variant <variant>
          Variant words.

  -q, --query <query_alias>
          Alias for positional article text.

```

Search research articles and biomedical papers. Sources: europepmc, litsense, litvar, pubmed, pubtator, semantic-scholar.
Listed choices accept any letter case; replies use their listed spelling.
Options apply to every listed source unless their description restricts them.

- `--author`: PubMed/Europe PMC author words.
PubMed and Europe PMC only: author words sent to the source's author field. Value form: string; example: --author Carpten.
- `--disease`: Disease words.
Available for command sources except LitVar. Value form: string; example: --disease meningioma.
- `--drug`: Drug words.
Available for command sources except LitVar. Value form: string; example: --drug 'drug words'.
- `--exact-term`: Search the typed drug term without resolving its other names. Value form: boolean; example: --exact-term.
- `--gene`: Gene symbol or ID.
Gene input; PubMed resolves it through HGNC. Value form: string; example: --gene NF2.
- `--journal`: PubMed/Europe PMC journal words.
PubMed and Europe PMC only: journal words sent to the source's journal field. Value form: string; example: --journal 'journal words'.
- `--limit`: Maximum page length. Value form: integer; example: --limit 5.
- `--no-preprints`: Exclude preprints.
PubMed and Europe PMC only: exclude preprints using the source's source filter. Value form: boolean; example: --no-preprints.
- `--offset`: Number of pool items to skip. Value form: integer; example: --offset 5.
- `--open-access`: Source open access.
PubMed, Europe PMC and Semantic Scholar only: require source open access; Semantic Scholar means a public PDF. Value form: boolean; example: --open-access; only with --source pubmed, europepmc or semantic-scholar.
- `--pool`: Stable result snapshot.
Stable snapshot from an earlier response. Value form: string; example: --pool <pool> (syntax; replace the placeholder with your retained value).
- `--exclude-retracted`: Retraction filter.
Include retracted papers by default. Exclusion is supported only by PubMed and Europe PMC. Value form: boolean; example: --exclude-retracted.
- `--include-retracted`: Same field as --exclude-retracted; sets it to include. Value form: boolean; example: --include-retracted.
- `--since`: First publication date.
PubMed, Europe PMC and Semantic Scholar only: inclusive lower publication-date bound; PubMed matches electronic or print date. Value form: string; example: --since 2024; only with --source pubmed, europepmc or semantic-scholar.
- `--sort`: rank, date or citations.
rank (default): source order; several sources take turns; source search cost.
date: source search cost; PubMed means date added and Europe PMC means newest first; other sources refuse it.
citations: most cited first across the whole pool; adds iCite calls, about 1 s per 1,000 PMIDs.
All sources support citations. Value form: string; example: --sort date.
- `--source`: Sources; default pubmed.
Sources to search in caller order, comma separated. Default: pubmed.
Several sources take turns, remove duplicates and show every source rank.
pubmed: keywords with PubMed MeSH mapping; dropped words appear as notices.
europepmc: words in open full text and preprints; incidental mentions are possible.
litvar: exact variant publications; requires --variant and a protein change needs --gene; unresolved inputs return candidates.
semantic-scholar: broad discovery; a shared keyless pool may return 429; an optional free key raises the limit.
litsense: sentence resemblance, not a match; unrelated hits and timeouts are possible.
pubtator: entity and word search including full text; needs the right entity ID; --gene uses HGNC Entrez identity and cannot run without that join. Value form: repeatable values (repeat the flag); example: --source europepmc.
- `text`: Source search words.
Words sent to each named source in its default fields; unsupported by LitVar, which requires a variant selector.
Form: search article TEXT; not --text.
- `--type`: Publication type.
PubMed and Europe PMC only: source publication-type filter. Value form: string; example: --type review.
- `--until`: Last publication date.
PubMed, Europe PMC and Semantic Scholar only: inclusive upper publication-date bound; PubMed matches electronic or print date. Value form: string; example: --until 2020; only with --source pubmed, europepmc or semantic-scholar.
- `--variant`: Variant words. Value form: string; example: --variant rs121434592.
- `--query`: Alias for positional article text. Value form: string; example: --query <query> (syntax; replace the placeholder with your retained value).

## biomcp search trial

```text
Search ClinicalTrials.gov or keyed NCI with native counts. NCI --facility matches exact organization names; a listed site is not necessarily enrolling.

Usage: biomcp search trial [OPTIONS] [query]

Arguments:
  [query]
          Trial text.
          General registry discovery text.

Options:
      --age <age>
          Age in years.
          ClinicalTrials.gov only; unstated source bounds count as no limit.

      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --biomarker-code <biomarker_code>
          NCI NCIt code.
          Exact NCIt biomarker concept listed by NCI in trial criteria; NCI source only. This listing does not establish biomarker positivity or patient eligibility.

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --biomarker-use <biomarker_use>
          NCI criterion; help search trial
          NCI criterion.
          Filter by NCI's native inclusion or exclusion criterion. Check each result's biomarker entry and reported problems.

      --condition <condition>...
          Disease; NCI needs NCIt.
          Conditions joined with OR. NCI needs NCIt codes such as C3224; use --condition-id for MONDO or DOID identifiers.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --condition-id <condition_id>
          MONDO/DOID/NCIT:C.
          Exact MONDO or DOID CURIE resolved by MyDisease; NCI also accepts explicit NCIT:C codes.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --count-only[=<count_only>]
          Total.
          Source total only; no returned study rows.

          [default: false]
          [possible values: true, false]

      --country <country>...
          Country label; help search trial
          Country label.
          Exact ClinicalTrials.gov country label; repeat for OR. Trials must list a site in a named country. NCI uses its native country filter.

      --criteria <criteria>
          Criteria text.
          ClinicalTrials.gov only: search registry eligibility text with other variant spellings; no eligibility judgment.

      --distance <distance>
          Radius, mi or km.
          ClinicalTrials.gov only: source distance with explicit mi or km units.

      --exact-condition[=<exact_condition>]
          Exact condition.
          Ask ClinicalTrials.gov to restrict matching to the registered condition phrase; its support is partial.

          [default: false]
          [possible values: true, false]

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --exact-term[=<exact_term>]
          Search typed drug names and variant text as written, without other names or spellings.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

      --facility <facility>
          Facility.
          Facility phrase; ClinicalTrials.gov broadens after zero exact matches. NCI matches exact names. Cannot combine with radius.

      --gene <gene>
          Gene symbol/ID.
          ClinicalTrials.gov only: resolve one checked gene for registry text discovery; no eligibility judgment.

      --has-results[=<has_results>]
          Posted results.
          ClinicalTrials.gov only: require posted source results. Each row carries has_results.

          [default: false]
          [possible values: true, false]

      --id <id>
          NCT identifier.
          ClinicalTrials.gov only: exact NCT identity in a conjunction with other filters.

      --intervention <intervention>...
          Treatment terms; OR.
          Intervention alternatives joined with OR; supported by ClinicalTrials.gov.

      --keyword <keyword>...
          Text; OR.
          Repeat for ClinicalTrials.gov general text alternatives joined with OR; different filters combine with AND. NCI accepts one keyword.

      --lat <lat>
          Latitude degrees.
          ClinicalTrials.gov only: radius center latitude in degrees.

      --limit <limit>
          Page size.
          Headers per page; NCI applies its own lower publisher limit.

      --lon <lon>
          Longitude degrees.
          ClinicalTrials.gov only: radius center longitude in degrees.

      --next-page <next_page>
          Page token.
          ClinicalTrials.gov: follow the returned token, selectors and offset unchanged; unbound tokens need a fresh first page.

      --offset <offset>
          Page offset.
          Zero-based offset of the first row in this continuation page.

      --phase <phase>
          Native phase; help search trial
          Native phase.
          Exact source phase. ClinicalTrials.gov: 1, 2, 3, 4, early-1, not-applicable, 1/2, 2/3. NCI: NA, 0, I, I_II, II, II_III, III, IV.

      --sex <sex>
          ClinicalTrials.gov only: registry sex facet; help search trial
          ClinicalTrials.gov only: registry sex facet.

      --source <source>
          clinicaltrials/nci; help search trial
          clinicaltrials/nci.
          Explicit provider; defaults to ClinicalTrials.gov and never falls back.

      --sponsor <sponsor>
          Sponsor.
          ClinicalTrials.gov only: sponsor name; broadens after zero exact matches. Use --sponsor-type for the funding category.

      --sponsor-type <sponsor_type>
          Funding class; help search trial
          Funding class.
          ClinicalTrials.gov only: registry sponsor funding category; use --sponsor for a sponsor name.

      --status <status>
          Native status; help search trial
          Native status.
          Exact source status. NCI: Active, Administratively Complete, Approved, Closed to Accrual, Closed to Accrual and Intervention, Complete, INACTIVE, Enrolling by Invitation, Temporarily Closed to Accrual, Temporarily Closed to Accrual and Intervention, Withdrawn. ClinicalTrials.gov uses registry labels.

      --study-type <study_type>
          Study type; help search trial
          Study type.
          ClinicalTrials.gov only: exact registry study type.

      --updated-since <updated_since>
          First date.
          ClinicalTrials.gov only: first included registry update date.

      --updated-until <updated_until>
          Last date.
          ClinicalTrials.gov only: last included registry update date.

      --variant <variant>
          Variant text.
          ClinicalTrials.gov only: general text or criteria, with other spellings, and the codon with --gene; no eligibility judgment.

```

Search clinical trials. Sources: ctgov, nci.
Listed choices accept any letter case; replies use their listed spelling.
Options apply to every listed source unless their description restricts them.

- `--age`: Age in years.
ClinicalTrials.gov only; unstated source bounds count as no limit. Value form: integer; example: --age 18.
- `--biomarker-code`: NCI NCIt code.
Exact NCIt biomarker concept listed by NCI in trial criteria; NCI source only. This listing does not establish biomarker positivity or patient eligibility. Value form: string; example: --biomarker-code C96022.
- `--biomarker-use`: NCI criterion.
Filter by NCI's source inclusion or exclusion criterion. Check each result's biomarker entry and reported problems. Value form: string; example: --biomarker-use inclusion.
- `--condition`: Disease; NCI needs NCIt.
Conditions joined with OR. NCI needs NCIt codes such as C3224; use --condition-id for MONDO or DOID identifiers. Value form: repeatable values (repeat the flag); example: --condition melanoma.
- `--condition-id`: MONDO/DOID/NCIT:C.
Exact MONDO or DOID CURIE resolved by MyDisease; NCI also accepts explicit NCIT:C codes. Value form: string; example: --condition-id NCIT:C3224.
- `--count-only`: Total.
Source total only; no returned study rows. Value form: boolean; example: --count-only.
- `--country`: Country label.
Exact ClinicalTrials.gov country label; repeat for OR. Trials must list a site in a named country. NCI uses its source country filter. Value form: repeatable values (repeat the flag); example: --country 'United States'.
- `--criteria`: Criteria text.
ClinicalTrials.gov only: search registry eligibility text with other variant spellings; no eligibility judgment. Value form: string; example: --criteria 'exact criteria'.
- `--distance`: Radius, mi or km.
ClinicalTrials.gov only: source distance with explicit mi or km units. Value form: string; example: --distance 50mi.
- `--exact-condition`: Exact condition.
Ask ClinicalTrials.gov to restrict matching to the registered condition phrase; its support is partial. Value form: boolean; example: --exact-condition.
- `--exact-term`: Search typed drug names and variant text as written, without other names or spellings. Value form: boolean; example: --exact-term.
- `--facility`: Facility.
Facility phrase; ClinicalTrials.gov broadens after zero exact matches. NCI matches exact names. Cannot combine with radius. Value form: string; example: --facility 'Dana-Farber Cancer Institute'.
- `--gene`: Gene symbol/ID.
ClinicalTrials.gov only: resolve one checked gene for registry text discovery; no eligibility judgment. Value form: string; example: --gene BRAF.
- `--has-results`: Posted results.
ClinicalTrials.gov only: require posted source results. Each row carries has_results. Value form: boolean; example: --has-results.
- `--id`: NCT identifier.
ClinicalTrials.gov only: exact NCT identity in a conjunction with other filters. Value form: string; example: --id NCT05273554.
- `--intervention`: Treatment terms; OR.
Intervention alternatives joined with OR; supported by ClinicalTrials.gov. Value form: repeatable values (repeat the flag); example: --intervention pembrolizumab.
- `--keyword`: Text; OR.
Repeat for ClinicalTrials.gov general text alternatives joined with OR; different filters combine with AND. NCI accepts one keyword. Value form: repeatable values (repeat the flag); example: --keyword survival.
- `--lat`: Latitude degrees.
ClinicalTrials.gov only: radius center latitude in degrees. Value form: string; example: --lat 42.3601.
- `--limit`: Page size.
Headers per page; NCI applies its own lower publisher limit. Value form: integer; example: --limit 1.
- `--lon`: Longitude degrees.
ClinicalTrials.gov only: radius center longitude in degrees. Value form: string; example: --lon -71.0589.
- `--next-page`: Page token.
ClinicalTrials.gov: follow the returned token, selectors and offset unchanged; unbound tokens need a fresh first page. Value form: string; example: --next-page <next-page> (syntax; replace the placeholder with your retained value).
- `--offset`: Page offset.
Zero-based offset of the first row in this continuation page. Value form: integer; example: --offset 0.
- `--phase`: Source phase.
Exact source phase. ClinicalTrials.gov: 1, 2, 3, 4, early-1, not-applicable, 1/2, 2/3. NCI: NA, 0, I, I_II, II, II_III, III, IV. Value form: string; example: --phase 3.
- `query`: Trial text.
General registry discovery text.
Form: search trial QUERY; not --query.
- `--sex`: ClinicalTrials.gov only: registry sex facet. Value form: string; example: --sex ALL.
- `--source`: clinicaltrials/nci.
Explicit source; defaults to ClinicalTrials.gov and never falls back. Value form: string; example: --source nci.
- `--sponsor`: Sponsor.
ClinicalTrials.gov only: sponsor name; broadens after zero exact matches. Use --sponsor-type for the funding category. Value form: string; example: --sponsor 'MD Anderson'.
- `--sponsor-type`: Funding class.
ClinicalTrials.gov only: registry sponsor funding category; use --sponsor for a sponsor name. Value form: string; example: --sponsor-type other.
- `--status`: Source status.
Exact source status. NCI: Active, Administratively Complete, Approved, Closed to Accrual, Closed to Accrual and Intervention, Complete, INACTIVE, Enrolling by Invitation, Temporarily Closed to Accrual, Temporarily Closed to Accrual and Intervention, Withdrawn. ClinicalTrials.gov uses registry labels. Value form: string; example: --status RECRUITING.
- `--study-type`: Study type.
ClinicalTrials.gov only: exact registry study type. Value form: string; example: --study-type INTERVENTIONAL.
- `--updated-since`: First date.
ClinicalTrials.gov only: first included registry update date. Value form: string; example: --updated-since <updated-since> (syntax; replace the placeholder with your retained value).
- `--updated-until`: Last date.
ClinicalTrials.gov only: last included registry update date. Value form: string; example: --updated-until <updated-until> (syntax; replace the placeholder with your retained value).
- `--variant`: Variant text.
ClinicalTrials.gov only: general text or criteria, with other spellings, and the codon with --gene; no eligibility judgment. Value form: string; example: --variant V600E.

## biomcp search variant

```text
Discover source candidates using one declared selector, without interpreting free text.

Usage: biomcp search variant [OPTIONS]

Options:
      --build <build>
          Optional caller assembly sent to MyVariant; regions; help search variant
          Optional caller assembly sent to MyVariant; regions require it explicitly.

      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --coding <coding>
          Coding HGVS change searched literally in ClinVar with --gene, without choosing a transcript.

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --condition <condition>
          Literal MyVariant clinvar.rcv.conditions.name phrase; source text is not a local diagnosis. Requires a declared protein or region selector.

      --gene <gene>
          Checked gene symbol for the dbNSFP substitution or ClinVar coding/synonymous selector.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --has <has>
          Require the selected component's identifier field. Only; help search variant
          Require the selected component's identifier field. Only source-proven mappings are offered.

      --limit <limit>
          Native candidates per page, default 10, maximum 50; all candidates remain candidates.

      --offset <offset>
          Native offset within this command's bounded result window; larger native totals remain visible.

      --protein <protein>
          Explicit substitution or synonymous change using checked amino-acid codes.

      --query <query>
          Raw MyVariant query, passed and echoed unchanged when component policy allows; requires --source myvariant. With disabled components use declared selectors instead.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --region <region>
          Exact one-based inclusive MyVariant region, such as chr7:55259500-55259600; requires --build.

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

      --review-status <review_status>
          Literal MyVariant clinvar.rcv.review_status phrase; no star conversion. Requires a declared protein or region selector.

      --source <source>
          Explicit source language; required for a raw query; help search variant
          Explicit source language; required for a raw query.

```

Search genetic variants by gene, change or region. Sources: myvariant.
Listed choices accept any letter case; replies use their listed spelling.
Options apply to every listed source unless their description restricts them.

- `--build`: Optional caller assembly sent to MyVariant; regions require it explicitly. Value form: string; example: --build GRCh37.
- `--coding`: Coding HGVS change searched literally in ClinVar with --gene, without choosing a transcript. Value form: string; example: --coding 'c.333G>T'.
- `--condition`: Exact MyVariant clinvar.rcv.conditions.name phrase; source text is not a local diagnosis. Requires a declared protein or region selector. Value form: string; example: --condition 'Carcinoma of colon (CRC)'.
- `--gene`: Checked gene symbol for the dbNSFP substitution or ClinVar coding/synonymous selector. Value form: string; example: --gene EGFR.
- `--has`: Require the selected component's identifier field. Only source-proven mappings are offered. Value form: string; example: --has clinvar.
- `--limit`: Source candidates per page, default 10, maximum 50; all candidates remain candidates. Value form: integer; example: --limit 50.
- `--offset`: Source offset within this command's bounded result window; larger source totals remain visible. Value form: integer; example: --offset 0.
- `--protein`: Explicit substitution or synonymous change using checked amino-acid codes. Value form: string; example: --protein L858R.
- `--query`: Raw MyVariant query, passed and echoed unchanged when component policy allows; requires --source myvariant. With disabled components use declared selectors instead. Value form: string; example: --query CA123643.
- `--region`: Exact one-based inclusive MyVariant region, such as chr7:55259500-55259600; requires --build. Value form: string; example: --region chr7:140453136-140453136.
- `--review-status`: Exact MyVariant clinvar.rcv.review_status phrase; no star conversion. Requires a declared protein or region selector. Value form: string; example: --review-status 'no assertion criteria provided'.
- `--source`: Explicit source language; required for a raw query. Value form: string; example: --source myvariant.

## biomcp get

```text
Retrieve one entity and its source sections.

Usage: biomcp get [OPTIONS] <COMMAND>

Commands:
  biomarker      Search NCI by exact NCIt code. Several native rows return candidates without choosing one. The public tier returns codes, name, type, trial count, synonyms, parent_ids, ancestor_ids and eligibility_criterion. No clinical interpretation is performed.
  author         Exact qualified records in caller order, with no name reconciliation.
  cell-line      One exact identity and an optional bounded named section.
  gwas           Source association or study and native context. Reports no personal risk, causal variant or gene, or clinical interpretation of a statistic.
  protein        Read source protein cards; a symbol returns candidates without choosing a protein.
                 Sections disclose native facts with independent row bounds; function keeps all paragraphs.
                 No variant effect, structure, interaction or clinical meaning is inferred. No bioinformatics tool runs.
  phenotype      Read an exact HP term from HPO or Monarch, or page native source sections. A term lookup answers terminology; an association answers what its publisher asserts. Neither establishes a diagnosis.
  pathway        Get an exact human Reactome R-HSA or WikiPathways WP card. Read whole summary, genes, Reactome events or bounded all sections. Membership names a source and release; it implies no therapy, response or actionability. No enrichment is calculated.
  study          Exact case-sensitive study card or bounded metadata section. No frequency, denominator, survival or comparison is calculated. Source counts keep their native names.
  pgx            PGx card or section. No phenotype, phase or dose is calculated.
  diagnostic     Get one exact source record or supported section. A listing is not authorization; a companion row is not a treatment recommendation.
  adverse-event  Exact native report identity and an optional independently paged section.
  disease        Exact concept or native SEER site with independently paged section.
  drug           Retrieve a drug by source-native ID, brand or generic name, with explicit source-scoped sections.
  gene           Get one to twenty human gene inputs in their original order.
  article        Get one to twenty article inputs in caller order.
  trial          Retrieve exact trial cards in input order; request bounded intact source sections with independent continuations.
  variant        Read an rsID, CAid, clinvar:N, VCV accession, reference-qualified HGVS or VCF-style genomic coordinates such as 7-140753336-A-T through the shared resolver. MyVariant identity lookups default to GRCh38; --build overrides it. Chromosome genomic HGVS requires --build. Use search variant --gene BRAF --protein V600E or search variant --gene PTEN --coding c.333G>T for gene shorthand. Every candidate remains available; source assertions do not classify the variant for the caller.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Retrieve one entity and its source sections.


## biomcp get biomarker

```text
Search NCI by exact NCIt code. Several native rows return candidates without choosing one. The public tier returns codes, name, type, trial count, synonyms, parent_ids, ancestor_ids and eligibility_criterion. No clinical interpretation is performed.

Usage: biomcp get biomarker [OPTIONS] <id>

Arguments:
  <id>
          Exact NCIt code, such as C96022.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --source <source>
          NCI is the biomarker concept source; help get biomarker
          NCI is the biomarker concept source.

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Get NCI biomarker concepts by code. Sources: nci.
Listed choices accept any letter case; replies use their listed spelling.
Options apply to every listed source unless their description restricts them.

- `id`: Exact NCIt code, such as C96022.
Form: get biomarker ID; not --id.
- `--source`: NCI is the biomarker concept source. Value form: string; example: --source nci.

## biomcp get author

```text
Exact qualified records in caller order, with no name reconciliation.

Usage: biomcp get author [OPTIONS] <ids>...

Arguments:
  <ids>...
          One or more exact qualified IDs.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Get research author details. Sources: orcid, semantic-scholar.

- `ids`: One or more exact qualified IDs.
Form: get author IDS...; not --ids.

## biomcp get cell-line

```text
One exact identity and an optional bounded named section.

Usage: biomcp get cell-line [OPTIONS] <id> [section]

Arguments:
  <id>
          One exact primary accession or qualified source ID.

  [section]
          Omit for the default card; all returns bounded first pages; help get cell-line

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --section-limit <section_limit>
          Explicit row limit for each requested collection; native variation defaults to three.

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --section-offset <section_offset>
          Offset within each independently requested collection.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Get cell line details and cross-references. Sources: cellosaurus.
Listed choices accept any letter case; replies use their listed spelling.
Options apply to every listed source unless their description restricts them.

- `id`: One exact primary accession or qualified source ID.
Form: get cell-line ID SECTION; not --id.
- `section`: Omit for the default card; all returns bounded first pages… (full rule with the sections).
Form: get cell-line ID SECTION; not --section.
- `--section-limit`: Explicit row limit for each requested collection; source variation defaults to three. Value form: integer; example: --section-limit 25.
- `--section-offset`: Offset within each independently requested collection. Value form: integer; example: --section-offset 1.

Omit for the default card; all returns bounded first pages without an assay count.

- `names`: Source display names.
- `identity`: Independent species, diseases and qualified warnings.
- `xrefs`: Complete source cross-reference rows.
- `variants`: Complete source variation rows.
- `chembl`: Exact accession-linked ChEMBL cell metadata.
- `all`: Bounded first page of every named section.

## biomcp get gwas

```text
Source association or study and native context. Reports no personal risk, causal variant or gene, or clinical interpretation of a statistic.

Usage: biomcp get gwas [OPTIONS] <id> [section]

Arguments:
  <id>
          Qualified association ID or exact GCST study accession.

  [section]
          Association loci or study associations and ancestries; help get gwas

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --section-limit <section_limit>
          Maximum native relation rows.

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --section-offset <section_offset>
          Native relation row offset.

      --snapshot <snapshot>
          Retained ancestry reply for a local page.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Get GWAS Catalog associations or studies. Sources: gwas.
Listed choices accept any letter case; replies use their listed spelling.
Options apply to every listed source unless their description restricts them.

- `id`: Qualified association ID or exact GCST study accession.
Form: get gwas ID SECTION; not --id.
- `section`: Association loci or study associations and ancestries… (full rule with the sections).
Form: get gwas ID SECTION; not --section.
- `--section-limit`: Maximum source relation rows. Value form: integer; example: --section-limit 1.
- `--section-offset`: Source relation row offset. Value form: integer; example: --section-offset 0.
- `--snapshot`: Retained ancestry reply for a local page. Value form: string; example: --snapshot <snapshot> (syntax; replace the placeholder with your retained value).

Association loci or study associations and ancestries.

- `loci`: Source association loci.
- `associations`: Source study association pages.
- `ancestries`: Source study ancestry rows.

## biomcp get protein

```text
Read source protein cards; a symbol returns candidates without choosing a protein.
Sections disclose native facts with independent row bounds; function keeps all paragraphs.
No variant effect, structure, interaction or clinical meaning is inferred. No bioinformatics tool runs.

Usage: biomcp get protein [OPTIONS] <input>...

Arguments:
  <input>...
          Source accessions or gene symbols, preserving caller order.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --section <section>
          Select native sections or all bounded first pages; function; help get protein

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --section-limit <section_limit>
          First-page row limit; function keeps one whole native text record and does not page its paragraphs.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Get protein details and source annotations. Sources: complex-portal, hgnc, interpro, uniprot.
Listed choices accept any letter case; replies use their listed spelling.
Options apply to every listed source unless their description restricts them.

- `input`: Source accessions or gene symbols, preserving caller order.
Form: get protein INPUT...; not --input.
- `--section`: Select source sections or all bounded first pages; function… (full rule with the sections). Value form: repeatable values (repeat the flag); example: --section function.
- `--section-limit`: First-page row limit; function keeps one whole source text record and does not page its paragraphs. Value form: integer; example: --section-limit 2.

Select source sections or all bounded first pages; function keeps one whole text record.

- `features`: Source UniProt features.
- `domains`: Source InterPro entry matches.
- `isoforms`: Source alternative products.
- `complexes`: Exact participant-admitted complex headers.
- `structures`: Source structure cross-reference links.
- `function`: One whole source function text record with all source paragraphs.
- `sequence`: Direct exact-identity sequence and audit metadata.
- `all`: Bounded first page of every named section.

## biomcp get phenotype

```text
Read an exact HP term from HPO or Monarch, or page native source sections. A term lookup answers terminology; an association answers what its publisher asserts. Neither establishes a diagnosis.

Usage: biomcp get phenotype [OPTIONS] <id> [section]

Arguments:
  <id>
          Exact HP identifier.

  [section]
          Absent means the source identity card; help get phenotype

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --section-limit <section_limit>
          Default five text/link rows, one complete association row or 50 ancestors.

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --section-offset <section_offset>
          Native section row offset.

      --snapshot <snapshot>
          Retained exact source reply for local text and link pages; unsupported for diseases, genes and ancestors.

      --source <source>
          HPO or Monarch for cards, synonyms and xrefs; Monarch only; help get phenotype
          HPO or Monarch for cards, synonyms and xrefs; Monarch only for hierarchy, diseases and genes; OLS only for ancestors. Defaults to HPO for cards, synonyms and xrefs, OLS for ancestors, otherwise Monarch.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Get human phenotype details and relationships. Sources: hpo, monarch, ols.
Listed choices accept any letter case; replies use their listed spelling.
Options apply to every listed source unless their description restricts them.

- `id`: Exact HP identifier.
Form: get phenotype ID SECTION; not --id.
- `section`: Absent means the source identity card… (full rule with the sections).
Form: get phenotype ID SECTION; not --section.
- `--section-limit`: Default five text/link rows, one complete association row or 50 ancestors. Value form: integer; example: --section-limit 1.
- `--section-offset`: Source section row offset. Value form: integer; example: --section-offset 0.
- `--snapshot`: Retained exact source reply for local text and link pages; unsupported for diseases, genes and ancestors. Value form: string; example: --snapshot <snapshot> (syntax; replace the placeholder with your retained value).
- `--source`: HPO or Monarch for cards, synonyms and xrefs; Monarch only for hierarchy, diseases and genes; OLS only for ancestors. Defaults to HPO for cards, synonyms and xrefs, OLS for ancestors, otherwise Monarch. Value form: string; example: --source monarch.

Absent means the source identity card.

- `synonyms`: Source synonyms.
- `xrefs`: Source cross references.
- `hierarchy`: Source direct parent and child links.
- `diseases`: Source disease association rows.
- `genes`: Source human gene association rows.
- `ancestors`: Every ancestor term up to the ontology root, from OLS.

## biomcp get pathway

```text
Get an exact human Reactome R-HSA or WikiPathways WP card. Read whole summary, genes, Reactome events or bounded all sections. Membership names a source and release; it implies no therapy, response or actionability. No enrichment is calculated.

Usage: biomcp get pathway [OPTIONS] <id> [section]

Arguments:
  <id>
          Exact human Reactome or WikiPathways identity without version suffixes.

  [section]
          Omit for the card; WikiPathways has summary and genes; help get pathway

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --fresh[=<fresh>]
          Request a new source observation.

          [default: false]
          [possible values: true, false]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --pool <pool>
          Retained snapshot, when continuing a source relation.

      --section-limit <section_limit>
          Bounded initial section size; each all section has its own first page.

      --section-offset <section_offset>
          Native relation offset for a selected genes or events section.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

```

Get human pathway details. Sources: reactome, wikipathways.
Listed choices accept any letter case; replies use their listed spelling.
Options apply to every listed source unless their description restricts them.

- `--fresh`: Request a new source observation. Value form: boolean; example: --fresh.
- `id`: Exact human Reactome or WikiPathways identity without version suffixes.
Form: get pathway ID SECTION; not --id.
- `--pool`: Retained snapshot, when continuing a source relation. Value form: string; example: --pool <pool> (syntax; replace the placeholder with your retained value).
- `section`: Omit for the card; WikiPathways has summary and genes… (full rule with the sections).
Form: get pathway ID SECTION; not --section.
- `--section-limit`: Bounded initial section size; each all section has its own first page. Value form: integer; example: --section-limit 5.
- `--section-offset`: Source relation offset for a selected genes or events section. Value form: integer; example: --section-offset 0.

Omit for the card; WikiPathways has summary and genes sections.

- `summary`: Complete source description.
- `genes`: Namespaced source participants with exact resolved gene joins.
- `events`: Source Reactome event objects and unresolved database references.
- `all`: Each source-supported section's bounded first page.

## biomcp get study

```text
Exact case-sensitive study card or bounded metadata section. No frequency, denominator, survival or comparison is calculated. Source counts keep their native names.

Usage: biomcp get study [OPTIONS] <id> [section]

Arguments:
  <id>
          Exact source study ID; no default or repair.

  [section]
          Omit for the card; all returns bounded first pages only; help get study

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --gene <gene>
          Exact cBioPortal HUGO gene symbol or Entrez ID; required for mutations.

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --molecular-profile <molecular_profile>
          Exact mutation profile ID; omitted selects the study's first mutation profile.

      --section-limit <section_limit>
          Maximum records in each selected section.

      --section-offset <section_offset>
          Offset within one named section; all requires zero.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Get cancer genomics study details. Sources: cbioportal.
Listed choices accept any letter case; replies use their listed spelling.
Options apply to every listed source unless their description restricts them.

- `--gene`: Exact cBioPortal HUGO gene symbol or Entrez ID; required for mutations. Value form: string; example: --gene BRAF.
- `id`: Exact source study ID; no default or repair.
Form: get study ID SECTION; not --id.
- `--molecular-profile`: Exact mutation profile ID; omitted selects the study's first mutation profile. Value form: string; example: --molecular-profile skcm_tcga_pan_can_atlas_2018_mutations.
- `section`: Omit for the card; all returns bounded first pages only… (full rule with the sections).
Form: get study ID SECTION; not --section.
- `--section-limit`: Maximum records in each selected section. Value form: integer; example: --section-limit 1.
- `--section-offset`: Offset within one named section; all requires zero. Value form: integer; example: --section-offset 45.

Omit for the card; all returns bounded first pages only.

- `profiles`: Five exact source molecular profile references.
- `sample-lists`: Five sample-list metadata records, without member sample IDs.
- `mutations`: Source mutation rows for the required gene and selected mutation profile.
- `samples`: Source sample identities and types only.
- `files`: Five repository file references; no automatic download.
- `all`: Bounded first pages; mutations are included only when gene is supplied.

## biomcp get pgx

```text
PGx card or section. No phenotype, phase or dose is calculated.

Usage: biomcp get pgx [OPTIONS] --source <source> [section]

Arguments:
  [section]
          One named section; omitted means the CPIC gene card; help get pgx

Options:
      --annotation-id <annotation_id>
          Checked ClinPGx accession, never a shared PaId.

      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --diplotype <diplotype>
          Unchanged declared expression required for the diplotype section only.

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --drug-id <drug_id>
          Exact published CPIC drug key; recommendations only.

      --gene <gene>
          Explicit source gene symbol.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --kind <kind>
          One ClinPGx collection; clinical is the default only on; help get pgx
          One ClinPGx collection; clinical is the default only on annotations.

      --section-limit <section_limit>
          Rows per page: three clinical, five elsewhere.

      --section-offset <section_offset>
          Offset within the selected section.

      --snapshot <snapshot>
          Retained collection for local continuation.

      --source <source>
          Required source, checked against the section; help get pgx
          Required source, checked against the section.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Get pharmacogenomic evidence and source sections. Sources: clinpgx, cpic.
Listed choices accept any letter case; replies use their listed spelling.
Options apply to every listed source unless their description restricts them.

- `--annotation-id`: Checked ClinPGx accession, never a shared PaId. Value form: string; example: --annotation-id <annotation-id> (syntax; replace the placeholder with your retained value); only with --source clinpgx.
- `--diplotype`: Unchanged declared expression required for the diplotype section only. Value form: string; example: --diplotype '*1/*2A'; only with --source cpic.
- `--drug-id`: Exact published CPIC drug key; recommendations only. Value form: string; example: --drug-id RxNorm:343047; only with --source cpic.
- `--gene`: Explicit source gene symbol. Value form: string; example: --gene DPYD.
- `--kind`: One ClinPGx collection; clinical is the default only on annotations. Value form: string; example: --kind clinical; only with --source clinpgx.
- `section`: One named section; omitted means the CPIC gene card… (full rule with the sections).
Form: get pgx SECTION; not --section.
- `--section-limit`: Rows per page: three clinical, five elsewhere. Value form: integer; example: --section-limit 1.
- `--section-offset`: Offset within the selected section. Value form: integer; example: --section-offset 0.
- `--snapshot`: Retained collection for local continuation. Value form: string; example: --snapshot <snapshot> (syntax; replace the placeholder with your retained value); only with --source clinpgx.
- `--source`: Required source, checked against the section. Value form: string; example: --source cpic.

One named section; omitted means the CPIC gene card.

- `recommendations`: Complete source recommendation contexts.
- `guidelines`: Source guideline references.
- `diplotype`: Declared source diplotype lookup.
- `annotations`: One ClinPGx annotation kind.

## biomcp get diagnostic

```text
Get one exact source record or supported section. A listing is not authorization; a companion row is not a treatment recommendation.

Usage: biomcp get diagnostic [OPTIONS] --source <source> <id> [section]

Arguments:
  <id>
          Exact native identity, including version for GTR.

  [section]
          GTR: genes, conditions, methods. PMA: regulatory,; help get diagnostic

Options:
      --bundle-revision <bundle_revision>
          Immutable stored bundle revision retained by a continuation.

      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --section-limit <section_limit>
          Maximum native rows; omitted means five rows, or one regulatory fragment.

      --section-offset <section_offset>
          Zero-based native row or fragment offset.

      --source <source>
          Explicit source: GTR accession/version, WHO Product ID,; help get diagnostic
          Explicit source: GTR accession/version, WHO Product ID, PMA P number,
          510(k) K or De Novo DEN number or device recall Z-NNNN-YYYY.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --supplement <supplement>
          Exact native PMA supplement; absent selects the base row.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Get diagnostic test or device details. Sources: fda-510k, fda-device-recall, fda-pma, gtr, who-ivd.
Listed choices accept any letter case; replies use their listed spelling.
Options apply to every listed source unless their description restricts them.

- `--bundle-revision`: Immutable stored bundle revision retained by a continuation. Value form: string; example: --bundle-revision <bundle-revision> (syntax; replace the placeholder with your retained value).
- `id`: Exact source identity, including version for GTR.
Form: get diagnostic ID SECTION; not --id.
- `section`: GTR: genes, conditions, methods. PMA: regulatory,… (full rule with the sections).
Form: get diagnostic ID SECTION; not --section.
- `--section-limit`: Maximum source rows; omitted means five rows, or one regulatory fragment. Value form: integer; example: --section-limit 2.
- `--section-offset`: Zero-based source row or fragment offset. Value form: integer; example: --section-offset 5.
- `--source`: Explicit source: GTR accession/version, WHO Product ID, PMA P number,
510(k) K or De Novo DEN number or device recall Z-NNNN-YYYY. Value form: string; example: --source who-ivd.
- `--supplement`: Exact source PMA supplement; absent selects the base row. Value form: string; example: --supplement S020.

GTR: genes, conditions, methods. PMA: regulatory, supplements, summary.
WHO, 510(k) and device recalls have no sections; omit for the card.

- `genes`: GTR source gene rows; five per page.
- `conditions`: GTR source condition rows; five per page.
- `methods`: GTR source method entries; five per page.
- `summary`: FDA SSED PDF text: 25 MiB, 100 pages and 6144 escaped bytes per fragment; scans without text are explicit.
- `regulatory`: Selected PMA submission statement; one fragment per page.
- `supplements`: PMA base application supplement headers; five per page.

## biomcp get adverse-event

```text
Exact native report identity and an optional independently paged section.

Usage: biomcp get adverse-event [OPTIONS] <id> [section]

Arguments:
  <id>
          Published report ID with no source prefix.

  [section]
          FAERS owns drugs, reactions and identifiers; MAUDE owns; help get adverse-event

Options:
      --drug-index <drug_index>
          Absolute source-array index; required only for identifier sections.

      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --section-limit <section_limit>
          Whole source rows per section page.

      --section-offset <section_offset>
          Offset within the selected native section.

      --snapshot <snapshot>
          Checked retained report snapshot for provider-free continuation.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --source <source>
          FAERS by default; MAUDE is explicit; help get adverse-event
          FAERS by default; MAUDE is explicit.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Get a drug, device or vaccine adverse event report. Sources: faers, maude.
Listed choices accept any letter case; replies use their listed spelling.
Options apply to every listed source unless their description restricts them.

- `--drug-index`: Absolute source-array index; required only for identifier sections. Value form: integer; example: --drug-index 0.
- `id`: Published report ID with no source prefix.
Form: get adverse-event ID SECTION; not --id.
- `section`: FAERS owns drugs, reactions and identifiers; MAUDE owns… (full rule with the sections).
Form: get adverse-event ID SECTION; not --section.
- `--section-limit`: Whole source rows per section page. Value form: integer; example: --section-limit 1.
- `--section-offset`: Offset within the selected source section. Value form: integer; example: --section-offset 0.
- `--snapshot`: Checked retained report snapshot for source-free continuation. Value form: string; example: --snapshot <snapshot> (syntax; replace the placeholder with your retained value).
- `--source`: FAERS by default; MAUDE is explicit. Value form: string; example: --source maude.

FAERS owns drugs, reactions and identifiers; MAUDE owns devices and narrative. openFDA's public FAERS data carries no case narrative; the FAERS card's case_event_date holds the case event date. Omitted means card only.

- `drugs`: Independent source drug rows.
- `reactions`: Independent source reaction rows.
- `unii`: Source UNII strings.
- `rxcui`: Source RxCUI strings.
- `product-ndc`: Source product NDC strings.
- `package-ndc`: Source package NDC strings.
- `spl-id`: Source SPL IDs.
- `spl-set-id`: Source SPL set IDs.
- `application-number`: Source application numbers.
- `devices`: Source device rows.
- `narrative`: Complete source MAUDE narrative rows; openFDA's public FAERS data carries no case narrative.

## biomcp get disease

```text
Exact concept or native SEER site with independently paged section.

Usage: biomcp get disease [OPTIONS] <id> [section]

Arguments:
  <id>
          Exact ontology CURIE or a SEER site code or name.

  [section]
          One named section; absent means identity card only; help get disease

Options:
      --age-range <age_range>
          Native age-range code.

      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --dataset <dataset>
          Requested native dataset selector, not proof of returned membership.

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --interval <interval>
          Survival-only interval code.

      --ontology <ontology>
          One OLS ontology; help get disease
          One OLS ontology.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --period <period>
          Incidence-only native period code.

      --race <race>
          Native race code.

      --section-limit <section_limit>
          Native rows per page: default 2 for Monarch genes, 1 for Monarch phenotypes, otherwise 5.

      --section-offset <section_offset>
          Native row offset within the selected section.

      --sex <sex>
          Native sex code.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --source <source>
          Default MyDisease; no automatic fallback; help get disease
          Default MyDisease; no automatic fallback.

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Get disease details and source statistics. Sources: monarch, mydisease, ols, orphadata, seer.
Listed choices accept any letter case; replies use their listed spelling.
Options apply to every listed source unless their description restricts them.

- `--age-range`: Source age-range code. Value form: string; example: --age-range 1.
- `--dataset`: Requested source dataset selector, not proof of returned membership. Value form: string; example: --dataset 4.
- `id`: Exact ontology CURIE or a SEER site code or name.
Form: get disease ID SECTION; not --id.
- `--interval`: Survival-only interval code. Value form: string; example: --interval 5.
- `--ontology`: One OLS ontology. Value form: string; example: --ontology mondo.
- `--period`: Incidence-only source period code. Value form: string; example: --period 5.
- `--race`: Source race code. Value form: string; example: --race 1.
- `section`: One named section; absent means identity card only… (full rule with the sections).
Form: get disease ID SECTION; not --section.
- `--section-limit`: Source rows per page: default 2 for Monarch genes, 1 for Monarch phenotypes, otherwise 5. Value form: integer; example: --section-limit 1.
- `--section-offset`: Source row offset within the selected section. Value form: integer; example: --section-offset 1.
- `--sex`: Source sex code. Value form: string; example: --sex 3.
- `--source`: Default MyDisease; no automatic fallback. Value form: string; example: --source seer.

One named section; absent means identity card only.

- `synonyms`: Publisher scoped synonyms.
- `xrefs`: Source cross references.
- `hierarchy`: Source parent relations.
- `genes`: Source gene association rows.
- `phenotypes`: Source phenotype association rows.
- `incidence`: Source stage incidence rows.
- `survival`: Annual survival rows.
- `prevalence`: Orphanet prevalence, age of onset and inheritance.

## biomcp get drug

```text
Retrieve a drug by source-native ID, brand or generic name, with explicit source-scoped sections.

Usage: biomcp get drug [OPTIONS] <input> [section]...

Arguments:
  <input>
          Source ID, brand or generic name.
          Source-native ID or complete brand or generic name; WHO references retain native spaces. Exact brands rank before generics; Drugs@FDA prefers NDA/BLA before ANDA, then earliest approval. Other sources retain search order. The chosen record reports up to five alternatives and their total. Generic names also accept one trailing listed salt or ester term: mesylate, hydrochloride, sodium, potassium, calcium, sulfate, maleate, tartrate, citrate, phosphate, acetate, besylate, fumarate, succinate, tosylate, bromide, chloride. Brand names identical to their own ingredient count as generic. USAN nomenclature: https://www.ama-assn.org/about/united-states-adopted-names-usan/united-states-adopted-names-naming-guidelines

  [section]...
          Source sections; help get drug

Options:
      --civic-cursor <civic_cursor>
          CIViC continuation.
          Native continuation cursor for that exact drug, profile and claim collection. Only with --source civic.

      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --civic-kind <civic_kind>
          CIViC collection; help get drug
          CIViC collection.
          Independent native evidence or assertion collection within the selected profile. Only with --source civic.

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --civic-profile <civic_profile>
          CIViC profile.
          Selected native CIViC molecular profile for a drug-scoped claim stream. Only with --source civic.

      --decision-from <decision_from>
          From date.
          Inclusive YYYY-MM-DD status date; Drugs@FDA approvals.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --decision-to <decision_to>
          To date.
          Inclusive YYYY-MM-DD status date; Drugs@FDA approvals.

      --document-id <document_id>
          FDA letter document ID.
          Exact native Drugs@FDA approval letter document ID. Text is bounded to 25 MiB, 100 pages
          and 6144 escaped bytes per fragment. No-text scans are explicit. Only with --source drugsfda.

      --evidence-level <evidence_level>...
          A–E; help get drug
          A–E
          Native CIViC evidence levels; repeat to include several levels.
          Only with --source civic on get drug, and the civic section on get variant.

      --evidence-status <evidence_status>
          Status; help get drug
          Status
          Native CIViC evidence status; omitted preserves the source's existing default.
          Only with --source civic on get drug, and the civic section on get variant.

      --label-section <label_section>
          Exact openFDA SPL fields.
          Exact published openFDA SPL section fields, including table fields, separated by commas; each is its own part of one reply. Omit it to list names.
          Only with --source openfda-label.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --partner <partner>
          Interaction partner.
          A DDInter ID or complete drug name, ignoring letter case; DDInter interactions only.
          Keeps only the pairs with that drug. A name DDInter does not use is tried under the
          names other sources state for it, as the drug itself is.

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

      --product-number <product_number>
          FDA product number.
          Explicit native application product number, only for approvals. Only with --source drugsfda.

      --section-limit <section_limit>
          Named section page size, refused on an identity-only card.
          At most 50 rows, or 200 for Drugs@FDA approvals.

      --section-offset <section_offset>
          Named section continuation, refused on an identity-only card.

      --source <source>
          Omit to infer the source of an unambiguous section; identity or; help get drug
          Omit to infer the source of an unambiguous section; identity or mixed sections use MyChem.

```

Get drug and medicine details. Sources: chembl, civic, ddinter, drugsfda, ema, fda-drug-recall, mychem, openfda-label, opentargets, who-pq.
Listed choices accept any letter case; replies use their listed spelling.
Options apply to every listed source unless their description restricts them.

- `--civic-cursor`: CIViC continuation.
Source continuation cursor for that exact drug, profile and claim collection. Only with --source civic. Value form: string; example: --civic-cursor 0:A2.
- `--civic-kind`: CIViC collection.
Independent source evidence or assertion collection within the selected profile. Only with --source civic. Value form: string; example: --civic-kind evidence.
- `--civic-profile`: CIViC profile.
Selected source CIViC molecular profile for a drug-scoped claim stream. Only with --source civic. Value form: integer; example: --civic-profile 1.
- `--decision-from`: From date.
Inclusive YYYY-MM-DD status date; Drugs@FDA approvals. Value form: string; example: --decision-from 2024-01-01; only with --source drugsfda.
- `--decision-to`: To date.
Inclusive YYYY-MM-DD status date; Drugs@FDA approvals. Value form: string; example: --decision-to 2024-12-31; only with --source drugsfda.
- `--document-id`: FDA letter document ID.
Exact source Drugs@FDA approval letter document ID. Text is bounded to 25 MiB, 100 pages
and 6144 escaped bytes per fragment. No-text scans are explicit. Only with --source drugsfda. Value form: string; example: --document-id 64255.
- `--evidence-level`: A–E
Source CIViC evidence levels; repeat to include several levels.
Only with --source civic on get drug, and the civic section on get variant. Value form: repeatable values (repeat the flag); example: --evidence-level A.
- `--evidence-status`: Status
Source CIViC evidence status; omitted preserves the source's existing default.
Only with --source civic on get drug, and the civic section on get variant. Value form: string; example: --evidence-status accepted.
- `input`: Source ID, brand or generic name.
Source-source ID or complete brand or generic name; WHO references retain source spaces. Exact brands rank before generics; Drugs@FDA prefers NDA/BLA before ANDA, then earliest approval. Other sources retain search order. The chosen record reports up to five alternatives and their total. Generic names also accept one trailing listed salt or ester term: mesylate, hydrochloride, sodium, potassium, calcium, sulfate, maleate, tartrate, citrate, phosphate, acetate, besylate, fumarate, succinate, tosylate, bromide, chloride. Brand names identical to their own ingredient count as generic. USAN nomenclature: https://www.ama-assn.org/about/united-states-adopted-names-usan/united-states-adopted-names-naming-guidelines
Form: get drug INPUT SECTION...; not --input.
- `--label-section`: Exact openFDA SPL fields.
Exact published openFDA SPL section fields, including table fields, separated by commas; each is its own part of one reply. Omit it to list names.
Only with --source openfda-label. Value form: string; example: --label-section indications_and_usage.
- `--partner`: Interaction partner.
A DDInter ID or complete drug name, ignoring letter case; DDInter interactions only.
Keeps only the pairs with that drug. A name DDInter does not use is tried under the
names other sources state for it, as the drug itself is. Value form: string; example: --partner DDInter5; only with --source ddinter.
- `--product-number`: FDA product number.
Explicit source application product number, only for approvals. Only with --source drugsfda. Value form: string; example: --product-number 001.
- `section`: Source sections.
Form: get drug INPUT SECTION...; not --section.
- `--section-limit`: Named section page size, refused on an identity-only card.
At most 50 rows, or 200 for Drugs@FDA approvals. Value form: integer; example: --section-limit 2.
- `--section-offset`: Named section continuation, refused on an identity-only card. Value form: integer; example: --section-offset 1.
- `--source`: Omit to infer the source of an unambiguous section; identity or mixed sections use MyChem. Value form: string; example: --source ema.

Named disclosures of the selected source, each its own part of one reply; empty means identity only.

- `xrefs`: All source identity cross-references and product links, mechanically paged.
- `targets`: Source target and mechanism relations.
- `indications`: Source indication wording.
- `civic`: Source CIViC therapy evidence and profile context.
- `label`: One exact source label heading.
- `approvals`: Source application and product history rows.
- `regulatory`: Source-scoped regulator events.
- `interactions`: Source DDInter partner rows and levels; five rows per default page, with explicit section limits preserved. DDInter's published files carry a level per pair and no management or mechanism text.
- `markets`: One row per market (Drugs@FDA, EMA, WHO prequalification), each from its own source; takes a name and no source.

## biomcp get gene

```text
Get one to twenty human gene inputs in their original order.

Usage: biomcp get gene [OPTIONS] <input>...

Arguments:
  <input>...
          Gene symbols or checked identifiers, in input order.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --section <section>
          Requested section words, preserving caller order and repetitions; help get gene

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Get gene identities and source annotations. Sources: civic, clingen, gencc, gnomad, hgnc, hpa, mygene, oncokb, reactome, uniprot.
Listed choices accept any letter case; replies use their listed spelling.
Options apply to every listed source unless their description restricts them.

- `input`: Gene symbols or checked identifiers, in input order.
Form: get gene INPUT...; not --input.
- `--section`: Requested section words, preserving caller order and repetitions… (full rule with the sections). Value form: repeatable values (repeat the flag); example: --section protein.

Requested section words, preserving caller order and repetitions.
Trailing section words select sections. Use --section WORD to keep inputs that equal a section word as inputs.

- `protein`: Reviewed human proteins.
- `civic`: CIViC gene curation.
- `clingen`: ClinGen validity and dosage.
- `gencc`: GenCC submitter assessments.
- `constraint`: Population constraint metrics.
- `expression`: Human Protein Atlas expression.
- `pathways`: Human Reactome pathways.
- `oncokb`: Keyed OncoKB annotations.
- `all`: Every landed section.

## biomcp get article

```text
Get one to twenty article inputs in caller order.

Usage: biomcp get article [OPTIONS] <input>...

Arguments:
  <input>...
          PubMed, PMC or DOI identifiers in input order.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --section <section>
          Requested section words; trailing words and --section share; help get article

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Get research article details. Sources: europepmc, icite, pmc-oa, pubmed, pubtator, semantic-scholar.
Listed choices accept any letter case; replies use their listed spelling.
Options apply to every listed source unless their description restricts them.

- `input`: PubMed, PMC or DOI identifiers in input order.
Form: get article INPUT...; not --input.
- `--section`: Requested section words; trailing words and --section share… (full rule with the sections). Value form: repeatable values (repeat the flag); example: --section annotations.

Requested section words; trailing words and --section share this field.

- `annotations`: Normalized text-mined entities from PubTator3.
- `fulltext`: Licensed PMC body outline.
- `supplements`: Supplementary files confirmed by PMC metadata.
- `tldr`: Machine-written Semantic Scholar summary and counts.
- `citations`: Citation counts from NIH iCite.
- `indexing`: MeSH, keywords and author indexing from the citation.
- `pdf`: Publisher PDF link without downloading.
- `all`: Every landed article section.

## biomcp get trial

```text
Retrieve exact trial cards in input order; request bounded intact source sections with independent continuations.

Usage: biomcp get trial [OPTIONS] <input>...

Arguments:
  <input>...
          One to twenty exact NCT identifiers, in caller order.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --limit <limit>
          Whole-record count per source collection page, up to 100; locations fill the page, other collections and text fragments also stop at their byte budget.

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --offset <offset>
          Character offset for ClinicalTrials.gov eligibility or NCI summary; whole-record offset for other collections.

      --pool <pool>
          Checked stored source record, supplied by a continuation.

      --section <section>
          Optional intact source sections. Default cards only; diseases; help get trial

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --source <source>
          Explicit provider; defaults to ClinicalTrials.gov and never; help get trial
          Explicit provider; defaults to ClinicalTrials.gov and never falls back.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Get clinical trial details. Sources: ctgov, nci.
Listed choices accept any letter case; replies use their listed spelling.
Options apply to every listed source unless their description restricts them.

- `input`: One to twenty exact NCT identifiers, in caller order.
Form: get trial INPUT...; not --input.
- `--limit`: Whole-record count per source collection page, up to 100; locations fill the page, other collections and text fragments also stop at their byte budget. Value form: integer; example: --limit 1.
- `--offset`: Character offset for ClinicalTrials.gov eligibility or NCI summary; whole-record offset for other collections. Value form: integer; example: --offset 0.
- `--pool`: Checked stored source record, supplied by a continuation. Value form: string; example: --pool <pool> (syntax; replace the placeholder with your retained value).
- `--section`: Optional intact source sections. Default cards only; diseases… (full rule with the sections). Value form: repeatable values (repeat the flag); example: --section outcomes.
- `--source`: Explicit source; defaults to ClinicalTrials.gov and never falls back. Value form: string; example: --source nci.

Optional intact source sections. Default cards only; diseases and summary require NCI. NCI locations, contacts, references and documents are unsupported.

- `eligibility`: Intact criteria and structured age/sex fields.
- `outcomes`: Primary, secondary and other outcome records.
- `results`: Posted outcome values and analyses, then participant flow, baseline and adverse-event tables (ClinicalTrials.gov).
- `flow`: Participant flow alone: started, completed and not completed by arm and period, with dropout reasons (ClinicalTrials.gov).
- `baseline`: Baseline characteristics alone: analysis population, groups and measures (ClinicalTrials.gov).
- `arms`: Arms and interventions with original relationships.
- `locations`: Whole source sites with their attached contacts.
- `contacts`: Central contacts and overall officials in their original roles.
- `references`: Registry publication identities and source reference types.
- `documents`: Posted document metadata and source URLs.
- `all`: Every implemented trial section, each with independent continuations; flow and baseline stay within results.
- `diseases`: Source NCI disease concepts, including source enrichment metadata.
- `summary`: Complete NCI summary, paged by original character offsets.

## biomcp get variant

```text
Read an rsID, CAid, clinvar:N, VCV accession, reference-qualified HGVS or VCF-style genomic coordinates such as 7-140753336-A-T through the shared resolver. MyVariant identity lookups default to GRCh38; --build overrides it. Chromosome genomic HGVS requires --build. Use search variant --gene BRAF --protein V600E or search variant --gene PTEN --coding c.333G>T for gene shorthand. Every candidate remains available; source assertions do not classify the variant for the caller.

Usage: biomcp get variant [OPTIONS] <input> [section]...

Arguments:
  <input>
          rs/CA/clinvar:N/VCV/HGVS/VCF
          rsID, CAid, clinvar:N, VCV accession, reference-qualified HGVS or VCF-style genomic coordinates.

  [section]...
          Sections; help get variant

Options:
      --build <build>
          Assembly; help get variant
          Assembly
          MyVariant identity assembly defaults to GRCh38; chromosome genomic HGVS requires an explicit build.

      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --civic-cursor <civic_cursor>
          Cursor
          Native CIViC cursor, retained unchanged.

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --civic-kind <civic_kind>
          Claims; help get variant
          Claims
          Native CIViC claim collection within the selected profile.

      --civic-profile <civic_profile>
          Selected CIViC molecular profile for native claim paging.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --disease <disease>
          Disease
          CIViC disease name text; CIViC matches it natively against evidence and assertion diseases.

      --drug <drug>
          Drug
          CIViC therapy name text; CIViC matches it natively against evidence and assertion therapies.

      --evidence-level <evidence_level>...
          A–E; help get variant
          A–E
          Native CIViC evidence levels; repeat to include several levels.
          Only with --source civic on get drug, and the civic section on get variant.

      --evidence-status <evidence_status>
          Status; help get variant
          Status
          Native CIViC evidence status; omitted preserves the source's existing default.
          Only with --source civic on get drug, and the civic section on get variant.

      --limit <limit>
          Candidate rows per page; more remain reachable through typed next commands.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --offset <offset>
          Candidate rows to skip in source order.

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

      --resolver <resolver>
          Resolver; help get variant
          Resolver.
          Resolver for reference-qualified notation; never repairs the input locally.

      --section-limit <section_limit>
          Rows per named source section page; omitted uses the native source default.

      --section-offset <section_offset>
          Source rows to skip independently of identity candidate paging.

      --tumor-type <tumor_type>
          Tumor
          Explicit tumor context for keyed OncoKB annotation.

```

Get genetic variants. Sources: alphagenome, car, civic, clinvar, erepo, gnomad, gwas, hotspots, mutalyzer, myvariant, oncokb, variantvalidator.
Listed choices accept any letter case; replies use their listed spelling.
Options apply to every listed source unless their description restricts them.

- `--build`: Assembly
MyVariant identity assembly defaults to GRCh38; chromosome genomic HGVS requires an explicit build. Value form: string; example: --build GRCh37.
- `--civic-cursor`: Cursor
Source CIViC cursor, retained unchanged. Value form: string; example: --civic-cursor 0:E2.
- `--civic-kind`: Claims
Source CIViC claim collection within the selected profile. Value form: string; example: --civic-kind evidence.
- `--civic-profile`: Selected CIViC molecular profile for source claim paging. Value form: integer; example: --civic-profile 12.
- `--disease`: Disease
CIViC disease name text; CIViC matches it natively against evidence and assertion diseases. Value form: string; example: --disease lung.
- `--drug`: Drug
CIViC therapy name text; CIViC matches it natively against evidence and assertion therapies. Value form: string; example: --drug erlotinib.
- `--evidence-level`: A–E
Source CIViC evidence levels; repeat to include several levels.
Only with --source civic on get drug, and the civic section on get variant. Value form: repeatable values (repeat the flag); example: --evidence-level A.
- `--evidence-status`: Status
Source CIViC evidence status; omitted preserves the source's existing default.
Only with --source civic on get drug, and the civic section on get variant. Value form: string; example: --evidence-status submitted.
- `input`: rs/CA/clinvar:N/VCV/HGVS/VCF
rsID, CAid, clinvar:N, VCV accession, reference-qualified HGVS or VCF-style genomic coordinates.
Form: get variant INPUT SECTION...; not --input.
- `--limit`: Candidate rows per page; more remain reachable through typed next commands. Value form: integer; example: --limit 10.
- `--offset`: Candidate rows to skip in source order. Value form: integer; example: --offset 0.
- `--resolver`: Resolver.
Resolver for reference-qualified notation; never repairs the input locally. Value form: string; example: --resolver variantvalidator.
- `section`: Sections.
Form: get variant INPUT SECTION...; not --section.
- `--section-limit`: Rows per named source section page; omitted uses the source default. Value form: integer; example: --section-limit 2.
- `--section-offset`: Source rows to skip independently of identity candidate paging. Value form: integer; example: --section-offset 0.
- `--tumor-type`: Tumor
Explicit tumor context for keyed OncoKB annotation. Value form: string; example: --tumor-type <tumor-type> (syntax; replace the placeholder with your retained value).

Named source sections; omitted by default. Each calls its own source. all keeps bounded pages and source skips; it never enables a missing key.

- `clinvar`: ClinVar source assertions.
- `erepo`: ClinGen source assertions.
- `population`: gnomAD population observations.
- `gwas`: GWAS Catalog associations.
- `civic`: CIViC molecular-profile evidence.
- `alphagenome`: Keyed AlphaGenome source numeric scorer values.
- `oncokb`: Keyed OncoKB annotation with explicit tumor context and build.
- `hotspots`: Cancer Hotspots candidate residue counts; no established exact allele join or clinical actionability.
- `predict`: MyVariant source predictor scores.
- `conservation`: MyVariant conservation scores.
- `cosmic`: Historical COSMIC fragments carried by MyVariant; parent-to-fragment allele identity remains unverified.
- `all`: Every implemented variant section.

## biomcp author

```text
Read papers attributed to a source author record.

Usage: biomcp author [OPTIONS] <COMMAND>

Commands:
  papers  One exact provider record's publication window. ORCID sends its whole public works list in one reply and pages groups locally.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Read papers attributed to a source author record.


## biomcp author papers

```text
One exact provider record's publication window. ORCID sends its whole public works list in one reply and pages groups locally.

Usage: biomcp author papers [OPTIONS] <id> [section]

Arguments:
  <id>
          Exact qualified provider record.

  [section]
          Detailed disclosure when requested; help author papers

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --full[=<full>]
          Legacy alias for details; normalized on construction.

          [default: false]
          [possible values: true, false]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

  -l, --limit <limit>
          Bounded row count.

      --offset <offset>
          Native or retained collection offset.

      --pool <pool>
          Retained collection identity.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

List an author's research papers. Sources: orcid, semantic-scholar.
Listed choices accept any letter case; replies use their listed spelling.
Options apply to every listed source unless their description restricts them.

- `--full`: Legacy alias for details; normalized on construction. Value form: boolean; example: --full.
- `id`: Exact qualified source record.
Form: author papers ID SECTION; not --id.
- `--limit`: Bounded row count. Value form: integer; example: --limit 5.
- `--offset`: Source or retained collection offset. Value form: integer; example: --offset 0.
- `--pool`: Retained collection identity. Value form: string; example: --pool <pool> (syntax; replace the placeholder with your retained value).
- `section`: Detailed disclosure when requested… (full rule with the sections).
Form: author papers ID SECTION; not --section.

Detailed disclosure when requested.

- `details`: Source detailed metadata for the same paper window.

## biomcp cell-line

```text
Read source cell line assay counts.

Usage: biomcp cell-line [OPTIONS] <COMMAND>

Commands:
  assay-count  A separate assay-count request, never implicit in metadata or all.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Read source cell line assay counts.


## biomcp cell-line assay-count

```text
A separate assay-count request, never implicit in metadata or all.

Usage: biomcp cell-line assay-count [OPTIONS] <id>

Arguments:
  <id>
          Exact ChEMBL cell identity returned by metadata.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Get a cell line's ChEMBL assay count. Sources: chembl.

- `id`: Exact ChEMBL cell identity returned by metadata.
Form: cell-line assay-count ID; not --id.

## biomcp protein

```text
Read protein sequences, features and source relationships.

Usage: biomcp protein [OPTIONS] <COMMAND>

Commands:
  features         Page five native UniProt features by exact type and displayed residue. No variant effect or clinical meaning is inferred; no bioinformatics tool runs.
  isoforms         Page five source isoforms, exact-participant complexes or PDB links. Source identities stay separate. No structure, interaction or clinical meaning is inferred; no bioinformatics tool runs.
  domains          Page five native InterPro matches with original types and fragments. No structure or clinical meaning is inferred; no bioinformatics tool runs.
  complexes        Page five source isoforms, exact-participant complexes or PDB links. Source identities stay separate. No structure, interaction or clinical meaning is inferred; no bioinformatics tool runs.
  complex-members  Page five native participants of an exact Complex Portal complex containing the anchor. No interaction or clinical meaning is inferred; no bioinformatics tool runs.
  structures       Page five source isoforms, exact-participant complexes or PDB links. Source identities stay separate. No structure, interaction or clinical meaning is inferred; no bioinformatics tool runs.
  sequence         Read one complete native sequence for an exact source accession.
                   No variant effect, structure or clinical meaning is inferred; no bioinformatics tool runs.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Read protein sequences, features and source relationships.


## biomcp protein features

```text
Page five native UniProt features by exact type and displayed residue. No variant effect or clinical meaning is inferred; no bioinformatics tool runs.

Usage: biomcp protein features [OPTIONS] <accession>

Arguments:
  <accession>
          Exact source protein sequence identity.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --limit <limit>
          Five rows by default, at most 25.

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --offset <offset>
          Native or retained page offset.

      --pool <pool>
          Retained collection identity.

      --position <position>
          Native one-based residue; zero fails during deserialization.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --type <type>
          Exact UniProt native label, without an alias map.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Get protein features by residue or region. Sources: uniprot.
Options apply to every listed source unless their description restricts them.

- `accession`: Exact source protein sequence identity.
Form: protein features ACCESSION; not --accession.
- `--limit`: Five rows by default, at most 25. Value form: integer; example: --limit 1.
- `--offset`: Source or retained page offset. Value form: integer; example: --offset 0.
- `--pool`: Retained collection identity. Value form: string; example: --pool <pool> (syntax; replace the placeholder with your retained value).
- `--position`: Source one-based residue; zero fails during deserialization. Value form: integer; example: --position 25.
- `--type`: Exact UniProt source label, without an alias map. Value form: string; example: --type Chain.

## biomcp protein isoforms

```text
Page five source isoforms, exact-participant complexes or PDB links. Source identities stay separate. No structure, interaction or clinical meaning is inferred; no bioinformatics tool runs.

Usage: biomcp protein isoforms [OPTIONS] <accession>

Arguments:
  <accession>
          Exact source protein sequence identity.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --limit <limit>
          Five rows by default, at most 25.

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --offset <offset>
          Native or retained page offset.

      --pool <pool>
          Retained collection identity.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Get protein isoforms and alternative products. Sources: uniprot.
Options apply to every listed source unless their description restricts them.

- `accession`: Exact source protein sequence identity.
Form: protein isoforms ACCESSION; not --accession.
- `--limit`: Five rows by default, at most 25. Value form: integer; example: --limit 1.
- `--offset`: Source or retained page offset. Value form: integer; example: --offset 0.
- `--pool`: Retained collection identity. Value form: string; example: --pool <pool> (syntax; replace the placeholder with your retained value).

## biomcp protein domains

```text
Page five native InterPro matches with original types and fragments. No structure or clinical meaning is inferred; no bioinformatics tool runs.

Usage: biomcp protein domains [OPTIONS] <accession>

Arguments:
  <accession>
          Exact source protein sequence identity.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --limit <limit>
          Five rows by default, at most 25.

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --offset <offset>
          Native or retained page offset.

      --pool <pool>
          Retained collection identity.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --type <type>
          Exact native InterPro type, not a normalized domain label.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Get protein domains and families from InterPro. Sources: interpro.
Options apply to every listed source unless their description restricts them.

- `accession`: Exact source protein sequence identity.
Form: protein domains ACCESSION; not --accession.
- `--limit`: Five rows by default, at most 25. Value form: integer; example: --limit 1.
- `--offset`: Source or retained page offset. Value form: integer; example: --offset 0.
- `--pool`: Retained collection identity. Value form: string; example: --pool <pool> (syntax; replace the placeholder with your retained value).
- `--type`: Exact source InterPro type, not a normalized domain label. Value form: string; example: --type <type> (syntax; replace the placeholder with your retained value).

## biomcp protein complexes

```text
Page five source isoforms, exact-participant complexes or PDB links. Source identities stay separate. No structure, interaction or clinical meaning is inferred; no bioinformatics tool runs.

Usage: biomcp protein complexes [OPTIONS] <accession>

Arguments:
  <accession>
          Exact source protein sequence identity.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --limit <limit>
          Five rows by default, at most 25.

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --offset <offset>
          Native or retained page offset.

      --pool <pool>
          Retained collection identity.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

List protein complexes containing a protein. Sources: complex-portal.
Options apply to every listed source unless their description restricts them.

- `accession`: Exact source protein sequence identity.
Form: protein complexes ACCESSION; not --accession.
- `--limit`: Five rows by default, at most 25. Value form: integer; example: --limit 1.
- `--offset`: Source or retained page offset. Value form: integer; example: --offset 0.
- `--pool`: Retained collection identity. Value form: string; example: --pool <pool> (syntax; replace the placeholder with your retained value).

## biomcp protein complex-members

```text
Page five native participants of an exact Complex Portal complex containing the anchor. No interaction or clinical meaning is inferred; no bioinformatics tool runs.

Usage: biomcp protein complex-members [OPTIONS] --complex <complex> <accession>

Arguments:
  <accession>
          Exact source protein sequence identity.

Options:
      --complex <complex>
          Exact complex whose native participants contain the anchor.

      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --limit <limit>
          Five rows by default, at most 25.

      --offset <offset>
          Native or retained page offset.

      --pool <pool>
          Retained collection identity.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

List the members of a protein complex. Sources: complex-portal.
Options apply to every listed source unless their description restricts them.

- `accession`: Exact source protein sequence identity.
Form: protein complex-members ACCESSION; not --accession.
- `--complex`: Exact complex whose source participants contain the anchor. Value form: string; example: --complex CPX-715.
- `--limit`: Five rows by default, at most 25. Value form: integer; example: --limit 1.
- `--offset`: Source or retained page offset. Value form: integer; example: --offset 0.
- `--pool`: Retained collection identity. Value form: string; example: --pool <pool> (syntax; replace the placeholder with your retained value).

## biomcp protein structures

```text
Page five source isoforms, exact-participant complexes or PDB links. Source identities stay separate. No structure, interaction or clinical meaning is inferred; no bioinformatics tool runs.

Usage: biomcp protein structures [OPTIONS] <accession>

Arguments:
  <accession>
          Exact source protein sequence identity.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --limit <limit>
          Five rows by default, at most 25.

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --offset <offset>
          Native or retained page offset.

      --pool <pool>
          Retained collection identity.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Get protein structure references from PDB. Sources: uniprot.
Options apply to every listed source unless their description restricts them.

- `accession`: Exact source protein sequence identity.
Form: protein structures ACCESSION; not --accession.
- `--limit`: Five rows by default, at most 25. Value form: integer; example: --limit 1.
- `--offset`: Source or retained page offset. Value form: integer; example: --offset 0.
- `--pool`: Retained collection identity. Value form: string; example: --pool <pool> (syntax; replace the placeholder with your retained value).

## biomcp protein sequence

```text
Read one complete native sequence for an exact source accession.
No variant effect, structure or clinical meaning is inferred; no bioinformatics tool runs.

Usage: biomcp protein sequence [OPTIONS] <isoform>

Arguments:
  <isoform>
          Exact isoform or base accession; no canonical suffix is synthesized.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Get a source protein sequence. Sources: uniprot.

- `isoform`: Exact isoform or base accession; no canonical suffix is synthesized.
Form: protein sequence ISOFORM; not --isoform.

## biomcp pathway

```text
Read pathway genes and source events.

Usage: biomcp pathway [OPTIONS] <COMMAND>

Commands:
  genes   Read Reactome exact references or WikiPathways GeneProduct data nodes. The details layer discloses complete Reactome physical contexts. Membership names a source and release; it implies no therapy, response or actionability. No enrichment is calculated.
  events  Read Reactome event objects and unresolved native integer references in source order. Membership names a source and release; it implies no therapy, response or actionability. No enrichment is calculated.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Read pathway genes and source events.


## biomcp pathway genes

```text
Read Reactome exact references or WikiPathways GeneProduct data nodes. The details layer discloses complete Reactome physical contexts. Membership names a source and release; it implies no therapy, response or actionability. No enrichment is calculated.

Usage: biomcp pathway genes [OPTIONS] <id> [details]

Arguments:
  <id>
          Checked source-owned human pathway identity.

  [details]
          Include every physical entity context for these references;; help pathway genes
          Include every physical entity context for these references; never clips source text.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

  -l, --limit <limit>
          Five complete relation rows by default, at most fifty.

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --offset <offset>
          Offset within the retained selected relation.

      --pool <pool>
          Immutable relation snapshot.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

List genes in a biological pathway. Sources: reactome, wikipathways.
Listed choices accept any letter case; replies use their listed spelling.
Options apply to every listed source unless their description restricts them.

- `details`: Include every physical entity context for these references; never clips source text.
Form: pathway genes ID DETAILS; not --details.
- `id`: Checked source-owned human pathway identity.
Form: pathway genes ID DETAILS; not --id.
- `--limit`: Five complete relation rows by default, at most fifty. Value form: integer; example: --limit 5.
- `--offset`: Offset within the retained selected relation. Value form: integer; example: --offset 0.
- `--pool`: Immutable relation snapshot. Value form: string; example: --pool <pool> (syntax; replace the placeholder with your retained value).

## biomcp pathway events

```text
Read Reactome event objects and unresolved native integer references in source order. Membership names a source and release; it implies no therapy, response or actionability. No enrichment is calculated.

Usage: biomcp pathway events [OPTIONS] <id>

Arguments:
  <id>
          Checked source-owned human pathway identity.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

  -l, --limit <limit>
          Five complete relation rows by default, at most fifty.

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --offset <offset>
          Offset within the retained selected relation.

      --pool <pool>
          Immutable relation snapshot.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

List events within a biological pathway. Sources: reactome.
Options apply to every listed source unless their description restricts them.

- `id`: Checked source-owned human pathway identity.
Form: pathway events ID; not --id.
- `--limit`: Five complete relation rows by default, at most fifty. Value form: integer; example: --limit 50.
- `--offset`: Offset within the retained selected relation. Value form: integer; example: --offset 0.
- `--pool`: Immutable relation snapshot. Value form: string; example: --pool <pool> (syntax; replace the placeholder with your retained value).

## biomcp article

```text
Read article text, references, citations and files.

Usage: biomcp article [OPTIONS] <COMMAND>

Commands:
  supplement         Read a checked supplement file or page its original rows.
  pdf                Download one publisher PDF to an explicit file or existing folder.
  text               Return the abstract card by default, or explicitly read body text.
  citations          Page the citing or referenced PubMed papers of one checked PMID.
  references         Page the citing or referenced PubMed papers of one checked PMID.
  citation-evidence  Read every source context and optional explicit full-text citation paragraph.
  recommendations    Recommendations in the provider's recent-paper pool order.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Read article text, references, citations and files.


## biomcp article supplement

```text
Read a checked supplement file or page its original rows.

Usage: biomcp article supplement [OPTIONS] <input> <file>

Arguments:
  <input>
          PubMed or PMC identity.

  <file>
          Exact file name from the supplements section.

Options:
      --find <find>
          Literal case-insensitive needle.

      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --offset <offset>
          Matching-row or text-fragment offset.

      --replace[=<replace>]
          Replace an existing destination only when explicitly requested with save.

          [default: false]
          [possible values: true, false]

      --rows <rows>
          Inclusive original row range.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --save <save>
          Save the whole original document to an explicit file or existing folder. Example: --save article-output.

      --sheet <sheet>
          Sheet name for row selection.

      --variant <variant>
          Explicit protein substitution.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Read a research article supplementary file. Sources: pmc-oa, pubmed.
Options apply to every listed source unless their description restricts them.

- `file`: Exact file name from the supplements section.
Form: article supplement INPUT FILE; not --file.
- `--find`: Exact case-insensitive needle. Value form: string; example: --find F1761I.
- `input`: PubMed or PMC identity.
Form: article supplement INPUT FILE; not --input.
- `--offset`: Matching-row or text-fragment offset. Value form: integer; example: --offset 3.
- `--replace`: Replace an existing destination only when explicitly requested with save. Value form: boolean; example: --replace.
- `--rows`: Inclusive original row range. Value form: string; example: --rows 84-86.
- `--save`: Save the whole original document to an explicit file or existing folder. Value form: string; example: --save article-output.
- `--sheet`: Sheet name for row selection. Value form: string; example: --sheet ST2.
- `--variant`: Explicit protein substitution. Value form: string; example: --variant P245S.

## biomcp article pdf

```text
Download one publisher PDF to an explicit file or existing folder.

Usage: biomcp article pdf [OPTIONS] --save <save> <input>

Arguments:
  <input>
          Article identity.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --replace[=<replace>]
          Replace an existing file only when explicitly requested.

          [default: false]
          [possible values: true, false]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --save <save>
          Destination file or existing folder.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Save a research article PDF. Sources: semantic-scholar.
Options apply to every listed source unless their description restricts them.

- `input`: Article identity.
Form: article pdf INPUT; not --input.
- `--replace`: Replace an existing file only when explicitly requested. Value form: boolean; example: --replace.
- `--save`: Destination file or existing folder. Value form: string; example: --save owned.pdf.

## biomcp article text

```text
Return the abstract card by default, or explicitly read body text.

Usage: biomcp article text [OPTIONS] <input>

Arguments:
  <input>
          PubMed or PMC identity. DOI selectors are not accepted by the reader.

Options:
      --find <find>
          Case-insensitive literal text to find.

      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --offset <offset>
          Number of matching rows already returned.

      --part <part>
          One-based outline part; omit reader selectors for the abstract card.

      --replace[=<replace>]
          Replace an existing destination only when explicitly requested with save.

          [default: false]
          [possible values: true, false]

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --save <save>
          Save the whole original document to an explicit file or existing folder. Example: --save article-output.

      --variant <variant>
          Explicit checked protein substitution. Example: --variant p.V600E.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Get the abstract card by default, or explicitly read article body text. Sources: europepmc, pmc-oa, pubmed.
Options apply to every listed source unless their description restricts them.

- `--find`: Case-insensitive exact text to find. Value form: string; example: --find AXIN2.
- `input`: PubMed or PMC identity. DOI selectors are not accepted by the reader.
Form: article text INPUT; not --input.
- `--offset`: Number of matching rows already returned. Value form: integer; example: --offset 0.
- `--part`: One-based outline part; omit reader selectors for the abstract card. Value form: integer; example: --part 1.
- `--replace`: Replace an existing destination only when explicitly requested with save. Value form: boolean; example: --replace.
- `--save`: Save the whole original document to an explicit file or existing folder. Value form: string; example: --save article-output.
- `--variant`: Explicit checked protein substitution. Value form: string; example: --variant p.V600E.

## biomcp article citations

```text
Page the citing or referenced PubMed papers of one checked PMID.

Usage: biomcp article citations [OPTIONS] <pmid>

Arguments:
  <pmid>
          PMID, PMC or DOI; iCite accepts only a PMID.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --limit <limit>
          Maximum papers on one page.

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --offset <offset>
          Number of pool items to skip.

      --pool <pool>
          Fixed snapshot from an earlier page.

      --source <source>
          Graph provider; iCite remains the default; help article citations
          Graph provider; iCite remains the default.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

List research papers citing an article. Sources: icite, pubmed, semantic-scholar.
Listed choices accept any letter case; replies use their listed spelling.
Options apply to every listed source unless their description restricts them.

- `--limit`: Maximum papers on one page. Value form: integer; example: --limit 1.
- `--offset`: Number of pool items to skip. Value form: integer; example: --offset 0.
- `pmid`: PMID, PMC or DOI; iCite accepts only a PMID.
Form: article citations PMID; not --pmid.
- `--pool`: Fixed snapshot from an earlier page. Value form: string; example: --pool <pool> (syntax; replace the placeholder with your retained value).
- `--source`: Graph source; iCite remains the default. Value form: string; example: --source semantic-scholar.

## biomcp article references

```text
Page the citing or referenced PubMed papers of one checked PMID.

Usage: biomcp article references [OPTIONS] <pmid>

Arguments:
  <pmid>
          PMID, PMC or DOI; iCite accepts only a PMID.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --limit <limit>
          Maximum papers on one page.

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --offset <offset>
          Number of pool items to skip.

      --pool <pool>
          Fixed snapshot from an earlier page.

      --source <source>
          Graph provider; iCite remains the default; help article citations
          Graph provider; iCite remains the default.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

List research papers referenced by an article. Sources: icite, pubmed, semantic-scholar.
Listed choices accept any letter case; replies use their listed spelling.
Options apply to every listed source unless their description restricts them.

- `--limit`: Maximum papers on one page. Value form: integer; example: --limit 1.
- `--offset`: Number of pool items to skip. Value form: integer; example: --offset 112.
- `pmid`: PMID, PMC or DOI; iCite accepts only a PMID.
Form: article references PMID; not --pmid.
- `--pool`: Fixed snapshot from an earlier page. Value form: string; example: --pool <pool> (syntax; replace the placeholder with your retained value).
- `--source`: Graph source; iCite remains the default. Value form: string; example: --source semantic-scholar.

## biomcp article citation-evidence

```text
Read every source context and optional explicit full-text citation paragraph.

Usage: biomcp article citation-evidence [OPTIONS] <citing> <cited>

Arguments:
  <citing>
          Citing PMID, PMC identity or DOI.

  <cited>
          Cited PMID, PMC identity or DOI.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --fulltext[=<fulltext>]
          Include the citing paper's own explicit reference paragraphs.

          [default: false]
          [possible values: true, false]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --offset <offset>
          Context fragment position returned by the advancing step.

      --paragraph-offset <paragraph_offset>
          Full-text fragment position returned by the advancing step.

      --pool <pool>
          Stored checked source scan and original hit positions.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Get research article citation evidence. Sources: pmc-oa, pubmed, semantic-scholar.
Options apply to every listed source unless their description restricts them.

- `cited`: Cited PMID, PMC identity or DOI.
Form: article citation-evidence CITING CITED; not --cited.
- `citing`: Citing PMID, PMC identity or DOI.
Form: article citation-evidence CITING CITED; not --citing.
- `--fulltext`: Include the citing paper's own explicit reference paragraphs. Value form: boolean; example: --fulltext.
- `--offset`: Context fragment position returned by the advancing step. Value form: integer; example: --offset 0.
- `--paragraph-offset`: Full-text fragment position returned by the advancing step. Value form: integer; example: --paragraph-offset 0.
- `--pool`: Stored checked source scan and original hit positions. Value form: string; example: --pool <pool> (syntax; replace the placeholder with your retained value).

## biomcp article recommendations

```text
Recommendations in the provider's recent-paper pool order.

Usage: biomcp article recommendations [OPTIONS] <seeds>...

Arguments:
  <seeds>...
          One to ten positive paper identities.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --limit <limit>
          Maximum recommendations requested, in native order.

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --negative <negative>...
          Up to ten negative paper identities.

      --offset <offset>
          Number of stored recommendations to skip.

      --pool <pool>
          Fixed recommendation pool.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Get related research article recommendations. Sources: semantic-scholar.
Options apply to every listed source unless their description restricts them.

- `--limit`: Maximum recommendations requested, in source order. Value form: integer; example: --limit 5.
- `--negative`: Up to ten negative paper identities. Value form: repeatable values (repeat the flag); example: --negative 33433946.
- `--offset`: Number of stored recommendations to skip. Value form: integer; example: --offset 1.
- `--pool`: Fixed recommendation pool. Value form: string; example: --pool <pool> (syntax; replace the placeholder with your retained value).
- `seeds`: One to ten positive paper identities.
Form: article recommendations SEEDS...; not --seeds.

## biomcp trial

```text
Read trial groups and public documents.

Usage: biomcp trial [OPTIONS] <COMMAND>

Commands:
  document  Download one exact ClinicalTrials.gov manifest PDF to an explicit caller path; never read its text. CLI and stdio MCP support saves; HTTP refuses with the source URL.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Read trial groups and public documents.


## biomcp trial document

```text
Download one exact ClinicalTrials.gov manifest PDF to an explicit caller path; never read its text. CLI and stdio MCP support saves; HTTP refuses with the source URL.

Usage: biomcp trial document [OPTIONS] --save <save> <input> <file>

Arguments:
  <input>
          Exact registry identity.

  <file>
          Exact filename from the registry manifest; never a URL or inferred path.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --replace[=<replace>]
          Replace an existing file only when explicitly requested.

          [default: false]
          [possible values: true, false]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --save <save>
          Caller-owned file or existing folder.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Save a clinical trial document. Sources: ctgov.
Options apply to every listed source unless their description restricts them.

- `file`: Exact filename from the registry manifest; never a URL or inferred path.
Form: trial document INPUT FILE; not --file.
- `input`: Exact registry identity.
Form: trial document INPUT FILE; not --input.
- `--replace`: Replace an existing file only when explicitly requested. Value form: boolean; example: --replace.
- `--save`: Caller-owned file or existing folder. Value form: string; example: --save protocol.pdf.

## biomcp variant

```text
Read source variant assertions, populations and recurrence.

Usage: biomcp variant [OPTIONS] <COMMAND>

Commands:
  assertions  Page assertions from one explicitly selected evidence source.
  population  Read a bounded ancestry-detail page from the exact source-given GRCh38 allele.
  recurrence  Observe exact-allele samples within one caller-selected source page.
  normalize   Send unchanged reference-qualified notation to explicitly named services.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Read source variant assertions, populations and recurrence.


## biomcp variant assertions

```text
Page assertions from one explicitly selected evidence source.

Usage: biomcp variant assertions [OPTIONS] --source <source> <input>

Arguments:
  <input>
          Supported variant ID; resolves one ClinVar Variation ID or ERepo CAid through the shared resolver.

Options:
      --assertion <assertion>
          Native assertion accession or document ID.

      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --build <build>
          Identity lookup assembly; omitted uses the shared; help variant assertions
          Identity lookup assembly; omitted uses the shared GRCh38 default.

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --limit <limit>
          Assertions per page; omitted uses the native source default.

      --offset <offset>
          Assertions to skip in source order.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --source <source>
          One native assertion source; shared identity; help variant assertions
          One native assertion source; shared identity conversion precedes source access when needed.

      --version <version>
          VCV archive or ERepo document version; RCV/SCV selectors match a returned native version.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Get genetic variant assertions from ClinVar or ClinGen. Sources: clinvar, erepo.
Listed choices accept any letter case; replies use their listed spelling.
Options apply to every listed source unless their description restricts them.

- `--assertion`: Source assertion accession or document ID. Value form: string; example: --assertion PCER000001171.
- `--build`: Identity lookup assembly; omitted uses the shared GRCh38 default. Value form: string; example: --build GRCh37.
- `input`: Supported variant ID; resolves one ClinVar Variation ID or ERepo CAid through the shared resolver.
Form: variant assertions INPUT; not --input.
- `--limit`: Assertions per page; omitted uses the source default. Value form: integer; example: --limit 1.
- `--offset`: Assertions to skip in source order. Value form: integer; example: --offset 0.
- `--source`: One source assertion source; shared identity conversion precedes source access when needed. Value form: string; example: --source clinvar.
- `--version`: VCV archive or ERepo document version; RCV/SCV selectors match a returned source version. Value form: string; example: --version 1.0.0.

## biomcp variant population

```text
Read a bounded ancestry-detail page from the exact source-given GRCh38 allele.

Usage: biomcp variant population [OPTIONS] <input>

Arguments:
  <input>
          Exact published allele identity; ambiguous candidates must first be selected.

Options:
      --build <build>
          Explicit source assembly when required by identity; help variant population
          Explicit source assembly when required by identity input.

      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --detail[=<detail>]
          Read source ancestry groups, preserving overlapping groups individually.

          [default: false]
          [possible values: true, false]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --limit <limit>
          Maximum rows per page.

      --offset <offset>
          Native rows to skip.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --resolver <resolver>
          Resolver for reference-qualified input; help variant population
          Resolver for reference-qualified input.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Get genetic variant population frequencies. Sources: car, gnomad, mutalyzer, myvariant, variantvalidator.
Listed choices accept any letter case; replies use their listed spelling.
Options apply to every listed source unless their description restricts them.

- `--build`: Explicit source assembly when required by identity input. Value form: string; example: --build GRCh38.
- `--detail`: Read source ancestry groups, preserving overlapping groups individually. Value form: boolean; example: --detail.
- `input`: Exact published allele identity; ambiguous candidates must first be selected.
Form: variant population INPUT; not --input.
- `--limit`: Maximum rows per page. Value form: integer; example: --limit 3.
- `--offset`: Source rows to skip. Value form: integer; example: --offset 10.
- `--resolver`: Resolver for reference-qualified input. Value form: string; example: --resolver variantvalidator.

## biomcp variant recurrence

```text
Observe exact-allele samples within one caller-selected source page.

Usage: biomcp variant recurrence [OPTIONS] --gene <gene> --profile <profile> --sample-set <sample_set> --source <source> --study <study> <input>

Arguments:
  <input>
          Variant identity resolving to one unique source-qualified genomic replacement with nonempty reference and alternate alleles (pure insertions and deletions are unavailable).

Options:
      --build <build>
          Identity lookup assembly; omitted uses the shared; help variant recurrence
          Identity lookup assembly; omitted uses the shared GRCh38 default.

      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --gene <gene>
          HGNC symbol resolved to the source's Entrez gene identifier.

      --limit <limit>
          Source mutation rows per page, including excluded alleles.

      --page <page>
          Zero-based native source page.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --profile <profile>
          Explicit mutation molecular profile.

      --sample-set <sample_set>
          Explicit sample list; selected size is never an allele denominator.

      --source <source>
          Explicit cohort source; help variant recurrence
          Explicit cohort source.

      --study <study>
          Explicit study; there is no default cohort.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Get genetic variant observations in a cohort. Sources: cbioportal.
Listed choices accept any letter case; replies use their listed spelling.
Options apply to every listed source unless their description restricts them.

- `--build`: Identity lookup assembly; omitted uses the shared GRCh38 default. Value form: string; example: --build GRCh37.
- `--gene`: HGNC symbol resolved to the source's Entrez gene identifier. Value form: string; example: --gene BRAF.
- `input`: Variant identity resolving to one unique source-qualified genomic replacement with nonempty reference and alternate alleles (pure insertions and deletions are unavailable).
Form: variant recurrence INPUT; not --input.
- `--limit`: Source mutation rows per page, including excluded alleles. Value form: integer; example: --limit 3.
- `--page`: Zero-based source page. Value form: integer; example: --page 0.
- `--profile`: Explicit mutation molecular profile. Value form: string; example: --profile msk_impact_2017_mutations.
- `--sample-set`: Explicit sample list; selected size is never an allele denominator. Value form: string; example: --sample-set msk_impact_2017_all.
- `--source`: Explicit cohort source. Value form: string; example: --source cbioportal.
- `--study`: Explicit study; there is no default cohort. Value form: string; example: --study msk_impact_2017.

## biomcp variant normalize

```text
Send unchanged reference-qualified notation to explicitly named services.

Usage: biomcp variant normalize [OPTIONS] --hgvs <hgvs> --source <source>

Options:
      --build <build>
          Caller assembly; required when VariantValidator is; help variant normalize
          Caller assembly; required when VariantValidator is selected.

      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --hgvs <hgvs>
          Reference-qualified HGVS, or chromosome genomic HGVS with an explicit build for CAR.
          Checked for reference and syntax only; no full HGVS parser. After a reference
          mismatch, VariantValidator names the gene's transcripts where the base fits.

      --limit <limit>
          Source mapping rows per service, with typed continuation for remaining rows.

      --offset <offset>
          Source mapping rows to skip per service.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --source <source>
          Explicit service; all preserves independent results; help variant normalize
          Explicit service; all preserves independent results without consensus.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Normalize reference-qualified genetic variant HGVS. Sources: car, mutalyzer, variantvalidator.
Listed choices accept any letter case; replies use their listed spelling.
Options apply to every listed source unless their description restricts them.

- `--build`: Caller assembly; required when VariantValidator is selected. Value form: string; example: --build GRCh38.
- `--hgvs`: Reference-qualified HGVS, or chromosome genomic HGVS with an explicit build for CAR.
Checked for reference and syntax only; no full HGVS parser. After a reference
mismatch, VariantValidator names the gene's transcripts where the base fits. Value form: string; example: --hgvs 'NM_004333.4:c.1799T>A'.
- `--limit`: Source mapping rows per service, with typed continuation for remaining rows. Value form: integer; example: --limit 10.
- `--offset`: Source mapping rows to skip per service. Value form: integer; example: --offset 0.
- `--source`: Explicit service; all preserves independent results without consensus. Value form: string; example: --source mutalyzer.

## biomcp sync

```text
Synchronize a local source bundle.

Usage: biomcp sync [OPTIONS] [bundle]

Arguments:
  [bundle]
          Local source bundle to synchronize.

          [default: hgnc]
          [possible values: hgnc, clingen, gencc, wikipathways, cbioportal-studies, ema, ddinter, who-pq, gtr, who-ivd, fda-cdx, orphadata]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Synchronize a local source bundle.
Listed choices accept any letter case; replies use their listed spelling.

- `bundle`: Local source bundle to synchronize.
Form: sync BUNDLE; not --bundle.

## biomcp skill

```text
Read embedded guidance and worked examples. No source requests. Overview and catalog are bounded by the embedded payload; install and status return one summary. Install writes only inside DIR and refuses symbolic links and parent traversal.

Usage: biomcp skill [OPTIONS] [COMMAND]

Commands:
  list     List embedded worked examples.
  render   Print the canonical agent guide.
  status   Report missing, unmanaged, locally_modified, stale or current guidance.
  install  Install into DIR/skills/biomcp; a skills or biomcp directory is accepted directly. Existing files remain unchanged without --force.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

Example: biomcp skill list

```

Read embedded guidance and worked examples. No source requests. Overview and catalog are bounded by the embedded payload; install and status return one summary. Install writes only inside DIR and refuses symbolic links and parent traversal.


## biomcp skill list

```text
List embedded worked examples.

Usage: biomcp skill list [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

Example: biomcp skill list

```

List embedded worked examples.


## biomcp skill render

```text
Print the canonical agent guide.

Usage: biomcp skill render [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

Example: biomcp skill render

```

Print the canonical agent guide.


## biomcp skill status

```text
Report missing, unmanaged, locally_modified, stale or current guidance.

Usage: biomcp skill status [OPTIONS] <directory>

Arguments:
  <directory>
          Directory to inspect or install BioMCP guidance.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

Example: biomcp skill status biomcp-guidance

```

Report missing, unmanaged, locally_modified, stale or current guidance.

- `directory`: Directory to inspect or install BioMCP guidance.

## biomcp skill install

```text
Install into DIR/skills/biomcp; a skills or biomcp directory is accepted directly. Existing files remain unchanged without --force.

Usage: biomcp skill install [OPTIONS] <directory>

Arguments:
  <directory>
          Directory to inspect or install BioMCP guidance.

Options:
      --force[=<force>]
          Repair managed files; retain unrelated files.

          [default: false]
          [possible values: true, false]

      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

Example: biomcp skill install biomcp-guidance

```

Install into DIR/skills/biomcp; a skills or biomcp directory is accepted directly. Existing files remain unchanged without --force.
Options are local to the command and independent of source selection.

- `directory`: Directory to inspect or install BioMCP guidance.
- `--force`: Repair managed files; retain unrelated files. Value form: boolean; example: --force.

## biomcp source

```text
List source names as JSON. No source requests or key reads.

Usage: biomcp source [OPTIONS] <COMMAND>

Commands:
  list  List the source names a hosted request may leave out.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

Example: biomcp source list

```

List source names as JSON. No source requests or key reads.


## biomcp source list

```text
List the source names a hosted request may leave out.

Usage: biomcp source list [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

Example: biomcp source list

```

List the source names a hosted request may leave out.


## biomcp version

```text
Print version; --verbose adds build and path provenance. No source requests or key reads.

Usage: biomcp version [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --verbose[=<verbose>]
          Include commit, target, config and cache paths.

          [default: false]
          [possible values: true, false]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

Example: biomcp version

```

Print version; --verbose adds build and path provenance. No source requests or key reads.
Options are local to the command and independent of source selection.

- `--verbose`: Include commit, target, config and cache paths. Value form: boolean; example: --verbose.

## biomcp mcp-config

```text
Print MCP configuration with this executable’s absolute path. JSON formats name their top-level object and retain released client aliases. The codex client prints its shell registration command. No source requests or key reads.

Usage: biomcp mcp-config [OPTIONS]

Options:
      --client <client>
          Choose a JSON client or the codex shell registration command; default mcp-servers.

          [default: mcp-servers]
          [possible values: mcp-servers, servers, codex]

      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

Example: biomcp mcp-config

```

Print MCP configuration with this executable’s absolute path. JSON formats name their top-level object and retain released client aliases. The codex client prints its shell registration command. No source requests or key reads.
Listed choices accept any letter case; replies use their listed spelling.
Options are local to the command and independent of source selection.

- `--client`: Choose a JSON client or the codex shell registration command; default mcp-servers. Value form: string; example: --client mcp-servers.

## biomcp health

```text
Report source terms and bounded reachability.

Usage: biomcp health [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

Example: biomcp health --offline

```

Report publisher terms and bounded source reachability; at most one page for each implemented source.


## biomcp store

```text
Manage local cached replies and snapshots.

Usage: biomcp store [OPTIONS] <COMMAND>

Commands:
  clean

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Manage local cached replies and snapshots.


## biomcp store clean

```text
Usage: biomcp store clean [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

Example: biomcp store clean

```

Remove expired replies, pools and snapshots and older incompatible cache files. Return removal counts in one bounded reply. Fresh state and bundles remain.


## biomcp key

```text
Register and manage source credentials without showing values.

Usage: biomcp key [OPTIONS] <COMMAND>

Commands:
  list    List source credentials and their origins without showing values.
  set
  remove

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

Example: biomcp key list

```

Register and manage source credentials without showing values.


## biomcp key list

```text
List source credentials and their origins without showing values.

Usage: biomcp key list [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

Example: biomcp key list

```

List source credentials and their origins without showing values.


## biomcp key set

```text
Usage: biomcp key set [OPTIONS] <source>

Arguments:
  <source>
          Source credential name to register or remove.

          [possible values: ncbi, semantic-scholar, orcid, oncokb, alphagenome, nci, openfda]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --replace[=<replace>]
          Replace an existing stored credential.

          [default: false]
          [possible values: true, false]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

Example: biomcp key set oncokb

```

Register one credential from a no-echo prompt or one standard-input line. Use --config PATH to choose the configuration file.
Listed choices accept any letter case; replies use their listed spelling.
Options are local to the command and independent of source selection.

- `source`: Source credential name to register or remove.
Form: key set SOURCE; not --source.
- `--replace`: Replace an existing stored credential. Value form: boolean; example: --replace.

## biomcp key remove

```text
Usage: biomcp key remove [OPTIONS] <source>

Arguments:
  <source>
          Source credential name to register or remove.

          [possible values: ncbi, semantic-scholar, orcid, oncokb, alphagenome, nci, openfda]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

Example: biomcp key remove oncokb

```

Remove a stored credential while preserving other settings.
Listed choices accept any letter case; replies use their listed spelling.

- `source`: Source credential name to register or remove.
Form: key remove SOURCE; not --source.

## biomcp serve

```text
Serve MCP over standard input or HTTP.

Usage: biomcp serve [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --tool-mode <tool-mode>
          Advertise one CLI tool or generated typed command tools; default single.

          [possible values: single, typed]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --hosted[=<hosted>]
          Refuse host file saves, health details and store or key changes over MCP.

          [default: false]
          [possible values: true, false]

      --rate-share <rate-share>
          Use this share of keyed source rates in hosted mode (greater than 0, at most 1).

          [default: 1]

      --http [<http>]
          Listen on an HTTP address; omit its value for the loopback default.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --http_max_calls <http_max_calls>
          Limit tool calls in flight. Example: --http_max_calls 8.

      --http_max_connections <http_max_connections>
          Limit open HTTP connections; must cover http_max_calls. Example: --http_max_connections 64.

      --allow-host <allow-host>
          Allow an HTTP Host authority; required for non-loopback listeners. Example: --allow-host api.example.org.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

Example: biomcp serve
Example: biomcp help serve

```

Serve MCP over stdio or --http [ADDR]. Non-loopback binds require --allow-host HOST or http.allowed_hosts; neither authenticates callers. HTTP refuses sync, store clean and --fresh. Stdio saves refuse --replace. --hosted refuses file saves, health details, sync, store clean, --fresh and key commands on every transport with a usage.mcp_unavailable error. MCP revisions: 2026-07-28, 2025-11-25, 2025-06-18, 2025-03-26. --rate-share scales only keyed hosted source rates. Hosted HTTP calls have bounded work and one whole-call deadline. Send MCP-Protocol-Version on HTTP requests. Without it, initialize uses its requested legacy revision; other requests default to 2025-03-26. Unsupported headers return HTTP 400.
Listed choices accept any letter case; replies use their listed spelling.
Options are local to the command and independent of source selection.

- `--tool-mode`: Advertise one CLI tool or generated typed command tools; default single. Value form: string; example: --tool-mode single.
- `--hosted`: Refuse host file saves, health details and store or key changes over MCP. Value form: boolean; example: --hosted.
- `--rate-share`: Use this share of keyed source rates in hosted mode (greater than 0, at most 1). Value form: string; example: --rate-share 1.
- `--http`: Listen on an HTTP address; omit its value for the loopback default. Value form: string; example: --http 127.0.0.1:8080.
- `--http_max_calls`: Limit tool calls in flight. Value form: string; example: --http_max_calls 8.
- `--http_max_connections`: Limit open HTTP connections; must cover http_max_calls. Value form: string; example: --http_max_connections 64.
- `--allow-host`: Allow an HTTP Host authority; required for non-loopback listeners. Value form: string; example: --allow-host api.example.org.

## biomcp schema

```text
List every schema identity and description with biomcp schema, or print one schema artifact with biomcp schema <id>.

Usage: biomcp schema [OPTIONS] [artifact-id]

Arguments:
  [artifact-id]
          Artifact ID to print; omit it to list available schemas.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

JSON reply readers ignore unknown fields and unknown growable enum values within a major. Query schemas remain closed.

Example: biomcp schema
Example: biomcp schema biomcp.help/6

```

List every schema identity and description with biomcp schema, or print one schema artifact with biomcp schema <id>.

- `artifact-id`: Artifact ID to print; omit it to list available schemas.
Form: schema ARTIFACT-ID; not --artifact-id.

## biomcp help

```text
Describe commands and their arguments.

Usage: biomcp help [OPTIONS] [COMMAND]

Commands:
  search             Search source records by entity and checked filters.
  get                Retrieve one entity and its source sections.
  author             Read papers attributed to a source author record.
  cell-line          Read source cell line assay counts.
  protein            Read protein sequences, features and source relationships.
  pathway            Read pathway genes and source events.
  article            Read article text, references, citations and files.
  trial              Read trial groups and public documents.
  variant            Read source variant assertions, populations and recurrence.
  sync               Synchronize a local source bundle.
  skill
  source
  version
  mcp-config
  health             Report source terms and bounded reachability.
  store              Manage local cached replies and snapshots.
  key                Register and manage source credentials without showing values.
  serve              Serve MCP over standard input or HTTP.
  schema
  biomarker
  papers
  organization
  assay-count
  gwas
  features
  isoforms
  domains
  complexes
  complex-members
  structures
  sequence
  phenotype
  genes
  events
  study
  pgx
  diagnostic
  adverse-event
  disease
  drug
  gene
  supplement
  pdf
  text
  citations
  references
  citation-evidence
  recommendations
  document
  assertions
  population
  recurrence
  normalize
  help               Describe commands and their arguments.
  network
  sources
  config
  exit               Explain process exit and error codes.
  questions          List every command by the question it answers.
  examples           Show each use case's first call in CLI and typed form.
  relations          Map entities through sections and search filters.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Print generated command help.

- `command`: Command path or discovery topic to describe.
Form: help COMMAND; not --command.

## biomcp help search

```text
Search source records by entity and checked filters.

Usage: biomcp help search [OPTIONS] [COMMAND]

Commands:
  biomarker
  author         Read papers attributed to a source author record.
  organization
  cell-line      Read source cell line assay counts.
  gwas
  protein        Read protein sequences, features and source relationships.
  phenotype
  pathway        Read pathway genes and source events.
  study
  pgx
  diagnostic
  adverse-event
  disease
  drug
  gene
  article        Read article text, references, citations and files.
  trial          Read trial groups and public documents.
  variant        Read source variant assertions, populations and recurrence.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Search source records by entity and checked filters.


## biomcp help search biomarker

```text
Usage: biomcp help search biomarker [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help search biomarker.


## biomcp help search author

```text
Read papers attributed to a source author record.

Usage: biomcp help search author [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Read papers attributed to a source author record.


## biomcp help search organization

```text
Usage: biomcp help search organization [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help search organization.


## biomcp help search cell-line

```text
Read source cell line assay counts.

Usage: biomcp help search cell-line [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Read source cell line assay counts.


## biomcp help search gwas

```text
Usage: biomcp help search gwas [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help search gwas.


## biomcp help search protein

```text
Read protein sequences, features and source relationships.

Usage: biomcp help search protein [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Read protein sequences, features and source relationships.


## biomcp help search phenotype

```text
Usage: biomcp help search phenotype [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help search phenotype.


## biomcp help search pathway

```text
Read pathway genes and source events.

Usage: biomcp help search pathway [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Read pathway genes and source events.


## biomcp help search study

```text
Usage: biomcp help search study [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help search study.


## biomcp help search pgx

```text
Usage: biomcp help search pgx [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help search pgx.


## biomcp help search diagnostic

```text
Usage: biomcp help search diagnostic [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help search diagnostic.


## biomcp help search adverse-event

```text
Usage: biomcp help search adverse-event [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help search adverse-event.


## biomcp help search disease

```text
Usage: biomcp help search disease [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help search disease.


## biomcp help search drug

```text
Usage: biomcp help search drug [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help search drug.


## biomcp help search gene

```text
Usage: biomcp help search gene [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help search gene.


## biomcp help search article

```text
Read article text, references, citations and files.

Usage: biomcp help search article [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Read article text, references, citations and files.


## biomcp help search trial

```text
Read trial groups and public documents.

Usage: biomcp help search trial [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Read trial groups and public documents.


## biomcp help search variant

```text
Read source variant assertions, populations and recurrence.

Usage: biomcp help search variant [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Read source variant assertions, populations and recurrence.


## biomcp help get

```text
Retrieve one entity and its source sections.

Usage: biomcp help get [OPTIONS] [COMMAND]

Commands:
  biomarker
  author         Read papers attributed to a source author record.
  cell-line      Read source cell line assay counts.
  gwas
  protein        Read protein sequences, features and source relationships.
  phenotype
  pathway        Read pathway genes and source events.
  study
  pgx
  diagnostic
  adverse-event
  disease
  drug
  gene
  article        Read article text, references, citations and files.
  trial          Read trial groups and public documents.
  variant        Read source variant assertions, populations and recurrence.
  clingen

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Retrieve one entity and its source sections.


## biomcp help get biomarker

```text
Usage: biomcp help get biomarker [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help get biomarker.


## biomcp help get author

```text
Read papers attributed to a source author record.

Usage: biomcp help get author [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Read papers attributed to a source author record.


## biomcp help get cell-line

```text
Read source cell line assay counts.

Usage: biomcp help get cell-line [OPTIONS] [section]

Arguments:
  [section]
          One section of the get command to describe.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Read source cell line assay counts.

- `section`: One section of the get command to describe.
Form: help get cell-line SECTION; not --section.

## biomcp help get gwas

```text
Usage: biomcp help get gwas [OPTIONS] [section]

Arguments:
  [section]
          One section of the get command to describe.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help get gwas.

- `section`: One section of the get command to describe.
Form: help get gwas SECTION; not --section.

## biomcp help get protein

```text
Read protein sequences, features and source relationships.

Usage: biomcp help get protein [OPTIONS] [section]

Arguments:
  [section]
          One section of the get command to describe.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Read protein sequences, features and source relationships.

- `section`: One section of the get command to describe.
Form: help get protein SECTION; not --section.

## biomcp help get phenotype

```text
Usage: biomcp help get phenotype [OPTIONS] [section]

Arguments:
  [section]
          One section of the get command to describe.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help get phenotype.

- `section`: One section of the get command to describe.
Form: help get phenotype SECTION; not --section.

## biomcp help get pathway

```text
Read pathway genes and source events.

Usage: biomcp help get pathway [OPTIONS] [section]

Arguments:
  [section]
          One section of the get command to describe.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Read pathway genes and source events.

- `section`: One section of the get command to describe.
Form: help get pathway SECTION; not --section.

## biomcp help get study

```text
Usage: biomcp help get study [OPTIONS] [section]

Arguments:
  [section]
          One section of the get command to describe.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help get study.

- `section`: One section of the get command to describe.
Form: help get study SECTION; not --section.

## biomcp help get pgx

```text
Usage: biomcp help get pgx [OPTIONS] [section]

Arguments:
  [section]
          One section of the get command to describe.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help get pgx.

- `section`: One section of the get command to describe.
Form: help get pgx SECTION; not --section.

## biomcp help get diagnostic

```text
Usage: biomcp help get diagnostic [OPTIONS] [section]

Arguments:
  [section]
          One section of the get command to describe.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help get diagnostic.

- `section`: One section of the get command to describe.
Form: help get diagnostic SECTION; not --section.

## biomcp help get adverse-event

```text
Usage: biomcp help get adverse-event [OPTIONS] [section]

Arguments:
  [section]
          One section of the get command to describe.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help get adverse-event.

- `section`: One section of the get command to describe.
Form: help get adverse-event SECTION; not --section.

## biomcp help get disease

```text
Usage: biomcp help get disease [OPTIONS] [section]

Arguments:
  [section]
          One section of the get command to describe.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help get disease.

- `section`: One section of the get command to describe.
Form: help get disease SECTION; not --section.

## biomcp help get drug

```text
Usage: biomcp help get drug [OPTIONS] [section]

Arguments:
  [section]
          One section of the get command to describe.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help get drug.

- `section`: One section of the get command to describe.
Form: help get drug SECTION; not --section.

## biomcp help get gene

```text
Usage: biomcp help get gene [OPTIONS] [section]

Arguments:
  [section]
          One section of the get command to describe.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help get gene.

- `section`: One section of the get command to describe.
Form: help get gene SECTION; not --section.

## biomcp help get article

```text
Read article text, references, citations and files.

Usage: biomcp help get article [OPTIONS] [section]

Arguments:
  [section]
          One section of the get command to describe.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Read article text, references, citations and files.

- `section`: One section of the get command to describe.
Form: help get article SECTION; not --section.

## biomcp help get trial

```text
Read trial groups and public documents.

Usage: biomcp help get trial [OPTIONS] [section]

Arguments:
  [section]
          One section of the get command to describe.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Read trial groups and public documents.

- `section`: One section of the get command to describe.
Form: help get trial SECTION; not --section.

## biomcp help get variant

```text
Read source variant assertions, populations and recurrence.

Usage: biomcp help get variant [OPTIONS] [section]

Arguments:
  [section]
          One section of the get command to describe.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Read source variant assertions, populations and recurrence.

- `section`: One section of the get command to describe.
Form: help get variant SECTION; not --section.

## biomcp help get clingen

```text
Usage: biomcp help get clingen [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help get clingen.


## biomcp help author

```text
Read papers attributed to a source author record.

Usage: biomcp help author [OPTIONS] [COMMAND]

Commands:
  papers

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Read papers attributed to a source author record.


## biomcp help author papers

```text
Usage: biomcp help author papers [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help author papers.


## biomcp help cell-line

```text
Read source cell line assay counts.

Usage: biomcp help cell-line [OPTIONS] [COMMAND]

Commands:
  assay-count

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Read source cell line assay counts.


## biomcp help cell-line assay-count

```text
Usage: biomcp help cell-line assay-count [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help cell-line assay-count.


## biomcp help protein

```text
Read protein sequences, features and source relationships.

Usage: biomcp help protein [OPTIONS] [COMMAND]

Commands:
  features
  isoforms
  domains
  complexes
  complex-members
  structures
  sequence

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Read protein sequences, features and source relationships.


## biomcp help protein features

```text
Usage: biomcp help protein features [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help protein features.


## biomcp help protein isoforms

```text
Usage: biomcp help protein isoforms [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help protein isoforms.


## biomcp help protein domains

```text
Usage: biomcp help protein domains [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help protein domains.


## biomcp help protein complexes

```text
Usage: biomcp help protein complexes [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help protein complexes.


## biomcp help protein complex-members

```text
Usage: biomcp help protein complex-members [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help protein complex-members.


## biomcp help protein structures

```text
Usage: biomcp help protein structures [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help protein structures.


## biomcp help protein sequence

```text
Usage: biomcp help protein sequence [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help protein sequence.


## biomcp help pathway

```text
Read pathway genes and source events.

Usage: biomcp help pathway [OPTIONS] [COMMAND]

Commands:
  genes
  events

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Read pathway genes and source events.


## biomcp help pathway genes

```text
Usage: biomcp help pathway genes [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help pathway genes.


## biomcp help pathway events

```text
Usage: biomcp help pathway events [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help pathway events.


## biomcp help article

```text
Read article text, references, citations and files.

Usage: biomcp help article [OPTIONS] [COMMAND]

Commands:
  supplement
  pdf
  text
  citations
  references
  citation-evidence
  recommendations

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Read article text, references, citations and files.


## biomcp help article supplement

```text
Usage: biomcp help article supplement [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help article supplement.


## biomcp help article pdf

```text
Usage: biomcp help article pdf [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help article pdf.


## biomcp help article text

```text
Usage: biomcp help article text [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help article text.


## biomcp help article citations

```text
Usage: biomcp help article citations [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help article citations.


## biomcp help article references

```text
Usage: biomcp help article references [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help article references.


## biomcp help article citation-evidence

```text
Usage: biomcp help article citation-evidence [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help article citation-evidence.


## biomcp help article recommendations

```text
Usage: biomcp help article recommendations [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help article recommendations.


## biomcp help trial

```text
Read trial groups and public documents.

Usage: biomcp help trial [OPTIONS] [COMMAND]

Commands:
  document

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Read trial groups and public documents.


## biomcp help trial document

```text
Usage: biomcp help trial document [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help trial document.


## biomcp help variant

```text
Read source variant assertions, populations and recurrence.

Usage: biomcp help variant [OPTIONS] [COMMAND]

Commands:
  assertions
  population
  recurrence
  normalize

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Read source variant assertions, populations and recurrence.


## biomcp help variant assertions

```text
Usage: biomcp help variant assertions [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help variant assertions.


## biomcp help variant population

```text
Usage: biomcp help variant population [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help variant population.


## biomcp help variant recurrence

```text
Usage: biomcp help variant recurrence [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help variant recurrence.


## biomcp help variant normalize

```text
Usage: biomcp help variant normalize [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help variant normalize.


## biomcp help sync

```text
Synchronize a local source bundle.

Usage: biomcp help sync [OPTIONS] [COMMAND]

Commands:
  hgnc
  clingen
  gencc
  wikipathways
  cbioportal-studies
  ema
  ddinter
  who-pq
  gtr
  who-ivd
  fda-cdx
  orphadata

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Synchronize a local source bundle.


## biomcp help sync hgnc

```text
Usage: biomcp help sync hgnc [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help sync hgnc.


## biomcp help sync clingen

```text
Usage: biomcp help sync clingen [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help sync clingen.


## biomcp help sync gencc

```text
Usage: biomcp help sync gencc [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help sync gencc.


## biomcp help sync wikipathways

```text
Usage: biomcp help sync wikipathways [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help sync wikipathways.


## biomcp help sync cbioportal-studies

```text
Usage: biomcp help sync cbioportal-studies [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help sync cbioportal-studies.


## biomcp help sync ema

```text
Usage: biomcp help sync ema [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help sync ema.


## biomcp help sync ddinter

```text
Usage: biomcp help sync ddinter [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help sync ddinter.


## biomcp help sync who-pq

```text
Usage: biomcp help sync who-pq [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help sync who-pq.


## biomcp help sync gtr

```text
Usage: biomcp help sync gtr [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help sync gtr.


## biomcp help sync who-ivd

```text
Usage: biomcp help sync who-ivd [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help sync who-ivd.


## biomcp help sync fda-cdx

```text
Usage: biomcp help sync fda-cdx [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help sync fda-cdx.


## biomcp help sync orphadata

```text
Usage: biomcp help sync orphadata [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help sync orphadata.


## biomcp help skill

```text
Usage: biomcp help skill [OPTIONS] [COMMAND]

Commands:
  list     List supported discovery topics.
  render
  status
  install

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help skill.


## biomcp help skill list

```text
List supported discovery topics.

Usage: biomcp help skill list [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

List supported discovery topics.


## biomcp help skill render

```text
Usage: biomcp help skill render [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help skill render.


## biomcp help skill status

```text
Usage: biomcp help skill status [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help skill status.


## biomcp help skill install

```text
Usage: biomcp help skill install [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help skill install.


## biomcp help source

```text
Usage: biomcp help source [OPTIONS] [COMMAND]

Commands:
  list  List supported discovery topics.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help source.


## biomcp help source list

```text
List supported discovery topics.

Usage: biomcp help source list [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

List supported discovery topics.


## biomcp help version

```text
Usage: biomcp help version [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help version.


## biomcp help mcp-config

```text
Usage: biomcp help mcp-config [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help mcp-config.


## biomcp help health

```text
Report source terms and bounded reachability.

Usage: biomcp help health [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Report source terms and bounded reachability.


## biomcp help store

```text
Manage local cached replies and snapshots.

Usage: biomcp help store [OPTIONS] [COMMAND]

Commands:
  clean

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Manage local cached replies and snapshots.


## biomcp help store clean

```text
Usage: biomcp help store clean [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help store clean.


## biomcp help key

```text
Register and manage source credentials without showing values.

Usage: biomcp help key [OPTIONS] [source] [COMMAND]

Commands:
  list    List supported discovery topics.
  set
  remove

Arguments:
  [source]
          Credential source to describe.

          [possible values: ncbi, semantic-scholar, orcid, oncokb, alphagenome, nci, openfda]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Register and manage source credentials without showing values.
Listed choices accept any letter case; replies use their listed spelling.

- `source`: Credential source to describe.
Form: help key SOURCE; not --source.

## biomcp help key list

```text
List supported discovery topics.

Usage: biomcp help key list [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

List supported discovery topics.


## biomcp help key set

```text
Usage: biomcp help key set [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help key set.


## biomcp help key remove

```text
Usage: biomcp help key remove [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help key remove.


## biomcp help serve

```text
Serve MCP over standard input or HTTP.

Usage: biomcp help serve [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Serve MCP over standard input or HTTP.


## biomcp help schema

```text
Usage: biomcp help schema [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help schema.


## biomcp help biomarker

```text
Usage: biomcp help biomarker [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help biomarker.


## biomcp help papers

```text
Usage: biomcp help papers [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help papers.


## biomcp help organization

```text
Usage: biomcp help organization [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help organization.


## biomcp help assay-count

```text
Usage: biomcp help assay-count [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help assay-count.


## biomcp help gwas

```text
Usage: biomcp help gwas [OPTIONS] [section]

Arguments:
  [section]
          One section of the get command to describe.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help gwas.

- `section`: One section of the get command to describe.
Form: help gwas SECTION; not --section.

## biomcp help features

```text
Usage: biomcp help features [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help features.


## biomcp help isoforms

```text
Usage: biomcp help isoforms [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help isoforms.


## biomcp help domains

```text
Usage: biomcp help domains [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help domains.


## biomcp help complexes

```text
Usage: biomcp help complexes [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help complexes.


## biomcp help complex-members

```text
Usage: biomcp help complex-members [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help complex-members.


## biomcp help structures

```text
Usage: biomcp help structures [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help structures.


## biomcp help sequence

```text
Usage: biomcp help sequence [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help sequence.


## biomcp help phenotype

```text
Usage: biomcp help phenotype [OPTIONS] [section]

Arguments:
  [section]
          One section of the get command to describe.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help phenotype.

- `section`: One section of the get command to describe.
Form: help phenotype SECTION; not --section.

## biomcp help genes

```text
Usage: biomcp help genes [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help genes.


## biomcp help events

```text
Usage: biomcp help events [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help events.


## biomcp help study

```text
Usage: biomcp help study [OPTIONS] [section]

Arguments:
  [section]
          One section of the get command to describe.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help study.

- `section`: One section of the get command to describe.
Form: help study SECTION; not --section.

## biomcp help pgx

```text
Usage: biomcp help pgx [OPTIONS] [section]

Arguments:
  [section]
          One section of the get command to describe.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help pgx.

- `section`: One section of the get command to describe.
Form: help pgx SECTION; not --section.

## biomcp help diagnostic

```text
Usage: biomcp help diagnostic [OPTIONS] [section]

Arguments:
  [section]
          One section of the get command to describe.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help diagnostic.

- `section`: One section of the get command to describe.
Form: help diagnostic SECTION; not --section.

## biomcp help adverse-event

```text
Usage: biomcp help adverse-event [OPTIONS] [section]

Arguments:
  [section]
          One section of the get command to describe.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help adverse-event.

- `section`: One section of the get command to describe.
Form: help adverse-event SECTION; not --section.

## biomcp help disease

```text
Usage: biomcp help disease [OPTIONS] [section]

Arguments:
  [section]
          One section of the get command to describe.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help disease.

- `section`: One section of the get command to describe.
Form: help disease SECTION; not --section.

## biomcp help drug

```text
Usage: biomcp help drug [OPTIONS] [section]

Arguments:
  [section]
          One section of the get command to describe.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help drug.

- `section`: One section of the get command to describe.
Form: help drug SECTION; not --section.

## biomcp help gene

```text
Usage: biomcp help gene [OPTIONS] [section]

Arguments:
  [section]
          One section of the get command to describe.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help gene.

- `section`: One section of the get command to describe.
Form: help gene SECTION; not --section.

## biomcp help supplement

```text
Usage: biomcp help supplement [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help supplement.


## biomcp help pdf

```text
Usage: biomcp help pdf [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help pdf.


## biomcp help text

```text
Usage: biomcp help text [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help text.


## biomcp help citations

```text
Usage: biomcp help citations [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help citations.


## biomcp help references

```text
Usage: biomcp help references [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help references.


## biomcp help citation-evidence

```text
Usage: biomcp help citation-evidence [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help citation-evidence.


## biomcp help recommendations

```text
Usage: biomcp help recommendations [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help recommendations.


## biomcp help document

```text
Usage: biomcp help document [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help document.


## biomcp help assertions

```text
Usage: biomcp help assertions [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help assertions.


## biomcp help population

```text
Usage: biomcp help population [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help population.


## biomcp help recurrence

```text
Usage: biomcp help recurrence [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help recurrence.


## biomcp help normalize

```text
Usage: biomcp help normalize [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help normalize.


## biomcp help help

```text
Describe commands and their arguments.

Usage: biomcp help help [OPTIONS] [command]...

Arguments:
  [command]...
          Command help path to describe.

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe commands and their arguments.

- `command`: Command help path to describe.
Form: help help COMMAND; not --command.

## biomcp help network

```text
Usage: biomcp help network [OPTIONS] [setting]

Arguments:
  [setting]
          Network setting to describe.

          [possible values: ca_bundle, proxy]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help network.
Listed choices accept any letter case; replies use their listed spelling.

- `setting`: Network setting to describe.
Form: help network SETTING; not --setting.

## biomcp help sources

```text
Usage: biomcp help sources [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help sources.


## biomcp help config

```text
Usage: biomcp help config [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Describe help config.


## biomcp help exit

```text
Explain process exit and error codes.

Usage: biomcp help exit [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Process exit codes:
0: complete or partial reply
1: no matching record
2: invalid command or arguments
3: expired result pool
4: source request failed
5: local configuration, storage or output failed
Error codes: usage.schema, usage.arguments, usage.mcp_unavailable, usage.unknown_command, usage.retired, local.failure, pool.expired, usage.pool_mismatch, usage.too_many_pmids, usage.too_broad, usage.unsupported_filter. Source problems and reasons appear in their reply fields.


## biomcp help questions

```text
List every command by the question it answers.

Usage: biomcp help questions [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

List every command by the question it answers.


## biomcp help examples

```text
Show each use case's first call in CLI and typed form.

Usage: biomcp help examples [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Show each use case's first call in CLI and typed form.


## biomcp help relations

```text
Map entities through sections and search filters.

Usage: biomcp help relations [OPTIONS]

Options:
      --format <format>
          Choose JSON or Markdown output; configuration can change the default.

          [default: json]
          [possible values: json, md]

      --disable-sources <disable-sources>
          Exclude source names or source terms groups. Example: --disable-sources civic.

      --source_pacing <source_pacing>
          Override source pacing with a JSON map of rate, keyed_rate and in_flight values. Example: --source_pacing '{}'.

      --config <config>
          Read configuration from this file. Example: --config config.json.

      --offline[=<offline>]
          Read local data without source requests.

          [default: false]
          [possible values: true, false]

      --fresh[=<fresh>]
          Request fresh source data instead of cached replies.

          [default: false]
          [possible values: true, false]

```

Map entities through sections and search filters.


