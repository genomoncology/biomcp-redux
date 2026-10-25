//! Drug source declarations; adapter implementations consume these same values.
use crate::quotas;
use biomcp_fetch::{KeyUse, Quota, Rate, SourceKey, SourceSpec, Terms};
use biomcp_types::{Source, Use};
use std::time::Duration;
/// Independent terms for the exposed MyChem projections.
pub mod components;
const FDA_TERMS: Terms = Terms {
    name: "openFDA terms; original label rights retained",
    url: "https://open.fda.gov/terms/",
    usage: Use::Open,
};
/// Direct and carried ChEMBL data share the publisher's license declaration.
pub const CHEMBL_TERMS: Terms = Terms {
    name: "ChEMBL CC BY-SA 3.0",
    url: "https://chembl.github.io/chembl-licensing/",
    usage: Use::Attribution,
};
// Conservative local policies, not claims about publisher quotas.
static CHEMBL_QUOTA: Quota = Quota::new(SourceKey::declare("chembl"), Rate::per_second(1), None, 1);
static OT_QUOTA: Quota = Quota::new(
    SourceKey::declare("opentargets"),
    Rate::per_second(1),
    None,
    1,
);
static FDA_QUOTA: Quota = Quota::new(SourceKey::declare("openfda"), Rate::per_second(1), None, 1);
static EMA_QUOTA: Quota = Quota::new(SourceKey::declare("ema"), Rate::per_second(1), None, 1);
static DDINTER_QUOTA: Quota =
    Quota::new(SourceKey::declare("ddinter"), Rate::per_second(1), None, 1);
const fn spec(
    source: Source,
    base: &'static str,
    terms: Terms,
    quota: &'static Quota,
) -> SourceSpec {
    SourceSpec {
        source,
        base,
        terms,
        quota,
        key: KeyUse::None,
        deadline: Duration::from_secs(15),
    }
}
/// Candidate identities retain each component's own rights.
pub static MYCHEM: SourceSpec = spec(
    Source::Mychem,
    "https://mychem.info/v1",
    Terms {
        name: "BioThings and upstream source terms",
        url: "https://docs.mychem.info/en/latest/doc/data_source.html",
        usage: Use::Attribution,
    },
    &quotas::BIOTHINGS,
);
/// Direct ChEMBL native records.
pub static CHEMBL: SourceSpec = spec(
    Source::Chembl,
    "https://www.ebi.ac.uk/chembl/api/data",
    CHEMBL_TERMS,
    &CHEMBL_QUOTA,
);
/// Open Targets keeps underlying source provenance.
pub static OPENTARGETS: SourceSpec = spec(
    Source::Opentargets,
    "https://api.platform.opentargets.org/api/v4",
    Terms {
        name: "Open Targets CC0; underlying source provenance retained",
        url: "https://platform-docs.opentargets.org/licence",
        usage: Use::Open,
    },
    &OT_QUOTA,
);
/// Label content retains original label rights.
pub static OPENFDA_LABEL: SourceSpec = spec(
    Source::OpenfdaLabel,
    "https://api.fda.gov/drug/label.json",
    FDA_TERMS,
    &FDA_QUOTA,
);
/// Native application and product records share the openFDA service quota.
pub static DRUGSFDA: SourceSpec = spec(
    Source::Drugsfda,
    "https://api.fda.gov/drug/drugsfda.json",
    FDA_TERMS,
    &FDA_QUOTA,
);
/// Reproduction requires attribution and preserves third-party exceptions.
pub static EMA: SourceSpec = spec(
    Source::Ema,
    "https://www.ema.europa.eu",
    Terms {
        name: "EMA reproduction and attribution terms; third-party exceptions",
        url: "https://www.ema.europa.eu/en/about-us/about-website/legal-notice",
        usage: Use::Attribution,
    },
    &EMA_QUOTA,
);
/// Noncommercial interaction data remain enabled until caller policy disables them.
pub static DDINTER: SourceSpec = spec(
    Source::Ddinter,
    "https://ddinter.scbdd.com",
    Terms {
        name: "DDInter CC BY-NC-SA 4.0",
        url: "https://ddinter.scbdd.com/terms/",
        usage: Use::Noncommercial,
    },
    &DDINTER_QUOTA,
);
