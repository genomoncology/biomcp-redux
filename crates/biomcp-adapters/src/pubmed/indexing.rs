use crate::xml_element::Element;
use biodata::{Text};
use biomcp_types::{Indexing, MeshHeading, MeshQualifier};
use biomcp_types::{Capped};
use biomcp_fetch::BadReply;
pub(super) fn parse(citation: Element<'_>, row: Element<'_>) -> Result<Indexing, BadReply> {
    let mut mesh = Vec::new();
    if let Some(list) = citation.child("MeshHeadingList")? {
        for heading in list.children("MeshHeading")? {
            let descriptor = heading.child("DescriptorName")?.ok_or(BadReply)?;
            let qualifiers = heading
                .children("QualifierName")?
                .into_iter()
                .map(|q| {
                    Ok(MeshQualifier {
                        name: text(q)?,
                        major: major(q)?,
                    })
                })
                .collect::<Result<Vec<_>, BadReply>>()?;
            mesh.push(MeshHeading {
                descriptor: text(descriptor)?,
                major: major(descriptor)?,
                qualifiers: qualifiers.into(),
            });
        }
    }
    let mut keywords = Vec::new();
    for list in citation.children("KeywordList")? {
        for keyword in list.children("Keyword")? {
            keywords.push(text(keyword)?);
        }
    }
    let mut affiliations = Vec::new();
    let mut orcids = Vec::new();
    if let Some(list) = row.child("AuthorList")? {
        for author in list.children("Author")? {
            for info in author.children("AffiliationInfo")? {
                for affiliation in info.children("Affiliation")? {
                    affiliations.push(text(affiliation)?);
                }
            }
            for identifier in author.children("Identifier")? {
                if identifier.attr("Source")?.as_deref() == Some("ORCID") {
                    let id = text(identifier)?;
                    if !orcids.contains(&id) {
                        orcids.push(id);
                    }
                }
            }
        }
    }
    Ok(Indexing {
        mesh: mesh.into(),
        keywords: keywords.into(),
        affiliations: Capped {
            total: affiliations.len() as u64,
            items: affiliations.into_iter().take(10).collect(),
        },
        orcids: orcids.into(),
    })
}
fn text(node: Element<'_>) -> Result<Text, BadReply> {
    Text::new(&node.text()?).map_err(|_| BadReply)
}
fn major(node: Element<'_>) -> Result<bool, BadReply> {
    match node.attr("MajorTopicYN")?.as_deref() {
        Some("Y") => Ok(true),
        Some("N") => Ok(false),
        _ => Err(BadReply),
    }
}
