use super::*;
pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test("cell-line-chembl/exact-native-assay-total", || {
        assay().map_err(Into::into)
    })]
}
fn fixture(name: &str) -> Result<String, String> {
    fs::read_to_string(super::super::launch::fixtures().join("fixtures/cell-line/chembl").join(format!("{name}.json")))
        .map_err(|e| e.to_string())
}
fn assay() -> Result<(), String> {
    let stub = Stub::for_source("www.ebi.ac.uk", "application/json", |request| {
        let target = request.split_whitespace().nth(1).ok_or("target")?;
        if canonical(target) == canonical("/chembl/api/data/assay.json?cell_chembl_id=CHEMBL3706573&limit=1&offset=0") {
            fixture("assays")
        } else if target == "/chembl/api/data/status.json" {
            fixture("status")
        } else { Err(format!("Unproved request: {target}")) }
    })?;
    let env = setup(&stub)?;
    let value = invoke(&env, &["cell-line", "assay-count", "CHEMBL3706573"], 0)?;
    path(&value, "/result/assay_count", json!(1408))?;
    path(&value, "/result/id", json!("CHEMBL3706573"))?;
    stub.finish()?;
    Ok(())
}
