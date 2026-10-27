use crate::xml_element::Element;

use biomcp_fetch::BadReply;
use biomcp_types::Capped;
use biomcp_types::{Indexing, MeshHeading, MeshQualifier};
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
                        name: q.text().and_then(|s| crate::mapped::text(&s))?,
                        major: major(q)?,
                    })
                })
                .collect::<Result<Vec<_>, BadReply>>()?;
            mesh.push(MeshHeading {
                descriptor: descriptor.text().and_then(|s| crate::mapped::text(&s))?,
                major: major(descriptor)?,
                qualifiers: qualifiers.into(),
            });
        }
    }
    let mut keywords = Vec::new();
    for list in citation.children("KeywordList")? {
        for keyword in list.children("Keyword")? {
            keywords.push(keyword.text().and_then(|s| crate::mapped::text(&s))?);
        }
    }
    let mut affiliations = Vec::new();
    let mut orcids = Vec::new();
    if let Some(list) = row.child("AuthorList")? {
        for author in list.children("Author")? {
            for info in author.children("AffiliationInfo")? {
                for affiliation in info.children("Affiliation")? {
                    affiliations.push(affiliation.text().and_then(|s| crate::mapped::text(&s))?);
                }
            }
            for identifier in author.children("Identifier")? {
                if identifier.attr("Source")?.as_deref() == Some("ORCID") {
                    let id = identifier.text().and_then(|s| crate::mapped::text(&s))?;
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

fn major(node: Element<'_>) -> Result<bool, BadReply> {
    match node.attr("MajorTopicYN")?.as_deref() {
        Some("Y") => Ok(true),
        Some("N") => Ok(false),
        _ => Err(BadReply),
    }
}
