//! Public schema contract tests; file I/O stays outside the pure library.
#![cfg(feature = "schema")]
#[path = "schemas/compatibility.rs"]
mod compatibility;
#[path = "schemas/compatibility_cases.rs"]
mod compatibility_cases;
#[path = "schemas/values.rs"]
mod values;

use biodata::*;
use schemars::{JsonSchema, generate::SchemaSettings};
use serde_json::Value;
use std::{fs, path::Path};

struct Contract {
    name: &'static str,
    id: &'static str,
    generate: fn() -> Value,
}

#[expect(
    clippy::expect_used,
    reason = "Schema test setup must serialize a generated schema"
)]
fn generate<T: JsonSchema>() -> Value {
    serde_json::to_value(
        SchemaSettings::draft2020_12()
            .into_generator()
            .into_root_schema_for::<T>(),
    )
    .expect("schema serializes")
}

macro_rules! contracts {
        ($($ty:ty => ($name:literal, $id:literal)),+ $(,)?) => {
            vec![$(Contract { name: $name, id: $id, generate: generate::<$ty> }),+]
        };
    }

fn common_registry() -> Vec<Contract> {
    contracts! {
        Accession => ("accession", "biodata.accession/1"),
        Chromosome => ("chromosome", "biodata.chromosome/1"),
        Base => ("base", "biodata.base/1"),
        Sequence => ("sequence", "biodata.sequence/1"),
        SequenceLocation => ("sequence-location", "biodata.sequence-location/1"),
        Allele => ("allele", "biodata.allele/1"),
        HgvsText => ("hgvs-text", "biodata.hgvs-text/1"),
        Expression => ("expression", "biodata.expression/1"),
        ExpressionSyntax => ("expression-syntax", "biodata.expression-syntax/1"),
        Representation => ("representation", "biodata.representation/1"),
        VariantId => ("variant-id", "biodata.variant-id/1"),
        Variant => ("variant", "biodata.variant/1"),
    }
}

fn registry() -> Vec<Contract> {
    let mut contracts = contracts! {
