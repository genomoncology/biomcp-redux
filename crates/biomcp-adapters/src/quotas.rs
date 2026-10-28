use biomcp_fetch::{Quota, Rate, SourceKey};

// Conservative local BioThings policy; no published numeric provider limit.
pub(crate) static BIOTHINGS: Quota = Quota::new(
    SourceKey::declare("biothings"),
    Rate::per_second(1),
    None,
    1,
);

// Conservative local limit for public published-file downloads.
pub(crate) static HGNC: Quota =
    Quota::new(SourceKey::declare("hgnc"), Rate::per_second(1), None, 1);

// Conservative local policy, not a published provider limit.
pub(crate) static ONCOKB: Quota =
    Quota::new(SourceKey::declare("oncokb"), Rate::per_second(1), None, 1);

// Conservative local gnomAD policy; no published numeric provider limit.
pub(crate) static GNOMAD: Quota =
    Quota::new(SourceKey::declare("gnomad"), Rate::per_second(1), None, 1);

// Conservative local policy; no published numeric provider limit.
pub(crate) static UNIPROT: Quota =
    Quota::new(SourceKey::declare("uniprot"), Rate::per_second(2), None, 2);

// Conservative local policy; no published numeric provider limit.
pub(crate) static REACTOME: Quota =
    Quota::new(SourceKey::declare("reactome"), Rate::per_second(2), None, 2);

// Conservative local policy; no published numeric provider limit.
pub(crate) static CIVIC: Quota =
    Quota::new(SourceKey::declare("civic"), Rate::per_second(2), None, 2);

// Conservative local policy; no published numeric provider limit.
pub(crate) static CLINGEN: Quota =
    Quota::new(SourceKey::declare("clingen"), Rate::per_second(1), None, 1);

// Conservative local policy; no published numeric provider limit.
pub(crate) static GENCC: Quota =
    Quota::new(SourceKey::declare("gencc"), Rate::per_second(1), None, 1);

// Conservative local policy; no published numeric provider limit.
pub(crate) static HPA: Quota = Quota::new(SourceKey::declare("hpa"), Rate::per_second(1), None, 1);

// Conservative public ORCID policy, independent of optional token.
pub(crate) static ORCID: Quota =
    Quota::new(SourceKey::declare("orcid"), Rate::per_second(1), None, 1);

// NCBI E-utilities documented shared unkeyed and keyed rates.
pub(crate) static NCBI: Quota = Quota::new(
    SourceKey::declare("ncbi-eutils"),
    Rate::per_second(3),
    Some(Rate::per_second(10)),
    3,
);

// Conservative local policy; no numeric provider rate has been verified.
pub(crate) static EUROPEPMC: Quota = Quota::new(
    SourceKey::declare("europepmc"),
    Rate::per_second(1),
    None,
    1,
);
// Conservative local PubTator3 policy; no published numeric provider limit checked.
pub(crate) static PUBTATOR: Quota =
    Quota::new(SourceKey::declare("pubtator"), Rate::per_second(3), None, 3);

// Local bucket policy, not a published provider rate.
pub(crate) static PMC_OA: Quota =
    Quota::new(SourceKey::declare("pmc-oa"), Rate::per_second(10), None, 2);
// Conservative local iCite policy; no published numeric provider limit verified.
pub(crate) static ICITE: Quota =
    Quota::new(SourceKey::declare("icite"), Rate::per_second(5), None, 4);

// Conservative local LitVar2 policy; no published numeric rate verified.
pub(crate) static LITVAR: Quota =
    Quota::new(SourceKey::declare("litvar"), Rate::per_second(3), None, 3);

// Conservative local LitSense2 policy; no published numeric rate verified.
pub(crate) static LITSENSE: Quota =
    Quota::new(SourceKey::declare("litsense"), Rate::per_second(3), None, 3);
// Conservative shared-pool keyless policy; keyed introductory provider limit.
pub(crate) static SEMANTIC_SCHOLAR: Quota = Quota::new(
    SourceKey::declare("semantic-scholar"),
    Rate::every_seconds(2),
    Some(Rate::per_second(1)),
    1,
);

// Conservative service-specific local limits; no verified published numeric limits.
pub(crate) static ALLELE_REGISTRY: Quota =
    Quota::new(SourceKey::declare("car"), Rate::per_second(1), None, 1);
pub(crate) static MUTALYZER: Quota = Quota::new(
    SourceKey::declare("mutalyzer"),
    Rate::per_second(1),
    None,
    1,
);
pub(crate) static VARIANT_VALIDATOR: Quota = Quota::new(
    SourceKey::declare("variantvalidator"),
    Rate::per_second(1),
    None,
    1,
);
// Conservative serial registry policy; no published numeric quota was established.
pub(crate) static CTGOV: Quota =
    Quota::new(SourceKey::declare("ctgov"), Rate::per_second(1), None, 1);
/// Independent credential-free publisher download quota.
pub(crate) static PUBLISHER_PDF: biomcp_fetch::Quota = Quota::new(
    SourceKey::declare("publisher-pdf"),
    Rate::per_second(2),
    None,
    1,
);

// Conservative serial NCI policy; no published numeric quota was established.
pub(crate) static NCI: Quota = Quota::new(SourceKey::declare("nci"), Rate::per_second(1), None, 1);

// Conservative serial public portal policy, not a claimed published provider quota.
pub(crate) static CBIOPORTAL: Quota = Quota::new(
    SourceKey::declare("cbioportal"),
    Rate::per_second(2),
    None,
    1,
);

// Conservative serial policy; no published numeric quota was verified.
pub(crate) static HOTSPOTS: Quota =
    Quota::new(SourceKey::declare("hotspots"), Rate::per_second(1), None, 1);

// Conservative policy within the documented 100 requests/minute quota.
pub(crate) static GWAS: Quota =
    Quota::new(SourceKey::declare("gwas"), Rate::per_second(1), None, 1);

// Conservative serial local policy; no published numeric quota was verified.
pub(crate) static CHEMBL: Quota =
    Quota::new(SourceKey::declare("chembl"), Rate::per_second(1), None, 1);

// Conservative serial local policy; no published numeric quota was verified.
pub(crate) static OPENTARGETS: Quota = Quota::new(
    SourceKey::declare("opentargets"),
    Rate::per_second(1),
    None,
    1,
);

// https://open.fda.gov/apis/authentication/: both modes allow 240 requests/minute.
// The provider enforces its daily limits (1,000 keyless; 120,000 keyed) with 429;
// shared transport retry handling honors Retry-After without inventing daily pacing.
pub(crate) static OPENFDA: Quota = Quota::new(
    SourceKey::declare("openfda"),
    Rate::per_second(4),
    Some(Rate::per_second(4)),
    1,
);

// Conservative serial local policy; no published numeric quota was verified.
pub(crate) static EMA: Quota = Quota::new(SourceKey::declare("ema"), Rate::per_second(1), None, 1);

// Conservative serial local policy; no published numeric quota was verified.
pub(crate) static DDINTER: Quota =
    Quota::new(SourceKey::declare("ddinter"), Rate::per_second(1), None, 1);

// Conservative serial local policy; no published numeric provider quota verified.
pub(crate) static OLS: Quota = Quota::new(SourceKey::declare("ols"), Rate::per_second(1), None, 1);

// Conservative serial local policy; no published numeric provider quota verified.
pub(crate) static MONARCH: Quota =
    Quota::new(SourceKey::declare("monarch"), Rate::per_second(1), None, 1);

// Conservative serial local policy; no published numeric provider quota verified.
pub(crate) static SEER: Quota =
    Quota::new(SourceKey::declare("seer"), Rate::per_second(1), None, 1);

// Conservative serial public-file policies; no published numeric quota is claimed.
pub(crate) static GTR: Quota = Quota::new(
    SourceKey::declare("gtr-files"),
    Rate::per_second(1),
    None,
    1,
);
pub(crate) static WHO: Quota =
    Quota::new(SourceKey::declare("who-ivd"), Rate::per_second(1), None, 1);
pub(crate) static FDA_WEBSITE: Quota = Quota::new(
    SourceKey::declare("fda-website"),
    Rate::per_second(1),
    None,
    1,
);

// Conservative serial local policies; no published numeric CPIC quota is claimed.
pub(crate) static CPIC: Quota =
    Quota::new(SourceKey::declare("cpic"), Rate::per_second(1), None, 1);
// ClinPGx publishes two requests per second; this serial policy stays below it.
pub(crate) static CLINPGX: Quota =
    Quota::new(SourceKey::declare("clinpgx"), Rate::per_second(1), None, 1);

/// CDC automated-query spacing: https://wonder.cdc.gov/wonder/help/wonder-api.html
pub(crate) static WONDER: Quota = Quota::new(
    SourceKey::declare("wonder"),
    Rate::every_seconds(120),
    None,
    1,
)
.refuse_wait();

// Conservative serial published-catalog download policy; not a provider quota.
pub(crate) static WIKIPATHWAYS: Quota = Quota::new(
    SourceKey::declare("wikipathways"),
    Rate::per_second(1),
    None,
    1,
);

// Conservative serial local policy; no published numeric provider quota verified.
pub(crate) static HPO: Quota = Quota::new(SourceKey::declare("hpo"), Rate::per_second(1), None, 1);

// Conservative serial local policies, not published provider limits.
pub(crate) static INTERPRO: Quota =
    Quota::new(SourceKey::declare("interpro"), Rate::per_second(1), None, 1);
pub(crate) static COMPLEX_PORTAL: Quota = Quota::new(
    SourceKey::declare("complex-portal"),
    Rate::per_second(1),
    None,
    1,
);

// Conservative local policy; no published numeric provider rate.
pub(crate) static CELLOSAURUS: Quota = Quota::new(
    SourceKey::declare("cellosaurus"),
    Rate::per_second(2),
    None,
    1,
);

// Conservative serial local policy; no published numeric provider quota verified.
pub(crate) static ALPHAGENOME: Quota = Quota::new(
    SourceKey::declare("alphagenome"),
    Rate::per_second(1),
    None,
    1,
);
