//! The fixed schema usage response, including its checked wire literals.
use serde::{Deserialize, Serialize};

// One literal defines both serde and schema, including the fixed message.
macro_rules! singleton {
    ($(#[$doc:meta])* $visibility:vis $name:ident { $( $variant:ident => $literal:literal ),+ }) => {
        $(#[$doc])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
        #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
        #[cfg_attr(feature = "schema", schemars(with = "str", extend("enum" = [$($literal),+])))]
        $visibility enum $name {
            $(#[doc = $literal]
            #[serde(rename = $literal)]
            $variant,)+
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                match String::deserialize(deserializer)?.as_str() {
                    $( $literal => Ok(Self::$variant), )+
                    value => Err(serde::de::Error::unknown_variant(value, &[$($literal),+])),
                }
            }
        }
    };
}

singleton! {
    /// The status of an invalid schema request.
    pub ErrorStatus { Usage => "usage", Local => "local", Expired => "expired" }
}

singleton! {
    /// Stable error codes for command usage, local failures and expired results.
    pub ErrorCode { SchemaUsage => "usage.schema",
 Arguments => "usage.arguments",
 McpUnavailable => "usage.mcp_unavailable",
 UnknownCommand => "usage.unknown_command",
 Retired => "usage.retired",
 Local => "local.failure",
 PoolExpired => "pool.expired",
 PoolMismatch => "usage.pool_mismatch",
 TooManyPmids => "usage.too_many_pmids",
 TooBroad => "usage.too_broad",
 UnsupportedFilter => "usage.unsupported_filter" }
}

singleton! {
    ErrorSchema { V1 => "biomcp.error/1" }
}

singleton! {
    /// Fixed input-free error and migration guidance.
    pub ErrorMessage { OrganizationTooBroad => "organization search is too broad; narrow --name, --city, --state, --country, \
        --postal-code, --family or --lat/--lon/--distance",
 GwasThreshold => "--p-value is not yet supported; threshold filtering is deferred",
 GwasPositiveThreshold => "--p-value threshold must be a positive probability in scientific notation",
 SourceSelector => "choose one of the source selector values",
 UnsupportedFilter => "a selected source does not support the requested filters",
 TooManyPmids => "citation ordering exceeds the PMID limit; narrow the search",
 SchemaUsage => "use biomcp schema to list known artifact IDs",
 Arguments => "expected a supported command with checked arguments",
 McpSync => "sync is unavailable over MCP; run it from the command line",
 McpStoreClean => "store clean is unavailable over MCP; run it from the command line",
 McpKey => "key is unavailable over MCP; run it from the command line",
 UnknownCommand => "unknown command; use the suggested help command",
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
        AdverseText => "use: search adverse-event <TEXT>; text stays literal",
        AdverseOutcome => "--outcome is retired; use --serious death or --serious hospitalization for report flags and \
            reactions for native outcomes; the old hospitalization and disability mappings were wrong",
        AdverseFrom => "renamed; use: --received-from",
        AdverseTo => "renamed; use: --received-to",
        // Preserve the published error/1 literal; active guidance uses AdverseSeriousChoices.
        AdverseSerious => "only --serious death and --serious hospitalization are supported",
        AdverseSeriousChoices => "--serious accepts any, death, hospitalization, lifethreatening, disabling, congenital-anomaly or other",
        AdverseFilter => "this adverse-event filter is not supported",
        AdverseFaers => "renamed; use: --source faers",
        AdverseMaude => "use: search adverse-event <TEXT> --source maude",
        AdverseRecalls => "recalls are outside this entity",
        AdverseSource => "choose --source faers or --source maude",
        VaersVaccine => "supported vaccines are MMR and COVID-19 vaccine; use MMR or COVID19",
        VaersInfluenza => "Influenza VAERS summaries are not available yet; supported vaccines are MMR and COVID-19.",
        VaersVaccineChoices => "supported vaccines are MMR and COVID19, or one exact native influenza code; use search \
            adverse-event influenza --source vaers to list influenza codes and labels",
        VaersInfluenzaCodes => "Choose one exact native influenza vaccine code; tables remain separate and are never summed. One \
            report may list several vaccines.",
        VaersFilter => "VAERS does not support FAERS filters; use search adverse-event TEXT --source faers for drug reports",
        VaersGet => "VAERS has no individual report get; use search adverse-event MMR --source vaers",
        AdverseVersion => "multi-version get is unproven",
        AdverseOutcomes => "use the card seriousness flags and the reactions section",
        AdverseConcomitant => "renamed; use: drugs",
        AdverseSection => "choose a named adverse-event section",
        DiagnosticSource => "choose --source gtr or fda-cdx for search; choose --source gtr, who-ivd, fda-pma or fda-510k for exact get",
        DiagnosticWho => "WHO discovery, type and marker search are deferred; use: get diagnostic PRODUCT-ID --source who-ivd",
        DiagnosticSearchSources => "choose --source gtr, who-ivd or fda-cdx for search; choose --source gtr, who-ivd, fda-pma or fda-510k for exact get",
        DiagnosticType => "use: --type",
        DiagnosticCondition => "use: --condition-id MedGen:CUI with --source gtr; use an exact product card with --source who-ivd",
        DiagnosticManufacturer => "manufacturer and laboratory selectors are deferred",
        DiagnosticSection => "name one diagnostic section per request",
        DiagnosticRegulatory => "use: get diagnostic ID --source fda-pma; search diagnostic --source fda-cdx --device-id ID",
        DiagnosticWhoMarker => "use: search diagnostic --source who-ivd --marker TEXT",
        DiagnosticWhoType => "WHO type search is deferred; use: search diagnostic --source who-ivd --marker TEXT",
        DiagnosticPma => "free-text PMA search is deferred; use: get diagnostic P-NUMBER --source fda-pma",
        Diagnostic510k => "free-text 510(k) search is deferred; use: get diagnostic K-NUMBER --source fda-510k",
        DiagnosticGtrSync => "use: biomcp sync gtr",
        DiagnosticWhoSync => "use: biomcp sync who-ivd",
        DiagnosticList => "use: biomcp help diagnostic",
        PgxGene => "use: search pgx --gene <SYMBOL> --source cpic",
        PgxGet => "use: get pgx --gene <SYMBOL> [recommendations|guidelines|diplotype|annotations] --source <SOURCE>",
        PgxText => "drug-text and guideline-text search are dropped; use search pgx --gene <SYMBOL> --source cpic",
        PgxSelector => "this PGx selector is not yet supported; --drug-id is supported only on recommendations",
        PgxInteractions => "use: search pgx --gene <SYMBOL> --source cpic; get pgx --gene <SYMBOL> --source cpic",
        PgxFrequencies => "frequency pages are deferred because no harvested question needs them",
        PgxSection => "name one PGx section per request",
        PgxSectionLimit => "use: --section-limit",
        PgxSectionOffset => "use: --section-offset",
        PgxSource => "choose --source cpic for search, the gene card, recommendations, guidelines or diplotype; annotations requires --source clinpgx",
        PgxPhase => "unphased variants cannot select a diplotype; --diplotype requires an unchanged declared expression",
        Pharmgkb => "pharmgkb is retired; use: clinpgx",
        NciDisease => "NCI disease filters require an NCIt identifier such as C3224; use --keyword for free text",
        VariantIdentifier => "get variant needs an rsID, CAid, ClinVar Variation ID or reference-qualified HGVS; chromosome \
            notation needs --build; use search variant --gene BRAF --protein V600E for gene/protein shorthand",
        VariantBuild => "renamed; use: --build",
        VariantProtein => "renamed; use: --protein with --gene",
        VariantPredict => "renamed; use: get variant <ID> predict",
        VariantPopulation => "use: variant population <ID> --detail",
        VariantStructure => "deferred; use: get gene <SYMBOL> protein",
        VariantOncokb => "use: get variant <ID> oncokb --build <GRCh37|GRCh38> --tumor-type <TEXT>; requires your oncokb key",
        VariantErepo => "use: variant assertions <CAid> --source erepo; select --assertion <PCERid> --version <VERSION> for details; no gene-wide fallback",
        VariantNormalize => "use: variant normalize --source <car|mutalyzer|variantvalidator|all> --hgvs <TEXT>; VariantValidator and all require --build",
        VariantRecurrence => "use: variant recurrence <ID> --source cbioportal --gene <SYMBOL> --study <ID> --profile <ID> \
            --sample-set <ID>; no default cohort",
        VariantCgi => "deferred; no proved current CGI section",
        VariantCspec => "deferred until a harvested question requires variant cspec",
        VariantQueryPolicy => "raw --query cannot enforce disabled source component terms; use declared protein or region \
            selectors and named filters, or clear the component restriction",
        TrialFacilityRadius => "the registry cannot bind --facility and radius to one site; choose one filter",
        TrialCriteria => "retired; use: --criteria for literal registry eligibility text",
        TrialVariant => "retired; use: --variant for explicit discovery text",
        TrialSince => "renamed; use: --updated-since",
        TrialUntil => "renamed; use: --updated-until",
        TrialResults => "renamed; use: --has-results",
        TrialPage => "renamed; use: --next-page with the opaque registry cursor",
        TrialNoExpansion => "retired; registry discovery never invents chemical aliases",
        TrialBatch => "use: get trial <NCT>... [sections]; --mode and --pdf are article-only",
        VariantTrials => "use: search trial --variant <text> for literal discovery; no eligibility judgment",
        DiseaseText => "use: search disease <TEXT>; source text stays literal",
        DiseaseInheritance => "dropped; no clinical inheritance phrase expansion",
        DiseasePhenotypeFilter => "deferred; no proved native HPO-ID disease filter",
        DiseaseOnset => "dropped; clinical-course fields are not an onset filter",
        DiseaseNoFallback => "retired; disease search never falls back or rewrites a zero result",
        DiseaseLimit => "use: --limit",
        DiseaseJson => "use: --format json",
        DiseaseFresh => "use: --fresh to bypass cached replies",
        DiseaseOntologySelector => "use: search disease <TEXT> --source ols --ontology <mondo|doid|ncit>; mesh is not supported",
        DiseasePathways => "dropped; no disease-to-pathway gene fanout",
        DiseaseDiagnostics => "dropped; no disease diagnostic catalog fanout",
        DiseaseVariants => "dropped; use exact variant commands without disease fanout",
        DiseaseModels => "dropped; model-organism evidence is outside this disease slice",
        DiseasePrevalence => "dropped; prevalence is not SEER incidence or survival",
        DiseaseFunding => "dropped; grant amounts do not answer the disease care questions",
        DiseaseCivic => "dropped; use: get variant <ID> civic; no disease-name join",
        DiseaseDisgenet => "dropped; no indispensable disease question establishes this source",
        DiseaseAll => "use: get disease <CURIE> <SECTION> --source <SOURCE>; no enrichment fanout",
        DiseaseClinicalFeatures => "renamed; use: get disease <CURIE> phenotypes --source monarch",
        DiseaseName => "use: search disease <TEXT>; select an exact CURIE before get",
        DiseaseSite => "use: search disease --source seer; then get disease <SITE> survival --source seer --dataset <CODE> --interval <CODE>",
        DiseaseOpenTargets => "deferred; Open Targets disease genes have no proved current section",
        DiseaseDrugs => "use: search drug --source chembl --indication <TEXT>; no disease synonym expansion",
        PhenotypeSet => "retired; use: search phenotype TEXT; inspect diseases and genes with: get phenotype HP:ID --source monarch",
        PhenotypeHelp => "use: help phenotype",
        PhenotypeHpoSearch => "HPO term search has no proved paging contract; use: search phenotype TEXT --source monarch",
        DiseaseHelp => "use: help search disease; help get disease",
        DiseaseTrials => "use: search trial --condition <text>; NCI requires an NCIt identifier",
        CellLineAllPaging => "all returns default first pages; use get cell-line <ID> <section> with --section-limit and --section-offset to page one section",
        CellLineDrugResponse => "PharmacoDB drug response is deferred until it publishes a data reuse grant",
        CellLineExpression => "gene cell-lines is deferred until an exact HPA cell identity join exists",
        DrugCellLines => "drug cell-lines is retired: cell-line screening is outside the cancer-care drug slice.",
        CvxUnavailable => "Vaccine identity lookup and sync are unavailable; search vaccine report summaries with `search adverse-event --source vaers`.",
        CvxSync => "cvx sync is retired: the cancer-care drug slice has no vaccine alias bridge.",
        EmaSync => "renamed; use: biomcp sync ema",
        DdinterSync => "renamed; use: biomcp sync ddinter",
        WhoSync => "who sync is retired: WHO prequalification is outside this question bank.",
        WhoPqSync => "renamed; use: biomcp sync who-pq",
        DrugQuery => "renamed; use: search drug <TEXT> --source <SOURCE>",
        DrugMechanism => "--mechanism is retired: semantic mechanism expansion is not supported; select a drug identity and read targets.",
        DrugPartnerSearch => "--interactions is retired: use get drug <ID> interactions --source ddinter.",
        DrugRegion => "--region is retired: select an explicit --source.",
        DrugWhoRegion => "--region who is retired: use --source who-pq.",
        DrugProductType => "--product-type is retired: WHO prequalification is outside this question bank.",
        DrugUnproved => "drug search filters --target, --indication, --type, --atc and --pharm-class are not yet supported; \
            select a drug identity to read source sections.",
        DrugName => "--name is retired: use search drug <TEXT> --source <SOURCE> and select a native identity.",
        DrugRaw => "--raw is retired: use get drug <SPL-SET-ID> label --source openfda-label --label-section <NATIVE-HEADING>.",
        DrugSafety => "safety is retired: use get drug <SPL-SET-ID> label --source openfda-label --label-section <NATIVE-HEADING>.",
        DrugShortage => "shortage is not yet supported in the cancer-care drug slice.",
        DrugAll => "all is not supported: select one named source section.",
        DrugInteractions => "renamed; use: get drug <ID> interactions --source ddinter --section-limit <N> --section-offset <N>",
        DrugTrials => "use: search trial --intervention <text>; no inferred aliases",
        TrialDocument => "use: trial document <NCT> <FILE> --save PATH; first list get trial <NCT> documents",
        Gene => "use: get gene <gene>", Pathways => "use: get gene <gene> pathways",
        Trials => "use: search trial --gene <gene>", Articles => "use: search article --gene <gene>",
        Drugs => "use: search drug --target <gene>", CellLines => "moved to the cell line entity",
        Variant => "moved to the variant entity", LocusGroup => "use: --locus-group",
        AuthorHelp => "use: help author; help search author; help get author",
        AuthorDeferred => "topic ranking, coauthors and cross-provider merging are not supported; use: search author TEXT or search article",
        AuthorOrcidSearch => "ORCID name search is not supported; use: search author TEXT or search article",
        ArticleText => "use: search article <text>",
        ScoreBlends => "no score blends in 1.0; each source keeps its own order",
        ArticlePdf => "use: get article <id> pdf", Since => "use: --since", Until => "use: --until",
        CitationSort => "use: --sort citations", GeneOption => "use: --gene",
        DiseaseOption => "use: --disease", AuthorOption => "use: --author",
        Supplements => "use: supplements", Supplement => "use: article supplement <id> <file>",
        Fulltext => "use: fulltext", ArticlePart => "use: article text <id> --part <n>",
        SupplementFind => "use: article supplement <id> <file> --find <text>",
        Annotations => "use: get article <id> annotations", ArticleBatch => "use: get article <id>...",
        VariantArticles => "use: search article --variant <variant>", DiseaseArticles => "use: search article --disease <disease>",
        ProteinQuery => "use: search protein <TEXT>; the positional query stays unchanged",
        ProteinSpecies => "--all-species is dropped; protein access stays within human-health scope",
        ProteinSelector => "--disease and --existence are not yet supported because no harvested question needs these selectors",
        ProteinPage => "use: --pool and --offset from the returned continuation",
        ProteinUnreviewed => "--include-unreviewed is not yet supported because no unreviewed positive was captured",
        ProteinString => "STRING is deferred because its mapping returns one STRING ID for the two distinct CDKN2A protein \
            products; use: get protein <ACCESSION> complexes",
        StudyList => "use: search study",
        StudyDownload => "use: sync cbioportal-studies for metadata; get study ID files for source file references; \
            whole-study analytical installs are retired",
        StudyCalculate => "local calculations are dropped by the entity boundary ruling; use: get study ID and the source study page",
        StudyCharts => "charts are retired",
        StudyDirectory => "BIOMCP_STUDY_DIR is retired; use: sync cbioportal-studies for metadata",
        PathwayText => "use: search pathway <text>",
        PathwayClass => "--type is retired: the accepted pathway class is enforced and needs no filter.",
        PathwayKegg => "KEGG pathways are dropped; use a Reactome R-HSA-… or WikiPathways WP… ID.",
        PathwayEnrich => "Gene-set enrichment is dropped; an agent can call g:Profiler itself.",
        PathwayDrugs => "use: get pathway ID genes; reverse gene-to-drug search is not supported; select a drug identity \
            and request its explicit source sections.",
        PathwayArticles => "use: search article <text>; a matching title does not establish pathway article linkage.",
        PathwayTrials => "use: search trial --gene <gene>; pathway-name trial search is retired.",
        PathwayHelp => "use: help search pathway; help get pathway",
        DrugArticles => "use: search article --drug <drug>", Pathway => "moved to the pathway entity",
        Band => "use: --band", Dropped => "dropped in 1.0", Expression => "use: expression",
        Drug => "moved to the drug entity",
 Diagnostic => "moved to the diagnostic entity",
 Deferred => "deferred",
 PoolExpired => "pool expired; restart the search",
 PoolMismatch => "pool selectors do not match this search"
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
struct ErrorDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(extend("format" = "uri")))]
    url: Option<crate::Text>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    source: Option<crate::Source>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    option: Option<Box<str>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    valid_values: Option<Box<[crate::Text]>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    count: Option<u64>,
    code: ErrorCode,
    message: ErrorMessage,
}

/// A versioned error response that never contains caller input.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ErrorEnvelope {
    schema: ErrorSchema,
    status: ErrorStatus,
    error: ErrorDetail,
    /// Bounded cache failure when local startup cannot open its store.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_error: Option<super::CacheProblem>,
    /// Typed restart when a snapshot cannot answer this query.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub next: Vec<super::Step>,
}

impl ErrorEnvelope {
    /// Refuse an organization collection beyond its bounded observation.
    pub fn organization_too_broad() -> Self {
        let mut error = Self::arguments();
        error.error.code = ErrorCode::TooBroad;
        error.error.message = ErrorMessage::OrganizationTooBroad;
        error.error.source = Some(crate::Source::Nci);
        error
    }
    /// Name a refused command with fixed guidance and no caller arguments.
    pub fn mcp_unavailable(message: ErrorMessage) -> Self {
        let mut error = Self::arguments();
        error.error.code = ErrorCode::McpUnavailable;
        error.error.message = message;
        error
    }
    /// Name a rejected configuration field or environment variable without its value.
    pub fn configuration(option: String) -> Self {
        let mut envelope = Self::local();
        envelope.error.option = Some(option.into_boxed_str());
        envelope
    }
    /// Refuse opaque source syntax when a component terms restriction is selected.
    pub fn variant_query_policy() -> Self {
        let mut error = Self::arguments();
        error.error.message = ErrorMessage::VariantQueryPolicy;
        error
    }
    /// Explain the published variant identifier boundary without interpreting caller text.
    pub fn variant_arguments() -> Self {
        let mut error = Self::arguments();
        error.error.message = ErrorMessage::VariantIdentifier;
        error
    }
    /// Construct the fixed response for every invalid schema request.
    pub fn schema_usage() -> Self {
        Self {
            local_error: None,
            next: Vec::new(),
            schema: ErrorSchema::V1,
            status: ErrorStatus::Usage,
            error: ErrorDetail {
                url: None,
                source: None,
                option: None,
                valid_values: None,
                count: None,
                code: ErrorCode::SchemaUsage,
                message: ErrorMessage::SchemaUsage,
            },
        }
    }
}

impl ErrorEnvelope {
    /// Report fixed migration guidance without accepting caller input.
    pub fn retired(message: ErrorMessage) -> Self {
        let valid_values = (message == ErrorMessage::VaersInfluenzaCodes).then(|| {
            crate::VAERS_INFLUENZA
                .iter()
                .filter_map(|(code, label)| crate::Text::new(&format!("{code}: {label}")).ok())
                .collect()
        });
        Self {
            local_error: None,
            next: Vec::new(),
            schema: ErrorSchema::V1,
            status: ErrorStatus::Usage,
            error: ErrorDetail {
                url: None,
                source: None,
                option: None,
                valid_values,
                count: None,
                code: ErrorCode::Retired,
                message,
            },
        }
    }
    /// Report an unknown command with recovery supplied only from the known grammar.
    pub fn unknown_command() -> Self {
        let mut error = Self::arguments();
        error.error.code = ErrorCode::UnknownCommand;
        error.error.message = ErrorMessage::UnknownCommand;
        error
    }
    /// Refuse invalid command arguments without echoing input.
    pub fn arguments() -> Self {
        Self {
            local_error: None,
            next: Vec::new(),
            schema: ErrorSchema::V1,
            status: ErrorStatus::Usage,
            error: ErrorDetail {
                url: None,
                source: None,
                option: None,
                valid_values: None,
                count: None,
                code: ErrorCode::Arguments,
                message: ErrorMessage::Arguments,
            },
        }
    }
    /// Name a grammar-owned argument without retaining its rejected value.
    pub fn argument(option: Box<str>) -> Self {
        Self::arguments().with_argument(option)
    }
    /// Name a checked argument and its supported vocabulary without echoing input.
    pub fn argument_values(option: Box<str>, values: Box<[crate::Text]>) -> Self {
        let mut error = Self::argument(option);
        error.error.valid_values = Some(values);
        error
    }
    /// Attach a grammar-owned identity while preserving fixed command guidance.
    pub fn with_argument(mut self, option: Box<str>) -> Self {
        self.error.option = Some(option);
        self
    }
    /// Direct composite readers to independently paged named sections.
    pub fn cell_line_all_paging() -> Self {
        let mut error = Self::arguments();
        error.error.message = ErrorMessage::CellLineAllPaging;
        error
    }
    /// Explain a failed checked GWAS command without changing its usage code.
    pub fn gwas_arguments(message: ErrorMessage) -> Self {
        let mut error = Self::arguments();
        error.error.message = message;
        error
    }
    /// Explain the exact positive threshold boundary before source access.
    pub fn gwas_threshold() -> Self {
        let mut error = Self::arguments();
        error.error.message = ErrorMessage::GwasPositiveThreshold;
        error
    }
    /// Explain explicit NCI identity and keyword modes.
    pub fn nci_disease() -> Self {
        let mut error = Self::arguments();
        error.error.message = ErrorMessage::NciDisease;
        error
    }
    /// Explain the registry's facility and radius limitation before any request.
    pub fn trial_facility_radius() -> Self {
        let mut error = Self::arguments();
        error.error.message = ErrorMessage::TrialFacilityRadius;
        error
    }
    /// Report a local failure without paths or provider diagnostics.
    pub fn local() -> Self {
        Self {
            local_error: None,
            next: Vec::new(),
            schema: ErrorSchema::V1,
            status: ErrorStatus::Local,
            error: ErrorDetail {
                url: None,
                source: None,
                option: None,
                valid_values: None,
                count: None,
                code: ErrorCode::Local,
                message: ErrorMessage::Local,
            },
        }
    }
}

impl ErrorEnvelope {
    /// Report an unavailable or mismatched pool with its typed restart.
    pub fn pool(mismatch: bool, next: super::Step) -> Self {
        Self {
            schema: ErrorSchema::V1,
            status: if mismatch {
                ErrorStatus::Usage
            } else {
                ErrorStatus::Expired
            },
            error: ErrorDetail {
                url: None,
                source: None,
                option: None,
                valid_values: None,
                count: None,
                code: if mismatch {
                    ErrorCode::PoolMismatch
                } else {
                    ErrorCode::PoolExpired
                },
                message: if mismatch {
                    ErrorMessage::PoolMismatch
                } else {
                    ErrorMessage::PoolExpired
                },
            },
            local_error: None,
            next: vec![next],
        }
    }
}

impl ErrorEnvelope {
    /// Refuse citation enrichment above its checked PMID window.
    pub fn too_many_pmids(count: u64) -> Self {
        Self {
            schema: ErrorSchema::V1,
            status: ErrorStatus::Usage,
            error: ErrorDetail {
                url: None,
                source: None,
                option: None,
                valid_values: None,
                count: Some(count),
                code: ErrorCode::TooManyPmids,
                message: ErrorMessage::TooManyPmids,
            },
            local_error: None,
            next: Vec::new(),
        }
    }
}

impl ErrorEnvelope {
    /// Refuse a source-native trial capability without borrowing article semantics.
    pub fn unsupported_trial_filter(source: crate::Source, option: crate::TrialFilter) -> Self {
        let mut out = Self::arguments();
        out.error.source = Some(source);
        out.error.option = Some(option.as_str().into());
        out.error.code = ErrorCode::UnsupportedFilter;
        out.error.message = ErrorMessage::UnsupportedFilter;
        out
    }
    /// Refuse a source/filter combination before requesting any source.
    pub fn unsupported_filter(source: crate::Source, option: crate::ArticleFilter) -> Self {
        let mut out = Self::arguments();
        out.error.source = Some(source);
        out.error.option = Some(option.as_str().into());
        out.error.code = ErrorCode::UnsupportedFilter;
        out.error.message = ErrorMessage::UnsupportedFilter;
        out
    }
}

impl ErrorEnvelope {
    /// Refuse a save while retaining the source URL for the caller.
    pub fn save_refused(url: crate::Text) -> Self {
        let mut out = Self::arguments();
        out.error.url = Some(url);
        out
    }
}

impl ErrorEnvelope {
    /// Refuse an absent or unknown native selector without echoing caller input.
    pub fn source_selector(
        source: crate::Source,
        option: Box<str>,
        valid_values: Box<[crate::Text]>,
    ) -> Self {
        let mut out = Self::arguments();
        out.error.source = Some(source);
        out.error.option = Some(option);
        out.error.valid_values = Some(valid_values);
        out.error.message = ErrorMessage::SourceSelector;
        out
    }
}
