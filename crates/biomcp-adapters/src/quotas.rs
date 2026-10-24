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
pub(crate) static HPA: Quota = Quota::new(SourceKey::declare("hpa"), Rate::per_second(1), None, 1);

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

/// Independent credential-free publisher download quota.
pub(crate) static PUBLISHER_PDF: biomcp_fetch::Quota = Quota::new(SourceKey::declare("publisher-pdf"), Rate::per_second(2), None, 1);
