//! Shape check: a PMA summary find keeps every match on long extracted lines (synthetic text).
use super::summary::passages;

type Check = Result<(), Box<dyn std::error::Error>>;

#[test]
fn finds_every_match_on_long_lines() -> Check {
    let filler = |words: usize| "lorem ".repeat(words);
    // One 6 KB line with two matches, then a line whose second match wraps onto the next.
    let long = format!(
        "Limit of Detection {}limit OF detection {}\n",
        filler(700),
        filler(330)
    );
    assert!(long.len() > 6144, "the first line is over 6 KB");
    let text = format!(
        "{long}Limit of Detection {}Limit of\nDetection end\n",
        filler(600)
    );
    let hits = passages(&text, &["limit of detection".into()]);
    let texts: Vec<_> = hits.iter().map(|hit| &*hit.text).collect();
    assert_eq!(texts.len(), 4, "every match gives a passage");
    let first = texts.first().ok_or("first passage")?;
    let second = texts.get(1).ok_or("second passage")?;
    let third = texts.get(2).ok_or("third passage")?;
    let fourth = texts.get(3).ok_or("fourth passage")?;
    assert!(first.starts_with("Limit of Detection"), "first match");
    assert!(
        second.contains("limit OF detection") && !second.contains('\n'),
        "later match on the same long line"
    );
    assert!(third.starts_with("Limit of Detection"), "next line's match");
    assert!(
        fourth.contains("Limit of\nDetection") && !fourth.contains("Limit of Detection"),
        "wrapped match after a covered one"
    );
    Ok(())
}

#[test]
fn match_inside_a_passage_adds_none() -> Check {
    let hits = passages(
        "Limit of Detection and its limit of detection.\n",
        &["limit of detection".into()],
    );
    assert_eq!(hits.len(), 1, "one passage holds both matches");
    Ok(())
}
