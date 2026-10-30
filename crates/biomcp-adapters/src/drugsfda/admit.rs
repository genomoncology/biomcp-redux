//! Keep native application facts independently of rejected nonidentity values.
use super::{raw::*, wire};
use crate::mapped::Problems;
impl From<wire::Application> for Application {
    fn from(row: wire::Application) -> Self {
        let mut p = Problems {
            source: biomcp_types::Source::Drugsfda,
            record: crate::mapped::record(&row.application_number),
            list: Vec::new(),
        };
        let products = p.fields("products", row.products).into_vec().into_iter().map(|v| product(v, &mut p)).collect();
        let submissions = p.fields("submissions", row.submissions).into_vec().into_iter().map(|v| submission(v, &mut p)).collect();
        let openfda = p.field("openfda", row.openfda).map(|v| OpenFda {
            spl_set_id: p.field("openfda.spl_set_id", v.spl_set_id).map(|values| values.into_iter().filter_map(|v| p.field("openfda.spl_set_id", Some(v))).collect()),
        });
        Self {
            application_number: row.application_number,
            sponsor_name: p.source_field("sponsor_name", row.sponsor_name),
            products, submissions, openfda,
            problems: p.list,
        }
    }
}
fn product(row: wire::Product, p: &mut Problems) -> Product {
    Product {
        product_number: row.product_number,
        brand_name: p.source_field("product.brand_name", row.brand_name),
        active_ingredients: p.fields("product.active_ingredients", row.active_ingredients).into_vec().into_iter().map(|v| ingredient(v, p)).collect(),
        dosage_form: p.field("product.dosage_form", row.dosage_form),
        route: p.field("product.route", row.route),
        marketing_status: p.field("product.marketing_status", row.marketing_status),
        reference_drug: p.field("product.reference_drug", row.reference_drug),
        reference_standard: p.field("product.reference_standard", row.reference_standard),
        te_code: p.field("product.te_code", row.te_code),
    }
}
fn ingredient(row: wire::Ingredient, p: &mut Problems) -> Ingredient {
    Ingredient {
        name: p.source_field("ingredient.name", row.name),
        strength: p.field("ingredient.strength", row.strength),
    }
}
fn submission(row: wire::Submission, p: &mut Problems) -> Submission {
    Submission {
        submission_type: p.field("submission.submission_type", row.submission_type),
        submission_number: row.submission_number,
        submission_status: p.field("submission.submission_status", row.submission_status),
        submission_status_date: p.field("submission.submission_status_date", row.submission_status_date).and_then(|v| p.optional("submission.submission_status_date", Some(date(&v)))),
        review_priority: p.field("submission.review_priority", row.review_priority),
        submission_class_code: p.field("submission.submission_class_code", row.submission_class_code),
        submission_class_code_description: p.field("submission.submission_class_code_description", row.submission_class_code_description),
        submission_property_type: p.field("submission.submission_property_type", row.submission_property_type).map(|values| values.into_iter().filter_map(|v| p.field("submission.submission_property_type", Some(v))).map(|v| property(v, p)).collect()),
        application_docs: p.field("submission.application_docs", row.application_docs).map(|values| values.into_iter().filter_map(|v| p.field("submission.application_docs", Some(v))).map(|v| document(v, p)).collect()),
    }
}
fn property(row: wire::Property, p: &mut Problems) -> Property {
    Property {
        code: p.field("property.code", row.code),
    }
}
fn document(row: wire::Document, p: &mut Problems) -> Document {
    Document {
        id: row.id,
        url: p.field("document.url", row.url),
        date: p.field("document.date", row.date).and_then(|v| p.optional("document.date", Some(date(&v)))),
        r#type: p.field("document.type", row.r#type),
    }
}
