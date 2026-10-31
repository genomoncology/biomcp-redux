//! Cross-entity commands require complete source context, never a drug-name inference.
use crate::{Action, Outcome};
use biodata::{Build, Chromosome, GeneSymbol, Text};
use biomcp_types::{
    DrugCivic, DrugCivicClaim, DrugCivicMember, DrugOncokbLookup, DrugOncokbMissing, Request,
    Section, StepKind, VariantGet, VariantInput, VariantResolver, VariantSection,
};

pub(super) fn civic(body: &mut DrugCivic, fresh: bool, out: &mut Outcome) {
    let Section::Data { data, .. } = &mut body.civic else {
        return;
    };
    for stream in [&mut data.evidence, &mut data.assertions]
        .into_iter()
        .flatten()
    {
        for row in &mut stream.records {
            let member = row
                .profile
                .as_ref()
                .and_then(|profile| match profile.members.as_ref() {
                    [member] if profile.compound == Some(false) => Some(member),
                    _ => None,
                });
            let gene = member.and_then(|m| {
                m.member
                    .feature
                    .as_ref()?
                    .as_str()
                    .parse::<GeneSymbol>()
                    .ok()
            });
            let build = member.and_then(build);
            let variant = member.zip(build).and_then(|(m, b)| genomic(m, b));
            let tumor = tumor(row).cloned();
            use DrugOncokbMissing as M;
            row.oncokb_missing = [
                (M::Gene, gene.is_some()),
                (M::Build, build.is_some()),
                (M::Variant, variant.is_some()),
                (M::TumorType, tumor.is_some()),
            ]
            .into_iter()
            .filter_map(|(field, present)| (!present).then_some(field))
            .collect();
            if gene.is_none() {
                continue;
            }
            let Some((variant, build, tumor)) =
                variant.zip(build).zip(tumor).map(|((v, b), t)| (v, b, t))
            else {
                continue;
            };
            if let Some(lookup) = member.and_then(|m| lookup(m, variant, build, tumor)) {
                let action = crate::next::Step::new(
                    StepKind::Disambiguate,
                    &[biomcp_types::Source::Oncokb],
                    Action::Run {
                        request: Request::GetVariant(lookup.query.clone()),
                        fresh,
                    },
                );
                row.oncokb_lookup = Some(lookup);
                if !out.next.contains(&action) {
                    out.next.push(action);
                }
            }
        }
    }
}
fn build(member: &DrugCivicMember) -> Option<Build> {
    match member.reference_build.as_ref()?.as_str() {
        "GRCH37" => Some(Build::GRCh37),
        "GRCH38" => Some(Build::GRCh38),
        _ => None,
    }
}
fn genomic(member: &DrugCivicMember, build: Build) -> Option<VariantInput> {
    let mut eligible = member
        .member
        .hgvs_descriptions
        .iter()
        .filter_map(|expression| {
            let (accession, description) = expression.as_str().split_once(':')?;
            if !description.starts_with("g.") {
                return None;
            }
            Chromosome::from_accession(accession.parse().ok()?, build).ok()?;
            expression.as_str().parse().ok()
        })
        .collect::<Vec<_>>();
    eligible.dedup();
    if eligible.len() == 1 {
        eligible.pop()
    } else {
        None
    }
}
fn tumor(row: &DrugCivicClaim) -> Option<&Text> {
    row.claim
        .statement
        .as_ref()
        .and_then(|s| s.condition.as_ref())
        .map(|c| &c.label)
        .or_else(|| {
            row.claim
                .native
                .as_ref()
                .and_then(|s| s.condition.as_ref())
                .and_then(|c| c.label.as_ref())
        })
}

fn lookup(
    member: &DrugCivicMember,
    variant: VariantInput,
    build: Build,
    tumor: Text,
) -> Option<DrugOncokbLookup> {
    let query = VariantGet::new(variant, VariantResolver::Car, Some(build))
        .ok()?
        .with_tumor_type(tumor)
        .ok()?
        .with_sections(vec![VariantSection::Oncokb].into());
    Some(DrugOncokbLookup {
        member: member.member.record.id.clone(),
        label: Text::new("Look up this source-listed genomic variant in the stated tumor context")
            .ok()?,
        query,
    })
}
