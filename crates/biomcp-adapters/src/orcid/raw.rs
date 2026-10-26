use biodata::{AuthorCard, AuthorHeader, AuthorId, OrcidId, Text};
use biomcp_fetch::{BadReply, Reply};
use serde::Deserialize;
#[derive(Deserialize)]
pub(super) struct Person {
    pub path: String,
    pub name: Option<Name>,
}
#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(super) struct Name {
    pub path: String,
    visibility: Option<String>,
    credit_name: Option<Wrapped>,
    given_names: Option<Wrapped>,
    family_name: Option<Wrapped>,
}
#[derive(Deserialize)]
pub(super) struct Wrapped { pub value: Text }
impl Person {
    pub fn card(self, id: &OrcidId) -> Result<Option<AuthorCard>, BadReply> {
        let Some(name) = self.name.filter(|n| n.visibility.as_deref() == Some("public")) else { return Ok(None) };
        let display = match name.credit_name {
            Some(n) => n.value,
            None => {
                let parts: Vec<_> = [name.given_names, name.family_name].into_iter().flatten().map(|n| n.value).collect();
                if parts.is_empty() { return Ok(None); }
                parts.iter().map(Text::as_str).collect::<Vec<_>>().join(" ").parse().map_err(|_| BadReply)?
            }
        };
        Ok(Some(AuthorCard {
            header: AuthorHeader { id: AuthorId::Orcid(id.clone()), name: display, affiliations: None, counts: None },
            url: Some(format!("https://orcid.org/{id}").parse().map_err(|_| BadReply)?),
        }))
    }
}
impl Reply for Person { fn check(&self) -> Result<(), BadReply> { Ok(()) } }
