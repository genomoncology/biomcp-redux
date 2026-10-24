use biomcp_fetch::{Quota, Rate, SourceKey};

// Conservative local BioThings policy; no published numeric provider limit.
pub(crate) static BIOTHINGS: Quota =
    Quota::new(SourceKey::declare("biothings"), Rate::per_second(1), None, 1);
