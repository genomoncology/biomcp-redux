//! Share the output projection with the trial arm byte check.
use super::{narrow, rows::Call, sections, select::schema};
use biomcp_types::Request;

/// Serialize the exact selected arm row used to decide whether it fits the page.
pub(crate) fn arm_projection<'a>(
    request: &Request,
    fields: &'a [String],
) -> Option<impl Fn(&biomcp_types::TrialArmRecord) -> Result<Vec<u8>, serde_json::Error> + use<'a>>
{
    use biomcp_types::{ResponseBody, TrialCards, TrialSection, TrialSource};
    let Request::GetTrial(query) = request else {
        return None;
    };
    if fields.is_empty()
        || query.source() != TrialSource::Ctgov
        || !query
            .sections()
            .iter()
            .any(|s| matches!(s, TrialSection::Arms | TrialSection::All))
    {
        return None;
    }
    let call = Call::of("get", "trial", &["arms".into()]);
    let list = sections::lists(&schema(TrialCards::SCHEMA)?, &call)
        .into_iter()
        .find(|list| {
            list.at
                .iter()
                .any(|step| matches!(step, super::rows::Step::Key(name) if name == "arms"))
        })?;
    let chosen = narrow::selection(&list.row, fields);
    (!chosen.is_empty()).then_some(move |arm: &biomcp_types::TrialArmRecord| {
        let mut value = serde_json::to_value(arm)?;
        narrow::row(&mut value, &list.row, &chosen);
        serde_json::to_vec(&value)
    })
}
