//! Numeric breaks that need more evidence than the distance between text operations.
use super::fonts::number;
use super::position::{Position, WORD_GAP};
use super::state::State;
use lopdf::Object;
use std::ops::Range;

/// A raised digit immediately after explicit multiplication by ten is an exponent.
pub(super) fn could_be_note(before: &str, after: &str) -> bool {
    before.ends_with(|c: char| c.is_ascii_digit())
        && after.starts_with(|c: char| c.is_ascii_digit())
        && !before
            .strip_suffix("10")
            .and_then(|prefix| prefix.trim_end().strip_suffix(['x', 'X', '×']))
            .is_some_and(|coefficient| {
                coefficient
                    .trim_end()
                    .ends_with(|c: char| c.is_ascii_digit())
            })
}

pub(super) const PHONE_PREFIX_BYTES: usize = b"(000) 000-".len();

/// A US phone number can wrap at the last hyphen in FDA letters.
pub(super) fn continue_phone(before: &str, after: &str) -> bool {
    let prefix = before.as_bytes().last_chunk::<PHONE_PREFIX_BYTES>();
    let Some([b'(', a, b, c, b')', b' ', d, e, f, b'-']) = prefix else {
        return false;
    };
    if ![a, b, c, d, e, f].into_iter().all(u8::is_ascii_digit) {
        return false;
    }
    let mut tail = after.trim_start().bytes();
    let digits = tail.by_ref().take(4).filter(u8::is_ascii_digit).count();
    digits == 4 && tail.next().is_none_or(|byte| !byte.is_ascii_alphanumeric())
}

/// Also keep a phone together when one text operation includes both halves.
pub(super) fn join_phones(shown: &mut String) {
    let mut kept = String::new();
    let mut from = 0;
    for (hyphen, _) in shown.match_indices("- ") {
        let at = hyphen + 1;
        let after = &shown[at..];
        if continue_phone(&shown[..at], after) {
            kept.push_str(&shown[from..at]);
            from = at + after.len() - after.trim_start().len();
        }
    }
    if from != 0 {
        kept.push_str(&shown[from..]);
        *shown = kept;
    }
}

/// Some PDFs leave holes in a visible text pass and fill them with invisible text at the
/// same positions. Keep one short, sparse run in hand and join only a complementary run
/// whose decimal point fits between digits from that first pass.
#[derive(Default)]
pub(super) struct Overlay(Option<Candidate>);

struct Candidate {
    position: Position,
    glyphs: Vec<Glyph>,
    output: Range<usize>,
}

struct Glyph {
    start: f32,
    end: f32,
    character: char,
    invisible: bool,
}

/// Enough glyphs for one line, kept only until another candidate or a matching pass.
const MAX_GLYPHS: usize = 512;

impl Overlay {
    /// A successful overlay moves the earlier run into `shown`; return its old byte range.
    pub(super) fn assemble(
        &mut self,
        position: Position,
        state: &State<'_>,
        operands: &[Object],
        shown: &mut String,
        at: usize,
    ) -> Option<Range<usize>> {
        let [Object::Array(items)] = operands else {
            return None;
        };
        if !upright(position) || !state.font.as_ref()?.sound {
            return None;
        }
        if state.invisible {
            let prior = self.0.as_ref()?;
            if !same_line(prior.position, position) || !shown.contains('.') {
                return None;
            }
            let glyphs = positioned(position, state, items)?;
            let merged = merge(&prior.glyphs, &glyphs, position.height * WORD_GAP)?;
            *shown = merged;
            return self.0.take().map(|candidate| candidate.output);
        }
        if sparse(items, shown) {
            self.0 = positioned(position, state, items).map(|glyphs| Candidate {
                position,
                glyphs,
                output: at..at + shown.len(),
            });
        }
        None
    }

    pub(super) fn drop_from(&mut self, at: usize) {
        if self
            .0
            .as_ref()
            .is_some_and(|candidate| candidate.output.end > at)
        {
            self.0 = None;
        }
    }
}

fn sparse(items: &[Object], shown: &str) -> bool {
    let digits = |word: &str| !word.is_empty() && word.bytes().all(|byte| byte.is_ascii_digit());
    let mut words = shown.split_whitespace();
    let mut last = words.next().is_some_and(digits);
    words.any(|word| {
        let next = digits(word);
        let pair = last && next;
        last = next;
        pair
    }) && items
        .iter()
        .any(|item| number(item).is_some_and(|adjust| adjust < -1000.0))
}

fn upright(position: Position) -> bool {
    let page = position.page;
    page.b == 0.0 && page.c == 0.0 && page.a > 0.0 && page.d > 0.0 && position.height.is_normal()
}

fn same_line(left: Position, right: Position) -> bool {
    let tolerance = right.height * WORD_GAP;
    (left.page.f - right.page.f).abs() <= tolerance
        && (left.height - right.height).abs() <= tolerance
}

/// Read individual positions only for candidate runs. A glyph with ambiguous decoding or
/// advance leaves the original two passes intact.
fn positioned(position: Position, state: &State<'_>, items: &[Object]) -> Option<Vec<Glyph>> {
    let font = state.font.as_ref()?;
    let widths = font.widths.as_ref()?;
    let mut x = position.page.e;
    let mut glyphs = Vec::new();
    let mut decoded = String::new();
    let mut count = 0;
    for item in items {
        if let Object::String(bytes, _) = item {
            for glyph in bytes.chunks(widths.code_bytes()) {
                count += 1;
                if count > MAX_GLYPHS || glyph.len() != widths.code_bytes() {
                    return None;
                }
                decoded.clear();
                match font.decode(glyph, &mut decoded) {
                    Ok(false) => {}
                    Ok(true) | Err(_) => return None,
                }
                let character = single_char(&decoded)?;
                let end = x + widths.glyph_advance(glyph, state) * position.page.a;
                if !end.is_finite() || end < x {
                    return None;
                }
                if !character.is_whitespace() {
                    glyphs.push(Glyph {
                        start: x,
                        end,
                        character,
                        invisible: state.invisible,
                    });
                }
                x = end;
            }
        } else if let Some(adjust) = number(item) {
            x -= adjust / 1000.0 * state.size * state.scale * position.page.a;
        }
    }
    Some(glyphs)
}

fn single_char(decoded: &str) -> Option<char> {
    let mut chars = decoded.chars();
    let first = chars.next()?;
    (first != '\u{fffd}' && chars.next().is_none()).then_some(first)
}

fn merge(visible: &[Glyph], invisible: &[Glyph], gap: f32) -> Option<String> {
    let mut glyphs: Vec<_> = visible.iter().chain(invisible).collect();
    glyphs.sort_by(|a, b| a.start.total_cmp(&b.start));
    // A partial overlay, duplicate text, or competing glyphs at one position is ambiguous.
    if invisible.first()?.start > visible.first()?.start
        || invisible.last()?.end < visible.last()?.end
        || glyphs
            .array_windows()
            .any(|[previous, next]| previous.end - next.start > gap)
        || !glyphs.windows(3).any(|triple| split_decimal(triple, gap))
    {
        return None;
    }
    let mut text = String::new();
    let mut end = glyphs.first()?.start;
    for glyph in glyphs {
        if glyph.start - end > gap {
            text.push(' ');
        }
        text.push(glyph.character);
        end = glyph.end;
    }
    Some(text)
}

fn split_decimal(glyphs: &[&Glyph], gap: f32) -> bool {
    let [left, point, right] = glyphs else {
        return false;
    };
    !left.invisible
        && point.invisible
        && !right.invisible
        && left.character.is_ascii_digit()
        && point.character == '.'
        && right.character.is_ascii_digit()
        && point.start - left.end <= gap
        && right.start - point.end <= gap
}
