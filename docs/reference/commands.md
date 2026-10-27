# Command reference

Generated from the command grammar and schema help. Run `make docs-generate` to update.

Options shown on the root command apply to its subcommands. Query sections and source rules follow each command.

## biomcp

```text
Usage: biomcp [OPTIONS] <COMMAND>

Commands:
  author      
  search      
  get         
  cell-line   
  protein     
  pathway     
  article     
  trial       
  variant     
  sync        
  skill       Read embedded guidance and worked examples. No source requests. Overview and catalog are bounded by the embedded payload; install and status return one summary. Install writes only inside DIR and refuses symbolic links and parent traversal.
  version     Print version; --verbose adds build, schema and path provenance. No source requests or key reads.
  mcp-config  Print MCP configuration with this executable’s absolute path. JSON formats name their top-level object and retain released client aliases. The codex client prints its shell registration command. No source requests or key reads.
  health      
  store       
  key         
  serve       
  help        

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh
          

-h, --help: Print scoped command help without source requests.
--version: Print the installed version without source requests.

```

## biomcp author

```text
Usage: biomcp author [OPTIONS] <COMMAND>

Commands:
  papers  

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp author papers

```text
Usage: biomcp author papers [OPTIONS] <id> [section]

Arguments:
  <id>
          

  [section]
          

Options:
      --format <format>
          [possible values: json, md]

      --full
          

      --disable-sources <disable-sources>
          

  -l, --limit <limit>
          

      --config <config>
          

      --offset <offset>
          

      --pool <pool>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

One exact provider record's publication window. ORCID sends its whole public works list in one reply and pages groups locally. Author IDs identify provider records, not resolved people. Counts cover that provider record. Missing metadata is unavailable. Topic-wide author ranking and cross-provider merging are unavailable. Semantic Scholar and ORCID are noncommercial; commercial users disable both with --disable-sources noncommercial or config disable_sources. Optional keys: biomcp key set semantic-scholar or S2_API_KEY; biomcp key set orcid or ORCID_ACCESS_TOKEN. IDs use semantic-scholar:ID (alias semanticscholar:ID) or orcid:ID. Search supports --source semantic-scholar (alias semanticscholar), --query/-q and --limit/-l; papers supports --limit/-l and Semantic Scholar details/--full.

- `--full`: Legacy alias for details; normalized on construction.
- `id`: Exact qualified provider record.
- `--limit`: Bounded row count.
- `--offset`: Native or retained collection offset.
- `--pool`: Retained collection identity.
- `section`: Detailed disclosure when requested.

Detailed disclosure when requested.

- `details`: Native detailed metadata for the same paper window.

## biomcp search

```text
Usage: biomcp search [OPTIONS] <COMMAND>

Commands:
  author         
  cell-line      
  gwas           
  protein        
  phenotype      
  pathway        
  study          
  pgx            
  diagnostic     
  adverse-event  
  disease        
  drug           
  gene           
  article        
  trial          
  variant        

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp search author

```text
Usage: biomcp search author [OPTIONS] [query]

Arguments:
  [query]
          

Options:
      --format <format>
          [possible values: json, md]

  -l, --limit <limit>
          

      --disable-sources <disable-sources>
          

      --offset <offset>
          

      --config <config>
          

      --pool <pool>
          

      --replay <replay>
          

      --source <source>
          

  -q, --query <query_option>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

Literal author name query and native window; positional text maps to query. Author IDs identify provider records, not resolved people. Counts cover that provider record. Missing metadata is unavailable. Topic-wide author ranking and cross-provider merging are unavailable. Semantic Scholar and ORCID are noncommercial; commercial users disable both with --disable-sources noncommercial or config disable_sources. Optional keys: biomcp key set semantic-scholar or S2_API_KEY; biomcp key set orcid or ORCID_ACCESS_TOKEN. IDs use semantic-scholar:ID (alias semanticscholar:ID) or orcid:ID. Search supports --source semantic-scholar (alias semanticscholar), --query/-q and --limit/-l; papers supports --limit/-l and Semantic Scholar details/--full.

- `--limit`: Bounded source page size.
- `--offset`: Native offset.
- `--pool`: Retained page identity.
- `query`: Literal query from the positional argument or --query.
- `--source`: Native author source.

## biomcp search cell-line

```text
Usage: biomcp search cell-line [OPTIONS] [query]

Arguments:
  [query]
          

Options:
      --format <format>
          [possible values: json, md]

  -l, --limit <limit>
          

      --disable-sources <disable-sources>
          

      --offset <offset>
          

      --config <config>
          

  -q, --query <query_option>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

One literal cell line search, with native order and no total claim. An assay count is not an activity or sensitivity count. No treatment response is inferred; no bioinformatics tool runs.

- `--limit`: Native page size.
- `--offset`: Native start offset.
- `query`: Literal source query; positional spelling and --query use the same field.

## biomcp search gwas

```text
Usage: biomcp search gwas [OPTIONS] [query]

Arguments:
  [query]
          

Options:
      --extended-geneset
          

      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

  -g, --gene <gene>
          

      --config <config>
          

      --include-child-traits
          

  -l, --limit <limit>
          

      --replay <replay>
          

      --offset <offset>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --p-value <p_value>
          

      --observe <observe>
          

      --pool <pool>
          

      --offline
          

      --pubmed <pubmed>
          

      --fresh
          

      --study <study>
          

      --trait <trait>
          

      --trait-id <trait_id>
          

      --variant <variant>

```

Source associations in default p-value-ascending order. Reports no personal risk, causal variant or gene, or clinical interpretation of a statistic.

- `--extended-geneset`: Source extended gene set; requires gene.
- `--gene`: Exact native mapped gene; legacy positional query aliases this field.
- `--include-child-traits`: Source child-trait expansion; requires trait_id.
- `--limit`: Maximum association rows.
- `--offset`: Native association offset.
- `--p-value`: Inclusive positive probability in scientific notation. Requires the default p-value-ascending order; no sort option is available. The qualifying total stays unknown until the ordered walk reaches its end.
- `--pool`: Retained threshold walk identity; requires a threshold.
- `--pubmed`: Exact source publication identity.
- `query`: Exact native mapped gene; legacy positional query aliases this field.
- `--study`: Exact source study accession.
- `--trait`: Literal native efo_trait label; never searches reported_trait prose.
- `--trait-id`: Exact native ontology identity.
- `--variant`: Exact source rsID.

## biomcp search protein

```text
Usage: biomcp search protein [OPTIONS] [query]

Arguments:
  [query]
          

Options:
      --format <format>
          [possible values: json, md]

      --gene <gene>
          

      --disable-sources <disable-sources>
          

      --limit <limit>
          

      --config <config>
          

      --offset <offset>
          

      --pool <pool>
          

      --replay <replay>
          

      --record <record>
          

      --reviewed
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

Search five reviewed human UniProt headers by literal text or exact gene.
No variant effect, structure, interaction or clinical meaning is inferred. No bioinformatics tool runs.

- `--gene`: Exact native gene field, without identifier inference.
- `--limit`: Five rows by default; no more than 25.
- `--offset`: Source/retained result offset.
- `--pool`: Exact retained collection identity.
- `query`: Literal text, exclusive with gene.
- `--reviewed`: Reviewed human scope stays enabled; unreviewed search is deferred.

## biomcp search phenotype

```text
Usage: biomcp search phenotype [OPTIONS] <text>

Arguments:
  <text>
          

Options:
      --format <format>
          [possible values: json, md]

  -l, --limit <limit>
          

      --disable-sources <disable-sources>
          

      --offset <offset>
          

      --config <config>
          

      --source <source>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

Search literal human HP term candidates through Monarch with bounded native pages. A term lookup answers terminology and does not establish a diagnosis.

- `--limit`: Maximum native headers, default five.
- `--offset`: Native source row offset.
- `--source`: Monarch only; HPO search is unsupported.
- `text`: Unchanged literal term search text.

## biomcp search pathway

```text
Usage: biomcp search pathway [OPTIONS] [query]

Arguments:
  [query]
          

Options:
      --format <format>
          [possible values: json, md]

      --fresh
          

      --disable-sources <disable-sources>
          

      --gene <gene>
          

      --config <config>
          

  -l, --limit <limit>
          

      --offset <offset>
          

      --replay <replay>
          

      --pool <pool>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --source <source>
          

      --observe <observe>
          

      --top-level
          

      --offline

```

Search Reactome (default) or WikiPathways human pathways. Membership names a source and release; it implies no therapy, response or actionability. No enrichment is calculated.

- `--fresh`: Request a new retained observation.
- `--gene`: Human gene input for exact resolution and UniProt mapping; Reactome only.
- `--limit`: Five rows by default, one to fifty. Reactome rows equals this limit without refill.
- `--offset`: Native Reactome Start row or retained WikiPathways catalog offset.
- `--pool`: Retained snapshot identity for deterministic continuation.
- `query`: Literal source query; exclusive with gene and top-level discovery.
- `--source`: Sources in caller order; repeated sources take turns without title joins.
- `--top-level`: Native human Reactome top-level discovery, exclusive with text and gene.

## biomcp search study

```text
Usage: biomcp search study [OPTIONS] [text]

Arguments:
  [text]
          

Options:
      --cancer-type <cancer_type>
          

      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --limit <limit>
          

      --config <config>
          

      --offset <offset>
          

      --pool <pool>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

Literal stored metadata search; text is never parsed as a query language. No frequency, denominator, survival or comparison is calculated. Source counts keep their native names.

- `--cancer-type`: Exact cBioPortal cancer-type code; no OncoTree conversion.
- `--limit`: Maximum headers per page.
- `--offset`: Zero-based offset in the checked snapshot.
- `--pool`: Opaque continuation retaining the catalog revision and selectors.
- `text`: Literal study ID, name or description text, without synonyms or typo repair.

## biomcp search pgx

```text
Usage: biomcp search pgx [OPTIONS] --gene <gene> --source <source>

Options:
      --cpic-level <cpic_level>
          

      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --gene <gene>
          

      --config <config>
          

      --limit <limit>
          

      --offset <offset>
          

      --replay <replay>
          

      --record <record>
          

      --source <source>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

Checked gene-keyed CPIC pair search. No phenotype, phase or dose is calculated.

- `--cpic-level`: Literal native level, including compound levels such as B/C.
- `--gene`: Explicit source gene symbol.
- `--limit`: Rows per native page.
- `--offset`: Native source offset.
- `--source`: Required CPIC source; ClinPGx is not supported for search.

## biomcp search diagnostic

```text
Usage: biomcp search diagnostic [OPTIONS] --source <source>

Options:
      --biomarker <biomarker>
          

      --format <format>
          [possible values: json, md]

      --bundle-revision <bundle_revision>
          

      --disable-sources <disable-sources>
          

      --condition-id <condition_id>
          

      --config <config>
          

      --device-id <device_id>
          

      --replay <replay>
          

      --drug-application <drug_application>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --gene <gene>
          

      --gene-id <gene_id>
          

      --observe <observe>
          

      --limit <limit>
          

      --offline
          

      --fresh
          

      --manufacturer <manufacturer>
          

      --marker <marker>
          

      --offset <offset>
          

      --source <source>
          

      --type <type>

```

Search one diagnostic source. A listing is not authorization; a companion row is not a treatment recommendation.

- `--biomarker`: Exact table-0 biomarker cell; unsupported in the group table.
- `--bundle-revision`: Immutable stored bundle revision retained by a continuation.
- `--condition-id`: Exact MedGen:CUI identifier only.
- `--device-id`: Exact PMA application, application/supplement or native K identity.
- `--drug-application`: Cell-owned typed NDA/BLA application.
- `--gene`: Exact source gene_symbol field, without alias expansion.
- `--gene-id`: Exact source GeneID, exclusive with gene symbol.
- `--limit`: Five results by default; at most fifty source rows or product headers.
- `--manufacturer`: Literal whole-word phrase in WHO's Manufacturer name cell.
- `--marker`: Literal whole-word phrase in WHO's Pathogen/Disease/Marker cell.
- `--offset`: Zero-based source result offset.
- `--source`: Explicit native GTR, WHO IVD or FDA companion discovery.
- `--type`: Exact native test_type value.

## biomcp search adverse-event

```text
Usage: biomcp search adverse-event [OPTIONS] <text> [section]

Arguments:
  <text>
          

  [section]
          

Options:
      --format <format>
          [possible values: json, md]

      --limit <limit>
          

      --disable-sources <disable-sources>
          

      --offset <offset>
          

      --config <config>
          

      --reaction <reaction>
          

      --received-from <received_from>
          

      --replay <replay>
          

      --received-to <received_to>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --serious <serious>
          

      --observe <observe>
          

      --snapshot <snapshot>
          

      --offline
          

      --source <source>
          

      --fresh

```

Search five report headers by default. VAERS accepts MMR, COVID19, COVID-19 vaccine or one exact qualified native influenza code, matched without case; use influenza to list codes and labels. Tables remain separate and are never summed. One report may list several vaccines. VAERS returns five reported symptom rows by default; seriousness and age select independent tables. Counts are reports, not people. Reports do not show that a vaccine caused an event and cannot measure risk or compare vaccine safety.

- `--limit`: Native report headers or VAERS rows per page; default five.
- `--offset`: Native row offset. VAERS continuation requires its retained snapshot.
- `--reaction`: FAERS MedDRA term phrase match, not exact equality.
- `--received-from`: Inclusive first FDA receipt lower bound, YYYY-MM-DD.
- `--received-to`: Inclusive first FDA receipt upper bound, YYYY-MM-DD.
- `section`: Independent VAERS table; omitted means reported symptoms.
- `--serious`: FAERS report flag: death or hospitalization; never a reaction outcome.
- `--snapshot`: Retained VAERS table for provider-free continuation.
- `--source`: FAERS by default; VAERS accepts only exact supported vaccine names.
- `text`: Literal drug text, MAUDE device phrase or exact supported VAERS vaccine.

Independent VAERS table; omitted means reported symptoms.

- `symptoms`: Native reported symptom rows.
- `seriousness`: Reports classified by native seriousness.
- `age`: Native age groups.

## biomcp search disease

```text
Usage: biomcp search disease [OPTIONS] [text]

Arguments:
  [text]
          

Options:
      --format <format>
          [possible values: json, md]

      --limit <limit>
          

      --disable-sources <disable-sources>
          

      --offset <offset>
          

      --config <config>
          

      --ontology <ontology>
          

      --replay <replay>
          

      --source <source>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

Literal discovery text or an explicit SEER catalog page.

- `--limit`: Maximum source rows on one page.
- `--offset`: Native row offset.
- `--ontology`: Exactly one ontology, default MONDO.
- `--source`: One explicit source, default OLS.
- `text`: Literal source search text; absent for the SEER site catalog.

## biomcp search drug

```text
Usage: biomcp search drug [OPTIONS] <query>

Arguments:
  <query>
          

Options:
      --format <format>
          [possible values: json, md]

      --limit <limit>
          

      --disable-sources <disable-sources>
          

      --name-selector <name_selector>
          

      --config <config>
          

      --offset <offset>
          

      --replay <replay>
          

      --source <source>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

Literal single-source candidate discovery, without an implicit identity selection.

- `--limit`: Native candidate rows per page.
- `--name-selector`: Explicit ChEMBL native name selector; omitted means preferred name.
- `--offset`: Native rows to skip.
- `query`: Literal source name selector, unchanged by the consumer.
- `--source`: Exactly one source; default MyChem never fans out.

## biomcp search gene

```text
Usage: biomcp search gene [OPTIONS] [query]

Arguments:
  [query]
          

Options:
      --band <band>
          

      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --limit <limit>
          

      --config <config>
          

      --locus-group <locus_group>
          

      --offset <offset>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

Search genes by text, cytoband, or their intersection.

- `--band`: A cytoband prefix.
- `--limit`: Number of results per page.
- `--locus-group`: Optional HGNC locus group intersected with the selector.
- `--offset`: Number of results to skip.
- `query`: Words matched against symbols, previous symbols, aliases and names.

## biomcp search article

```text
Usage: biomcp search article [OPTIONS] [text]

Arguments:
  [text]
          

Options:
      --author <author>
          

      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --disease <disease>
          

      --config <config>
          

      --drug <drug>
          

      --gene <gene>
          

      --replay <replay>
          

      --journal <journal>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --limit <limit>
          

      --no-preprints
          

      --observe <observe>
          

      --offline
          

      --offset <offset>
          

      --fresh
          

      --open-access
          

      --pool <pool>
          

      --exclude-retracted
          

      --include-retracted
          

      --since <since>
          

      --sort <sort>
          

      --source <source>
          

      --type <type>
          

      --until <until>
          

      --variant <variant>

```

Checked search selectors and pool cursor.

- `--author`: Author words sent to PubMed's author field.
- `--disease`: Disease words.
- `--drug`: Drug words.
- `--gene`: Gene input; PubMed resolves it through HGNC.
- `--journal`: Journal words sent to PubMed's journal field.
- `--limit`: Maximum page length.
- `--no-preprints`: Exclude preprints using the source's native filter.
- `--offset`: Number of pool items to skip.
- `--open-access`: Require the source's native open-access filter; Semantic Scholar means a public PDF.
- `--pool`: Stable snapshot from an earlier response.
- `--exclude-retracted`: Include retracted papers by default, or exclude them using a native filter.
- `--include-retracted`: Include retracted papers by default, or exclude them using a native filter.
- `--since`: Inclusive lower publication-date bound at the stated precision; PubMed matches either electronic or print date.
- `--sort`: rank (default): native provider order; several sources take turns; native search cost.
date: native search cost; PubMed means date added and Europe PMC means newest first; other sources refuse it.
citations: most cited first across the whole pool; adds iCite calls, about 1 s per 1,000 PMIDs.
All sources support citations.
- `--source`: Sources to search in caller order, comma separated. Default: pubmed.
Several sources take turns, remove duplicates and show every native rank.
pubmed: keywords with PubMed MeSH mapping; dropped words appear as notices.
europepmc: words in open full text and preprints; incidental mentions are possible.
litvar: exact variant publications; requires --variant and a protein change needs --gene; unresolved inputs return candidates.
semantic-scholar: broad discovery; a shared keyless pool may return 429; an optional free key raises the limit.
litsense: sentence resemblance, not a match; unrelated hits and timeouts are possible.
pubtator: entity and word search including full text; needs the right entity ID; --gene uses HGNC Entrez identity and cannot run without that join.
- `text`: Words sent to each named source in its default fields.
- `--type`: Publication-type filter.
- `--until`: Inclusive upper publication-date bound at the stated precision; PubMed matches either electronic or print date.
- `--variant`: Variant words.

## biomcp search trial

```text
Usage: biomcp search trial [OPTIONS] [query]

Arguments:
  [query]
          

Options:
      --age <age>
          

      --format <format>
          [possible values: json, md]

      --condition <condition>
          

      --disable-sources <disable-sources>
          

      --condition-id <condition_id>
          

      --config <config>
          

      --count-only
          

      --replay <replay>
          

      --criteria <criteria>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --distance <distance>
          

      --facility <facility>
          

      --observe <observe>
          

      --gene <gene>
          

      --offline
          

      --fresh
          

      --has-results
          

      --id <id>
          

      --intervention <intervention>
          

      --keyword <keyword>
          

      --lat <lat>
          

      --limit <limit>
          

      --lon <lon>
          

      --next-page <next_page>
          

      --phase <phase>
          

      --sex <sex>
          

      --source <source>
          

      --sponsor <sponsor>
          

      --sponsor-type <sponsor_type>
          

      --status <status>
          

      --study-type <study_type>
          

      --updated-since <updated_since>
          

      --updated-until <updated_until>
          

      --variant <variant>

```

Search one provider with bounded pages and source-native counts; default ClinicalTrials.gov, explicit keyed NCI, never a fallback.

- `--age`: Age in years; unstated source bounds count as no limit.
- `--condition`: Registry condition text, or an NCIt code for NCI; use --keyword for NCI free text.
- `--condition-id`: Exact MONDO or DOID CURIE resolved by MyDisease; NCI also accepts explicit NCIT:C codes.
- `--count-only`: Source total only; no returned study rows.
- `--criteria`: Search literal registry eligibility text for discovery only; no eligibility judgment.
- `--distance`: Source distance with explicit mi or km units.
- `--facility`: Exact registry facility field; cannot combine with radius.
- `--gene`: Resolve one checked gene for registry text discovery only; no eligibility judgment.
- `--has-results`: Require posted source results.
- `--id`: Exact NCT identity in a conjunction with other filters.
- `--intervention`: Literal source intervention text.
- `--keyword`: NCI free text; explicit keyword mode never substitutes for a disease identity.
- `--lat`: Radius center latitude in degrees.
- `--limit`: Headers per page; NCI applies its own lower publisher limit.
- `--lon`: Radius center longitude in degrees.
- `--next-page`: Opaque registry continuation; keep filters unchanged.
- `--phase`: Registry phase: 1, 2, 3, 4, early-1, not-applicable, 1/2 or 2/3.
- `query`: General registry discovery text.
- `--sex`: Registry sex facet.
- `--source`: Explicit provider; defaults to ClinicalTrials.gov and never falls back.
- `--sponsor`: Exact lead sponsor field.
- `--sponsor-type`: Registry sponsor funding facet.
- `--status`: Exact registry overall status.
- `--study-type`: Exact registry study type.
- `--updated-since`: First included registry update date.
- `--updated-until`: Last included registry update date.
- `--variant`: Literal variant text for registry discovery only; no eligibility judgment.

## biomcp search variant

```text
Usage: biomcp search variant [OPTIONS]

Options:
      --build <build>
          

      --format <format>
          [possible values: json, md]

      --condition <condition>
          

      --disable-sources <disable-sources>
          

      --config <config>
          

      --gene <gene>
          

      --has <has>
          

      --replay <replay>
          

      --limit <limit>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --offset <offset>
          

      --observe <observe>
          

      --protein <protein>
          

      --offline
          

      --query <query>
          

      --fresh
          

      --region <region>
          

      --review-status <review_status>
          

      --source <source>

```

Discover source candidates using one declared selector, without interpreting free text.

- `--build`: Optional caller assembly sent to MyVariant; regions require it explicitly.
- `--condition`: Literal MyVariant clinvar.rcv.conditions.name phrase; source text is not a local diagnosis. Requires a declared protein or region selector.
- `--gene`: Exact checked gene symbol for the fixed dbNSFP gene/protein field query; obeys component terms switches.
- `--has`: Require the selected component's identifier field. Only source-proven mappings are offered.
- `--limit`: Native candidates per page, default 10, maximum 50; all candidates remain candidates.
- `--offset`: Native offset within this command's bounded result window; larger native totals remain visible.
- `--protein`: Explicit substitution using checked one-letter or three-letter amino-acid codes.
- `--query`: Raw MyVariant query, passed and echoed unchanged when component policy allows; requires --source myvariant. With disabled components use declared selectors instead.
- `--region`: Exact one-based inclusive MyVariant region, such as chr7:55259500-55259600; requires --build.
- `--review-status`: Literal MyVariant clinvar.rcv.review_status phrase; no star conversion. Requires a declared protein or region selector.
- `--source`: Explicit source language; required for a raw query.

## biomcp get

```text
Usage: biomcp get [OPTIONS] <COMMAND>

Commands:
  author         
  cell-line      
  gwas           
  protein        
  phenotype      
  pathway        
  study          
  pgx            
  diagnostic     
  adverse-event  
  disease        
  drug           
  gene           
  article        
  trial          
  variant        

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp get author

```text
Usage: biomcp get author [OPTIONS] <ids>...

Arguments:
  <ids>...
          

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

Exact qualified records in caller order, with no name reconciliation. Author IDs identify provider records, not resolved people. Counts cover that provider record. Missing metadata is unavailable. Topic-wide author ranking and cross-provider merging are unavailable. Semantic Scholar and ORCID are noncommercial; commercial users disable both with --disable-sources noncommercial or config disable_sources. Optional keys: biomcp key set semantic-scholar or S2_API_KEY; biomcp key set orcid or ORCID_ACCESS_TOKEN. IDs use semantic-scholar:ID (alias semanticscholar:ID) or orcid:ID. Search supports --source semantic-scholar (alias semanticscholar), --query/-q and --limit/-l; papers supports --limit/-l and Semantic Scholar details/--full.

- `ids`: One or more exact qualified IDs.

## biomcp get cell-line

```text
Usage: biomcp get cell-line [OPTIONS] <id> [section]

Arguments:
  <id>
          

  [section]
          

Options:
      --format <format>
          [possible values: json, md]

      --section-limit <section_limit>
          

      --disable-sources <disable-sources>
          

      --section-offset <section_offset>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

One exact identity and an optional bounded named section. An assay count is not an activity or sensitivity count. No treatment response is inferred; no bioinformatics tool runs.

- `id`: One exact primary accession or qualified source ID.
- `section`: Omit for the default card; all returns bounded first pages without an assay count.
- `--section-limit`: Explicit row limit for each requested collection; native variation defaults to three.
- `--section-offset`: Offset within each independently requested collection.

Omit for the default card; all returns bounded first pages without an assay count.

- `names`: Native display names.
- `identity`: Independent species, diseases and qualified warnings.
- `xrefs`: Complete source cross-reference rows.
- `variants`: Complete native variation rows.
- `chembl`: Exact accession-linked ChEMBL cell metadata.
- `all`: Bounded first page of every named section.

## biomcp get gwas

```text
Usage: biomcp get gwas [OPTIONS] <id> [section]

Arguments:
  <id>
          

  [section]
          

Options:
      --format <format>
          [possible values: json, md]

      --section-limit <section_limit>
          

      --disable-sources <disable-sources>
          

      --section-offset <section_offset>
          

      --config <config>
          

      --snapshot <snapshot>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

Source association or study and native context. Reports no personal risk, causal variant or gene, or clinical interpretation of a statistic.

- `id`: Qualified association ID or exact GCST study accession.
- `section`: Association loci or study associations and ancestries.
- `--section-limit`: Maximum native relation rows.
- `--section-offset`: Native relation row offset.
- `--snapshot`: Retained ancestry reply for a local page.

Association loci or study associations and ancestries.

- `loci`: Native association loci.
- `associations`: Native study association pages.
- `ancestries`: Native study ancestry rows.

## biomcp get protein

```text
Usage: biomcp get protein [OPTIONS] <input>...

Arguments:
  <input>...
          

Options:
      --format <format>
          [possible values: json, md]

      --section <section>
          

      --disable-sources <disable-sources>
          

      --section-limit <section_limit>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

Read source protein cards; a symbol returns candidates without choosing a protein.
Sections disclose native facts with independent row bounds; function keeps all paragraphs.
No variant effect, structure, interaction or clinical meaning is inferred. No bioinformatics tool runs.

- `input`: Source accessions or gene symbols, preserving caller order.
- `--section`: Select native sections or all bounded first pages; function keeps one whole text record.
- `--section-limit`: First-page row limit; function keeps one whole native text record and does not page its paragraphs.

Select native sections or all bounded first pages; function keeps one whole text record.

- `features`: Native UniProt features.
- `domains`: Native InterPro entry matches.
- `isoforms`: Native alternative products.
- `complexes`: Exact participant-admitted complex headers.
- `structures`: Native structure cross-reference links.
- `function`: One whole native function text record with all source paragraphs.
- `sequence`: Direct exact-identity sequence and audit metadata.
- `all`: Bounded first page of every named section.

## biomcp get phenotype

```text
Usage: biomcp get phenotype [OPTIONS] <id> [section]

Arguments:
  <id>
          

  [section]
          

Options:
      --format <format>
          [possible values: json, md]

      --section-limit <section_limit>
          

      --disable-sources <disable-sources>
          

      --section-offset <section_offset>
          

      --config <config>
          

      --snapshot <snapshot>
          

      --replay <replay>
          

      --source <source>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

Read an exact HP term from HPO or Monarch, or page native source sections. A term lookup answers terminology; an association answers what its publisher asserts. Neither establishes a diagnosis.

- `id`: Exact HP identifier.
- `section`: Absent means the source identity card.
- `--section-limit`: Default five text/link rows or one complete association row.
- `--section-offset`: Native section row offset.
- `--snapshot`: Retained exact source reply for local text and link pages; unsupported for diseases and genes.
- `--source`: HPO or Monarch for cards, synonyms and xrefs; Monarch only for hierarchy, diseases and genes. Defaults to HPO for cards, synonyms and xrefs, otherwise Monarch.

Absent means the source identity card.

- `synonyms`: Native synonyms.
- `xrefs`: Native cross references.
- `hierarchy`: Native direct parent and child links.
- `diseases`: Native disease association rows.
- `genes`: Native human gene association rows.

## biomcp get pathway

```text
Usage: biomcp get pathway [OPTIONS] <id> [section]

Arguments:
  <id>
          

  [section]
          

Options:
      --format <format>
          [possible values: json, md]

      --fresh
          

      --disable-sources <disable-sources>
          

      --pool <pool>
          

      --config <config>
          

      --section-limit <section_limit>
          

      --replay <replay>
          

      --section-offset <section_offset>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline

```

Get an exact human Reactome R-HSA or WikiPathways WP card. Read whole summary, genes, Reactome events or bounded all sections. Membership names a source and release; it implies no therapy, response or actionability. No enrichment is calculated.

- `--fresh`: Request a new source observation.
- `id`: Exact human Reactome or WikiPathways identity without version suffixes.
- `--pool`: Retained snapshot, when continuing a source relation.
- `section`: Omit for the card; WikiPathways has summary and genes sections.
- `--section-limit`: Bounded initial section size; each all section has its own first page.
- `--section-offset`: Native relation offset for a selected genes or events section.

Omit for the card; WikiPathways has summary and genes sections.

- `summary`: Complete native source description.
- `genes`: Namespaced native participants with exact resolved gene joins.
- `events`: Native Reactome event objects and unresolved database references.
- `all`: Each source-supported section's bounded first page.

## biomcp get study

```text
Usage: biomcp get study [OPTIONS] <id> [section]

Arguments:
  <id>
          

  [section]
          

Options:
      --format <format>
          [possible values: json, md]

      --section-limit <section_limit>
          

      --disable-sources <disable-sources>
          

      --section-offset <section_offset>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

Exact case-sensitive study card or bounded metadata section. No frequency, denominator, survival or comparison is calculated. Source counts keep their native names.

- `id`: Exact source study ID; no default or repair.
- `section`: Omit for the card; all returns bounded first pages only.
- `--section-limit`: Maximum records in each selected section.
- `--section-offset`: Offset within one named section; all requires zero.

Omit for the card; all returns bounded first pages only.

- `profiles`: Five exact native molecular profile references.
- `sample-lists`: Five sample-list metadata records, without member sample IDs.
- `files`: Five repository file references; no automatic download.
- `all`: Each named section's bounded first page; never a gene aggregate request.

## biomcp get pgx

```text
Usage: biomcp get pgx [OPTIONS] --source <source> [section]

Arguments:
  [section]
          

Options:
      --annotation-id <annotation_id>
          

      --format <format>
          [possible values: json, md]

      --diplotype <diplotype>
          

      --disable-sources <disable-sources>
          

      --config <config>
          

      --drug-id <drug_id>
          

      --gene <gene>
          

      --replay <replay>
          

      --kind <kind>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --section-limit <section_limit>
          

      --observe <observe>
          

      --section-offset <section_offset>
          

      --offline
          

      --snapshot <snapshot>
          

      --fresh
          

      --source <source>

```

PGx card or section. No phenotype, phase or dose is calculated.

- `--annotation-id`: Checked ClinPGx accession, never a shared PaId.
- `--diplotype`: Unchanged declared expression required for the diplotype section only.
- `--drug-id`: Exact published CPIC drug key; recommendations only.
- `--gene`: Explicit source gene symbol.
- `--kind`: One ClinPGx collection; clinical is the default only on annotations.
- `section`: One named section; omitted means the CPIC gene card.
- `--section-limit`: Rows per page: three clinical, five elsewhere.
- `--section-offset`: Offset within the selected section.
- `--snapshot`: Retained collection for local continuation.
- `--source`: Required source, checked against the section.

One named section; omitted means the CPIC gene card.

- `recommendations`: Complete native recommendation contexts.
- `guidelines`: Native guideline references.
- `diplotype`: Declared source diplotype lookup.
- `annotations`: One ClinPGx annotation kind.

## biomcp get diagnostic

```text
Usage: biomcp get diagnostic [OPTIONS] --source <source> <id> [section]

Arguments:
  <id>
          

  [section]
          

Options:
      --bundle-revision <bundle_revision>
          

      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --section-limit <section_limit>
          

      --config <config>
          

      --section-offset <section_offset>
          

      --replay <replay>
          

      --source <source>
          

      --record <record>
          

      --supplement <supplement>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

Get one exact source record or supported section. A listing is not authorization; a companion row is not a treatment recommendation.

- `--bundle-revision`: Immutable stored bundle revision retained by a continuation.
- `id`: Exact native identity, including version for GTR.
- `section`: gtr: genes, conditions, methods. fda-pma: regulatory, supplements. WHO and 510(k) have no sections; omit for the card.
- `--section-limit`: Maximum native rows; omitted means five rows, or one regulatory fragment.
- `--section-offset`: Zero-based native row or fragment offset.
- `--source`: Explicit source; gtr needs accession/version, who-ivd a known Product ID, fda-pma a P number, fda-510k a K number.
- `--supplement`: Exact native PMA supplement; absent selects the base row.

gtr: genes, conditions, methods. fda-pma: regulatory, supplements. WHO and 510(k) have no sections; omit for the card.

- `genes`: GTR native gene rows; five per page.
- `conditions`: GTR native condition rows; five per page.
- `methods`: GTR native method entries; five per page.
- `regulatory`: Selected PMA submission statement; one fragment per page.
- `supplements`: PMA base application supplement headers; five per page.

## biomcp get adverse-event

```text
Usage: biomcp get adverse-event [OPTIONS] <id> [section]

Arguments:
  <id>
          

  [section]
          

Options:
      --drug-index <drug_index>
          

      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --section-limit <section_limit>
          

      --config <config>
          

      --section-offset <section_offset>
          

      --replay <replay>
          

      --snapshot <snapshot>
          

      --record <record>
          

      --source <source>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

Exact native report identity and an optional independently paged section.

- `--drug-index`: Absolute source-array index; required only for identifier sections.
- `id`: Published report ID with no source prefix.
- `section`: FAERS owns drugs, reactions and identifiers; MAUDE owns devices and narrative. Omitted means card only.
- `--section-limit`: Whole source rows per section page.
- `--section-offset`: Offset within the selected native section.
- `--snapshot`: Checked retained report snapshot for provider-free continuation.
- `--source`: FAERS by default; MAUDE is explicit.

FAERS owns drugs, reactions and identifiers; MAUDE owns devices and narrative. Omitted means card only.

- `drugs`: Independent native drug rows.
- `reactions`: Independent native reaction rows.
- `unii`: Native UNII strings.
- `rxcui`: Native RxCUI strings.
- `product-ndc`: Native product NDC strings.
- `package-ndc`: Native package NDC strings.
- `spl-id`: Native SPL IDs.
- `spl-set-id`: Native SPL set IDs.
- `application-number`: Native application numbers.
- `devices`: Native device rows.
- `narrative`: Complete native narrative rows.

## biomcp get disease

```text
Usage: biomcp get disease [OPTIONS] <id> [section]

Arguments:
  <id>
          

  [section]
          

Options:
      --age-range <age_range>
          

      --format <format>
          [possible values: json, md]

      --dataset <dataset>
          

      --disable-sources <disable-sources>
          

      --config <config>
          

      --interval <interval>
          

      --ontology <ontology>
          

      --replay <replay>
          

      --period <period>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --race <race>
          

      --observe <observe>
          

      --section-limit <section_limit>
          

      --offline
          

      --section-offset <section_offset>
          

      --fresh
          

      --sex <sex>
          

      --source <source>

```

Exact concept or native SEER site with independently paged section.

- `--age-range`: Native age-range code.
- `--dataset`: Requested native dataset selector, not proof of returned membership.
- `id`: Exact ontology CURIE or an explicit SEER catalog site ID.
- `--interval`: Survival-only interval code.
- `--ontology`: One OLS ontology.
- `--period`: Incidence-only native period code.
- `--race`: Native race code.
- `section`: One named section; absent means identity card only.
- `--section-limit`: Native rows per page: default 2 for Monarch genes, 1 for Monarch phenotypes, otherwise 5.
- `--section-offset`: Native row offset within the selected section.
- `--sex`: Native sex code.
- `--source`: Default MyDisease; no automatic fallback.

One named section; absent means identity card only.

- `synonyms`: Publisher scoped synonyms.
- `xrefs`: Native cross references.
- `hierarchy`: Native parent relations.
- `genes`: Native gene association rows.
- `phenotypes`: Native phenotype association rows.
- `incidence`: Native stage incidence rows.
- `survival`: Annual survival rows.

## biomcp get drug

```text
Usage: biomcp get drug [OPTIONS] --source <source> <input> [section]...

Arguments:
  <input>
          

  [section]...
          

Options:
      --civic-cursor <civic_cursor>
          

      --format <format>
          [possible values: json, md]

      --civic-kind <civic_kind>
          

      --disable-sources <disable-sources>
          

      --civic-profile <civic_profile>
          

      --config <config>
          

      --label-section <label_section>
          

      --replay <replay>
          

      --product-number <product_number>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --section-limit <section_limit>
          

      --observe <observe>
          

      --section-offset <section_offset>
          

      --offline
          

      --source <source>
          

      --fresh

```

Exact native ID retrieval with an explicitly selected source.

- `--civic-cursor`: Native continuation cursor for that exact drug, profile and claim collection.
- `--civic-kind`: Independent native evidence or assertion collection within the selected profile.
- `--civic-profile`: Selected native CIViC molecular profile for a drug-scoped claim stream.
- `input`: Exact source-native ID; WHO references retain native spaces. No name or alias resolution.
- `--label-section`: Exact native label heading, required only for the label section.
- `--product-number`: Explicit native application product number, only for approvals.
- `section`: Explicit named disclosures; empty means identity only.
- `--section-limit`: Named section page size, refused on an identity-only card.
- `--section-offset`: Named section continuation, refused on an identity-only card.
- `--source`: Required source; there is no default exact-ID resolver.

Explicit named disclosures; empty means identity only.

- `xrefs`: All native identity cross-references and product links, mechanically paged.
- `targets`: Native target and mechanism relations.
- `indications`: Native indication wording.
- `civic`: Native CIViC therapy evidence and profile context.
- `label`: One exact native label heading.
- `approvals`: Native application and product history rows.
- `regulatory`: Source-scoped regulator events.
- `interactions`: Native DDInter partner rows and levels; five rows per default page, with explicit section limits preserved.

## biomcp get gene

```text
Usage: biomcp get gene [OPTIONS] <input>...

Arguments:
  <input>...
          

Options:
      --format <format>
          [possible values: json, md]

      --section <section>
          

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

Get one to twenty human gene inputs in their original order.

- `input`: Gene symbols or checked identifiers, in input order.
- `--section`: Requested section words, preserving caller order and repetitions.
Trailing section words select sections. Use --section WORD to keep inputs that equal a section word as inputs.

Requested section words, preserving caller order and repetitions.
Trailing section words select sections. Use --section WORD to keep inputs that equal a section word as inputs.

- `protein`: Reviewed human proteins.
- `civic`: CIViC gene curation.
- `clingen`: ClinGen validity and dosage.
- `constraint`: Population constraint metrics.
- `expression`: Human Protein Atlas expression.
- `pathways`: Human Reactome pathways.
- `oncokb`: Keyed OncoKB annotations.
- `all`: Every landed section.

## biomcp get article

```text
Usage: biomcp get article [OPTIONS] <input>...

Arguments:
  <input>...
          

Options:
      --format <format>
          [possible values: json, md]

      --section <section>
          

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

Get one to twenty article inputs in caller order.

- `input`: PubMed, PMC or DOI identifiers in input order.
- `--section`: Requested section words; trailing words and --section share this field.

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
Usage: biomcp get trial [OPTIONS] <input>...

Arguments:
  <input>...
          

Options:
      --format <format>
          [possible values: json, md]

      --limit <limit>
          

      --disable-sources <disable-sources>
          

      --offset <offset>
          

      --config <config>
          

      --pool <pool>
          

      --replay <replay>
          

      --section <section>
          

      --record <record>
          

      --source <source>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

Retrieve exact trial cards in input order; request bounded intact source sections with independent continuations.

- `input`: One to twenty exact NCT identifiers, in caller order.
- `--limit`: Whole-record count per source collection page; text fragments follow the text budget.
- `--offset`: Character offset for ClinicalTrials.gov eligibility or NCI summary; whole-record offset for other collections.
- `--pool`: Checked stored source record, supplied by a continuation.
- `--section`: Optional intact source sections. Default cards only; diseases and summary require NCI. NCI locations, contacts, references and documents are unsupported.
- `--source`: Explicit provider; defaults to ClinicalTrials.gov and never falls back.

Optional intact source sections. Default cards only; diseases and summary require NCI. NCI locations, contacts, references and documents are unsupported.

- `eligibility`: Intact criteria and structured age/sex fields.
- `outcomes`: Primary, secondary and other outcome records.
- `arms`: Arms and interventions with original relationships.
- `locations`: Whole source sites with their attached contacts.
- `contacts`: Central contacts and overall officials in their original roles.
- `references`: Registry publication identities and native reference types.
- `documents`: Posted document metadata and source URLs.
- `all`: Every implemented trial section, each with independent continuations.
- `diseases`: Native NCI disease concepts, including source enrichment metadata.
- `summary`: Complete NCI summary, paged by original character offsets.

## biomcp get variant

```text
Usage: biomcp get variant [OPTIONS] <input> [section]...

Arguments:
  <input>
          

  [section]...
          

Options:
      --build <build>
          

      --format <format>
          [possible values: json, md]

      --civic-cursor <civic_cursor>
          

      --disable-sources <disable-sources>
          

      --civic-kind <civic_kind>
          

      --config <config>
          

      --civic-profile <civic_profile>
          

      --replay <replay>
          

      --limit <limit>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --offset <offset>
          

      --observe <observe>
          

      --resolver <resolver>
          

      --offline
          

      --section-limit <section_limit>
          

      --fresh
          

      --section-offset <section_offset>
          

      --tumor-type <tumor_type>

```

Read an rsID, CAid, ClinVar Variation ID or reference-qualified HGVS; chromosome notation requires --build. Use search variant --gene BRAF --protein V600E for shorthand. Every candidate remains available; source assertions do not classify the variant for the caller.

- `--build`: Required for chromosome notation; no assembly is guessed.
- `--civic-cursor`: Native CIViC cursor, retained unchanged.
- `--civic-kind`: Native CIViC claim collection within the selected profile.
- `--civic-profile`: Selected CIViC molecular profile for native claim paging.
- `input`: Published rsID, CAid, ClinVar Variation ID or reference-qualified HGVS.
- `--limit`: Candidate rows per page; more remain reachable through typed next commands.
- `--offset`: Candidate rows to skip in source order.
- `--resolver`: Resolver for reference-qualified notation; never repairs the input locally.
- `section`: Named source sections; omitted by default. Each calls its own source. all keeps bounded pages and source skips; it never enables a missing key.
- `--section-limit`: Rows per named source section page; omitted uses the native source default.
- `--section-offset`: Source rows to skip independently of identity candidate paging.
- `--tumor-type`: Explicit tumor context for keyed OncoKB annotation.

Named source sections; omitted by default. Each calls its own source. all keeps bounded pages and source skips; it never enables a missing key.

- `clinvar`: ClinVar native assertions.
- `erepo`: ClinGen native assertions.
- `population`: gnomAD population observations.
- `gwas`: GWAS Catalog associations.
- `civic`: CIViC molecular-profile evidence.
- `oncokb`: Keyed OncoKB annotation with explicit tumor context and build.
- `hotspots`: Cancer Hotspots candidate residue counts; no established exact allele join or clinical actionability.
- `predict`: MyVariant source predictor scores.
- `conservation`: MyVariant conservation scores.
- `cosmic`: Historical COSMIC fragments carried by MyVariant; parent-to-fragment allele identity remains unverified.
- `all`: Every implemented variant section.

## biomcp cell-line

```text
Usage: biomcp cell-line [OPTIONS] <COMMAND>

Commands:
  assay-count  

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp cell-line assay-count

```text
Usage: biomcp cell-line assay-count [OPTIONS] <id>

Arguments:
  <id>
          

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

A separate assay-count request, never implicit in metadata or all. An assay count is not an activity or sensitivity count. No treatment response is inferred; no bioinformatics tool runs.

- `id`: Exact ChEMBL cell identity returned by metadata.

## biomcp protein

```text
Usage: biomcp protein [OPTIONS] <COMMAND>

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
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp protein features

```text
Usage: biomcp protein features [OPTIONS] <accession>

Arguments:
  <accession>
          

Options:
      --format <format>
          [possible values: json, md]

      --limit <limit>
          

      --disable-sources <disable-sources>
          

      --offset <offset>
          

      --config <config>
          

      --pool <pool>
          

      --position <position>
          

      --replay <replay>
          

      --record <record>
          

      --type <type>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

Page five native UniProt features by exact type and displayed residue. No variant effect or clinical meaning is inferred; no bioinformatics tool runs.

- `accession`: Exact source protein sequence identity.
- `--limit`: Five rows by default, at most 25.
- `--offset`: Native or retained page offset.
- `--pool`: Retained collection identity.
- `--position`: Native one-based residue; zero fails during deserialization.
- `--type`: Exact UniProt native label, without an alias map.

## biomcp protein isoforms

```text
Usage: biomcp protein isoforms [OPTIONS] <accession>

Arguments:
  <accession>
          

Options:
      --format <format>
          [possible values: json, md]

      --limit <limit>
          

      --disable-sources <disable-sources>
          

      --offset <offset>
          

      --config <config>
          

      --pool <pool>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

Page five source isoforms, exact-participant complexes or PDB links. Source identities stay separate. No structure, interaction or clinical meaning is inferred; no bioinformatics tool runs.

- `accession`: Exact source protein sequence identity.
- `--limit`: Five rows by default, at most 25.
- `--offset`: Native or retained page offset.
- `--pool`: Retained collection identity.

## biomcp protein domains

```text
Usage: biomcp protein domains [OPTIONS] <accession>

Arguments:
  <accession>
          

Options:
      --format <format>
          [possible values: json, md]

      --limit <limit>
          

      --disable-sources <disable-sources>
          

      --offset <offset>
          

      --config <config>
          

      --pool <pool>
          

      --replay <replay>
          

      --type <type>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

Page five native InterPro matches with original types and fragments. No structure or clinical meaning is inferred; no bioinformatics tool runs.

- `accession`: Exact source protein sequence identity.
- `--limit`: Five rows by default, at most 25.
- `--offset`: Native or retained page offset.
- `--pool`: Retained collection identity.
- `--type`: Exact native InterPro type, not a normalized domain label.

## biomcp protein complexes

```text
Usage: biomcp protein complexes [OPTIONS] <accession>

Arguments:
  <accession>
          

Options:
      --format <format>
          [possible values: json, md]

      --limit <limit>
          

      --disable-sources <disable-sources>
          

      --offset <offset>
          

      --config <config>
          

      --pool <pool>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

Page five source isoforms, exact-participant complexes or PDB links. Source identities stay separate. No structure, interaction or clinical meaning is inferred; no bioinformatics tool runs.

- `accession`: Exact source protein sequence identity.
- `--limit`: Five rows by default, at most 25.
- `--offset`: Native or retained page offset.
- `--pool`: Retained collection identity.

## biomcp protein complex-members

```text
Usage: biomcp protein complex-members [OPTIONS] --complex <complex> <accession>

Arguments:
  <accession>
          

Options:
      --complex <complex>
          

      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --limit <limit>
          

      --config <config>
          

      --offset <offset>
          

      --pool <pool>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

Page five native participants of an exact Complex Portal complex containing the anchor. No interaction or clinical meaning is inferred; no bioinformatics tool runs.

- `accession`: Exact source protein sequence identity.
- `--complex`: Exact complex whose native participants contain the anchor.
- `--limit`: Five rows by default, at most 25.
- `--offset`: Native or retained page offset.
- `--pool`: Retained collection identity.

## biomcp protein structures

```text
Usage: biomcp protein structures [OPTIONS] <accession>

Arguments:
  <accession>
          

Options:
      --format <format>
          [possible values: json, md]

      --limit <limit>
          

      --disable-sources <disable-sources>
          

      --offset <offset>
          

      --config <config>
          

      --pool <pool>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

Page five source isoforms, exact-participant complexes or PDB links. Source identities stay separate. No structure, interaction or clinical meaning is inferred; no bioinformatics tool runs.

- `accession`: Exact source protein sequence identity.
- `--limit`: Five rows by default, at most 25.
- `--offset`: Native or retained page offset.
- `--pool`: Retained collection identity.

## biomcp protein sequence

```text
Usage: biomcp protein sequence [OPTIONS] <isoform>

Arguments:
  <isoform>
          

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

Read one complete native sequence for an exact source accession.
No variant effect, structure or clinical meaning is inferred; no bioinformatics tool runs.

- `isoform`: Exact isoform or base accession; no canonical suffix is synthesized.

## biomcp pathway

```text
Usage: biomcp pathway [OPTIONS] <COMMAND>

Commands:
  genes   
  events  

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp pathway genes

```text
Usage: biomcp pathway genes [OPTIONS] <id> [details]

Arguments:
  <id>
          

  [details]
          

Options:
      --format <format>
          [possible values: json, md]

  -l, --limit <limit>
          

      --disable-sources <disable-sources>
          

      --offset <offset>
          

      --config <config>
          

      --pool <pool>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

Read Reactome exact references or WikiPathways GeneProduct data nodes. The details layer discloses complete Reactome physical contexts. Membership names a source and release; it implies no therapy, response or actionability. No enrichment is calculated.

- `details`: Include every physical entity context for these references; never clips source text.
- `id`: Checked source-owned human pathway identity.
- `--limit`: Five complete relation rows by default, at most fifty.
- `--offset`: Offset within the retained selected relation.
- `--pool`: Immutable relation snapshot.

## biomcp pathway events

```text
Usage: biomcp pathway events [OPTIONS] <id>

Arguments:
  <id>
          

Options:
      --format <format>
          [possible values: json, md]

  -l, --limit <limit>
          

      --disable-sources <disable-sources>
          

      --offset <offset>
          

      --config <config>
          

      --pool <pool>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

Read Reactome event objects and unresolved native integer references in source order. Membership names a source and release; it implies no therapy, response or actionability. No enrichment is calculated.

- `id`: Checked source-owned human pathway identity.
- `--limit`: Five complete relation rows by default, at most fifty.
- `--offset`: Offset within the retained selected relation.
- `--pool`: Immutable relation snapshot.

## biomcp article

```text
Usage: biomcp article [OPTIONS] <COMMAND>

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
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp article supplement

```text
Usage: biomcp article supplement [OPTIONS] <input> <file>

Arguments:
  <input>
          

  <file>
          

Options:
      --find <find>
          

      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --offset <offset>
          

      --config <config>
          

      --replace
          

      --replay <replay>
          

      --rows <rows>
          

      --record <record>
          

      --save <save>
          

      --case-cache <case-cache>
          

      --sheet <sheet>
          

      --observe <observe>
          

      --variant <variant>
          

      --offline
          

      --fresh

```

Read a checked supplement file or page its original rows.

- `file`: Exact file name from the supplements section.
- `--find`: Literal case-insensitive needle.
- `input`: PubMed or PMC identity.
- `--offset`: Matching-row or text-fragment offset.
- `--replace`: Replace an existing destination only when explicitly requested with save.
- `--rows`: Inclusive original row range.
- `--save`: Save the whole original document to an explicit file or existing folder.
- `--sheet`: Sheet name for row selection.
- `--variant`: Explicit protein substitution.

## biomcp article pdf

```text
Usage: biomcp article pdf [OPTIONS] --save <save> <input>

Arguments:
  <input>
          

Options:
      --format <format>
          [possible values: json, md]

      --replace
          

      --disable-sources <disable-sources>
          

      --save <save>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

Download one publisher PDF to an explicit file or existing folder.

- `input`: Article identity.
- `--replace`: Replace an existing file only when explicitly requested.
- `--save`: Destination file or existing folder.

## biomcp article text

```text
Usage: biomcp article text [OPTIONS] <input>

Arguments:
  <input>
          

Options:
      --find <find>
          

      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --offset <offset>
          

      --config <config>
          

      --part <part>
          

      --replace
          

      --replay <replay>
          

      --record <record>
          

      --save <save>
          

      --case-cache <case-cache>
          

      --variant <variant>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

Read one bounded body part, find matching rows, or save the complete Markdown document.

- `--find`: Case-insensitive literal text to find.
- `input`: PubMed or PMC identity. DOI selectors are not accepted by the reader.
- `--offset`: Number of matching rows already returned.
- `--part`: One-based outline part number.
- `--replace`: Replace an existing destination only when explicitly requested with save.
- `--save`: Save the whole original document to an explicit file or existing folder.
- `--variant`: Explicit checked protein substitution.

## biomcp article citations

```text
Usage: biomcp article citations [OPTIONS] <pmid>

Arguments:
  <pmid>
          

Options:
      --format <format>
          [possible values: json, md]

      --limit <limit>
          

      --disable-sources <disable-sources>
          

      --offset <offset>
          

      --config <config>
          

      --pool <pool>
          

      --replay <replay>
          

      --source <source>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

Page the citing or referenced PubMed papers of one checked PMID.

- `--limit`: Maximum papers on one page.
- `--offset`: Number of pool items to skip.
- `pmid`: PMID, PMC or DOI; iCite accepts only a PMID.
- `--pool`: Fixed snapshot from an earlier page.
- `--source`: Graph provider; iCite remains the default.

## biomcp article references

```text
Usage: biomcp article references [OPTIONS] <pmid>

Arguments:
  <pmid>
          

Options:
      --format <format>
          [possible values: json, md]

      --limit <limit>
          

      --disable-sources <disable-sources>
          

      --offset <offset>
          

      --config <config>
          

      --pool <pool>
          

      --replay <replay>
          

      --source <source>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

Page the citing or referenced PubMed papers of one checked PMID.

- `--limit`: Maximum papers on one page.
- `--offset`: Number of pool items to skip.
- `pmid`: PMID, PMC or DOI; iCite accepts only a PMID.
- `--pool`: Fixed snapshot from an earlier page.
- `--source`: Graph provider; iCite remains the default.

## biomcp article citation-evidence

```text
Usage: biomcp article citation-evidence [OPTIONS] <citing> <cited>

Arguments:
  <citing>
          

  <cited>
          

Options:
      --format <format>
          [possible values: json, md]

      --fulltext
          

      --disable-sources <disable-sources>
          

      --offset <offset>
          

      --config <config>
          

      --paragraph-offset <paragraph_offset>
          

      --pool <pool>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

Read every source context and optional explicit full-text citation paragraph.

- `cited`: Cited PMID, PMC identity or DOI.
- `citing`: Citing PMID, PMC identity or DOI.
- `--fulltext`: Include the citing paper's own explicit reference paragraphs.
- `--offset`: Context fragment position returned by the advancing step.
- `--paragraph-offset`: Full-text fragment position returned by the advancing step.
- `--pool`: Stored checked source scan and original hit positions.

## biomcp article recommendations

```text
Usage: biomcp article recommendations [OPTIONS] <seeds>...

Arguments:
  <seeds>...
          

Options:
      --format <format>
          [possible values: json, md]

      --limit <limit>
          

      --disable-sources <disable-sources>
          

      --negative <negative>...
          

      --config <config>
          

      --offset <offset>
          

      --pool <pool>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

Recommendations in the provider's recent-paper pool order.

- `--limit`: Maximum recommendations requested, in native order.
- `--negative`: Up to ten negative paper identities.
- `--offset`: Number of stored recommendations to skip.
- `--pool`: Fixed recommendation pool.
- `seeds`: One to ten positive paper identities.

## biomcp trial

```text
Usage: biomcp trial [OPTIONS] <COMMAND>

Commands:
  document  

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp trial document

```text
Usage: biomcp trial document [OPTIONS] --save <save> <input> <file>

Arguments:
  <input>
          

  <file>
          

Options:
      --format <format>
          [possible values: json, md]

      --replace
          

      --disable-sources <disable-sources>
          

      --save <save>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

Download one exact ClinicalTrials.gov manifest PDF to an explicit caller path; never read its text. CLI and stdio MCP support saves; HTTP refuses with the source URL.

- `file`: Exact filename from the registry manifest; never a URL or inferred path.
- `input`: Exact registry identity.
- `--replace`: Replace an existing file only when explicitly requested.
- `--save`: Caller-owned file or existing folder.

## biomcp variant

```text
Usage: biomcp variant [OPTIONS] <COMMAND>

Commands:
  assertions  
  population  
  recurrence  
  normalize   

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp variant assertions

```text
Usage: biomcp variant assertions [OPTIONS] --source <source> <input>

Arguments:
  <input>
          

Options:
      --assertion <assertion>
          

      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --limit <limit>
          

      --config <config>
          

      --offset <offset>
          

      --replay <replay>
          

      --source <source>
          

      --record <record>
          

      --version <version>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

Page assertions from one explicitly selected evidence source.

- `--assertion`: Native assertion accession or document ID.
- `input`: Checked Variation ID for ClinVar, or CAid for ERepo.
- `--limit`: Assertions per page; omitted uses the native source default.
- `--offset`: Assertions to skip in source order.
- `--source`: One native source; no fallback or text search.
- `--version`: VCV archive or ERepo document version; RCV/SCV selectors match a returned native version.

## biomcp variant population

```text
Usage: biomcp variant population [OPTIONS] <input>

Arguments:
  <input>
          

Options:
      --build <build>
          

      --format <format>
          [possible values: json, md]

      --detail
          

      --disable-sources <disable-sources>
          

      --config <config>
          

      --limit <limit>
          

      --offset <offset>
          

      --replay <replay>
          

      --record <record>
          

      --resolver <resolver>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

Read a bounded ancestry-detail page from the exact source-given GRCh38 allele.

- `--build`: Explicit source assembly when required by identity input.
- `--detail`: Read source ancestry groups, preserving overlapping groups individually.
- `input`: Exact published allele identity; ambiguous candidates must first be selected.
- `--limit`: Maximum rows per page.
- `--offset`: Native rows to skip.
- `--resolver`: Resolver for reference-qualified input.

## biomcp variant recurrence

```text
Usage: biomcp variant recurrence [OPTIONS] --gene <gene> --profile <profile> --sample-set <sample_set> --source <source> --study <study> <input>

Arguments:
  <input>
          

Options:
      --format <format>
          [possible values: json, md]

      --gene <gene>
          

      --disable-sources <disable-sources>
          

      --limit <limit>
          

      --config <config>
          

      --page <page>
          

      --profile <profile>
          

      --replay <replay>
          

      --record <record>
          

      --sample-set <sample_set>
          

      --case-cache <case-cache>
          

      --source <source>
          

      --observe <observe>
          

      --study <study>
          

      --offline
          

      --fresh

```

Observe exact-allele samples within one caller-selected source page.

- `--gene`: HGNC symbol resolved to the source's Entrez gene identifier.
- `input`: Published variant identity; ambiguous identities are never selected silently.
- `--limit`: Source mutation rows per page, including excluded alleles.
- `--page`: Zero-based native source page.
- `--profile`: Explicit mutation molecular profile.
- `--sample-set`: Explicit sample list; selected size is never an allele denominator.
- `--source`: Explicit cohort source.
- `--study`: Explicit study; there is no default cohort.

## biomcp variant normalize

```text
Usage: biomcp variant normalize [OPTIONS] --hgvs <hgvs> --source <source>

Options:
      --build <build>
          

      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --hgvs <hgvs>
          

      --config <config>
          

      --limit <limit>
          

      --offset <offset>
          

      --replay <replay>
          

      --record <record>
          

      --source <source>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

Send unchanged reference-qualified notation to explicitly named services.

- `--build`: Caller assembly; required when VariantValidator is selected.
- `--hgvs`: Unchanged reference-qualified HGVS; no local molecular parse.
- `--limit`: Source mapping rows per service, with typed continuation for remaining rows.
- `--offset`: Source mapping rows to skip per service.
- `--source`: Explicit service; all preserves independent results without consensus.

## biomcp sync

```text
Usage: biomcp sync [OPTIONS] [bundle]

Arguments:
  [bundle]
          [possible values: wikipathways, cbioportal-studies, hgnc, clingen, ema, ddinter, who-pq, gtr, who-ivd, fda-cdx]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

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
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

Read embedded guidance and worked examples. No source requests. Overview and catalog are bounded by the embedded payload; install and status return one summary. Install writes only inside DIR and refuses symbolic links and parent traversal.


## biomcp skill list

```text
List embedded worked examples.

Usage: biomcp skill list [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

List embedded worked examples.


## biomcp skill render

```text
Print the canonical agent guide.

Usage: biomcp skill render [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

Print the canonical agent guide.


## biomcp skill status

```text
Report missing, unmanaged, locally_modified, stale or current guidance.

Usage: biomcp skill status [OPTIONS] <directory>

Arguments:
  <directory>
          

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

Report missing, unmanaged, locally_modified, stale or current guidance.

- `directory`: 

## biomcp skill install

```text
Install into DIR/skills/biomcp; a skills or biomcp directory is accepted directly. Existing files remain unchanged without --force.

Usage: biomcp skill install [OPTIONS] <directory>

Arguments:
  <directory>
          

Options:
      --force
          Repair managed files; retain unrelated files.

      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

Install into DIR/skills/biomcp; a skills or biomcp directory is accepted directly. Existing files remain unchanged without --force.

- `directory`: 
- `--force`: Repair managed files; retain unrelated files.

## biomcp version

```text
Print version; --verbose adds build, schema and path provenance. No source requests or key reads.

Usage: biomcp version [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --verbose
          Include commit, target, schema versions, config and cache paths.

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

Print version; --verbose adds build, schema and path provenance. No source requests or key reads.

- `--verbose`: Include commit, target, schema versions, config and cache paths.

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
          [possible values: json, md]

      --absolute-path
          Retained compatibility flag; executable paths are always absolute.

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

Print MCP configuration with this executable’s absolute path. JSON formats name their top-level object and retain released client aliases. The codex client prints its shell registration command. No source requests or key reads.

- `--client`: Choose a JSON client or the codex shell registration command; default mcp-servers.
- `--absolute-path`: Retained compatibility flag; executable paths are always absolute.

## biomcp health

```text
Usage: biomcp health [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

Report publisher terms and bounded source reachability; at most one page for each implemented source.


## biomcp store

```text
Usage: biomcp store [OPTIONS] <COMMAND>

Commands:
  clean  

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp store clean

```text
Usage: biomcp store clean [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

Remove expired replies, pools and snapshots and older incompatible cache files. Return removal counts in one bounded reply. Fresh state and bundles remain.


## biomcp key

```text
Usage: biomcp key [OPTIONS] <COMMAND>

Commands:
  list    
  set     
  remove  

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp key list

```text
Usage: biomcp key list [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

List supported credential names and their origin without values.


## biomcp key set

```text
Usage: biomcp key set [OPTIONS] <source>

Arguments:
  <source>
          [possible values: ncbi, semantic-scholar, orcid, oncokb, alphagenome, nci, openfda]

Options:
      --format <format>
          [possible values: json, md]

      --replace
          

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp key remove

```text
Usage: biomcp key remove [OPTIONS] <source>

Arguments:
  <source>
          [possible values: ncbi, semantic-scholar, orcid, oncokb, alphagenome, nci, openfda]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp serve

```text
Usage: biomcp serve [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --http [<http>]
          

      --allow-host <allow-host>
          

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

Serve MCP over stdio or --http [ADDR]. Non-loopback binds require --allow-host HOST or http.allowed_hosts; neither authenticates callers. HTTP refuses sync, store clean and --fresh. Stdio saves refuse --replace.


## biomcp help

```text
Usage: biomcp help [OPTIONS] [COMMAND]

Commands:
  author      
  search      
  get         
  cell-line   
  protein     
  pathway     
  article     
  trial       
  variant     
  sync        
  skill       
  version     
  mcp-config  
  health      
  store       
  key         
  serve       
  network     
  sources     
  config      

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help author

```text
Usage: biomcp help author [OPTIONS] [COMMAND]

Commands:
  papers  

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help author papers

```text
Usage: biomcp help author papers [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help search

```text
Usage: biomcp help search [OPTIONS] [COMMAND]

Commands:
  author         
  cell-line      
  gwas           
  protein        
  phenotype      
  pathway        
  study          
  pgx            
  diagnostic     
  adverse-event  
  disease        
  drug           
  gene           
  article        
  trial          
  variant        

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help search author

```text
Usage: biomcp help search author [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help search cell-line

```text
Usage: biomcp help search cell-line [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help search gwas

```text
Usage: biomcp help search gwas [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help search protein

```text
Usage: biomcp help search protein [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help search phenotype

```text
Usage: biomcp help search phenotype [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help search pathway

```text
Usage: biomcp help search pathway [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help search study

```text
Usage: biomcp help search study [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help search pgx

```text
Usage: biomcp help search pgx [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help search diagnostic

```text
Usage: biomcp help search diagnostic [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help search adverse-event

```text
Usage: biomcp help search adverse-event [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help search disease

```text
Usage: biomcp help search disease [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help search drug

```text
Usage: biomcp help search drug [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help search gene

```text
Usage: biomcp help search gene [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help search article

```text
Usage: biomcp help search article [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help search trial

```text
Usage: biomcp help search trial [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help search variant

```text
Usage: biomcp help search variant [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help get

```text
Usage: biomcp help get [OPTIONS] [COMMAND]

Commands:
  author         
  cell-line      
  gwas           
  protein        
  phenotype      
  pathway        
  study          
  pgx            
  diagnostic     
  adverse-event  
  disease        
  drug           
  gene           
  article        
  trial          
  variant        

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help get author

```text
Usage: biomcp help get author [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help get cell-line

```text
Usage: biomcp help get cell-line [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help get gwas

```text
Usage: biomcp help get gwas [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help get protein

```text
Usage: biomcp help get protein [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help get phenotype

```text
Usage: biomcp help get phenotype [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help get pathway

```text
Usage: biomcp help get pathway [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help get study

```text
Usage: biomcp help get study [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help get pgx

```text
Usage: biomcp help get pgx [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help get diagnostic

```text
Usage: biomcp help get diagnostic [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help get adverse-event

```text
Usage: biomcp help get adverse-event [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help get disease

```text
Usage: biomcp help get disease [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help get drug

```text
Usage: biomcp help get drug [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help get gene

```text
Usage: biomcp help get gene [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help get article

```text
Usage: biomcp help get article [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help get trial

```text
Usage: biomcp help get trial [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help get variant

```text
Usage: biomcp help get variant [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help cell-line

```text
Usage: biomcp help cell-line [OPTIONS] [COMMAND]

Commands:
  assay-count  

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help cell-line assay-count

```text
Usage: biomcp help cell-line assay-count [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help protein

```text
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
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help protein features

```text
Usage: biomcp help protein features [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help protein isoforms

```text
Usage: biomcp help protein isoforms [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help protein domains

```text
Usage: biomcp help protein domains [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help protein complexes

```text
Usage: biomcp help protein complexes [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help protein complex-members

```text
Usage: biomcp help protein complex-members [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help protein structures

```text
Usage: biomcp help protein structures [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help protein sequence

```text
Usage: biomcp help protein sequence [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help pathway

```text
Usage: biomcp help pathway [OPTIONS] [COMMAND]

Commands:
  genes   
  events  

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help pathway genes

```text
Usage: biomcp help pathway genes [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help pathway events

```text
Usage: biomcp help pathway events [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help article

```text
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
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help article supplement

```text
Usage: biomcp help article supplement [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help article pdf

```text
Usage: biomcp help article pdf [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help article text

```text
Usage: biomcp help article text [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help article citations

```text
Usage: biomcp help article citations [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help article references

```text
Usage: biomcp help article references [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help article citation-evidence

```text
Usage: biomcp help article citation-evidence [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help article recommendations

```text
Usage: biomcp help article recommendations [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help trial

```text
Usage: biomcp help trial [OPTIONS] [COMMAND]

Commands:
  document  

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help trial document

```text
Usage: biomcp help trial document [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help variant

```text
Usage: biomcp help variant [OPTIONS] [COMMAND]

Commands:
  assertions  
  population  
  recurrence  
  normalize   

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help variant assertions

```text
Usage: biomcp help variant assertions [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help variant population

```text
Usage: biomcp help variant population [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help variant recurrence

```text
Usage: biomcp help variant recurrence [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help variant normalize

```text
Usage: biomcp help variant normalize [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help sync

```text
Usage: biomcp help sync [OPTIONS] [COMMAND]

Commands:
  wikipathways        
  cbioportal-studies  
  ema                 
  ddinter             
  who-pq              
  gtr                 
  who-ivd             
  fda-cdx             

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help sync wikipathways

```text
Usage: biomcp help sync wikipathways [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help sync cbioportal-studies

```text
Usage: biomcp help sync cbioportal-studies [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help sync ema

```text
Usage: biomcp help sync ema [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help sync ddinter

```text
Usage: biomcp help sync ddinter [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help sync who-pq

```text
Usage: biomcp help sync who-pq [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help sync gtr

```text
Usage: biomcp help sync gtr [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help sync who-ivd

```text
Usage: biomcp help sync who-ivd [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help sync fda-cdx

```text
Usage: biomcp help sync fda-cdx [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help skill

```text
Usage: biomcp help skill [OPTIONS] [COMMAND]

Commands:
  list     
  render   
  status   
  install  

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help skill list

```text
Usage: biomcp help skill list [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help skill render

```text
Usage: biomcp help skill render [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help skill status

```text
Usage: biomcp help skill status [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help skill install

```text
Usage: biomcp help skill install [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help version

```text
Usage: biomcp help version [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help mcp-config

```text
Usage: biomcp help mcp-config [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help health

```text
Usage: biomcp help health [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help store

```text
Usage: biomcp help store [OPTIONS] [COMMAND]

Commands:
  clean  

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help store clean

```text
Usage: biomcp help store clean [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help key

```text
Usage: biomcp help key [OPTIONS] [source] [COMMAND]

Commands:
  list    
  set     
  remove  

Arguments:
  [source]
          [possible values: ncbi, semantic-scholar, orcid, oncokb, alphagenome, nci, openfda]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help key list

```text
Usage: biomcp help key list [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help key set

```text
Usage: biomcp help key set [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help key remove

```text
Usage: biomcp help key remove [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help serve

```text
Usage: biomcp help serve [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help network

```text
Usage: biomcp help network [OPTIONS] [setting]

Arguments:
  [setting]
          [possible values: ca_bundle, proxy]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help sources

```text
Usage: biomcp help sources [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp help config

```text
Usage: biomcp help config [OPTIONS]

Options:
      --format <format>
          [possible values: json, md]

      --disable-sources <disable-sources>
          

      --config <config>
          

      --replay <replay>
          

      --record <record>
          

      --case-cache <case-cache>
          

      --observe <observe>
          

      --offline
          

      --fresh

```

## biomcp sync [hgnc|clingen]

Synchronize a local source bundle.


## biomcp schema <artifact id>

Print a checked-in schema artifact.


## biomcp help [COMMAND...]

Print generated command help.


## biomcp key set <source> [--replace]

Register one credential from a no-echo prompt or one standard-input line. Use --config PATH to choose the configuration file.


## biomcp key remove <source>

Remove a stored credential while preserving other settings.


## biomcp sync wikipathways

Replace WikiPathways search/get metadata catalogs atomically; reads never sync. Membership names a source and release; it implies no therapy, response or actionability. No enrichment is calculated.


## biomcp sync cbioportal-studies

Replace the complete validated local bundle atomically. Reads use stored data and never synchronize. Failed replacement preserves the last usable bundle.


## biomcp sync ema

Replace the complete validated local bundle atomically. Reads use stored data and never synchronize. Failed replacement preserves the last usable bundle.


## biomcp sync ddinter

Replace the complete validated local bundle atomically. Reads use stored data and never synchronize. Failed replacement preserves the last usable bundle.


## biomcp sync who-pq

Replace the complete validated local bundle atomically. Reads use stored data and never synchronize. Failed replacement preserves the last usable bundle.


## biomcp sync gtr

Replace the complete validated local bundle atomically. Reads use stored data and never synchronize. Failed replacement preserves the last usable bundle.


## biomcp sync who-ivd

Replace the complete validated local bundle atomically. Reads use stored data and never synchronize. Failed replacement preserves the last usable bundle.


## biomcp sync fda-cdx

Replace the complete validated local bundle atomically. Reads use stored data and never synchronize. Failed replacement preserves the last usable bundle.


## biomcp help key ncbi

Show credential registration and environment fallback names.


## biomcp help key semantic-scholar

Show credential registration and environment fallback names.


## biomcp help key orcid

Show credential registration and environment fallback names.


## biomcp help key oncokb

Show credential registration and environment fallback names.


## biomcp help key alphagenome

Show credential registration and environment fallback names.


## biomcp help key nci

Show credential registration and environment fallback names.


## biomcp help key openfda

Show credential registration and environment fallback names.


