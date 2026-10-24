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
pub(crate) static NCBI: Quota=Quota::new(SourceKey::declare("ncbi-eutils"),Rate::per_second(3),Some(Rate::per_second(10)),3);
