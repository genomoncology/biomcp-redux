//! Fixed refusal and migration messages shared by the runtime and schema.
use super::literals::singleton;

singleton! {
    /// Fixed input-free error and migration guidance.
    pub ErrorMessage { BiomarkerTooBroad => "biomarker search is too broad; narrow --name or get one exact NCIt code",
        OrganizationTooBroad => "organization search is too broad; narrow --name, --city, --state, --country, \
        --postal-code, --family or --lat/--lon/--distance",
 VariantNormalizeForms => "normalization accepts reference-qualified HGVS, or chromosome genomic HGVS with CAR \
 and --build; convert rsIDs, clinvar:N, VCV accessions and genomic coordinates with get variant <ID> first",
 VariantNormalizeRoute => "normalization needs reference-qualified HGVS; for gene shorthand use search variant \
 --gene <SYMBOL> --coding <CHANGE> or --protein <CHANGE>, then get variant <ID>",
 GwasPositiveThreshold => "--p-value threshold must be a positive probability in scientific notation",
 // No longer sent: WhoSearchFilters replaced it, and the 1.0 schema candidate keeps the literal.
 WhoSearchArguments => "WHO PQ search requires a name or --prequalified-from/--prequalified-to date range",
 WhoSearchFilters => "WHO PQ search requires a name, --therapeutic-area or a --prequalified-from/--prequalified-to date range",
 WhoIvdYears => "WHO IVD publishes only the prequalification year; give --prequalified-from/--prequalified-to as YYYY",
 SourceSelector => "choose one of the source selector values", UnsupportedFilter => "a selected source does not support the requested filters",
 DrugFdaRegulatorySection => "Drugs@FDA does not have a regulatory section. Use approvals for the application's FDA submission history.",
 DrugBiosimilarsSection => "biosimilars is not a drug get section. Search the FDA Purple Book for the reference drug's biosimilar roster.",
 FieldsLabelText => "--fields selects fragment row fields from valid_values. Find the text at result.label.data.text in each label reply.",
 TooManyPmids => "citation ordering exceeds the PMID limit; narrow the search",
 McpResultTooBroad => "MCP tool result exceeds the character cap in error.count; narrow \
 --limit or --section-limit, select one source or section, or fetch one exact record",
 SchemaUsage => "use biomcp schema to list known artifact IDs", Arguments => "expected a supported command with checked arguments",
 // No longer sent since 1064, which names the cause; the 1.0 schema keeps the literal.
 ArgumentGuidance => "option is missing or invalid; choose an accepted value or form and use the suggested help command",
 ArgumentMissing => "required option missing: add error.option with a value valid_values accepts",
 ArgumentNoValue => "option given without a value: give error.option a value valid_values accepts",
 ArgumentUnknown => "unknown option: this command has no error.option; run next, or use an option valid_values lists",
 GeneSourceSection => "get gene rejects --source. Use clingen for ClinGen, gencc for GenCC, or constraint for gnomAD.",
 PathwaySourceImplicit => "get pathway rejects --source. The Reactome stable ID selects Reactome.",
 ArgumentValue => "value not accepted: give error.option a value valid_values accepts",
 ArgumentTextUnplaced => "this command takes no text outside an option; next gives the words to error.option",
 ArgumentRepeated => "option repeated: give error.option once, in the form valid_values shows",
 ArgumentCombination => "error.option is not accepted with the other options as given; valid_values names every option involved",
 ArgumentSectionUnknown => "unknown section: error.option is not a section of this record; valid_values lists them",
 ArgumentSectionNotId => "unknown section: error.option was read as a section, not an ID; valid_values lists the sections; \
 to read it as an ID, give it first or use --section",
 ArgumentSectionNotIdTyped => "unknown section: error.option was read as a section, not an ID; valid_values lists the sections; \
 to read it as an ID, give it first or set the section field",
 ArgumentIdAfterSection => "error.option follows a section word and is not a section; put IDs before sections",
 FieldsUnknown => "--fields takes row fields from valid_values, or a dotted path inside one; next puts each name it can place in the field \
 that holds it; error.unplaced lists the names it cannot place",
 FieldsUnsaid => "--fields takes row fields from valid_values, or a dotted path inside one; next puts each name it can place in the field \
 that holds it; error.unplaced lists the names it cannot place, but one that is not letters, digits, _ and . is not repeated",
 FieldsRowsOnly => "--fields does not narrow this call yet; it narrows search rows and some sections' rows; drop --fields to read it whole",
 SavedNameUnknown => "no saved result has this name; --pool and --snapshot take only a name from a reply's next \
 step; run the suggested command",
 OffsetWithoutSaved => "a nonzero offset needs the --pool or --snapshot name from a reply's next step; run the \
 suggested first-page command, then follow its next command",
 ArticleGetLimit => "get article takes 1 to 20 IDs per call; split a longer list into several calls",
 LabelHistoryOffset => "--offset needs --every-version: label-history pages only while it reads every version; add --every-version, or drop \
 --offset (also spelled --section-offset)",
 LabelHistorySectionUnknown => "label-history reads only the label sections in valid_values; give --label-section one of them, or read another \
 section with: get drug <NAME> label --label-section NAME",
 LabelHistoryOffsetIncomplete => "--offset needs --every-version, and label-history also needs --find TEXT and --label-section with a section \
 from valid_values; add what is missing, or drop --offset (also spelled --section-offset)",
 LabelHistoryOffsetSection => "--offset needs --every-version, and label-history reads only the label sections in valid_values; add \
 --every-version and give --label-section one of them, or read another section with: get drug <NAME> label --label-section NAME",
 LabelSectionMissing => "add --label-section NAME with NAME from valid_values; example: biomcp get drug imatinib label \
 --label-section indications_and_usage",
 McpSync => "sync is unavailable over MCP; run it from the command line",
 McpStoreClean => "store clean is unavailable over MCP; run it from the command line", McpKey => "key is unavailable over MCP; run it from the command line",
 McpHosted => "this command is unavailable in hosted mode", HostedHttp => "hosted mode runs only over HTTP behind the hosting front end; add --http",
 SavedRecordUnknown => "no saved search or list has this name; biomcp saved show lists the names",
 SavedNameTaken => "a saved search or list already has this name; choose another name or delete it with biomcp saved \
 delete NAME",
 SavedNewer => "saved.sqlite was written by a newer BioMCP; upgrade BioMCP to read it; the file is unchanged", SavedNoteUnknown => "no saved \
 note has this ID; biomcp saved show lists the notes",
 SavedAccount => "saved work in hosted mode needs a signed-in account; sign in to save searches, lists and notes",
 HostedKey => "add the source key in your account settings", UnknownCommand => "unknown command; use the suggested help command",
 Local => "local configuration, cache or recording failed",

        GwasSearchArguments => "use: search gwas with --gene, --trait, --trait-id, --variant, --study or --pubmed; \
            --include-child-traits requires --trait-id; --extended-geneset requires --gene; use exact GCST \
            accessions, rsIDs, positive PMIDs and native trait IDs without repair; --limit is 1..50 and \
            --offset is a nonnegative integer; source p-value order is fixed with no --sort or local prefix \
            intersection",
        GwasGetArguments => "use: get gwas association:<positive ID> [loci] or get gwas <exact GCST accession> \
            [associations|ancestries]; bare numbers are unsupported; sections must match the resource kind; \
            --section-limit is 1..50 and --section-offset is a nonnegative integer; identities are never \
            repaired",
        TrialSectionPool => "nonzero --offset requires --pool from the returned continuation; run the suggested \
            first-page command, then follow its next command",
        TrialSearchCursor => "nonzero --offset requires --next-page from the returned continuation; run the suggested \
            first-page command, then follow its next command",
        TrialContinuationRejected => "ClinicalTrials.gov rejected this continuation request; run the suggested fresh \
            first-page search, then follow its next command",
        TrialContinuationUnavailable => "this trial continuation cannot verify its position and search selectors; run the \
            suggested fresh first-page search, then follow its next command",
        FaersOffset => "FAERS supports --offset up to 25000; narrow the filters to retrieve more reports",
        FaersDrugNames => "pass --drug once per literal name to search several names with OR; OR inside one --drug value \
            is not supported",
        FaersQueryLength => "FAERS_QUERY_MAX_BYTES is 4096 bytes for the escaped native query including filters; use fewer or shorter --drug names",
        AdverseReceivedCounts => "--aggregate received-year or received-month needs --received-from and --received-to; \
            example: --aggregate received-year --received-from 2021 --received-to 2025",
        AdverseText => "use: search adverse-event <TEXT>; text stays literal",
        AdverseOutcome => "--outcome is retired; use --serious death or --serious hospitalization for report flags and \
            reactions for native outcomes; the old hospitalization and disability mappings were wrong",
        AdverseFrom => "renamed; use: --received-from", AdverseTo => "renamed; use: --received-to",
        // Preserve the published error/1 literal; active guidance uses AdverseSeriousChoices.
        AdverseSeriousChoices => "--serious accepts any, death, hospitalization, lifethreatening, disabling, congenital-anomaly or other",
        AdverseFilter => "this adverse-event filter is not supported", AdverseFaers => "renamed; use: --source faers",
        AdverseMaude => "use: search adverse-event <TEXT> --source maude", AdverseRecalls => "recalls are outside this entity",
        AdverseDeviceRecalls => "use: search diagnostic --source fda-device-recall; covers every medical device",
        AdverseSource => "choose --source faers or --source maude",
        VaersVaccineChoices => "supported vaccines are MMR and COVID19, or one exact native influenza code; use search \
            adverse-event influenza --source vaers to list influenza codes and labels",
        VaersInfluenzaCodes => "Choose one exact native influenza vaccine code; tables remain separate and are never summed. One \
            report may list several vaccines.",
        VaersFilter => "VAERS does not support FAERS filters; use search adverse-event TEXT --source faers for drug reports",
        VaersGet => "VAERS has no individual report get; use search adverse-event MMR --source vaers",
        AdverseVersion => "multi-version get is unproven", AdverseOutcomes => "use the card seriousness flags and the reactions section",
        AdverseConcomitant => "renamed; use: drugs", AdverseSection => "choose a named adverse-event section",
        DiagnosticSearchSources => "choose --source gtr, who-ivd or fda-cdx for search; choose --source gtr, who-ivd, fda-pma or fda-510k for exact get",
        DiagnosticRecallSources => "choose --source gtr, who-ivd, fda-cdx or fda-device-recall for search; \
            choose --source gtr, who-ivd, fda-pma, fda-510k or fda-device-recall for exact get",
        DiagnosticSources => "choose --source gtr, fda-cdx, fda-pma, fda-510k, who-ivd or fda-device-recall for search; \
            choose --source gtr, who-ivd, fda-pma, fda-510k or fda-device-recall for exact get", DiagnosticType => "use: --type",
        DiagnosticCondition => "use: --condition-id MedGen:CUI with --source gtr; use an exact product card with --source who-ivd",
        DiagnosticManufacturer => "manufacturer and laboratory selectors are deferred", DiagnosticSection => "name one diagnostic section per request",
        DiagnosticRegulatory => "use: get diagnostic ID --source fda-pma; search diagnostic --source fda-cdx --device-id ID",
        DiagnosticWhoMarker => "use: search diagnostic --source who-ivd --marker TEXT",
        DiagnosticWhoType => "WHO type search is deferred; use: search diagnostic --source who-ivd --marker TEXT",
        DiagnosticPma => "free-text PMA search is deferred; use: get diagnostic P-NUMBER --source fda-pma",
        Diagnostic510k => "free-text 510(k) search is deferred; use: get diagnostic K-NUMBER --source fda-510k",
        DiagnosticGtrSync => "use: biomcp sync gtr", DiagnosticWhoSync => "use: biomcp sync who-ivd", DiagnosticList => "use: biomcp help diagnostic",
        PgxGene => "use: search pgx --gene <SYMBOL> --source cpic",
        PgxGet => "use: get pgx --gene <SYMBOL> [recommendations|guidelines|diplotype|annotations] --source <SOURCE>",
        PgxText => "drug-text and guideline-text search are dropped; use search pgx --gene <SYMBOL> --source cpic",
        PgxSelector => "this PGx selector is not yet supported; --drug-id is supported only on recommendations",
        PgxInteractions => "use: search pgx --gene <SYMBOL> --source cpic; get pgx --gene <SYMBOL> --source cpic",
        PgxFrequencies => "frequency pages are deferred because no harvested question needs them", PgxSection => "name one PGx section per request",
        PgxSectionLimit => "use: --section-limit", PgxSectionOffset => "use: --section-offset",
        PgxSource => "choose --source cpic for search, the gene card, recommendations, guidelines or diplotype; annotations requires --source clinpgx",
        PgxPhenotype => "--phenotype filters one drug's CPIC recommendation table; add --drug-id, such as --drug-id RxNorm:32968",
        PgxDrugId => "--drug-id takes CPIC's drug key, not a drug name, such as --drug-id RxNorm:32968 for clopidogrel; \
            search pgx --gene <SYMBOL> --source cpic lists each drug's drugid",
        PgxPhase => "unphased variants cannot select a diplotype; --diplotype requires an unchanged declared expression",
        Pharmgkb => "pharmgkb is retired; use: clinpgx",
        NciDisease => "NCI disease filters require an NCIt identifier such as C3224 (melanoma); use --keyword for free text",
        VariantIdentifier => "get variant needs an rsID, CAid, ClinVar Variation ID or reference-qualified HGVS; chromosome \
            notation needs --build; use search variant --gene BRAF --protein V600E or \
            search variant --gene PTEN --coding c.333G>T for gene shorthand",
        VariantBuild => "renamed; use: --build", VariantProtein => "renamed; use: --protein with --gene",
        VariantPredict => "renamed; use: get variant <ID> predict", VariantPopulation => "use: variant population <ID> --detail",
        VariantStructure => "deferred; use: get gene <SYMBOL> protein",
        VariantOncokb => "use: get variant <ID> oncokb --build <GRCh37|GRCh38> --tumor-type <TEXT>; requires your oncokb key",
        VariantErepo => "use: variant assertions <CAid> --source erepo; select --assertion <PCERid> --version <VERSION> for details; no gene-wide fallback",
        VariantNormalize => "use: variant normalize --source <car|mutalyzer|variantvalidator|all> --hgvs <TEXT>; VariantValidator and all require --build",
        VariantRecurrence => "use: variant recurrence <ID> --source cbioportal --gene <SYMBOL> --study <ID> --profile <ID> \
            --sample-set <ID>; no default cohort",
        VariantCgi => "deferred; no proved current CGI section", VariantCspec => "deferred until a harvested question requires variant cspec",
        VariantRawQuery => "--query takes MyVariant query syntax and needs --source myvariant; for a gene and change use \
            --gene <SYMBOL> --coding <CHANGE> or --protein <CHANGE>",
        VariantQueryPolicy => "raw --query cannot enforce disabled source component terms; use declared protein or region \
            selectors and named filters, or clear the component restriction",
        TrialFacilityRadius => "the registry cannot bind --facility and radius to one site; choose one filter",
        DeviceRecallClassificationProductCode => "--classification selects device enforcement records, which carry no product code; \
            drop --product-code, or drop --classification and read each returned row's classification",
        // The three *Ascii messages are no longer sent (0841); removing them narrows the checked-in
        // error schema, which needs a reviewed transition first.
        DeviceRecallNameAscii => "openFDA refuses letters outside ASCII in a device name; spell the name in ASCII letters, \
            such as e for é",
        DiagnosticIntendedUsePma => "--intended-use matches the FDA PMA approval-order statement and needs --source fda-pma; \
            openFDA publishes no intended-use text for 510(k) records, so select them with --product-code, \
            whose FDA classification name describes the device type",
        DiagnosticIntendedUseAscii => "--intended-use needs ASCII text with a letter or digit, such as e for é, because openFDA \
            refuses any non-ASCII character in a phrase",
        DiagnosticClassificationName510k => "--classification-name matches FDA's device classification name on 510(k) \
            and De Novo records and needs --source fda-510k",
        DiagnosticAggregate510k => "--aggregate product-code counts 510(k) records by FDA product code and needs --source fda-510k",
        DiagnosticGeneOrGeneId => "--gene and --gene-id each name one gene, by symbol or by NCBI GeneID; give one of them, not both",
        DiagnosticClassificationNameAscii => "--classification-name needs ASCII text with a letter or digit, such as e for é, \
            because openFDA refuses any non-ASCII character in a phrase",
        TrialCriteria => "retired; use: --criteria for literal registry eligibility text",
        TrialVariant => "retired; use: --variant for explicit discovery text", TrialSince => "renamed; use: --updated-since",
        TrialUntil => "renamed; use: --updated-until", TrialResults => "renamed; use: --has-results",
        TrialPage => "renamed; use: --next-page with the opaque registry cursor",
        TrialNoExpansion => "retired; registry discovery never invents chemical aliases",
        TrialBatch => "use: get trial <NCT>... [sections]; --mode and --pdf are article-only",
        VariantTrials => "use: search trial --variant <text> for literal discovery; no eligibility judgment",
        DiseaseText => "use: search disease <TEXT>; source text stays literal", DiseaseInheritance => "dropped; no clinical inheritance phrase expansion",
        DiseasePhenotypeFilter => "deferred; no proved native HPO-ID disease filter", DiseaseOnset => "dropped; clinical-course fields are not an onset filter",
        DiseaseNoFallback => "retired; disease search never falls back or rewrites a zero result",
        DiseaseLimit => "use: --limit", OffsetPastText => "--offset is past the end of this text; valid_values gives its offsets in characters",
        DiseaseJson => "use: --format json", DiseaseFresh => "use: --fresh to bypass cached replies",
        DiseaseOntologySelector => "use: search disease <TEXT> --source ols --ontology <mondo|doid|ncit>; mesh is not supported",
        DiseasePathways => "dropped; no disease-to-pathway gene fanout", DiseaseDiagnostics => "dropped; no disease diagnostic catalog fanout",
        DiseaseVariants => "dropped; use exact variant commands without disease fanout",
        DiseaseModels => "dropped; model-organism evidence is outside this disease slice",
        DiseasePrevalence => "dropped; prevalence is not SEER incidence or survival",
        DiseasePrevalenceSource => "prevalence comes only from --source orphadata for a MONDO, DOID or Orphanet ID; \
            use: get disease <CURIE> prevalence --source orphadata",
        DiseaseFunding => "dropped; grant amounts do not answer the disease care questions",
        DiseaseCivic => "dropped; use: get variant <ID> civic; no disease-name join",
        DiseaseDisgenet => "dropped; no indispensable disease question establishes this source",
        DiseaseAll => "use: get disease <CURIE> <SECTION> --source <SOURCE>; no enrichment fanout",
        DiseaseClinicalFeatures => "renamed; use: get disease <CURIE> phenotypes --source monarch",
        DiseaseName => "use: search disease <TEXT>; select an exact CURIE before get",
        DiseaseSite => "use: search disease --source seer; then get disease <SITE> survival --source seer --dataset <CODE> --interval <CODE>",
        DiseaseStatistic => "incidence and survival come only from --source seer for a SEER cancer site, not an ontology ID; \
            use: search disease --source seer, then get disease <SITE> incidence --source seer",
        DiseaseOpenTargets => "deferred; Open Targets disease genes have no proved current section",
        // Retain the old error/4 wire value for legacy artifact deserialization.
        DiseaseDrugs => "use: search drug --source chembl --indication <TEXT>; no disease synonym expansion",
        DiseaseLabelSearch => "use: search drug --source openfda-label --indication <TEXT>; no disease synonym expansion",
        PhenotypeSet => "retired; use: search phenotype TEXT; inspect diseases and genes with: get phenotype HP:ID --source monarch",
        PhenotypeHelp => "use: help phenotype", TrialOffsetDeep => "ClinicalTrials.gov offsets reach row 5000; follow --next-page from next to go deeper",
        PhenotypeHpoSearch => "HPO term search has no proved paging contract; use: search phenotype TEXT --source monarch",
        PhenotypeSetOptions => "--group, --metric and --search-mode rank genes or diseases for HP IDs only; give the text \
            as HP IDs separated by commas, such as: search phenotype HP:0001263,HP:0001250 --group genes",
        PhenotypeSetOffset => "Monarch returns one ranked list of at most 50 rows for a set of HP IDs and has no further \
            pages; drop --offset and raise --limit up to 50",
        PhenotypeSetEmpty => "Monarch ranked nothing for this set, which it does when it knows none of the IDs; check \
            each ID with the next steps",
        PhenotypeSetUnknown => "Monarch has no term for an HP ID in this set and would rank without it; check each ID with \
            the next steps, or rank only the IDs Monarch knows",
        DiseaseHelp => "use: help search disease; help get disease", DiseaseTrials => "use: search trial --condition <text>; NCI requires an NCIt identifier",
        CellLineAllPaging => "all returns default first pages; use get cell-line <ID> <section> with --section-limit and --section-offset to page one section",
        CellLineDrugResponse => "PharmacoDB drug response is deferred until it publishes a data reuse grant",
        CellLineExpression => "gene cell-lines is deferred until an exact HPA cell identity join exists",
        DrugCellLines => "drug cell-lines is retired: cell-line screening is outside the cancer-care drug slice.",
        CvxUnavailable => "Vaccine identity lookup and sync are unavailable; search vaccine report summaries with `search adverse-event --source vaers`.",

        EmaSync => "renamed; use: biomcp sync ema", DdinterSync => "renamed; use: biomcp sync ddinter",

        WhoPqSync => "renamed; use: biomcp sync who-pq", DrugQuery => "renamed; use: search drug <TEXT> --source <SOURCE>",
        DrugMechanism => "--mechanism is retired: semantic mechanism expansion is not supported; select a drug identity and read targets.",
        DrugPartnerSearch => "--interactions is retired: use get drug <ID> interactions --source ddinter.",
        DrugRegion => "--region is retired: select an explicit --source.", DrugWhoRegion => "--region who is retired: use --source who-pq.",
        DrugProductType => "--product-type is retired: WHO prequalification is outside this question bank.",
        // Retain the old error/4 wire value for legacy artifact deserialization.
        DrugUnproved => "drug search filters --target, --indication, --type, --atc and --pharm-class are not yet supported; \
            select a drug identity to read source sections.",
        DrugUnsupportedFilters => "drug search filters --target, --type, --atc and --pharm-class are not yet supported; \
            select a drug identity to read source sections.",
        DrugName => "--name is retired: use search drug <TEXT> --source <SOURCE> and select a native identity.",
        DrugRaw => "--raw is retired: use get drug <SPL-SET-ID> label --source openfda-label --label-section <NATIVE-HEADING>.",
        DrugSafety => "safety is retired: use get drug <SPL-SET-ID> label --source openfda-label --label-section <NATIVE-HEADING>.",
        DrugShortage => "shortage is not yet supported in the cancer-care drug slice.", DrugAll => "all is not supported: select one named source section.",
        DrugMarketsSource => "markets takes a drug name and at most --source; drop error.option, as in: get drug <NAME> markets",
        DrugMarketsSourceUnknown => "markets reads one source per market: drugsfda (US), ema (EU) and who-pq (WHO prequalification); \
            give --source one of valid_values, or drop it to read every market",
        DrugInteractions => "renamed; use: get drug <ID> interactions --source ddinter --section-limit <N> --section-offset <N>",
        DrugTrials => "use: search trial --intervention <text>; no inferred aliases",
        TrialDocument => "use: trial document <NCT> <FILE> --save PATH; first list get trial <NCT> documents",
        Gene => "use: get gene <gene>", GeneSearch => "use: search gene <query>", Pathways => "use: get gene <gene> pathways",
        Trials => "use: search trial --gene <gene>", Articles => "use: search article --gene <gene>", Drugs => "use: search drug --target <gene>",
        Variant => "moved to the variant entity", LocusGroup => "use: --locus-group", AuthorHelp => "use: help author; help search author; help get author",
        AuthorDeferred => "topic ranking, coauthors and cross-provider merging are not supported; use: search author TEXT or search article",
        AuthorOrcidSearch => "ORCID name search is not supported; use: search author TEXT or search article", ArticleText => "use: search article <text>",
        ScoreBlends => "no score blends in 1.0; each source keeps its own order",
        ArticlePdf => "use: get article <id> pdf", Since => "use: --since", Until => "use: --until",
        CitationSort => "use: --sort citations", GeneOption => "use: --gene", DiseaseOption => "use: --disease", AuthorOption => "use: --author",
        Supplements => "use: supplements", Supplement => "use: article supplement <id> <file>",
        Fulltext => "use: fulltext", ArticlePart => "use: article text <id> --part <n>", SupplementFind => "use: article supplement <id> <file> --find <text>",
        Annotations => "use: get article <id> annotations", ArticleBatch => "use: get article <id>...",
        VariantArticles => "use: search article --variant <variant>", DiseaseArticles => "use: search article --disease <disease>",
        ProteinQuery => "use: search protein <TEXT>; the positional query stays unchanged",
        ProteinSpecies => "--all-species is dropped; protein access stays within human-health scope",
        ProteinSelector => "--disease and --existence are not yet supported because no harvested question needs these selectors",
        ProteinPage => "use: --pool and --offset from the returned continuation",
        ProteinUnreviewed => "--include-unreviewed is not yet supported because no unreviewed positive was captured",
        ProteinString => "STRING is deferred because its mapping returns one STRING ID for the two distinct CDKN2A protein \
            products; use: get protein <ACCESSION> complexes", // Unsent since 1029; kept for the 1.0 schema.
        StudyList => "use: search study", StudyDownload => "use: sync cbioportal-studies for metadata; get study ID files for source file references; \
            whole-study analytical installs are retired",
        StudyMutationProfile => "the mutations section reads only a mutation profile of this study; choose \
            --molecular-profile from valid_values",
        StudyCalculate => "local calculations are dropped by the entity boundary ruling; use: get study ID and the source study page",
        StudyCharts => "charts are retired", StudyDirectory => "BIOMCP_STUDY_DIR is retired; use: sync cbioportal-studies for metadata",
        PathwayText => "use: search pathway <text>", PathwayClass => "--type is retired: the accepted pathway class is enforced and needs no filter.",
        PathwayKegg => "KEGG pathways are dropped; use a Reactome R-HSA-… or WikiPathways WP… ID.",
        PathwayEnrich => "Gene-set enrichment is dropped; an agent can call g:Profiler itself.",
        PathwayDrugs => "use: get pathway ID genes; reverse gene-to-drug search is not supported; select a drug identity \
            and request its explicit source sections.",
        PathwayArticles => "use: search article <text>; a matching title does not establish pathway article linkage.",
        PathwayTrials => "use: search trial --gene <gene>; pathway-name trial search is retired.", PathwayHelp => "use: help search pathway; help get pathway",
        DrugArticles => "use: search article --drug <drug>", Pathway => "moved to the pathway entity",
        Band => "use: --band", Dropped => "dropped in 1.0", Expression => "use: expression", Drug => "moved to the drug entity",
 Diagnostic => "moved to the diagnostic entity", PoolExpired => "pool expired; restart the search",
 PoolMismatch => "pool selectors do not match this search"
    }
}
