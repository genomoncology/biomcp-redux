//! Small counterexamples exercise the same inventory used by the repository check.
use super::{Check, Value, case_data::Step, tally::Inventory};
use serde_json::json;
use std::collections::BTreeSet;

fn help() -> Value {
    json!({"result":{"commands":[
        {"command":[],"arguments":[{"name":"--format","value_type":"string"}]},
        {"command":["get","trial"],"arguments":[
            {"name":"--limit","value_type":"string","aliases":["-n"]}
        ],"sections":[{"word":"eligibility"}]}
    ]}})
}
fn step(args: &[&str]) -> Result<Step, Box<dyn std::error::Error>> {
    Ok(serde_json::from_value(
        json!({"run":args,"expect":{"/result/id":"NCT1"}}),
    )?)
}
fn covered() -> Result<Inventory, Box<dyn std::error::Error>> {
    let mut inventory = Inventory::new(&help(), &[])?;
    inventory.step(&step(&[
        "get",
        "trial",
        "NCT1",
        "eligibility",
        "--limit",
        "1",
    ])?);
    inventory.step(&step(&["--format=md", "get", "trial", "NCT1", "-n", "2"])?);
    inventory.step(&step(&["get", "trial", "eligibility"])?);
    Ok(inventory)
}
fn pending(inventory: &Inventory) -> BTreeSet<String> {
    inventory
        .calls
        .iter()
        .filter(|(_, count)| **count == 0)
        .map(|(name, _)| name.clone())
        .collect()
}
#[test]
fn three_hand_counts_match_and_an_identity_never_counts_as_a_section() -> Check {
    let inventory = covered()?;
    assert_eq!(inventory.calls["command get trial"], 3);
    assert_eq!(inventory.calls["option get trial --limit"], 2);
    assert_eq!(inventory.calls["section get trial eligibility"], 1);
    assert_eq!(inventory.calls["global --format"], 1);
    inventory.check(&pending(&inventory))
}
#[test]
fn a_new_option_without_a_case_fails() -> Check {
    let original = covered()?;
    let mut help = help();
    help.pointer_mut("/result/commands/1/arguments")
        .and_then(Value::as_array_mut)
        .ok_or("missing proof arguments")?
        .push(json!({"name":"--planted","value_type":"boolean"}));
    let mut changed = Inventory::new(&help, &[])?;
    changed.step(&step(&[
        "get",
        "trial",
        "NCT1",
        "eligibility",
        "--limit",
        "1",
        "--format=json",
    ])?);
    let failure = changed
        .check(&pending(&original))
        .expect_err("uncovered option passed")
        .to_string();
    assert!(failure.contains("option get trial --planted"), "{failure}");
    Ok(())
}
#[test]
fn removing_the_only_section_call_fails() -> Check {
    let original = covered()?;
    let mut changed = Inventory::new(&help(), &[])?;
    changed.step(&step(&[
        "get",
        "trial",
        "NCT1",
        "--limit",
        "1",
        "--format=json",
    ])?);
    let failure = changed
        .check(&pending(&original))
        .expect_err("uncovered section passed")
        .to_string();
    assert!(
        failure.contains("section get trial eligibility"),
        "{failure}"
    );
    Ok(())
}
#[test]
fn option_values_cannot_impersonate_options_or_sections() -> Check {
    let mut inventory = Inventory::new(&help(), &[])?;
    inventory.step(&step(&["get", "trial", "NCT1", "--limit", "eligibility"])?);
    assert_eq!(inventory.calls["section get trial eligibility"], 0);
    inventory.step(&step(&["--format=--limit", "get", "trial", "NCT1"])?);
    assert_eq!(inventory.calls["option get trial --limit"], 1);
    Ok(())
}
#[test]
fn cli_commands_never_impersonate_named_mcp_tools() -> Check {
    let tools = [
        json!({"name":"biomcp"}),
        json!({"name":"search"}),
        json!({"name":"fetch"}),
    ];
    let mut inventory = Inventory::new(&help(), &tools)?;
    inventory.step(&step(&["search", "trial", "melanoma"])?);
    assert_eq!(inventory.calls["tool search"], 0);
    let named: Step = serde_json::from_value(
        json!({"mcp_call":{"name":"search","arguments":{"query":"trial melanoma"}},"expect":{"/results":[]}}),
    )?;
    inventory.step(&named);
    assert_eq!(inventory.calls["tool search"], 1);
    assert_eq!(inventory.calls["tool biomcp"], 0);
    let legacy: Step =
        serde_json::from_value(json!({"mcp":["help"],"expect":{"/result/commands":[]}}))?;
    inventory.step(&legacy);
    assert_eq!(inventory.calls["tool biomcp"], 1);
    assert_eq!(inventory.calls["tool fetch"], 0);
    Ok(())
}
