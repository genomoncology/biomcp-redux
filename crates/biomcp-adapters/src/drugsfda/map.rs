//! Preserve independent native FDA products and application submission facts.
use super::{Application, Product, Submission};
use biodata::{Attribution, AuthorizationDate, AuthorizationEvent};
use biomcp_fetch::RequestError;
use biomcp_types::{
    DrugFdaCandidate, DrugFdaDocument, DrugFdaIngredient, DrugFdaProduct, DrugFdaProperty,
    DrugFdaSubmission,
};
impl Application {
    /// Expose an application name only when all native products share exact brand wording.
    pub fn candidate(&self) -> DrugFdaCandidate {
        let identity = self.header();
        let name = self
            .products
            .first()
            .filter(|first| {
                self.products
                    .iter()
                    .all(|p| p.brand_name == first.brand_name)
            })
            .and_then(|product| product.brand_name.clone());
        DrugFdaCandidate {
            id: identity.id.clone(),
            name,
            identity,
            products: self.products.iter().map(Product::mapped).collect(),
        }
    }
}
impl Product {
    /// Map a native product without associating it with any submission.
    pub fn mapped(&self) -> DrugFdaProduct {
        DrugFdaProduct {
            product_number: self.product_number.clone(),
            brand_name: self.brand_name.clone(),
            active_ingredients: self
                .active_ingredients
                .iter()
                .map(|i| DrugFdaIngredient {
                    name: i.name.clone(),
                    strength: i.strength.clone().map(Into::into),
                })
                .collect(),
            dosage_form: self.dosage_form.clone(),
            route: self.route.clone(),
            marketing_status: self.marketing_status.clone(),
            reference_drug: self.reference_drug.clone(),
            reference_standard: self.reference_standard.clone(),
            te_code: self.te_code.clone(),
        }
    }
}
impl Submission {
    /// Retain the application owner and native date roles without assigning product applicability.
    pub fn mapped(&self, application: &Application) -> Result<DrugFdaSubmission, RequestError> {
        let role = "submission_status_date".parse().map_err(|_| super::bad())?;
        Ok(DrugFdaSubmission {
            event: AuthorizationEvent {
                record: application.application_number.clone(),
                product_number: None,
                jurisdiction: None,
                status: self.submission_status.clone(),
                kind: self.submission_type.clone(),
                dates: self
                    .submission_status_date
                    .map(|date| AuthorizationDate { role, date })
                    .into_iter()
                    .collect(),
                indication: None,
                attribution: Attribution {
                    source: "drugsfda".parse().map_err(|_| super::bad())?,
                    record: Some(
                        application
                            .application_number
                            .to_string()
                            .parse()
                            .map_err(|_| super::bad())?,
                    ),
                    version: None,
                    release: None,
                    submitter: application.sponsor_name.clone(),
                    evaluated: None,
                    url: None,
                },
            },
            submission_number: self.submission_number.clone(),
            review_priority: self.review_priority.clone(),
            submission_class_code: self.submission_class_code.clone(),
            submission_class_code_description: self
                .submission_class_code_description
                .clone()
                .map(Into::into),
            submission_property_type: self.submission_property_type.as_ref().map(|p| {
                p.iter()
                    .map(|p| DrugFdaProperty {
                        code: p.code.clone(),
                    })
                    .collect()
            }),
            application_docs: self
                .application_docs
                .as_ref()
                .map(|docs| {
                    docs.iter()
                        .map(|d| {
                            Ok(DrugFdaDocument {
                                id: d.id.clone(),
                                url: d.url.clone(),
                                date: d.date,
                                r#type: d.r#type.clone(),
                            })
                        })
                        .collect::<Result<Box<[_]>, RequestError>>()
                })
                .transpose()?,
        })
    }
}
