use serde_json::{Value, json};
pub(super) fn exact(a: &[Value]) -> bool {
    let ids = a
        .iter()
        .take(11)
        .map(|v| v.pointer("/args/2"))
        .collect::<Vec<_>>();
    let expected = [
        "P200010",
        "NDA210496",
        "BLA125084",
        "P200010",
        "NDA210497",
        "P200010",
        "NDA210498",
        "P200010",
        "NDA210499",
        "P200010",
        "NDA210500",
    ];
    a.iter()
        .take(11)
        .all(|v| v.get("kind") == Some(&json!("pivot")))
        && ids
            == expected
                .iter()
                .map(|id| json!(id))
                .collect::<Vec<_>>()
                .iter()
                .map(Some)
                .collect::<Vec<_>>()
        && [0, 3, 5, 7, 9]
            .into_iter()
            .zip(["S026", "S027", "S028", "S029", "S030"])
            .all(|(i, id)| a.get(i).and_then(|v| v.pointer("/args/6")) == Some(&json!(id)))
}
