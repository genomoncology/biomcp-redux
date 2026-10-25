//! Entirely authored synthetic source rows; no publisher data or guidance.
use serde_json::{Value, json};
pub(super) fn scenario(name: &str) -> (Vec<String>, Value) {
    let mut args = vec![
        if name == "pairs" { "search" } else { "get" },
        "pgx",
        "--gene",
        "DPYD",
        "--source",
        "cpic",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect::<Vec<_>>();
    if name == "pairs" {
        args.extend([
            "--limit".into(),
            "1".into(),
            "--cpic-level".into(),
            "B/C".into(),
        ]);
    } else if matches!(name, "view" | "table" | "guidelines") {
        args.extend(["--section-limit".into(), "1".into()]);
    }
    let body = match name {
        "pairs" => {
            json!([{"pairid":1,"drugid":"RxNorm:10324","drugname":"Synthetic drug","genesymbol":"DPYD","guidelinename":null,"guidelineurl":null,"cpiclevel":"B/C","clinpgxlevel":"Synthetic level","pgxtesting":null,"pmids":["12345678"],"usedforrecommendation":"No","provisional":false}])
        }
        "card" => {
            json!([{"symbol":"DPYD","hgncid":"HGNC:2865","ncbiid":"1806","ensemblid":"ENSG00000188641","clinpgxid":"PA145","lookupmethod":"SYNTHETIC","version":7}])
        }
        "view" | "table" => {
            args.push("recommendations".into());
            let mut row = json!({"lookupkey":{"DPYD":"n/a","TPMT":"Synthetic other gene"},"implications":{"DPYD":"Synthetic context","TPMT":"Independent synthetic context"},"phenotypes":{"DPYD":"Uncertain synthetic result","TPMT":"Synthetic label"},"activityscore":{},"drugrecommendation":"Synthetic source text without clinical advice.","classification":"Synthetic classification","population":"Synthetic population","comments":"Keep this synthetic qualification."});
            if name == "view" {
                row["recommendationid"] = json!(1);
                row["drugname"] = json!("Synthetic drug");
                row["guidelinename"] = json!("Synthetic guideline");
                row["guidelineurl"] = json!("https://example.org/synthetic");
            } else {
                args.extend(["--drug-id".into(), "RxNorm:10324".into()]);
                row["id"] = json!(1);
                row["drugid"] = json!("RxNorm:10324");
                row["guidelineid"] = json!(2);
                row["allelestatus"] = json!({"DPYD":"Synthetic status"});
                row["version"] = json!(3);
                row["dosinginformation"] = json!(false);
                row["alternatedrugavailable"] = json!(true);
                row["otherprescribingguidance"] = json!(false);
            }
            json!([row])
        }
        "guidelines" => {
            args.push("guidelines".into());
            json!([{"guideline_name":"Synthetic guideline","guideline_url":"https://example.org/synthetic","drugs":["Synthetic A","Synthetic B"],"genes":[{"symbol":"DPYD","url":"https://example.org/gene/DPYD"},{"symbol":"TPMT","url":"https://example.org/gene/TPMT"}]}])
        }
        _ => {
            args.extend(["diplotype".into(), "--diplotype".into(), "*1/*2A".into()]);
            if name == "empty" {
                json!([])
            } else {
                json!([{"genesymbol":"DPYD","diplotype":"*1/*2A","function1":"Synthetic uncertainty","function2":"n/a","activityvalue1":"n/a","activityvalue2":"n/a","totalactivityscore":"n/a","description":"Synthetic unresolved mapping","diplotypekey":{"DPYD":{"*1":1,"*2A":1}},"generesult":"Synthetic result","ehrpriority":"Synthetic priority","consultationtext":"Synthetic source consultation","lookupkey":{"DPYD":"n/a"}}])
            }
        }
    };
    (args, body)
}
