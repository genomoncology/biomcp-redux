//! The FDA Purple Book's terms, local health and manual synchronization declaration.
use super::read::{self, SPEC};
use crate::decl::{BundleDecl, Guidance, Probe, SourceDecl};

pub(crate) static DECL: SourceDecl = SourceDecl {
    spec: &crate::purple_book::read::SPEC,
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
    name: read::NAME,
    help: read::HELP,
    source: SPEC.source,
    order: 12,
    sync: |fetch| read::sync_file(fetch, None),
    state: |leg| read::probe(leg).map(|()| biomcp_types::LocalBundleState::Fresh),
};

impl crate::decl::Bundle {
    /// Synchronize a local FDA Purple Book snapshot.
    pub const PURPLE_BOOK: Self = Self(&BUNDLE);
}
