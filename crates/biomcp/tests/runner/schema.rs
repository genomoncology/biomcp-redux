use jsonschema::{Retrieve, Uri, Validator};
use serde_json::Value;
use std::{
    collections::BTreeMap,
    error::Error,
    sync::{Arc, OnceLock},
};

pub(super) const ARTIFACTS: &[(&str, &str)] = include!(concat!(env!("OUT_DIR"), "/schemas.rs"));
// Only a resolution anchor. The custom retriever never performs external I/O.
const BASE: &str = "https://schema.invalid/";
const DIAGNOSTIC_CHARS: usize = 160;
static REGISTRY: OnceLock<Result<Registry, String>> = OnceLock::new();

pub(super) fn validate(value: &Value, args: &[String], success: bool) -> Result<(), String> {
    REGISTRY
        .get_or_init(|| {
            let artifacts = ARTIFACTS
                .iter()
                .map(|(id, text)| {
                    serde_json::from_str(text)
                        .map(|value| ((*id).to_owned(), value))
                        .map_err(|_| format!("schema validation: invalid artifact {}", bounded(id)))
                })
                .collect::<Result<_, _>>()?;
            Registry::new(artifacts)
        })
        .as_ref()
        .map_err(Clone::clone)?
        .validate(value, args, success)
}

pub(super) struct Registry {
    artifacts: BTreeMap<String, Value>,
    validators: BTreeMap<String, Validator>,
}

impl Registry {
    pub(super) fn new(artifacts: BTreeMap<String, Value>) -> Result<Self, String> {
        let retriever = Memory(Arc::new(
            artifacts
                .iter()
                .map(|(id, value)| (format!("{BASE}{id}"), value.clone()))
                .collect(),
        ));
        let options = jsonschema::options()
            .with_base_uri(BASE)
            .with_retriever(retriever);
        let validators = artifacts
            .iter()
            .map(|(id, value)| {
                if value.get("$id").and_then(Value::as_str) != Some(id) {
                    return Err(format!(
                        "schema validation: artifact identity mismatch {}",
                        bounded(id)
                    ));
                }
                options
                    .build(value)
                    .map(|validator| (id.clone(), validator))
                    .map_err(|_| {
                        format!("schema validation: cannot compile artifact {}", bounded(id))
                    })
            })
            .collect::<Result<_, _>>()?;
        Ok(Self {
            artifacts,
            validators,
        })
    }

    pub(super) fn validate(
        &self,
        value: &Value,
        args: &[String],
        success: bool,
    ) -> Result<(), String> {
        if let [command, id] = args {
            if success && command == "schema" {
                if let Some(artifact) = self.artifacts.get(id) {
                    return if value == artifact {
                        Ok(())
                    } else {
                        Err(format!(
                            "schema validation: raw artifact differs from requested {}",
                            bounded(id)
                        ))
                    };
                }
            }
        }
        let id = value
            .get("schema")
            .and_then(Value::as_str)
            .ok_or("schema validation: result requires a string schema identity")?;
        let validator = self
            .validators
            .get(id)
            .ok_or_else(|| format!("schema validation: unknown identity {}", bounded(id)))?;
        if let Some(error) = validator.iter_errors(value).next() {
            return Err(format!(
                "schema validation: {} at {} violates {} ({})",
                bounded(id),
                bounded(&error.instance_path().to_string()),
                bounded(&error.schema_path().to_string()),
                error.kind().keyword()
            ));
        }
        Ok(())
    }
}

#[derive(Clone)]
struct Memory(Arc<BTreeMap<String, Value>>);

impl Retrieve for Memory {
    fn retrieve(&self, uri: &Uri<String>) -> Result<Value, Box<dyn Error + Send + Sync>> {
        self.0
            .get(uri.as_str())
            .cloned()
            .ok_or_else(|| "unregistered schema reference; external retrieval disabled".into())
    }
}

fn bounded(text: &str) -> String {
    let mut chars = text.chars();
    let prefix: String = chars.by_ref().take(DIAGNOSTIC_CHARS).collect();
    if chars.next().is_some() {
        format!("{prefix}…")
    } else {
        prefix
    }
}
