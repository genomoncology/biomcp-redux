//! Component terms have one home for queries, filters, labels and health.
use biodata::{Source, Use};
use biomcp_fetch::Terms;
/// One aggregator component with independent publisher terms.
#[derive(Clone, Copy, Debug)]
pub struct Component {
    /// Aggregator that carries this component.
    pub source: Source,
    /// Native component name.
    pub name: &'static str,
    /// Component publisher's terms, independent of public API reachability.
    pub terms: Terms,
}
/// CADD data retain their published noncommercial terms.
pub const CADD: Component = Component {
    source: Source::MyVariant,
    name: "cadd",
    terms: Terms {
        name: "CADD noncommercial data terms",
        url: "https://cadd.gs.washington.edu/license",
        usage: Use::Noncommercial,
    },
};
/// The captured dbNSFP academic branch retains its component restrictions.
pub const DBNSFP: Component = Component {
    source: Source::MyVariant,
    name: "dbnsfp",
    terms: Terms {
        name: "dbNSFP academic branch and individual component terms",
        url: "https://sites.google.com/site/jpopgen/dbNSFP",
        usage: Use::Noncommercial,
    },
};
/// MyVariant's historical COSMIC copy supplies no blanket reuse grant.
pub const COSMIC: Component = Component {
    source: Source::MyVariant,
    name: "cosmic",
    terms: Terms {
        name: "Historical COSMIC snapshot terms",
        url: "http://cancer.sanger.ac.uk/cosmic/license",
        usage: Use::Restricted,
    },
};
/// Component declarations used consistently by health and named sections.
pub const COMPONENTS: &[Component] = &[CADD, DBNSFP, COSMIC];
/// Reuse the direct source declaration for components that have one.
pub fn terms(name: &str) -> Option<Terms> {
    if name == "clinvar" {
        Some(crate::clinvar::SPEC.terms)
    } else {
        COMPONENTS
            .iter()
            .find(|component| component.name == name)
            .map(|component| component.terms)
    }
}

/// Convert a declaration to its public response label without repeating its values.
pub fn label(terms: Terms) -> Result<biodata::ComponentTerms, biomcp_fetch::InputError> {
    Ok(biodata::ComponentTerms {
        usage: terms.usage,
        name: biodata::Text::new(terms.name)
            .map_err(|_| biomcp_fetch::InputError::Serialization)?,
        url: biodata::Text::new(terms.url).map_err(|_| biomcp_fetch::InputError::Serialization)?,
    })
}
