use super::super::response_cases::checked_case;
use super::Check;
use serde_json::{Value, json};

fn run(change: impl Fn(&mut Value), expect: Value) -> Check {
    checked(change, expect).map(|_| ())
}

fn checked(
    change: impl Fn(&mut Value),
    expect: Value,
) -> Result<Value, Box<dyn std::error::Error>> {
    checked_case(
        "gene/main",
        &["get", "gene", "BRAF"],
        |source, record| {
            if source != "mygene" {
                return;
            }
            let Some(body) = record.pointer_mut("/exchanges/0/body/value") else {
                return;
            };
            if body
                .as_array()
                .is_some_and(|rows| rows.iter().any(|v| v["query"] == "673"))
            {
                change(body);
            }
        },
        expect,
        0,
    )
}

pub(super) fn check() -> Check {
    strand()?;
    positions()?;
    multicontig()?;
    identities()?;
    summaries()
}

fn strand() -> Check {
    run(
        |_| {},
        json!({"/result/items/0/answer/gene/hgnc_id": "HGNC:1097",
        "/result/items/0/answer/gene/spans": {"count": 2}}),
    )?;
    eprintln!("GREEN retained MyGene interval before strand mutation");
    let answer = checked(
        |body| body[0]["genomic_pos"]["strand"] = json!(0),
        json!({"/result/items/0/answer/gene/hgnc_id": "HGNC:1097",
            "/result/items/0/answer/gene/spans": {"count": 2},
            "/problems": {"count": 1}, "/problems/*/field": ["genomic_pos.strand"]}),
    )?;
    if answer
        .pointer("/result/items/0/answer/gene/spans/0/strand")
        .is_some()
    {
        return Err("bad MyGene strand survived in the retained interval".into());
    }
    Ok(())
}

fn positions() -> Check {
    for position in [
        json!({"chr":"7","start":0,"end":20}),
        json!({"chr":"7","start":20,"end":10}),
        json!({"chr":"7","start":10,"end":20,"strand":"bad"}),
        json!({"chr":"7","start":"bad","end":20}),
        json!({"chr":"","start":10,"end":20}),
        json!(42),
    ] {
        run(
            |body| body[0]["genomic_pos"] = position.clone(),
            json!({
            "/result/items/0/answer/gene/spans": {"count": 1},
            "/problems/*/field": {"includes": "genomic_pos"}}),
        )?;
    }
    for position in [
        json!({"chr":"7","start":20,"end":20}),
        json!({"chr":"7","start":1,"end":2,"strand":1}),
        json!({"chr":"7","start":10,"end":20,"strand":null}),
    ] {
        let answer = checked(
            |body| body[0]["genomic_pos"] = position.clone(),
            json!({
            "/result/items/0/answer/gene/spans": {"count": 2},
            "/result/items/0/answer/gene/spans/0/span/start": position["start"].as_u64().ok_or("start")? - 1,
            "/result/items/0/answer/gene/spans/0/span/end": position["end"]}),
        )?;
        if position["strand"].is_null()
            && answer
                .pointer("/result/items/0/answer/gene/spans/0/strand")
                .is_some()
        {
            return Err("unstated source strand produced a strand".into());
        }
    }
    run(
        |body| {
            body[0]["genomic_pos"] = json!([
        {"chr":"7","start":10,"end":20,"strand":1}, 42,
        {"chr":"7","start":30,"end":40,"strand":-1}])
        },
        json!({
            "/result/items/0/answer/gene/spans": {"count": 3},
            "/result/items/0/answer/gene/spans/0/span/start": 9,
            "/result/items/0/answer/gene/spans/1/span/start": 29,
            "/problems/*/field": ["genomic_pos"]}),
    )
}

fn multicontig() -> Check {
    run(
        |body| {
            body[0]["genomic_pos"] = json!([
        {"chr":"X","start":10,"end":20}, {"chr":"Y","start":30,"end":40}])
        },
        json!({"/result/items/0/answer/gene/spans/*/chromosome": ["X", "Y", "7"]}),
    )
}

fn identities() -> Check {
    let changes: [fn(&mut Value); 7] = [
        |body| {
            let hit = body[0].clone();
            body.as_array_mut().unwrap().push(hit);
        },
        |body| body[0]["entrezgene"] = json!("1438"),
        |body| {
            body[0].as_object_mut().unwrap().remove("entrezgene");
        },
        |body| body[0]["query"] = json!("12345"),
        |body| body[0]["query"] = json!("invalid"),
        |body| body[0]["HGNC"] = json!("2435"),
        |body| {
            body.as_array_mut().unwrap().clear();
        },
    ];
    for change in changes {
        run(
            change,
            json!({"/result/items/0/answer/gene/hgnc_id": "HGNC:1097",

            "/result/items/0/status": "partial"}),
        )?;
    }
    for malformed in [false, true] {
        run(
            |body| {
                body[0].as_object_mut().unwrap().remove("HGNC");
                if malformed {
                    body[0]["HGNC"] = json!("not-an-id");
                }
            },
            json!({"/result/items/0/status": "partial", "/problems/*/field": {"includes": "hgnc_id"}}),
        )?;
    }
    for mode in 0..3 {
        run(
            |body| match mode {
                0 => {
                    body[0].as_object_mut().unwrap().remove("query");
                }
                1 => body[0]["query"] = json!(""),
                _ => *body = json!({"error":"synthetic invalid input"}),
            },
            json!({"/sources/1/reason": "bad-reply", "/result/items/0/status": "partial"}),
        )?;
    }
    Ok(())
}

fn summaries() -> Check {
    for text in [
        "Whole text without a marker",
        "Whole text [provided by RefSeq, Autumn 2017].",
        "Whole text [provided by RefSeq, Aug 0000].",
        "Whole text [provided by RefSeq, Aug 2017]. trailing",
    ] {
        run(
            |body| body[0]["summary"] = json!(text),
            json!({
            "/result/items/0/answer/gene/summary": {"equals": {"text": text, "source": "refseq"}}}),
        )?;
    }
    for bad in [json!(42), json!(" ")] {
        let answer = checked(
            |body| body[0]["summary"] = bad.clone(),
            json!({
            "/result/items/0/status": "partial", "/problems/*/field": ["summary"]}),
        )?;
        if answer
            .pointer("/result/items/0/answer/gene/summary")
            .is_some()
        {
            return Err("malformed summary survived in the gene answer".into());
        }
    }
    let answer = checked(
        |body| {
            body[0].as_object_mut().unwrap().remove("summary");
            body[0].as_object_mut().unwrap().remove("genomic_pos");
        },
        json!({"/result/items/0/answer/gene/spans": {"count": 1}}),
    )?;
    if answer
        .pointer("/result/items/0/answer/gene/summary")
        .is_some()
    {
        return Err("missing source summary produced a summary".into());
    }
    Ok(())
}
