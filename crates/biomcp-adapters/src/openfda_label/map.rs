use super::{Record, bad};
use biodata::{Attribution, DrugIdentityLevel, DrugLabelReference, PartialDate, Text};
use biomcp_fetch::{BadReply, RequestError};
use biomcp_types::{DrugCard, DrugHeader, DrugSection, DrugSource};
impl Record {
    pub(super) fn revised(&self) -> Result<PartialDate, BadReply> {
        let date = &self.effective_time;
        if date.len() != 8 || !date.bytes().all(|b| b.is_ascii_digit()) {
            return Err(BadReply);
        }
        format!("{}-{}-{}", &date[..4], &date[4..6], &date[6..])
            .parse()
            .map_err(|_| BadReply)
    }
    fn url(&self) -> Result<Text, RequestError> {
        format!(
            "https://dailymed.nlm.nih.gov/dailymed/drugInfo.cfm?setid={}",
            self.set_id
        )
        .parse()
        .map_err(|_| bad())
    }
    /// Identity comes from the SPL endpoint; no name or field-shape identity join occurs.
    pub fn header(&self) -> Result<DrugHeader, RequestError> {
        self.check().map_err(|_| bad())?;
        Ok(DrugHeader {
            matched_name: None,
            id: self.set_id.clone(),
            source: DrugSource::OpenfdaLabel,
            version: Some(self.version.clone()),
            name: Some(
                self.openfda
                    .brand_name
                    .as_deref()
                    .unwrap_or_default()
                    .first()
                    .or_else(|| {
                        self.openfda
                            .generic_name
                            .as_deref()
                            .unwrap_or_default()
                            .first()
                    })
                    .cloned()
                    .map(Into::into)
                    .ok_or_else(bad)?,
            ),
            identity_level: DrugIdentityLevel::SplSet,
            r#type: None,
            url: Some(self.url()?),
        })
    }
    /// Version and revision ID remain source-scoped and separate from the set ID.
    pub fn reference(&self) -> Result<DrugLabelReference, RequestError> {
        self.check().map_err(|_| bad())?;
        let url = self.url()?;
        Ok(DrugLabelReference {
            id: self.set_id.clone(),
            version: Some(self.version.clone()),
            revised: Some(self.revised().map_err(|_| bad())?),
            url: Some(url.clone()),
            attribution: Attribution {
                source: "openfda-label".parse().map_err(|_| bad())?,
                record: Some(self.id.to_string().parse().map_err(|_| bad())?),
                version: Some(self.version.clone()),
                release: None,
                submitter: None,
                evaluated: None,
                url: Some(url),
            },
        })
    }
    /// An identity-only card does not copy any clinical text or authorization facts.
    pub fn card(&self, retrieved: Option<PartialDate>) -> Result<DrugCard, RequestError> {
        let alternate_names = self
            .openfda
            .brand_name
            .iter()
            .flatten()
            .chain(self.openfda.generic_name.iter().flatten())
            .cloned()
            .map(Into::into)
            .collect();
        Ok(DrugCard {
            header: self.header()?,
            alternate_names,
            cross_references: Box::default(),
            parents: Box::default(),
            products: Box::default(),
            formulations: Box::default(),
            version: Some(self.version.clone()),
            updated: Some(self.revised().map_err(|_| bad())?),
            retrieved,
            available_sections: if super::HEADINGS
                .iter()
                .any(|name| self.parts(name).is_some())
            {
                vec![DrugSection::Label].into()
            } else {
                Box::default()
            },
            next: Box::default(),
        })
    }
}
