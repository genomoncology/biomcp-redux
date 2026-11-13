//! Drugs@FDA application search by name group, applicant prefix and requested order.
use super::{Page, applicant_prefix, literal, page};
use biodata::Text;
use biomcp_fetch::{Leg, RequestError};
use biomcp_types::DrugFdaSort;
/// Discover exact brand or ingredient names, then try their native word fields after a zero total.
/// An applicant narrows either tier by a native sponsor_name prefix wildcard, or selects alone.
pub fn search(
    leg: &Leg<'_>,
    text: Option<&Text>,
    applicant: Option<&Text>,
    limit: u32,
    offset: u32,
    sort: Option<DrugFdaSort>,
) -> Result<Page, RequestError> {
    let names: Vec<Text> = text.cloned().into_iter().collect();
    let at = |step| {
        let selector = selector(&names, applicant, step);
        page(leg, &selector, sort, limit, offset, None)
    };
    let exact = at(NameStep::Exact)?;
    if exact.total > 0 || text.is_none() {
        return Ok(exact);
    }
    at(NameStep::Words)
}

/// BioMCP's own bound on one native Drugs@FDA request, so a name group stays one request.
const NAME_QUERY_MAX_BYTES: usize = 4096;

/// Which name step a Drugs@FDA search pages, chosen on its first page as the typed name's own
/// search would choose it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum NameStep {
    /// Exact brand and ingredient names.
    Exact,
    /// Word matches, after the typed name has no exact match.
    Words,
}

fn sponsor(applicant: &Text) -> String {
    let mut term = String::from("sponsor_name:");
    for c in applicant_prefix(applicant).chars() {
        if !c.is_alphanumeric() {
            term.push('\\');
        }
        term.push(c);
    }
    term.push('*');
    term
}

/// One native selector for a name group at one step, narrowed by the applicant prefix.
fn selector(names: &[Text], applicant: Option<&Text>, step: NameStep) -> String {
    let suffix = if step == NameStep::Exact {
        ".exact"
    } else {
        ""
    };
    let names = (!names.is_empty()).then(|| {
        names
            .iter()
            .map(|name| {
                let name = literal(name.as_str());
                format!(
                    "products.brand_name{suffix}:{name} OR products.active_ingredients.name{suffix}:{name}"
                )
            })
            .collect::<Vec<_>>()
            .join(" OR ")
    });
    match (names, applicant.map(sponsor)) {
        (Some(names), Some(sponsor)) => format!("({names}) AND {sponsor}"),
        (names, sponsor) => names.or(sponsor).unwrap_or_default(),
    }
}

/// Whether one request can carry this name group.
pub fn names_fit(names: &[Text], applicant: Option<&Text>) -> bool {
    selector(names, applicant, NameStep::Exact).len() <= NAME_QUERY_MAX_BYTES
}

/// Search one bounded name group in one native request; never combine per-name totals.
/// The first page takes the step the typed name alone would reach, so the group finds at least
/// what the typed name finds.
pub fn search_widened(
    leg: &Leg<'_>,
    (names, typed): (&[Text], &Text),
    applicant: Option<&Text>,
    (limit, offset): (u32, u32),
    selected: Option<NameStep>,
    sort: Option<DrugFdaSort>,
) -> (Result<Page, RequestError>, NameStep) {
    let at = |step| {
        page(
            leg,
            &selector(names, applicant, step),
            sort,
            limit,
            offset,
            None,
        )
    };
    if let Some(step) = selected {
        return (at(step), step);
    }
    let exact = at(NameStep::Exact);
    let found = match &exact {
        Err(_) => return (exact, NameStep::Exact),
        Ok(page) => page.total > 0,
    };
    if found && names.len() > 1 {
        let typed = std::slice::from_ref(typed);
        match page(
            leg,
            &selector(typed, applicant, NameStep::Exact),
            None,
            1,
            0,
            None,
        ) {
            Ok(page) if page.total > 0 => return (exact, NameStep::Exact),
            Ok(_) => {}
            Err(error) => return (Err(error), NameStep::Exact),
        }
    } else if found {
        return (exact, NameStep::Exact);
    }
    (at(NameStep::Words), NameStep::Words)
}
