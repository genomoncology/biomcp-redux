//! Whether a PDF's embedded text decodes to gibberish rather than English.
//!
//! A font without a usable encoding decodes each glyph to the wrong character: seven FDA
//! labels read as "7KHUH LV D ULVN" for "There is a risk" (1068). Such text is no answer to a
//! search, so the reader reports it unread. The check errs toward letting odd text through:
//! the text must hold at least [`WORDS`] words, and fewer than one in [`SHARE`] of them may be
//! among the commonest English words. English prose, letters and labels alike, runs far above
//! that, because "the", "of", "and" and "to" alone are several words in every hundred.
const WORDS: usize = 200;
/// One word in this many, or more, must be a common English word.
const SHARE: usize = 100;
/// Short function words every English document uses, lowercase.
const COMMON: [&str; 24] = [
    "the", "of", "and", "to", "in", "a", "is", "for", "with", "or", "be", "are", "was", "were",
    "that", "by", "as", "on", "at", "not", "this", "an", "from", "have",
];

/// Whether `text` is long enough to judge and almost none of its words are common English.
pub(super) fn garbled(text: &str) -> bool {
    let mut words = 0usize;
    let mut common = 0usize;
    for word in text
        .split(|c: char| !c.is_alphabetic())
        .filter(|word| !word.is_empty())
    {
        words += 1;
        if COMMON.iter().any(|known| word.eq_ignore_ascii_case(known)) {
            common += 1;
        }
    }
    words >= WORDS && common.saturating_mul(SHARE) < words
}

#[cfg(test)]
mod tests {
    use super::garbled;

    #[test]
    fn english_is_never_garbled_and_shifted_glyphs_are() {
        let english = "There is a risk of serious infections in patients treated with the drug. ";
        let shifted = "7KHUH LV D ULVN RI VHULRXV LQIHFWLRQV LQ SDWLHQWV WUHDWHG ZLWK WKH GUXJ. ";
        assert!(
            !garbled(&english.repeat(30)),
            "English prose is not garbled"
        );
        assert!(garbled(&shifted.repeat(30)), "shifted glyphs are garbled");
        assert!(!garbled(&shifted.repeat(2)), "too few words to judge");
        assert!(!garbled(""), "no text is not garbled");
    }
}
