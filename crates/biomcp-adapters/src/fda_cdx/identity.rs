use biodata::{DiagnosticCell, DiagnosticDeviceId, DiagnosticDrugApplication, FdaDrugApplication};

fn parameter<'a>(href: &'a str, keys: &[&str]) -> Option<&'a str> {
    if !href.starts_with("https://www.accessdata.fda.gov/") { return None; }
    href.split_once('?')?.1.split('&').find_map(|pair| {
        let (key, value) = pair.split_once('=')?;
        keys.iter().any(|v|key.eq_ignore_ascii_case(v)).then_some(value)
    })
}
fn tokens(text: &str) -> impl Iterator<Item=&str> {
    text.split(|c:char| !c.is_ascii_alphanumeric() && c!='/')
        .filter(|v|!v.is_empty())
}
fn applications(text: &str) -> Vec<FdaDrugApplication> {
    let mut result = Vec::new();
    let mut words = tokens(text).peekable();
    while let Some(word) = words.next() {
        let value = if matches!(word, "NDA"|"BLA") {
            words.peek().map(|v|format!("{word}{v}"))
        } else { Some(word.to_owned()) };
        if let Some(id) = value.and_then(|v|v.parse::<FdaDrugApplication>().ok())
            && !result.contains(&id) { result.push(id); }
    }
    result
}
pub(super) fn drugs(cell: &DiagnosticCell) -> (Box<[DiagnosticDrugApplication]>, bool) {
    let text = applications(&cell.text);
    let mut result = Vec::new();
    let mut conflict = false;
    for link in &cell.links {
        let Some(number) = parameter(&link.href, &["ApplNo", "varApplNo"]) else { continue; };
        let matching: Vec<_> = text.iter().filter(|id| id.as_str().get(3..)==Some(number)).collect();
        if let [id] = matching.as_slice() {
            let label = link.label.as_deref().map(applications).unwrap_or_default();
            if !label.is_empty() && (label.len()!=1 || !label.contains(id)) { conflict = true; continue; }
            if !result.iter().any(|v: &DiagnosticDrugApplication|&v.id==*id) {
                result.push(DiagnosticDrugApplication { id:(*id).clone(), url:link.href.clone() });
            }
        } else { conflict = true; }
    }
    if conflict { result.clear(); }
    (result.into(), conflict)
}
fn linked_device(value: &str) -> Option<DiagnosticDeviceId> {
    let value = value.to_ascii_uppercase();
    if let Ok(id) = value.parse() { return Some(id); }
    let (base, sub) = value.split_at_checked(7)?;
    format!("{base}/{sub}").parse().ok()
}
pub(super) fn devices(cell: &DiagnosticCell) -> (Box<[DiagnosticDeviceId]>, bool) {
    let text: Vec<DiagnosticDeviceId> = tokens(&cell.text).filter_map(|v|v.parse().ok()).collect();
    let mut result = Vec::new();
    let mut conflict = false;
    for link in &cell.links {
        let Some(value) = parameter(&link.href, &["ID"]) else { continue; };
        let Some(id) = linked_device(value) else { continue; };
        let labels: Vec<DiagnosticDeviceId> = link.label.as_deref().map(tokens).into_iter().flatten().filter_map(|v|v.parse().ok()).collect();
        if !text.contains(&id) || (!labels.is_empty() && !labels.contains(&id)) { conflict = true; }
        else if !result.contains(&id) { result.push(id); }
    }
    if conflict { result.clear(); }
    (result.into(), conflict)
}
