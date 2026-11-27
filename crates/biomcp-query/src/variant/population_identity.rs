//! One population join owner for get-variant sections and the population command.
use crate::{Ctx, status};
use biodata::Build;
use biomcp_adapters::allele_registry;
use biomcp_fetch::{Deadline, RequestError};
use biomcp_types::{
    Problem, Reason, Source, SourceOutcome, VariantGet, VariantHeader, VariantNamespace,
};

fn native_coordinate(query: &VariantGet) -> Option<&str> {
    let id = query.input().as_str();
    let id = id.strip_prefix("chr").unwrap_or(id);
    let (chromosome, _) = id.split_once('-')?;
    (query.build() == Some(Build::GRCh38)
        && query.input().coordinate_form()
        && !matches!(chromosome, "M" | "MT"))
    .then_some(id)
}

pub(super) fn use_native_coordinate(fetch: &biomcp_fetch::Fetch, query: &VariantGet) -> bool {
    native_coordinate(query).is_some()
        && (query.input().single_base_coordinate() || fetch.disabled(Source::AlleleRegistry))
}

pub(super) fn resolve(
    cx: &Ctx<'_>,
    query: &VariantGet,
    deadline: Deadline<'_>,
) -> super::identity::Identity {
    let mut found = super::identity::read(cx, query, deadline);
    let has_allele = super::population::join(&found.rows).is_some()
        || matches!(found.rows.as_slice(), [row] if registry_indel(row));
    let registry_form = match query.input().namespace() {
        VariantNamespace::Rs | VariantNamespace::ClinVar => true,
        VariantNamespace::Chromosome => !query.input().coordinate_form(),
        _ => false,
    };
    if registry_form
        && !cx.fetch.disabled(Source::AlleleRegistry)
        && found.rows.len() <= 1
        && !has_allele
        && !found
            .sources
            .iter()
            .any(|source| source.source == Source::AlleleRegistry)
    {
        super::identity::registry_alias(cx, query, deadline.after_stage(found.elapsed), &mut found);
    }
    found
}

#[derive(Default)]
pub(super) struct Join {
    pub id: Option<String>,
    pub sources: Vec<SourceOutcome>,
    pub problems: Vec<Problem>,
    pub local: Option<RequestError>,
    pub reason: Option<Reason>,
    pub elapsed: std::time::Duration,
    /// A raw indel ID can hit directly, but only normalization can establish an absence.
    pub unnormalized: bool,
}

pub(super) fn read(
    cx: &Ctx<'_>,
    query: &VariantGet,
    rows: &[VariantHeader],
    deadline: Deadline<'_>,
) -> Join {
    if query.input().single_base_coordinate()
        && let Some(id) = native_coordinate(query)
    {
        return Join {
            id: Some(id.into()),
            ..Join::default()
        };
    }
    let mut join = canonical(cx, rows, deadline);
    if join.id.is_none()
        && join.reason.is_none()
        && join.problems.is_empty()
        && rows.len() < 2
        && let Some(id) = native_coordinate(query)
    {
        join.id = Some(id.into());
        join.unnormalized = true;
    }
    join
}

fn canonical(cx: &Ctx<'_>, rows: &[VariantHeader], deadline: Deadline<'_>) -> Join {
    let mut join = Join {
        id: super::population::join(rows),
        ..Join::default()
    };
    if join.id.is_some() {
        return join;
    }
    let [row] = rows else { return join };
    if !registry_indel(row) {
        return join;
    }
    let ids: std::collections::BTreeSet<_> = row
        .identity
        .ids
        .iter()
        .filter(|id| matches!(id, biodata::VariantId::Ca(_)))
        .collect();
    if ids.len() != 1 {
        return join;
    }
    let Some(id) = ids.first() else { return join };
    let leg = cx
        .fetch
        .leg(&allele_registry::SPEC, deadline)
        .fresh(cx.fresh);
    let started = leg.remaining();
    let result = allele_registry::population_id(&leg, &id.to_string());
    join.elapsed = started.saturating_sub(leg.remaining());
    let error = result.as_ref().err().copied();
    join.sources
        .push(status::source(Source::AlleleRegistry, leg.finish(), error));
    match result {
        Ok(mapped) => {
            join.id = mapped.value.map(|id| id.to_string());
            join.problems = mapped.problems;
        }
        Err(error) => {
            join.local = status::local(error);
            join.reason = Some(status::reason(error).unwrap_or(Reason::Unavailable));
        }
    }
    join
}

fn registry_indel(row: &VariantHeader) -> bool {
    row.identity.representations.iter().any(|r| {
        r.source == Source::AlleleRegistry.text()
            && r.build == Some(Build::GRCh38)
            && r.allele.as_ref().is_some_and(|a| {
                a.reference().to_string().len() != 1 || a.alternate().to_string().len() != 1
            })
    })
}
