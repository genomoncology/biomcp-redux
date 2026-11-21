//! FDA potential safety signals by product, signal or follow-up text.
use super::fda_list;
use crate::{Ctx, Outcome, status};
use biomcp_adapters::fda_signals;
use biomcp_fetch::RequestError;
use biomcp_types::{DrugSafetySignalPage, DrugSearch, ReplyBody, Source};

const SOURCE: Source = Source::FdaSafetySignals;
const CAUTION: &str = concat!(
    "The appearance of a drug or biological product on a quarterly report does not mean that FDA has concluded ",
    "that the drug or biological product has the listed risk or that FDA has identified a causal relationship ",
    "between the drug or biological product and the listed risk at the time the report is posted."
);

pub(crate) fn search(cx: &Ctx<'_>, q: &DrugSearch) -> Outcome {
    let mut out = status::empty(DrugSafetySignalPage::into_reply(q.clone(), None));
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&fda_signals::SPEC, deadline).fresh(cx.fresh);
    let result = fda_signals::read(&leg);
    let mut report = leg.finish();
    let error = match result {
        Ok(found) => {
            let needle = q.query().map(|text| text.as_str().to_lowercase());
            let matching = found.items.into_iter().filter(|row| {
                fda_list::matches(
                    needle.as_deref(),
                    &[
                        row.product.as_str(),
                        row.signal.as_str(),
                        &row.additional_information,
                    ],
                )
            });
            let items = fda_list::page(cx, q, matching, &mut out);
            report.stale |= found.seed_date.is_some() || found.unread_quarters != 0;
            if let Some(RequestError::Source(error)) = found.index_error {
                report.failures.push(error);
            }
            out.body = DrugSafetySignalPage {
                total: out.page.as_ref().map_or(0, |page| page.total),
                seed_date: found.seed_date,
                unread_quarters: found.unread_quarters,
                caution: CAUTION.into(),
                items,
            }
            .with_query(q.clone());
            None
        }
        Err(error) => Some(error),
    };
    fda_list::source(SOURCE, &mut out, report, error);
    command.finish(deadline.remaining());
    out
}
