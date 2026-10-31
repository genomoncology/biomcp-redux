use super::{Record, bad};
use biodata::{Attribution, DrugIdentityLevel, DrugLabelReference, PartialDate, Text};
use biomcp_fetch::RequestError;
use biomcp_types::{DrugCard, DrugHeader, DrugSection, DrugSource};
impl Record {
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
            version: self.version.clone(),
            name: self
                .openfda
                .brand_name
                .iter()
                .flatten()
                .next()
                .or_else(|| self.openfda.generic_name.iter().flatten().next())
                .cloned()
                .map(Into::into),
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
            version: self.version.clone(),
            revised: self.effective_time,
            url: Some(url.clone()),
            attribution: Attribution {
                source: "openfda-label".parse().map_err(|_| bad())?,
                record: Some(self.id.to_string().parse().map_err(|_| bad())?),
                version: self.version.clone(),
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
            version: self.version.clone(),
            updated: self.effective_time,
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
