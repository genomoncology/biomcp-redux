//! Page text whose word breaks follow where glyphs sit, not where content-stream operations end.
//!
//! lopdf 0.47's `extract_text` adds a space after every `TJ` array. Word-made FDA summaries
//! break one word across several text operations whenever the character spacing changes, so
//! that space split words: P150047's summary read "plasmid sta ndard" and "teste d" (0968).
//! This reader keeps lopdf's decoding, its line breaks at `T*`, `'` and `"`, and its space
//! for a wide `TJ` kerning gap. It adds a space between operations only when the next glyph
//! starts a word gap or a line away from where the last one ended, as poppler's `pdftotext`
//! decides, and keeps lopdf's line break at `ET` unless the next text object runs on in the
//! same word. A font without glyph widths keeps lopdf's space after each `TJ` array.
use super::fonts::{Widths, number, widths};
use super::state::{Matrix, State};
use lopdf::{Document, Encoding, Object, ObjectId, content::Content};
use std::collections::BTreeMap;

/// A gap wider than this share of the font size is a word break: poppler's `pdftotext`
/// default, and the tenth of an em of lopdf's `TJ` kerning rule.
const WORD_GAP: f32 = 0.1;
/// A baseline move wider than this share of the font size starts another line.
const LINE_GAP: f32 = 0.5;
/// lopdf's `TJ` kerning rule: an adjustment below this many thousandths of an em is a space.
const KERNING_SPACE: f32 = -100.0;

/// Embedded text of one page, its expanded content bounded by `limit` bytes.
pub(super) fn page(document: &Document, page: ObjectId, limit: usize) -> lopdf::Result<String> {
    let mut fonts = BTreeMap::new();
    for (name, font) in document.get_page_fonts(page)? {
        let encoding = font.get_font_encoding_with_limit(document, limit)?;
        fonts.insert(name, (encoding, widths(document, font)));
    }
    let content = Content::decode(&document.get_page_content_with_limit(page, limit)?)?;
    let mut reader = Reader::default();
    for operation in &content.operations {
        reader.apply(&operation.operator, &operation.operands, &fonts)?;
    }
    if reader.pending {
        reader.newline();
    }
    Ok(reader.text)
}

type Fonts<'a> = BTreeMap<Vec<u8>, (Encoding<'a>, Option<Widths>)>;

/// Where on the page the last shown glyph ended and the font height there. Without widths
/// the end is unknown, and `array` keeps lopdf's space after a `TJ` array.
#[derive(Clone, Copy)]
struct End {
    x: f32,
    y: f32,
    height: f32,
    measured: bool,
    array: bool,
}

#[derive(Default)]
struct Reader {
    text: String,
    state: State,
    saved: Vec<State>,
    line: Matrix,
    matrix: Matrix,
    end: Option<End>,
    /// An `ET` line break waiting to see whether the next text continues the same word.
    pending: bool,
}

/// How the next shown text sits against the last: run on, a word gap away, or on another line.
#[derive(Clone, Copy, PartialEq)]
enum Place {
    Joined,
    Word,
    Line,
}

fn float(operands: &[Object], at: usize) -> f32 {
    operands
        .get(at)
        .and_then(number)
        .unwrap_or(0.0)
}

impl Reader {
    fn apply(
        &mut self,
        operator: &str,
        operands: &[Object],
        fonts: &Fonts<'_>,
    ) -> lopdf::Result<()> {
        match operator {
            "BT" => {
                self.line = Matrix::default();
                self.matrix = self.line;
            }
            // lopdf ends each text object with a line break. Word draws table-cell text in
            // separate text objects, so the break is kept only if the next text is not the
            // same word running on ("Mutatio" then "ns" on one baseline in P150047).
            "ET" if self.end.is_some_and(|end| end.measured) => self.pending = true,
            "ET" => {
                self.newline();
                self.end = None;
            }
            "cm" => {
                if let Some(matrix) = Matrix::from(operands) {
                    self.state.ctm = matrix.times(self.state.ctm);
                }
            }
            "q" => self.saved.push(self.state.clone()),
            "Q" => {
                if let Some(state) = self.saved.pop() {
                    self.state = state;
                }
            }
            "Tc" => self.state.char_space = float(operands, 0),
            "Tw" => self.state.word_space = float(operands, 0),
            "Tz" => self.state.scale = float(operands, 0) / 100.0,
            "TL" => self.state.leading = float(operands, 0),
            "Tf" => {
                self.state.font = match operands.first() {
                    Some(Object::Name(name)) => name.clone(),
                    _ => Vec::new(),
                };
                self.state.size = float(operands, 1);
            }
            "Td" => self.move_line(float(operands, 0), float(operands, 1)),
            "TD" => {
                self.state.leading = -float(operands, 1);
                self.move_line(float(operands, 0), float(operands, 1));
            }
            "Tm" => {
                self.line = Matrix::from(operands).unwrap_or_default();
                self.matrix = self.line;
            }
            "T*" => self.next_line(),
            "Tj" | "TJ" => self.show(operands, fonts)?,
            "'" => {
                self.next_line();
                self.show(operands, fonts)?;
            }
            "\"" => {
                self.state.word_space = float(operands, 0);
                self.state.char_space = float(operands, 1);
                self.next_line();
                self.show(operands.get(2..).unwrap_or_default(), fonts)?;
            }
            _ => {}
        }
        Ok(())
    }

    fn newline(&mut self) {
        self.pending = false;
        if !self.text.ends_with('\n') {
            self.text.push('\n');
        }
    }

    fn move_line(&mut self, x: f32, y: f32) {
        self.line = self.line.moved(x, y);
        self.matrix = self.line;
    }

    fn next_line(&mut self) {
        self.move_line(0.0, -self.state.leading);
        self.newline();
    }

    /// Show strings and `TJ` adjustments, after a space when the pen moved a word gap away.
    fn show(&mut self, operands: &[Object], fonts: &Fonts<'_>) -> lopdf::Result<()> {
        let Some((encoding, widths)) = fonts.get(&self.state.font) else {
            return Ok(());
        };
        let start = self.matrix;
        let mut shown = String::new();
        let mut advance = 0.0;
        let mut array = false;
        for operand in operands {
            match operand {
                Object::String(bytes, _) => {
                    encoding.write_to_string(bytes, &mut shown)?;
                    advance += self.advance(bytes, widths.as_ref());
                }
                Object::Array(items) => {
                    array = true;
                    for item in items {
                        if let Object::String(bytes, _) = item {
                            encoding.write_to_string(bytes, &mut shown)?;
                            advance += self.advance(bytes, widths.as_ref());
                        } else if let Some(adjust) = number(item) {
                            if adjust < KERNING_SPACE && !ends_blank(&shown) {
                                shown.push(' ');
                            }
                            advance -= adjust / 1000.0 * self.state.size * self.state.scale;
                        }
                    }
                }
                _ => {}
            }
        }
        if shown.is_empty() {
            return Ok(());
        }
        let place = self.end.map_or(Place::Joined, |end| self.place(end));
        if std::mem::take(&mut self.pending) {
            if place != Place::Joined {
                self.newline();
            }
        } else if place != Place::Joined
            && !ends_blank(&self.text)
            && !shown.starts_with(char::is_whitespace)
        {
            self.text.push(' ');
        }
        self.text.push_str(&shown);
        self.matrix = start.moved(advance, 0.0);
        let page = self.matrix.times(self.state.ctm);
        self.end = Some(End {
            x: page.e,
            y: page.f,
            height: self.height(),
            measured: widths.is_some(),
            array,
        });
        Ok(())
    }

    /// Horizontal text-space advance of one string, zero when the font has no widths.
    fn advance(&self, bytes: &[u8], widths: Option<&Widths>) -> f32 {
        let Some(widths) = widths else {
            return 0.0;
        };
        let codes: Vec<u32> = if widths.two_byte {
            bytes
                .chunks(2)
                .map(|pair| {
                    pair.iter()
                        .fold(0, |code, byte| code * 256 + u32::from(*byte))
                })
                .collect()
        } else {
            bytes.iter().map(|byte| u32::from(*byte)).collect()
        };
        let state = &self.state;
        codes
            .into_iter()
            .map(|code| {
                let word = if !widths.two_byte && code == 32 {
                    state.word_space
                } else {
                    0.0
                };
                (widths.of(code) / 1000.0 * state.size + state.char_space + word) * state.scale
            })
            .sum()
    }

    /// The font size on the page, measured along the text's vertical axis.
    fn height(&self) -> f32 {
        let page = self.matrix.times(self.state.ctm);
        self.state.size * page.c.hypot(page.d)
    }

    /// Where the pen now sits against where the last glyph ended.
    fn place(&self, end: End) -> Place {
        if !end.measured {
            return if end.array {
                Place::Word
            } else {
                Place::Joined
            };
        }
        let page = self.matrix.times(self.state.ctm);
        let height = end.height.max(self.height());
        let length = page.a.hypot(page.b);
        if height <= 0.0 || length <= 0.0 {
            return Place::Word;
        }
        let (x, y) = (page.e - end.x, page.f - end.y);
        let (along_x, along_y) = (page.a / length, page.b / length);
        let along = x * along_x + y * along_y;
        let across = (x * along_y - y * along_x).abs();
        if across > LINE_GAP * height {
            Place::Line
        } else if along > WORD_GAP * height || along < -height {
            Place::Word
        } else {
            Place::Joined
        }
    }
}

fn ends_blank(text: &str) -> bool {
    text.is_empty() || text.ends_with(char::is_whitespace)
}
