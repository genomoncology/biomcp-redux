//! Each selected source section runs only for its exact candidate on the current page.
use crate::Ctx;
use biodata::{Problem, SourceOutcome, VariantGet, VariantHeader, VariantSection};
use biomcp_fetch::{Deadline, RequestError};
type Result = (Vec<SourceOutcome>, Vec<Problem>, Option<RequestError>);
pub(super) fn apply(
    cx: &Ctx<'_>,
    query: &VariantGet,
    candidates: &mut [VariantHeader],
    deadline: Deadline<'_>,
) -> Result {
    let mut sources = Vec::new();
    let mut problems = Vec::new();
    let mut local = None;
    for candidate in candidates {
        let mut sections = candidate.sections.take().unwrap_or_default();
        for section in query.sections() {
            let (incoming, issues, error) = match section {
                VariantSection::Predict => {
                    let (value, sources, problems, error) = super::predict::section(
                        cx,
                        query,
                        std::slice::from_ref(candidate),
                        deadline,
                    );
                    sections.predict = Some(value);
                    (sources, problems, error)
                }
                VariantSection::Conservation => {
                    let (value, sources, problems, error) = super::conservation::section(
                        cx,
                        query,
                        std::slice::from_ref(candidate),
                        deadline,
                    );
                    sections.conservation = Some(value);
                    (sources, problems, error)
                }
                VariantSection::Cosmic => {
                    let (value, sources, problems, error) = super::cosmic::section(
                        cx,
                        query,
                        std::slice::from_ref(candidate),
                        deadline,
                    );
                    sections.cosmic = Some(value);
                    (sources, problems, error)
                }
                _ => continue,
            };
            sources.extend(incoming);
            problems.extend(issues);
            local = local.or(error);
        }
        if sections.predict.is_some()
            || sections.conservation.is_some()
            || sections.cosmic.is_some()
        {
            candidate.sections = Some(sections);
        }
    }
    (sources, problems, local)
}
