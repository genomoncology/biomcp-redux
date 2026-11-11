use biomcp_types::{ArticleItem, ArticleSource};
/// Take turns over native rows, preserving ranks before duplicate suppression.
pub(super) fn fold(
    legs: Vec<(ArticleSource, Vec<ArticleItem>)>,
    disclose: bool,
) -> Vec<ArticleItem> {
    let mut items: Vec<ArticleItem> = Vec::new();
    let depth = legs.iter().map(|(_, rows)| rows.len()).max().unwrap_or(0);
    for rank in 0..depth {
        for (source, rows) in &legs {
            let Some(row) = rows.get(rank) else {
                continue;
            };
            let native_rank = row
                .found_by
                .get(source)
                .copied()
                .unwrap_or((rank + 1) as u32);
            if let Some(existing) = items
                .iter_mut()
                .find(|item| row.key.is_some() && item.key == row.key)
            {
                if existing.sentence.is_none() {
                    existing.sentence = row.sentence.clone();
                    // The reason, and whether the pool is whole, travel with the text.
                    if existing.text_withheld.is_none() {
                        existing.text_withheld = row.text_withheld.clone();
                    }
                }
                // A row from a source that states no access flag takes one stated by another source.
                if existing.open_access.is_none() {
                    existing.open_access = row.open_access;
                }
                if disclose {
                    existing.found_by.entry(*source).or_insert(native_rank);
                }
            } else {
                let mut row = row.clone();
                if disclose {
                    row.found_by.insert(*source, native_rank);
                } else {
                    row.found_by.clear();
                }
                items.push(row);
            }
        }
    }
    items
}

#[cfg(test)]
mod tests {
    use super::fold;
    use biodata::Pmid;
    use biomcp_types::{ArticleItem, ArticleKey, ArticleSource};

    fn row(id: u32, open_access: Option<bool>) -> Result<ArticleItem, String> {
        let id: Pmid = id.to_string().parse().map_err(|_| "synthetic PMID")?;
        Ok(ArticleItem {
            text_withheld: None,
            context_license: None,
            key: Some(ArticleKey::Pubmed(id)),
            pmid: Some(id),
            found_by: Default::default(),
            cited_by: None,
            open_access,
            sentence: None,
            intents: None,
            influential: None,
            contexts: None,
            header: None,
        })
    }

    #[test]
    fn a_merged_row_keeps_the_europe_pmc_access_flag_and_others_state_none() -> Result<(), String> {
        let legs = vec![
            (
                ArticleSource::Pubmed,
                vec![row(1111, None)?, row(2222, None)?],
            ),
            (
                ArticleSource::Europepmc,
                vec![row(1111, Some(true))?, row(3333, Some(false))?],
            ),
        ];
        let flags: Vec<_> = fold(legs, true)
            .iter()
            .map(|item| (item.pmid.map(|id| id.to_string()), item.open_access))
            .collect();
        let expected = [
            (Some("1111".to_string()), Some(true)),
            (Some("2222".to_string()), None),
            (Some("3333".to_string()), Some(false)),
        ];
        assert_eq!(
            flags, expected,
            "only Europe PMC states the flag, and a merge keeps it"
        );
        Ok(())
    }
}
