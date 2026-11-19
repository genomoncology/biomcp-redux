//! Whether a PDF's embedded text decodes to gibberish rather than English.
//!
//! A font without a usable encoding decodes each glyph to the wrong character. Seven FDA labels
//! read "7KHUH LV D ULVN" for "There is a risk"; four Rinvoq labels drew their body in a font
//! that decodes to punctuation such as "3)(!%-F%(+(,)" (1068). Such text answers no search,
//! so the reader reports it unread.
//!
//! The check errs toward letting odd text through. It ignores lines that repeat three times
//! or more, such as FDA's per-page stamp, which is English even on a garbled label. What is
//! left is garbled when it holds at least [`WORDS`] words of which fewer than one in
//! [`COMMON_SHARE`] is among the commonest English words, or at least [`CHARACTERS`] visible
//! characters of which fewer than half are letters. Over 105 FDA labels and 59 approval
//! letters, every readable document had a common-word share of at least 0.21 and a letter
//! share of at least 0.81; every garbled one was under 0.05 or under 0.10.
use std::collections::BTreeMap;

/// Fewest words for the common-word test.
const WORDS: usize = 200;
/// At least one word in this many must be a common English word.
const COMMON_SHARE: usize = 10;
/// Fewest visible characters for the letter test.
const CHARACTERS: usize = 2000;
/// Short function words every English document uses, lowercase.
const COMMON: [&str; 24] = [
    "the", "of", "and", "to", "in", "a", "is", "for", "with", "or", "be", "are", "was", "were",
    "that", "by", "as", "on", "at", "not", "this", "an", "from", "have",
];
/// A line seen this many times is page furniture, not body text.
const REPEATED: usize = 3;

/// Whether `text`, apart from repeated lines, is long enough to judge and is not English.
pub(super) fn garbled(text: &str) -> bool {
    let mut seen = BTreeMap::new();
    for line in text.lines().map(str::trim) {
        *seen.entry(line).or_insert(0usize) += 1;
    }
    let (mut words, mut common, mut characters, mut letters) = (0usize, 0usize, 0usize, 0usize);
    for line in text.lines().map(str::trim) {
        if seen.get(line).is_some_and(|count| *count >= REPEATED) {
            continue;
        }
        for c in line.chars().filter(|c| !c.is_whitespace()) {
            characters += 1;
            letters += usize::from(c.is_ascii_alphabetic());
        }
        for word in line
            .split(|c: char| !c.is_ascii_alphabetic())
            .filter(|word| !word.is_empty())
        {
            words += 1;
            common += usize::from(COMMON.iter().any(|known| word.eq_ignore_ascii_case(known)));
        }
    }
    words >= WORDS && common.saturating_mul(COMMON_SHARE) < words
        || characters >= CHARACTERS && letters.saturating_mul(2) < characters
}

#[cfg(test)]
mod tests {
    use super::garbled;

    #[test]
    fn english_is_never_garbled_and_misdecoded_fonts_are() {
        let english = "There is a risk of serious infections in patients treated with the drug.\n";
        let shifted = "7KHUH LV D ULVN RI VHULRXV LQIHFWLRQV LQ SDWLHQWV WUHDWHG ZLWK WKH GUXJ\n";
        let symbols = "3)(!%-F%(+(,) 6,3( ,3%%+3 # @((>(%S1/TD+3D(( +3>(+() +31F! %-BF!\n";
        let stamp = "This label may not be the latest approved by FDA.\n";
        let numbered =
            |line: &str, n: usize| -> String { (0..n).map(|i| format!("{i} {line}")).collect() };
        assert!(
            !garbled(&numbered(english, 30)),
            "English prose is not garbled"
        );
        assert!(
            garbled(&numbered(shifted, 30)),
            "shifted glyphs are garbled"
        );
        assert!(
            garbled(&numbered(symbols, 60)),
            "punctuation glyphs are garbled"
        );
        let stamped = format!("{}{}", stamp.repeat(30), numbered(shifted, 30));
        assert!(
            garbled(&stamped),
            "a repeated English stamp does not hide gibberish"
        );
        assert!(!garbled(&numbered(shifted, 2)), "too little text to judge");
        assert!(!garbled(""), "no text is not garbled");
    }
}
