//! Feature cell-line-sections: callers retrieve whole native rows independently.
use super::*;
mod expected;
pub(super) fn run(measure: bool) -> Result<(), String> {
    let stub = recorded_stub()?;
    let env = setup(&stub)?;
    cache()?;
    for id in [
        "CVCL_2119",
        "CVCL_0064",
        "CVCL_1844",
        "CVCL_0007",
        "CVCL_0002",
        "CVCL_A794",
        "CVCL_8821",
    ] {
        let source: Value =
            serde_json::from_str(&fixture(&format!("portable-cellosaurus-cell-{id}.json"))?)
                .map_err(|e| e.to_string())?;
        let row = source
            .pointer("/Cellosaurus/cell-line-list/0")
            .ok_or("native card")?;
        for section in ["names", "identity", "xrefs", "variants"] {
            let phase = format!("sections/native/{id}/{section}");
            let value = parity(&phase, &env, &["get", "cell-line", id, section], 0, measure)?;
            expected::page(
                row,
                section,
                &value,
                0,
                if section == "variants" { 3 } else { 5 },
            )?;
        }
        let value = parity(
            &format!("sections/full/{id}/variants"),
            &env,
            &["get", "cell-line", id, "variants", "--section-limit", "25"],
            0,
            measure,
        )?;
        expected::page(row, "variants", &value, 0, 25)?;
    }
    boundaries(&env, measure)?;
    stub.finish()?;
    Ok(())
}
fn cache() -> Result<(), String> {
    let stub = recorded_stub()?;
    let env = setup(&stub)?;
    for section in ["names", "identity", "xrefs", "variants"] {
        let value = invoke(&env, &["get", "cell-line", "CVCL_2119", section], 0)?;
        path(&value, "/status", json!("complete"))?;
    }
    let requests = stub.finish()?;
    if requests
        .iter()
        .filter(|(_, r)| r.contains("/cell-line/CVCL_2119?"))
        .count()
        != 1
    {
        return Err("later section pages repeated the complete request".into());
    }
    Ok(())
}
fn boundaries(env: &Environment, measure: bool) -> Result<(), String> {
    for (section, limit, offset) in [
        ("names", "1", "1"),
        ("variants", "25", "100"),
        ("identity", "1", "1"),
        ("xrefs", "1", "1"),
    ] {
        let phase = format!("sections/bounds/{section}/{limit}/{offset}");
        let value = parity(
            &phase,
            env,
            &[
                "get",
                "cell-line",
                "CVCL_2119",
                section,
                "--section-limit",
                limit,
                "--section-offset",
                offset,
            ],
            0,
            measure,
        )?;
        let source: Value =
            serde_json::from_str(&fixture("portable-cellosaurus-cell-CVCL_2119.json")?)
                .map_err(|e| e.to_string())?;
        expected::page(
            source
                .pointer("/Cellosaurus/cell-line-list/0")
                .ok_or("native")?,
            section,
            &value,
            offset.parse().map_err(|_| "offset")?,
            limit.parse().map_err(|_| "limit")?,
        )?;
    }
    for limit in ["0", "26"] {
        let value = invoke(
            env,
            &[
                "get",
                "cell-line",
                "CVCL_2119",
                "names",
                "--section-limit",
                limit,
            ],
            2,
        )?;
        path(&value, "/status", json!("usage"))?;
    }
    Ok(())
}
