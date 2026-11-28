//! Read structural variant observations for a gene in one native molecular profile.
use super::profiles::{selected, structural_variant_profiles};
use super::rows::{MAX_BYTES, acquired, gene, problems};
use super::{Records, SPEC};
use crate::raw::Field;
use biodata::{NcbiGeneId, StudyId, StudyProfileId, Text};
use biomcp_fetch::{BadReply, Freshness, Input, Leg, Read, Reply, RequestError};
use biomcp_types::{StudyGet, StudyStructuralVariant};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Scope {
    study_id: StudyId,
    molecular_profile_id: StudyProfileId,
    site1_entrez_gene_id: Option<i32>,
    site2_entrez_gene_id: Option<i32>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Variant {
    sample_id: Text,
    site1_hugo_symbol: Option<Field<Text>>,
    site2_hugo_symbol: Option<Field<Text>>,
    variant_class: Option<Field<Text>>,
    event_info: Option<Field<Text>>,
}
#[derive(Deserialize)]
#[serde(transparent)]
struct Variants(Vec<super::super::scope::Row<Variant, Scope>>);
impl Reply for Variants {
    fn check(&self) -> Result<(), BadReply> {
        Ok(())
    }
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Filter<'a> {
    entrez_gene_ids: [u32; 1],
    molecular_profile_ids: [&'a StudyProfileId; 1],
}

/// Read matching observations, checking both breakends for the selected gene.
pub fn structural_variants(
    leg: &Leg<'_>,
    q: &StudyGet,
) -> Result<(Records<StudyStructuralVariant>, StudyProfileId, NcbiGeneId), RequestError> {
    let gene = gene(leg, q)?;
    let profile = selected(structural_variant_profiles(leg, q)?, q)?;
    let body = Filter {
        entrez_gene_ids: [gene.get()],
        molecular_profile_ids: [&profile],
    };
    let endpoint = SPEC
        .post_json("/structural-variant/fetch")
        .fresh(Freshness::Days(1))
        .bytes(MAX_BYTES);
    let received = leg.call(
        &endpoint,
        Input::json(&[], &body),
        Read::<Variants>::json()
            .check(&|v| {
                v.0.iter()
                    .all(|row| {
                        let scope = &row.scope;
                        scope.study_id == *q.id()
                            && scope.molecular_profile_id == profile
                            && [scope.site1_entrez_gene_id, scope.site2_entrez_gene_id]
                                .into_iter()
                                .flatten()
                                .any(|id| u32::try_from(id) == Ok(gene.get()))
                    })
                    .then_some(())
                    .ok_or(BadReply)
            })
            .cache_if(&|v| v.0.iter().all(|r| r.value.as_ref().is_some())),
    )?;
    let mut problems = problems(q.id());
    let total = received.value.0.len();
    let rows: Vec<_> = received
        .value
        .0
        .into_iter()
        .map(|r| {
            let row = problems.field("sampleId", Some(r.value))?;
            problems.record = Some(row.sample_id.clone());
            Some(StudyStructuralVariant {
                sample_id: row.sample_id,
                site1_hugo_symbol: problems.field("site1HugoSymbol", row.site1_hugo_symbol),
                site2_hugo_symbol: problems.field("site2HugoSymbol", row.site2_hugo_symbol),
                variant_class: problems.field("variantClass", row.variant_class),
                event_info: problems.field("eventInfo", row.event_info),
            })
        })
        .collect();
    Ok((
        Records {
            incomplete: false,
            rejected: total - rows.iter().flatten().count(),
            window: crate::NativeWindow::local(rows, q.section_offset(), q.section_limit()),
            problems: problems.list,
            acquired: acquired(received.meta.fetched)?,
        },
        profile,
        gene,
    ))
}
