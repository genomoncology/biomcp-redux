//! Explain the Drugs@FDA coverage boundary for products identified by the Purple Book.
use crate::{Ctx, Outcome};
use biodata::Text;
use biomcp_adapters::purple_book::{self, CberProduct};
use biomcp_types::{
    DrugApprovals, DrugGet, DrugLabel, DrugSearch, DrugSection, DrugSource, ReplyBody, Request,
    Source, SourceNotice, SourceStatus, StepKind,
};

const SOURCE: Source = Source::PurpleBook;

pub(super) fn product(cx: &Ctx<'_>, name: &Text) -> Option<CberProduct> {
    if cx.fetch.disabled(SOURCE) {
        return None;
    }
    purple_book::cber_product(name.as_str())
}

pub(super) fn for_query(cx: &Ctx<'_>, query: &DrugGet) -> Option<CberProduct> {
    match (query.source(), query.sections()) {
        (DrugSource::OpenfdaLabel, [DrugSection::Label])
        | (DrugSource::Drugsfda, [DrugSection::Approvals]) => product(cx, query.input()),
        _ => None,
    }
}

pub(super) fn unresolved(
    cx: &Ctx<'_>,
    query: &DrugGet,
    product: CberProduct,
    mut discovery: Outcome,
) -> Outcome {
    let label = query.source() == DrugSource::OpenfdaLabel;
    discovery.body = if label {
        DrugLabel::into_reply(query.clone(), None)
    } else {
        DrugApprovals::into_reply(query.clone(), None)
    };
    disclose(cx, product, label, &mut discovery);
    crate::status::item(&mut discovery, true, false, true);
    discovery
}

pub(super) fn disclose(cx: &Ctx<'_>, product: CberProduct, label_missing: bool, out: &mut Outcome) {
    let mut source = biomcp_adapters::registry::outcome(SOURCE, SourceStatus::Ok);
    let mut message = format!(
        "Drugs@FDA excludes products regulated by FDA’s Center for Biologics Evaluation and Research (CBER). Purple Book record: {}",
        product.url()
    );
    if label_missing {
        use std::fmt::Write;
        let _ = write!(
            message,
            ". For the label, search DailyMed: https://dailymed.nlm.nih.gov/dailymed/search.cfm?labeltype=all&query=BLA{}",
            product.number
        );
    }
    source.notices.push(SourceNotice::RequestGuidance {
        data_date: None,
        message: message.into(),
        retry_after_seconds: None,
    });
    out.sources.push(source);
    if let Some(search) = crate::next::checked(
        Text::new(&format!("BLA{}", product.number))
            .map_err(|_| biomcp_types::QueryInvalid::new(&["query"]))
            .and_then(|name| DrugSearch::named(name, DrugSource::PurpleBook, 1)),
        SOURCE,
        &mut out.problems,
    ) {
        out.next.push(crate::next::Step::run(
            StepKind::Disambiguate,
            &[SOURCE],
            Request::SearchDrug(search),
            cx.fresh,
        ));
    }
}
