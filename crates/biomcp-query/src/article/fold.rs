use biodata::{ArticleItem, ArticleSource};
/// Take turns over native rows, preserving ranks before duplicate suppression.
pub(super) fn fold(legs: Vec<(ArticleSource, Vec<ArticleItem>)>, disclose: bool) -> Vec<ArticleItem> {
    let mut items: Vec<ArticleItem> = Vec::new();
    let depth = legs.iter().map(|(_,rows)|rows.len()).max().unwrap_or(0);
    for rank in 0..depth {
        for (source, rows) in &legs {
            let Some(row) = rows.get(rank) else { continue; };
            if let Some(existing) = items.iter_mut().find(|item|item.key == row.key) {
                if disclose { existing.found_by.entry(*source).or_insert((rank+1) as u32); }
            } else {
                let mut row = row.clone();
                if disclose { row.found_by.insert(*source, (rank+1) as u32); }
                items.push(row);
            }
        }
    }
    items
}
