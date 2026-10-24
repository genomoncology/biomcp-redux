//! Public schema contract tests; file I/O stays outside the pure library.
#![cfg(feature = "schema")]
#[path = "schemas/compatibility.rs"]
mod compatibility;
#[path = "schemas/compatibility_cases.rs"]
mod compatibility_cases;
#[path = "schemas/values.rs"]
mod values;

use biodata::{ErrorEnvelope, HgncId, PartialDate, Position1, Problem, ProblemCode, Reason, Source, Span0, Text};
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

fn registry() -> [Contract; 10] {
    [
        Contract {
            name: "error",
            id: "biomcp.error/1",
            generate: generate::<ErrorEnvelope>,
        },
        Contract {
            name: "hgnc-id",
            id: "biodata.hgnc-id/1",
            generate: generate::<HgncId>,
        },
        Contract {
            name: "partial-date",
            id: "biodata.partial-date/1",
            generate: generate::<PartialDate>,
        },
        Contract {
            name: "position1",
            id: "biodata.position1/1",
            generate: generate::<Position1>,
        },
        Contract {
            name: "span0",
            id: "biodata.span0/1",
            generate: generate::<Span0>,
        },
        Contract {
            name: "text",
            id: "biodata.text/1",
            generate: generate::<Text>,
        },
        Contract {
            name: "source",
            id: "biodata.source/1",
            generate: generate::<Source>,
        },
        Contract {
            name: "reason",
            id: "biodata.reason/1",
            generate: generate::<Reason>,
        },
        Contract {
            name: "problem-code",
            id: "biodata.problem-code/1",
            generate: generate::<ProblemCode>,
        },
        Contract {
            name: "problem",
            id: "biodata.problem/1",
            generate: generate::<Problem>,
        },
    ]
}

#[test]
#[expect(
    clippy::disallowed_methods,
    reason = "Schema contract tests own file and environment I/O outside the pure library"
)]
fn checked_in_schemas_match_types() {
    let update = std::env::var_os("UPDATE_SCHEMAS").is_some_and(|value| value == "1");
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("schemas");
    // Collect and check the whole registry before writing any contract.
    let pending: Vec<_> = registry()
        .into_iter()
        .map(|contract| {
            let mut new = (contract.generate)();
            new["$id"] = contract.id.into();
            let path = root.join(format!("{}.json", contract.name));
            if path.exists() {
                let old: Value =
                    serde_json::from_str(&fs::read_to_string(&path).expect("read contract"))
                        .expect("parse contract");
                compatibility::check(&old, &new)
                    .unwrap_or_else(|error| panic!("{}: {error}", contract.name));
            } else {
                assert!(update, "missing {}", path.display());
            }
            let bytes = format!(
                "{}\n",
                serde_json::to_string_pretty(&new).expect("serialize schema")
            );
            (path, bytes)
        })
        .collect();
    if update {
        fs::create_dir_all(&root).expect("create schema directory");
    }
    for (path, bytes) in pending {
        if update {
            fs::write(&path, bytes).expect("write contract");
        } else {
            assert_eq!(
                fs::read_to_string(&path).expect("read contract"),
                bytes,
                "schema drift at {}; review compatibility before UPDATE_SCHEMAS=1",
                path.display()
            );
        }
    }
}
