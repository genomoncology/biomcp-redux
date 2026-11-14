use crate::Outcome;
use biodata::{FaersReport, Text};
use biomcp_adapters::{
    NativePositions,
    faers::{
        Extras, Positions,
        words::{self, AGE_UNIT, PATIENT_SEX, REPORTER_QUALIFICATION, Words, label, off_list},
    },
};
use biomcp_fetch::RequestError;
use biomcp_types::ReplyBody;
use biomcp_types::{
    AdverseEventGet, AdverseEventHeader, AdverseEventPatient, AdverseEventReporter,
    AdverseEventSection as S, AdverseEventSeriousness, AdverseEventSource,
    AdverseEventUnmappedCode, Request, StepKind,
};
mod identifiers;
mod rows;
pub(super) fn render(
    q: &AdverseEventGet,
    r: FaersReport,
    url: Text,
    snapshot: Option<Text>,
    positions: Positions,
    extras: Extras,
    out: &mut Outcome,
) -> Result<(), RequestError> {
    let navigation = snapshot
        .clone()
        .map_or_else(|| q.clone(), |t| q.with_snapshot(t));
    let header = AdverseEventHeader {
        id: r.safetyreportid.clone(),
        source: AdverseEventSource::Faers,
        version: r.safetyreportversion.clone(),
        serious: r.serious.clone(),
        serious_label: None,
        event_type: None,
        first_received: r.receivedate.clone(),
    };
    let components = if q.section() == Some(S::Reactions) {
        biomcp_adapters::adverse_event_sources::components::labels(
            biomcp_types::Source::Faers,
            biomcp_types::Source::Meddra,
        )?
    } else {
        Box::default()
    };
    match q.section() {
        None => {
            card(q, r, url, header, snapshot, (&positions, extras), out)?;
            for section in [S::Drugs, S::Reactions] {
                section_step(&navigation, section, None, out);
            }
        }
        Some(S::Drugs) => drugs(q, &r, &positions, header, snapshot, components, out),
        Some(S::Reactions) => {
            let total = positions
                .reactions
                .as_ref()
                .map_or(r.reactions.len() as u64, |p| u64::from(p.total));
            let records = rows::reactions(q, &r.reactions, positions.reactions.as_ref());
            page(
                &navigation,
                q.section_offset()
                    .saturating_add(q.section_limit())
                    .min(total as u32),
                total,
                out,
            );
            out.body = (biomcp_types::AdverseEventReactions {
                header,
                snapshot,
                reactions: biomcp_types::AdverseEventRecords {
                    records,
                    offset: q.section_offset(),
                    total,
                },
                components,
            })
            .with_query(q.clone());
        }
        Some(section) => {
            let drug = NativePositions::enumerate(&r.drugs, positions.drugs.as_ref())
                .find(|(index, _)| Some(*index) == q.drug_index())
                .map(|(_, row)| row);
            identifiers::render(q, drug, section, header, snapshot, components, out)?;
        }
    }
    Ok(())
}
fn drugs(
    q: &AdverseEventGet,
    r: &FaersReport,
    positions: &Positions,
    header: AdverseEventHeader,
    snapshot: Option<Text>,
    components: Box<[biomcp_types::ComponentLabel]>,
    out: &mut Outcome,
) {
    let navigation = snapshot
        .clone()
        .map_or_else(|| q.clone(), |t| q.with_snapshot(t));
    let total = positions
        .drugs
        .as_ref()
        .map_or(r.drugs.len() as u64, |p| u64::from(p.total));
    let records = rows::drugs(q, &r.drugs, positions.drugs.as_ref());
    page(
        &navigation,
        q.section_offset()
            .saturating_add(q.section_limit())
            .min(total as u32),
        total,
        out,
    );
    out.body = (biomcp_types::AdverseEventDrugs {
        header,
        snapshot,
        drugs: biomcp_types::AdverseEventRecords {
            records,
            offset: q.section_offset(),
            total,
        },
        components,
    })
    .with_query(q.clone());
    let initial = out.next.len();
    for (index, drug) in
        NativePositions::enumerate(&r.drugs, positions.drugs.as_ref()).filter(|(index, _)| {
            *index >= q.section_offset()
                && *index < q.section_offset().saturating_add(q.section_limit())
        })
    {
        if let Some(ids) = &drug.openfda {
            for section in identifiers::available(ids) {
                if out.next.len() < initial + 3 {
                    section_step(&navigation, section, Some(index), out);
                }
            }
        }
    }
}
fn section_step(q: &AdverseEventGet, s: S, index: Option<u32>, out: &mut Outcome) {
    if let Some(q) = crate::next::checked(
        q.with_section(s, index),
        biomcp_types::Source::Faers,
        &mut out.problems,
    ) {
        out.next.push(crate::next::Step::run(
            StepKind::Section,
            &[biomcp_types::Source::Faers],
            Request::GetAdverseEvent(q),
            false,
        ));
    }
}
fn page(q: &AdverseEventGet, offset: u32, total: u64, out: &mut Outcome) {
    if offset > q.section_offset() && u64::from(offset) < total {
        out.next.push(crate::next::Step::run(
            StepKind::Page,
            &[biomcp_types::Source::Faers],
            Request::GetAdverseEvent(q.at_section_offset(offset)),
            false,
        ));
    }
}

fn card(
    q: &AdverseEventGet,
    r: FaersReport,
    url: Text,
    mut header: AdverseEventHeader,
    snapshot: Option<Text>,
    (positions, extras): (&Positions, Extras),
    out: &mut Outcome,
) -> Result<(), RequestError> {
    header.serious_label = label(words::SERIOUS, header.serious.as_deref());
    let mut unmapped = Vec::new();
    let sex = r.patient.as_ref().and_then(|p| p.sex.as_deref());
    let qualification = r.reporter.as_ref().and_then(|p| p.qualification.as_deref());
    let age_unit = extras.age_unit.as_deref();
    let flags = r.seriousness.as_ref();
    for (field, list, code) in [
        ("patient.patientsex", PATIENT_SEX, sex),
        ("patient.patientonsetageunit", AGE_UNIT, age_unit),
        (
            "primarysource.qualification",
            REPORTER_QUALIFICATION,
            qualification,
        ),
        ("serious", words::SERIOUS, header.serious.as_deref()),
        (
            "seriousnessdeath",
            words::SERIOUSNESS_DEATH,
            flags.and_then(|n| n.death.as_deref()),
        ),
        (
            "seriousnesslifethreatening",
            words::SERIOUSNESS_LIFETHREATENING,
            flags.and_then(|n| n.lifethreatening.as_deref()),
        ),
        (
            "seriousnesshospitalization",
            words::SERIOUSNESS_HOSPITALIZATION,
            flags.and_then(|n| n.hospitalization.as_deref()),
        ),
        (
            "seriousnessdisabling",
            words::SERIOUSNESS_DISABLING,
            flags.and_then(|n| n.disabling.as_deref()),
        ),
        (
            "seriousnesscongenitalanomali",
            words::SERIOUSNESS_CONGENITALANOMALI,
            flags.and_then(|n| n.congenitalanomali.as_deref()),
        ),
        (
            "seriousnessother",
            words::SERIOUSNESS_OTHER,
            flags.and_then(|n| n.other.as_deref()),
        ),
    ] {
        unmapped_code(field, list, code, &mut unmapped);
    }
    out.body = (biomcp_types::AdverseEventFaersCard {
        header,
        snapshot,
        latest_received: r.receiptdate,
        latest_received_format: r.receiptdateformat,
        transmitted: r.transmissiondate,
        transmitted_format: r.transmissiondateformat,
        first_received_format: r.receivedateformat,
        seriousness: r.seriousness.map(|native| AdverseEventSeriousness {
            death_label: label(words::SERIOUSNESS_DEATH, native.death.as_deref()),
            lifethreatening_label: label(
                words::SERIOUSNESS_LIFETHREATENING,
                native.lifethreatening.as_deref(),
            ),
            hospitalization_label: label(
                words::SERIOUSNESS_HOSPITALIZATION,
                native.hospitalization.as_deref(),
            ),
            disabling_label: label(words::SERIOUSNESS_DISABLING, native.disabling.as_deref()),
            congenitalanomali_label: label(
                words::SERIOUSNESS_CONGENITALANOMALI,
                native.congenitalanomali.as_deref(),
            ),
            other_label: label(words::SERIOUSNESS_OTHER, native.other.as_deref()),
            native,
        }),
        case_event_date: extras.case_event_date,
        patient: r.patient.map(|native| AdverseEventPatient {
            sex_label: label(PATIENT_SEX, native.sex.as_deref()),
            age_unit_label: label(
                AGE_UNIT,
                native
                    .age
                    .as_ref()
                    .and_then(|a| a.unit)
                    .map(biodata::FaersAgeUnit::as_str),
            ),
            native,
        }),
        reporter: r.reporter.map(|native| AdverseEventReporter {
            qualification_label: label(REPORTER_QUALIFICATION, native.qualification.as_deref()),
            native,
        }),
        unmapped_codes: unmapped.into(),
        drug_count: positions
            .drugs
            .as_ref()
            .map_or(r.drugs.len() as u32, |p| p.total),
        reaction_count: positions
            .reactions
            .as_ref()
            .map_or(r.reactions.len() as u32, |p| p.total),
        source_url: Text::new("https://open.fda.gov/apis/drug/event/")
            .map_err(|_| biomcp_fetch::InputError::Serialization)?,
        report_url: url,
        available_sections: vec![S::Drugs, S::Reactions].into(),
        components: Box::default(),
    })
    .with_query(q.clone());
    Ok(())
}

/// Name a present code that is not on openFDA's published list for its field.
pub(super) fn unmapped_code(
    field: &str,
    list: Words,
    code: Option<&str>,
    out: &mut Vec<AdverseEventUnmappedCode>,
) {
    if let Some(code) = code.filter(|_| off_list(list, code)) {
        out.push(AdverseEventUnmappedCode {
            field: field.into(),
            code: code.into(),
        });
    }
}
