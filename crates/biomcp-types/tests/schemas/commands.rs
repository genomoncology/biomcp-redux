//! Bind command paths and query schema publishing to the shared command declaration.
use super::*;

macro_rules! query_contracts {
    (rewrites { $($rewrite:ident),+ } $( $(#[$doc:meta])* $variant:ident($query:ident) =>
        ($verb:literal, $entity:literal, $schema_type:ident, $stem:literal, $schema:literal,
         $grammar_type:ident, $grammar_stem:literal, $grammar_schema:literal, $spelling_schema:literal, $order:literal, $via:ident); )+) => {{
        let contracts = contracts! {
            $($schema_type => ($stem, $schema, query),
              $grammar_type => ($grammar_stem, $grammar_schema, query),)+
        };
        let mut seen = std::collections::BTreeSet::new();
        contracts.into_iter().filter(|contract| seen.insert(contract.id)).collect()
    }};
}

pub(super) fn registry() -> Vec<Contract> {
    biomcp_types::command_types!(query_contracts)
}

fn validate(rows: &[CommandSpec], schemas: &std::collections::BTreeSet<String>) -> Result<(), String> {
    for (index, row) in rows.iter().enumerate() {
        if !schemas.contains(row.schema) {
            return Err(format!("command schema has no file: {}", row.schema));
        }
        if let Some(prior) = rows[..index].iter()
            .find(|prior| (prior.verb, prior.entity) == (row.verb, row.entity))
            && (row.via == Rewrite::Direct || row.via != prior.via || row.schema != prior.schema)
        {
            return Err(format!("duplicate path has no shared rewrite: {} {}", row.verb, row.entity));
        }
    }
    Ok(())
}

#[test]
fn every_path_has_one_grammar_or_one_shared_rewrite() -> Result<(), Box<dyn std::error::Error>> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("schemas");
    let schemas = fs::read_dir(root)?.map(|entry| {
        let schema: Value = serde_json::from_str(&fs::read_to_string(entry?.path())?)?;
        Ok(schema["$id"].as_str().unwrap_or_default().to_owned())
    }).collect::<Result<std::collections::BTreeSet<_>, Box<dyn std::error::Error>>>()?;
    validate(COMMANDS, &schemas)?;
    let mut duplicate = COMMANDS.to_vec();
    let mut row = duplicate[0];
    row.via = Rewrite::Direct;
    duplicate.push(row);
    assert!(validate(&duplicate, &schemas).is_err(), "an unshared duplicate path must fail");
    let missing = CommandSpec { schema: "biomcp.missing-command/1", ..COMMANDS[0] };
    assert!(validate(&[missing], &schemas).is_err(), "a schema without a file must fail");
    Ok(())
}
