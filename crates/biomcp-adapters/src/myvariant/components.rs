//! Component terms have one home for queries, filters, labels and health.
use biodata::{Source, Use};
use biomcp_fetch::Terms;
/// One aggregator component with independent publisher terms.
#[derive(Clone, Copy, Debug)]
pub struct Component {
    /// Aggregator that carries this component.
    pub source: Source,
    /// Explicit native source switch, when this component has one.
    pub native_source: Option<Source>,
    /// Native component name.
    pub name: &'static str,
    /// Component publisher's terms, independent of public API reachability.
    pub terms: Terms,
}
/// CADD data retain their published noncommercial terms.
pub const CADD: Component = Component {
    source: Source::MyVariant,
    native_source: None,
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
    native_source: None,
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
    native_source: None,
    name: "cosmic",
    terms: Terms {
        name: "Historical COSMIC snapshot terms",
        url: "http://cancer.sanger.ac.uk/cosmic/license",
        usage: Use::Restricted,
    },
};
/// ClinVar retains the independently reviewed submission-specific rights label.
pub const CLINVAR: Component = Component {
    source: Source::MyVariant,
    native_source: Some(Source::ClinVar),
    name: "clinvar",
    terms: Terms {
        name: "NCBI policy; submission rights remain source-specific",
        url: "https://www.ncbi.nlm.nih.gov/clinvar/docs/maintenance_use/",
        usage: Use::Restricted,
    },
};
/// Component declarations used consistently by health and named sections.
pub const COMPONENTS: &[Component] = &[CADD, DBNSFP, COSMIC, CLINVAR];
/// Resolve component terms from their canonical declaration.
pub fn terms(name: &str) -> Option<Terms> {
    COMPONENTS.iter().find(|c| c.name == name).map(|c| c.terms)
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

/// All components carried by the aggregator.
pub fn all() -> &'static [Component] { COMPONENTS }
/// Whether source or component policy blocks a carried component.
pub fn disabled(fetch: &biomcp_fetch::Fetch, name: &str) -> bool {
    policy(
        name,
        |source| fetch.disabled(source),
        |usage| fetch.disabled_use(usage),
    )
}
/// Inspect that same component policy from a deadline-scoped adapter leg.
pub fn disabled_on_leg(leg: &biomcp_fetch::Leg<'_>, name: &str) -> bool {
    policy(
        name,
        |source| leg.disabled_source(source),
        |usage| leg.disabled_use(usage),
    )
}
fn policy(
    name: &str,
    source_disabled: impl Fn(Source) -> bool,
    use_disabled: impl Fn(Use) -> bool,
) -> bool {
    source_disabled(Source::MyVariant)
        || COMPONENTS.iter().find(|c| c.name == name).and_then(|c| c.native_source).is_some_and(&source_disabled)
        || terms(name).is_some_and(|terms| use_disabled(terms.usage))
}
/// Opaque source queries cannot be safely scoped when a component is switched off.
pub fn restricted(fetch: &biomcp_fetch::Fetch) -> bool {
    !fetch.disabled(Source::MyVariant)
        && all()
            .iter()
            .any(|component| disabled(fetch, component.name))
}
