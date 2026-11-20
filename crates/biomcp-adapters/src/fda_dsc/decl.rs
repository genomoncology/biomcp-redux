//! Source facts shared by discovery, health and commands.
use crate::decl::{Guidance, Probe, SourceDecl};
pub(crate) static DECL: SourceDecl = SourceDecl {
    spec: &super::spec::SPEC,
    display: "FDA Drug Safety Communications",
    discovery_aliases: &["DSC"],
    registry_order: Some(65),
    probe: Probe::Get(super::INDEX_PATH, &[]),
    components: &[],
    auxiliary: &[],
    bundles: &[],
    guidance: Guidance {
        deadline: None,
        quota: Some(QUOTA),
    },
    key: None,
};
/// Why a page read waits: FDA's robots.txt asks every client to wait 30 seconds between pages.
const QUOTA: &str = "BioMCP waits 30 seconds between www.fda.gov page reads, as FDA's robots.txt asks \
                     (Crawl-Delay: 30), so a communication's page may be read on a later call.";
