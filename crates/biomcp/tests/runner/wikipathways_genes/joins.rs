//! Exact xref namespaces resolve through approved identifiers, without aliases or labels.
use super::{Check, Stub, Value, json};
use crate::runner::pathway_budgets as budgets;
const HGNC: &str = "hgnc_id\tsymbol\tname\tlocus_group\tstatus\tentrez_id\tensembl_gene_id\talias_symbol\nHGNC:1\tSYNTHONE\tSynthetic one\tprotein-coding \
    gene\tApproved\t101\tENSG00000000001\tALIASONE\nHGNC:2\tSYNTHTWO\tSynthetic two\tprotein-coding \
    gene\tApproved\t102\tENSG00000000002\t\n";
fn body() -> String {
    let mut xml = String::from(
        "<Pathway xmlns=\"http://pathvisio.org/GPML/2013a\" Name=\"Synthetic pathway\" \
            Data-Source=\"WikiPathways\" Version=\"WP382_r1\" Organism=\"Homo sapiens\">",
    );
    for (n, (namespace, id)) in [
        ("Entrez Gene", "101"),
        ("Ensembl", "ENSG00000000001"),
        ("HGNC", "1"),
        ("HGNC", "HGNC:2"),
        ("HGNC", "SYNTHTWO"),
        ("HGNC", "ALIASONE"),
        ("Ensembl", "ENSG00000000001.1"),
        ("Unmapped", "101"),
    ]
    .into_iter()
    .enumerate()
    {
        xml.push_str(&format!(
            "<DataNode GraphId=\"n{n}\" TextLabel=\"SYNTHONE\" Type=\"GeneProduct\"><Xref \
            Database=\"{namespace}\" ID=\"{id}\"/></DataNode>"
        ));
    }
    xml + "</Pathway>"
}
pub(super) fn run() -> Check {
    let stub = peer()?;
    let env = super::super::variant::source_environment(&stub)?;
    for source in ["wikipathways", "hgnc"] {
        budgets::invoke(
            "wikipathways_genes/joins/run/1",
            &env,
            vec!["sync".into(), source.into()],
            0,
        )?;
    }
    let value = budgets::invoke(
        "wikipathways_genes/joins/run/2",
        &env,
        vec!["pathway".into(), "genes".into(), "WP382".into()],
        0,
    )?;
    let genes = value
        .pointer("/result/items")
        .and_then(Value::as_array)
        .ok_or("members")?
        .iter()
        .map(|row| row.get("gene").cloned())
        .collect::<Vec<_>>();
    if genes
        != [
            Some(json!("HGNC:1")),
            Some(json!("HGNC:1")),
            Some(json!("HGNC:1")),
            Some(json!("HGNC:2")),
            Some(json!("HGNC:2")),
        ]
    {
        return Err(format!("Exact xref joins changed: {value}"));
    }
    let steps = value.get("next").and_then(Value::as_array).ok_or("steps")?;
    if steps
        .iter()
        .filter_map(|s| s.get("kind"))
        .collect::<Vec<_>>()
        != [&json!("pivot"), &json!("pivot"), &json!("page")]
    {
        return Err(format!(
            "Gene pivots were duplicated or reordered: {steps:?}"
        ));
    }
    let last = steps.last().ok_or("page step")?;
    let mut args: Vec<String> =
        serde_json::from_value(last.get("args").ok_or("page args")?.clone())
            .map_err(|e| e.to_string())?;
    args.push("--offline".into());
    let page = budgets::invoke("wikipathways_genes/joins/run/3", &env, args, 0)?;
    if page
        .pointer("/result/items")
        .and_then(Value::as_array)
        .ok_or("remaining nodes")?
        .iter()
        .any(|row| row.get("gene").is_some())
    {
        return Err(format!(
            "Alias, label or version stripping resolved a gene: {page}"
        ));
    }
    if stub.finish()?.len() != 5 {
        return Err("A retained page fetched source data".into());
    }
    Ok(())
}

fn peer() -> Result<Stub, String> {
    Stub::for_sources(
        &["www.wikipathways.org", "storage.googleapis.com"],
        |request| {
            if request.contains(".gpml") {
                "application/octet-stream"
            } else if request.contains(".txt") {
                "text/tab-separated-values"
            } else {
                "application/json"
            }
        },
        |request| {
            if request.contains("findPathwaysByText.json") {
                Ok(super::catalog(true))
            } else if request.contains("getPathwayInfo.json") {
                Ok(super::catalog(false))
            } else if request.contains("hgnc_complete_set.txt") {
                Ok(HGNC.into())
            } else if request.contains("withdrawn.txt") {
                Ok(
                    "HGNC_ID\tSTATUS\tWITHDRAWN_SYMBOL\nHGNC:3\tEntry Withdrawn\tSYNTHWITHDRAWN\n"
                        .into(),
                )
            } else if request.contains(".gpml") {
                Ok(body())
            } else {
                Err(format!("Unexpected source request {request}"))
            }
        },
    )
}
