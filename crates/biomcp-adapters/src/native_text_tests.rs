//! Line breaks in admitted source text read as one space; other controls stay visible.
use crate::native_text::admit;

#[test]
fn line_breaks_read_as_one_space() {
    let text = |value: &str| admit(value).map(|t| t.as_str().to_owned());
    assert_eq!(text("a\r\nb").as_deref(), Ok("a b"));
    assert_eq!(text("a\rb").as_deref(), Ok("a b"));
    assert_eq!(text("a\n\n \n\r\nb").as_deref(), Ok("a b"));
    assert_eq!(text("a\tb\nc").as_deref(), Ok("a\\u{0009}b c"));
}
