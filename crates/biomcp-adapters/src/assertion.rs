use biodata::{
    AssertionDetail, AssertionDomain, AssertionKind, Attribution, Coding, Proposition, Statement,
    Term, Text, UnmappedAssertion, VariantAssertion, VariantRef,
};
/// Map only the reviewed source categories, retaining every other source label natively.
pub(crate) fn row(
    kind: AssertionKind,
    domain: AssertionDomain,
    label: Text,
    attribution: Attribution,
    subject: VariantRef,
    detail: AssertionDetail,
    review: Option<Text>,
) -> VariantAssertion {
    let proposition = match domain {
        AssertionDomain::SomaticClinicalImpact => Some(Proposition::ClinicalSignificance),
        AssertionDomain::Oncogenicity => Some(Proposition::Oncogenicity),
        AssertionDomain::Germline => match label.as_str() {
            "Pathogenic"
            | "Likely pathogenic"
            | "Uncertain significance"
            | "Likely benign"
            | "Benign"
            | "Pathogenic/Likely pathogenic"
            | "Benign/Likely benign"
            | "Conflicting classifications of pathogenicity" => Some(Proposition::Pathogenicity),
            _ => None,
        },
    };
    let mut row = VariantAssertion {
        kind,
        domain,
        statement_unmapped: proposition.is_none(),
        statement: None,
        unmapped: None,
        detail: Some(detail),
    };
    if let Some(proposition) = proposition {
        let coding = acmg(&label, row.detail.as_ref());
        let mut statement =
            Statement::new(subject, proposition, Term { label, coding }, attribution);
        statement.strength = review.map(|label| Term {
            label,
            coding: None,
        });
        if let Some(detail) = &row.detail {
            if detail.conditions.len() == 1 {
                statement.condition = detail.conditions.first().and_then(|c| c.concept());
            }
            if detail.origins.len() == 1 {
                statement.allele_origin = detail.origins.first().cloned().map(|label| Term {
                    label,
                    coding: None,
                });
            }
            statement.citations = detail.citations.clone();
        }
        row.statement = Some(statement);
    } else {
        row.unmapped = Some(UnmappedAssertion {
            classification: label,
            attribution,
            review_status: review,
            conditions: row
                .detail
                .as_ref()
                .map(|d| d.conditions.clone())
                .unwrap_or_default(),
        });
    }
    row
}
fn acmg(label: &Text, detail: Option<&AssertionDetail>) -> Option<Coding> {
    let detail = detail?;
    if !detail
        .methods
        .iter()
        .any(|m| m.as_str() == "ACMG Guidelines, 2015")
        || !detail.citations.iter().any(|p| p.to_string() == "25741868")
    {
        return None;
    }
    use biodata::AcmgAmp2015Code as C;
    let code = match label.as_str() {
        "Pathogenic" => C::Pathogenic,
        "Likely pathogenic" => C::LikelyPathogenic,
        "Uncertain significance" => C::UncertainSignificance,
        "Likely benign" => C::LikelyBenign,
        "Benign" => C::Benign,
        _ => return None,
    };
    Some(Coding::AcmgAmp2015(code))
}
