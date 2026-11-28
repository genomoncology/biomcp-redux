//! Recognize variant classes outside the gnomAD short-variant endpoint.
use biodata::hgvs::{self, Change, Location, NaBody, NaEdit, Position, PositionAnchor};
use biomcp_adapters::gnomad::variant::{DATASET, RELEASE};
use biomcp_types::{
    Reason, Source, SourceOutcome, SourceState, SourceStatus, VariantHeader, VariantInput,
};

pub(super) fn dataset(sources: &[SourceOutcome]) -> Option<String> {
    sources
        .iter()
        .any(|source| {
            source.source == Source::Gnomad
                && (source.state != SourceState::Skipped
                    || source.reason == Some(Reason::OutOfScope))
        })
        .then(|| format!("{DATASET}/{RELEASE}"))
}

pub(super) fn skipped() -> SourceOutcome {
    biomcp_adapters::registry::outcome(
        Source::Gnomad,
        SourceStatus::Skipped(Some(Reason::OutOfScope)),
    )
}

pub(super) fn input_outside(input: &VariantInput) -> bool {
    outside(&input.native_hgvs())
}

pub(super) fn candidate_outside(candidates: &[VariantHeader]) -> bool {
    let [candidate] = candidates else {
        return false;
    };
    candidate
        .identity
        .representations
        .iter()
        .any(|form| outside(form.expression.value().as_str()))
}

fn outside(notation: &str) -> bool {
    let Some((reference, body)) = notation.split_once(':') else {
        return false;
    };
    let reference = reference.split('(').next().unwrap_or(reference);
    if body.starts_with("m.") || matches!(reference, "NC_012920.1" | "chrM" | "chrMT") {
        return true;
    }
    // HGVS repeat and copy counts follow the motif or the end of a range.
    let repeat = body.split_once('[').is_some_and(|(prefix, suffix)| {
        prefix
            .as_bytes()
            .last()
            .is_some_and(|b| b.is_ascii_digit() || b"ACGTN)".contains(b))
            && suffix.split_once(']').is_some_and(|(count, _)| {
                count.bytes().all(|b| b"0123456789_()?".contains(&b)) && !count.is_empty()
            })
    });
    if repeat && matches!(body.get(..2), Some("g." | "c." | "n.")) {
        return true;
    }
    let biodata::Outcome::Parsed(hgvs::Hgvs {
        change: Change::Nucleotide(change),
        ..
    }) = hgvs::parse(body).outcome
    else {
        return false;
    };
    match change.body {
        NaBody::Edit {
            edit: NaEdit::Inversion,
            ..
        } => true,
        NaBody::Edit {
            edit: NaEdit::Deletion(_) | NaEdit::Duplication(_),
            location: Location::Range(start, end),
        } => structural_span(start, end),
        _ => false,
    }
}

// gnomAD classifies structural spans starting at 50 bp.
fn structural_span(start: Position, end: Position) -> bool {
    match (start, end) {
        (
            Position {
                anchor: PositionAnchor::Absolute(start),
                offset: None,
            },
            Position {
                anchor: PositionAnchor::Absolute(end),
                offset: None,
            },
        ) => end.get().saturating_sub(start.get()) >= 49,
        _ => false,
    }
}
