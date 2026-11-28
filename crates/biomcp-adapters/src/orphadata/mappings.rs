//! Orphanet-to-OMIM relations as published in Orphadata product 1.
use super::raw::{Availability, Label, Term, label, term};
use crate::mapped::{Mapped, Problems};
use biodata::{Attribution, Curie, DiseaseEndpoint, DiseaseRelation};
use biomcp_fetch::{
    BadReply, Bundle, DocumentMedia, Fetch, Find, Freshness, KeyKind, Keys, Leg, RequestError, Row,
    RowFormat, encode_bundle_row,
};
use biomcp_types::{DiseaseCard, DiseaseConcept, DiseaseSection};
use serde::{Deserialize, Serialize};

pub(super) const NAME: &str = "orphadata-xrefs";
const FILE: Bundle<Native> = Bundle {
    spec: &super::SPEC,
    name: NAME,
    path: "/data/xml/en_product1.xml",
    fresh: Freshness::Days(30),
    row: std::marker::PhantomData,
    format: RowFormat::Document {
        media: DocumentMedia::Xml,
    },
};

pub(super) fn sync(fetch: &Fetch) -> Result<Mapped<biomcp_types::SyncResult>, RequestError> {
    fetch
        .sync(&FILE)
        .map(|info| crate::mapped::bundle_sync(FILE.spec.source, info))
}

pub(super) fn state(leg: &Leg<'_>) -> Result<biomcp_types::LocalBundleState, RequestError> {
    leg.stored_bundle_state(&FILE)
}

#[derive(Deserialize)]
struct Jdbor {
    #[serde(rename = "@date")]
    date: String,
    #[serde(rename = "@version")]
    version: Option<String>,
    #[serde(rename = "Availability")]
    availability: Availability,
    #[serde(rename = "DisorderList")]
    disorders: Disorders,
}

#[derive(Deserialize)]
struct Disorders {
    #[serde(rename = "Disorder")]
    items: Option<Vec<Disorder>>,
}

#[derive(Deserialize)]
struct Disorder {
    #[serde(rename = "OrphaCode")]
    code: Option<Label>,
    #[serde(rename = "Name")]
    name: Option<Label>,
    #[serde(rename = "DisorderType")]
    disorder_type: Option<Term>,
    #[serde(rename = "ExternalReferenceList")]
    references: Option<References>,
}

#[derive(Deserialize)]
struct References {
    #[serde(rename = "ExternalReference")]
    items: Option<Vec<Reference>>,
}

#[derive(Deserialize)]
struct Reference {
    #[serde(rename = "Source")]
    source: Option<Label>,
    #[serde(rename = "Reference")]
    id: Option<Label>,
    #[serde(rename = "DisorderMappingRelation")]
    relation: Option<Term>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Native {
    code: Option<String>,
    name: Option<String>,
    #[serde(default)]
    disorder_type: Option<String>,
    date: String,
    version: Option<String>,
    omim: Vec<Omim>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Omim {
    id: Option<String>,
    relation: Option<String>,
}

impl Row for Native {
    const REQUIRE_ACCEPTED_ROWS: bool = true;

    fn decode_document(body: &[u8]) -> Result<Vec<String>, BadReply> {
        let text = std::str::from_utf8(body).map_err(|_| BadReply)?;
        let document: Jdbor = quick_xml::de::from_str(text).map_err(|_| BadReply)?;
        document.availability.check()?;
        if document.date.trim().is_empty() {
            return Err(BadReply);
        }
        let disorders = document
            .disorders
            .items
            .filter(|rows| !rows.is_empty())
            .ok_or(BadReply)?;
        disorders
            .into_iter()
            .map(|disorder| {
                let omim = disorder
                    .references
                    .into_iter()
                    .flat_map(|list| list.items.into_iter().flatten())
                    .filter_map(|row| {
                        (label(row.source).as_deref() == Some("OMIM")).then(|| Omim {
                            id: label(row.id),
                            relation: term(row.relation),
                        })
                    })
                    .collect();
                encode_bundle_row(&Self {
                    code: label(disorder.code),
                    name: label(disorder.name),
                    disorder_type: term(disorder.disorder_type),
                    date: document.date.clone(),
                    version: document.version.clone(),
                    omim,
                })
            })
            .collect()
    }

    fn keys(&self) -> Keys {
        let exact = self
            .code
            .as_deref()
            .filter(|code| digits(code))
            .map(|code| (KeyKind::Id, code.into()))
            .into_iter()
            .collect();
        Keys {
            exact,
            words: Vec::new(),
        }
    }

    fn data_date(&self) -> Option<&str> {
        Some(&self.date)
    }
}

fn digits(value: &str) -> bool {
    !value.is_empty() && value.bytes().all(|byte| byte.is_ascii_digit())
}

fn disorder_rows(
    leg: &Leg<'_>,
    id: &Curie,
) -> Result<(Curie, Vec<Native>, biomcp_fetch::BundleInfo), RequestError> {
    let code = super::orpha_code(id.as_str()).ok_or(biomcp_fetch::InputError::Path)?;
    let canonical = format!("ORPHA:{code}")
        .parse()
        .map_err(|_| biomcp_fetch::InputError::Serialization)?;
    let (rows, info) = leg.stored_rows(&FILE, Find::Exact(KeyKind::Id, code))?;
    Ok((canonical, rows, info))
}

impl Native {
    fn attribution(&self, cx: &mut Problems) -> Attribution {
        Attribution {
            source: FILE.spec.source.text(),
            record: cx.record.clone(),
            release: cx.text("JDBOR.date", Some(&self.date)),
            version: cx.text("JDBOR.version", self.version.as_deref()),
            url: cx.text("url", Some(&format!("{}{}", FILE.spec.base, FILE.path))),
            approved: None,
            published: None,
            evaluated: None,
            submitter: None,
        }
    }
}

/// Read the disorder identity and classification from the product 1 catalog.
pub fn card(leg: &Leg<'_>, id: &Curie) -> Result<Mapped<Option<DiseaseCard>>, RequestError> {
    let (returned_id, rows, info) = disorder_rows(leg, id)?;
    let mut cx = Problems {
        source: FILE.spec.source,
        record: crate::mapped::record(&returned_id),
        list: crate::mapped::bundle_problems(FILE.spec.source, &info),
    };
    let value = rows.first().map(|row| DiseaseCard {
        requested_id: id.clone(),
        returned_id: returned_id.clone(),
        alias: None,
        concept: DiseaseConcept {
            id: Some(returned_id),
            label: cx.text("Name", row.name.as_deref()),
        },
        definition: None,
        disorder_type: cx.text("DisorderType", row.disorder_type.as_deref()),
        obsolete: None,
        consider: None,
        replaced_by: None,
        attribution: row.attribution(&mut cx),
        sections: Box::new([DiseaseSection::Prevalence]),
        components: Box::default(),
    });
    Ok(Mapped {
        value,
        problems: cx.list,
        refusals: crate::bundle_mapping::bundle_refusals(&info),
    })
}

/// Read OMIM mappings for the requested ORPHA code, keeping the publisher's direction.
pub fn omim_xrefs(leg: &Leg<'_>, id: &Curie) -> Result<Mapped<Vec<DiseaseRelation>>, RequestError> {
    let (subject_id, rows, info) = disorder_rows(leg, id)?;
    let mut mapped = Mapped {
        value: Vec::new(),
        problems: crate::mapped::bundle_problems(FILE.spec.source, &info),
        refusals: crate::bundle_mapping::bundle_refusals(&info),
    };
    for row in rows {
        let mut cx = Problems {
            source: FILE.spec.source,
            record: crate::mapped::record(&subject_id),
            list: Vec::new(),
        };
        let subject = DiseaseEndpoint {
            id: subject_id.clone(),
            label: cx.text("Name", row.name.as_deref()),
        };
        let attribution = row.attribution(&mut cx);
        for mapping in row.omim {
            if let Some(relation) = relation(mapping, &subject, &attribution, &mut cx) {
                mapped.value.push(relation);
            } else {
                mapped.refusals = crate::Refusal::Refused;
            }
        }
        mapped.problems.extend(cx.list);
    }
    Ok(mapped)
}

fn relation(
    mapping: Omim,
    subject: &DiseaseEndpoint,
    attribution: &Attribution,
    cx: &mut Problems,
) -> Option<DiseaseRelation> {
    let field = "ExternalReference.Reference";
    let number = cx.required(field, mapping.id.as_deref().filter(|value| digits(value)))?;
    let id = cx.optional(field, Some(format!("OMIM:{number}").parse()))?;
    let code = mapping
        .relation
        .as_deref()
        .and_then(|text| text.split_whitespace().next())
        .unwrap_or("ExternalReference");
    let predicate = cx.optional(
        "DisorderMappingRelation",
        Some(format!("orphadata:{code}").parse()),
    )?;
    Some(DiseaseRelation {
        subject: subject.clone(),
        predicate,
        object: DiseaseEndpoint { id, label: None },
        attribution: attribution.clone(),
        description: mapping.relation.map(Into::into),
    })
}
