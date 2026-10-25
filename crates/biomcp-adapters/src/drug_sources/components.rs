//! Only reviewed MyChem fields may enter a source projection.
use crate::registry::Component;
use biomcp_fetch::{Fetch, Leg, Terms};
use biomcp_types::{Source, Use};
const fn component(name: &'static str, native_source: Source, terms: Terms) -> Component {
    Component {
        source: Source::Mychem,
        native_source: Some(native_source),
        name,
        terms,
    }
}
/// Source declarations are shared by selection, health and response labeling.
pub const COMPONENTS: &[Component] = &[
    component(
        "drugbank",
        Source::DrugbankOpen,
        Terms {
            name: "DrugBank Open identity subset CC0 (captured metadata)",
            url: "https://go.drugbank.com/releases/latest#open-data",
            usage: Use::Open,
        },
    ),
    component("chembl", Source::Chembl, super::CHEMBL_TERMS),
    component(
        "drugcentral",
        Source::Drugcentral,
        Terms {
            name: "DrugCentral CC BY-SA 4.0",
            url: "http://drugcentral.org/privacy",
            usage: Use::Attribution,
        },
    ),
    component(
        "gtopdb",
        Source::Gtopdb,
        Terms {
            name: "GtoPdb CC BY-SA 4.0",
            url: "https://www.guidetopharmacology.org/download.jsp",
            usage: Use::Attribution,
        },
    ),
    component(
        "unii",
        Source::Unii,
        Terms {
            name: "UNII public domain (captured metadata)",
            url: "https://www.nlm.nih.gov/web_policies.html",
            usage: Use::Open,
        },
    ),
    component(
        "ndc",
        Source::Ndc,
        Terms {
            name: "NDC public domain (captured metadata)",
            url: "https://www.fda.gov/about-fda/about-website/website-policies",
            usage: Use::Open,
        },
    ),
    component("openfda", Source::OpenfdaXref, super::FDA_TERMS),
];
/// Look up independent terms without granting rights to undeclared fields.
pub fn terms(name: &str) -> Option<Terms> {
    COMPONENTS.iter().find(|c| c.name == name).map(|c| c.terms)
}
/// Inspect source and component policy before reading or constructing a response.
pub fn disabled(fetch: &Fetch, name: &str) -> bool {
    COMPONENTS
        .iter()
        .find(|c| c.name == name)
        .is_none_or(|c| c.disabled(fetch))
}
/// Filter the exact source projection before its request/cache/replay identity exists.
/// Identity and provider result metadata are the only unlabeled fields retained.
pub fn allowed_fields(leg: &Leg<'_>, fields: &str) -> String {
    fields
        .split(',')
        .filter(|field| {
            let name = field.split('.').next().unwrap_or(field);
            matches!(*field, "_id" | "_score")
                || COMPONENTS
                    .iter()
                    .find(|c| c.name == name)
                    .is_some_and(|c| !c.disabled_on_leg(leg))
        })
        .collect::<Vec<_>>()
        .join(",")
}
