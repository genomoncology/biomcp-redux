//! The FDA Purple Book's terms, local health and manual synchronization declaration.
use super::{SPEC, read};
use crate::decl::{BundleDecl, Guidance, Probe, SourceDecl};

pub(crate) static DECL: SourceDecl = SourceDecl {
    spec: &SPEC,
    display: "FDA Purple Book",
    discovery_aliases: &[],
    registry_order: Some(65),
    probe: Probe::Custom(read::probe),
    components: &[],
    auxiliary: &[],
    bundles: &[&BUNDLE],
    guidance: Guidance {
        deadline: None,
        quota: None,
    },
    key: None,
};

static BUNDLE: BundleDecl = BundleDecl {
    name: SPEC.source.as_str(),
    help: read::HELP,
    source: SPEC.source,
    order: 12,
    sync: |fetch| read::sync_file(fetch, None),
    state: |leg| read::probe(leg).map(|()| biomcp_types::LocalBundleState::Fresh),
};
