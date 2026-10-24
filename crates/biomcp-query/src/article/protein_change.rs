//! Reader-only literal matching over the shared checked substitution value.
use biodata::{ProteinChange, Text};
#[derive(Debug)]
pub(super) struct Selection {
    pub(super) forms: Option<[String; 4]>,
    pub(super) needles: Vec<String>,
}
impl Selection {
    pub(super) fn new(find: Option<&Text>, variant: Option<&Text>) -> Result<Self, ()> {
        let forms = variant
            .map(|v| v.as_str().parse::<ProteinChange>().map(|p| p.forms()).map_err(|_| ()))
            .transpose()?;
        let needles = forms.clone().map(|v| v.to_vec()).unwrap_or_else(|| {
            find.map(|v| vec![v.as_str().to_owned()])
                .unwrap_or_default()
        });
        Ok(Self { forms, needles })
    }
}
