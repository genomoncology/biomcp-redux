//! Search the FDA withdrawal list by drug, indication, or withdrawal date.
use super::fda_list;
use crate::{Ctx, Outcome, status};
use biomcp_adapters::fda_withdrawals;
use biomcp_fetch::RequestError;
use biomcp_types::{DrugSearch, DrugWithdrawalPage, ReplyBody, Source};

const SOURCE: Source = Source::FdaWithdrawnApprovals;

pub(crate) fn search(cx: &Ctx<'_>, q: &DrugSearch) -> Outcome {
    let mut out = status::empty(DrugWithdrawalPage::into_reply(q.clone(), None));
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx
        .fetch
        .leg(&fda_withdrawals::SPEC, deadline)
        .fresh(cx.fresh);
    let result = fda_withdrawals::read(&leg);
    let mut report = leg.finish();
    let error = match result {
        Ok(found) => {
            let needle = q.query().map(|text| text.as_str().to_lowercase());
            let matching = found.items.into_iter().filter(|row| {
                fda_list::matches(needle.as_deref(), &[&row.drug, &row.indication])
                    || needle.as_deref().is_some_and(|needle| {
                        fda_withdrawals::matches_date(needle, &row.withdrawal_date)
                    })
            });
            let items = fda_list::page(cx, q, matching, &mut out);
            report.stale |= found.fallback.is_some();
            if let Some(RequestError::Source(error)) = found.fallback {
                report.failures.push(error);
            }
            out.body = DrugWithdrawalPage {
                total: out.page.as_ref().map_or(0, |page| page.total),
                seed_date: found.fallback.map(|_| fda_withdrawals::SEED_DATE.into()),
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
