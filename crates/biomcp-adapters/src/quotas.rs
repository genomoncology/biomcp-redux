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

// Conservative local gnomAD policy; no published numeric provider limit.
pub(crate) static GNOMAD: Quota =
    Quota::new(SourceKey::declare("gnomad"), Rate::per_second(1), None, 1);

// Conservative local policy; no published numeric provider limit.
pub(crate) static REACTOME: Quota =
    Quota::new(SourceKey::declare("reactome"), Rate::per_second(2), None, 2);
