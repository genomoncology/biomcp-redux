//! Page text whose word breaks follow where glyphs sit, not where content-stream operations end.
//!
//! lopdf 0.47's `extract_text` adds a space after every `TJ` array. Word-made FDA summaries
//! break one word across several text operations whenever the character spacing changes, so
//! that space split words: P150047's summary read "plasmid sta ndard" and "teste d" (0968).
//! This reader keeps lopdf's decoding, its line breaks at `ET`, `T*`, `'` and `"`, and its
//! space for a wide `TJ` kerning gap, and adds a space between operations only when the next
//! glyph starts a word gap or a line away from where the last one ended, as poppler's
//! `pdftotext` decides. A font without glyph widths keeps lopdf's space after each `TJ` array.
use lopdf::{Dictionary, Document, Encoding, Object, ObjectId, content::Content};
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
    Ok(reader.text)
}

type Fonts<'a> = BTreeMap<Vec<u8>, (Encoding<'a>, Option<Widths>)>;

/// Glyph widths in thousandths of an em, keyed by the first code of each run of codes.
struct Widths {
    two_byte: bool,
    runs: BTreeMap<u32, (u32, f32)>,
    default: f32,
}

impl Widths {
    fn of(&self, code: u32) -> f32 {
        self.runs
            .range(..=code)
            .next_back()
            .filter(|(_, (last, _))| code <= *last)
            .map_or(self.default, |(_, (_, width))| *width)
    }
}

fn widths(document: &Document, font: &Dictionary) -> Option<Widths> {
    match font.get(b"Subtype").and_then(Object::as_name).ok()? {
        b"Type0" => composite_widths(document, font),
        b"Type3" => None,
        _ => simple_widths(document, font),
    }
}

fn number(document: &Document, object: &Object) -> Option<f32> {
    document.dereference(object).ok()?.1.as_float().ok()
}

fn array<'a>(document: &'a Document, object: &'a Object) -> Option<&'a Vec<Object>> {
    document.dereference(object).ok()?.1.as_array().ok()
}

/// PDF 32000-1 §9.6.1: `Widths` from `FirstChar`; a code outside them has `MissingWidth`.
fn simple_widths(document: &Document, font: &Dictionary) -> Option<Widths> {
    let first = number(document, font.get(b"FirstChar").ok()?)? as u32;
    let list = array(document, font.get(b"Widths").ok()?)?;
    let mut runs = BTreeMap::new();
    for (at, width) in list.iter().enumerate() {
        let code = first.saturating_add(at as u32);
        runs.insert(code, (code, number(document, width)?));
    }
    let default = font
        .get_deref(b"FontDescriptor", document)
        .and_then(Object::as_dict)
        .ok()
        .and_then(|descriptor| descriptor.get(b"MissingWidth").ok())
        .and_then(|width| number(document, width))
        .unwrap_or(0.0);
    Some(Widths {
        two_byte: false,
        runs,
        default,
    })
}

/// PDF 32000-1 §9.7.4.3: the descendant font's `W` runs over two-byte codes, else `DW`.
fn composite_widths(document: &Document, font: &Dictionary) -> Option<Widths> {
    let descendant = array(document, font.get(b"DescendantFonts").ok()?)?.first()?;
    let descendant = document.dereference(descendant).ok()?.1.as_dict().ok()?;
    let default = descendant
        .get(b"DW")
        .ok()
        .and_then(|width| number(document, width))
        .unwrap_or(1000.0);
    let mut runs = BTreeMap::new();
    if let Ok(list) = descendant.get(b"W") {
        let mut items = array(document, list)?.iter();
        while let Some(first) = items.next() {
            let first = number(document, first)? as u32;
            match document.dereference(items.next()?).ok()?.1 {
                Object::Array(each) => {
                    for (at, width) in each.iter().enumerate() {
                        let code = first.saturating_add(at as u32);
                        runs.insert(code, (code, number(document, width)?));
                    }
                }
                last => {
                    let last = last.as_float().ok()? as u32;
                    runs.insert(first, (last, number(document, items.next()?)?));
                }
            }
        }
    }
    Some(Widths {
        two_byte: true,
        runs,
        default,
    })
}

/// A text or line matrix `[a b c d e f]`.
#[derive(Clone, Copy)]
struct Matrix {
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    e: f32,
    f: f32,
}

impl Default for Matrix {
    fn default() -> Self {
        Self {
            a: 1.0,
            b: 0.0,
            c: 0.0,
            d: 1.0,
            e: 0.0,
            f: 0.0,
        }
    }
}

impl Matrix {
    fn from(operands: &[Object]) -> Option<Self> {
        let mut values = operands.iter().map(|value| value.as_float().ok());
        Some(Self {
            a: values.next()??,
            b: values.next()??,
            c: values.next()??,
            d: values.next()??,
            e: values.next()??,
            f: values.next()??,
        })
    }
    /// This matrix moved by `(x, y)` in its own text space.
    fn moved(self, x: f32, y: f32) -> Self {
        Self {
            e: self.e + self.a * x + self.c * y,
            f: self.f + self.b * x + self.d * y,
            ..self
        }
    }
}

/// Text state parameters that `q` and `Q` save and restore with the graphics state.
#[derive(Clone)]
struct State {
    char_space: f32,
    word_space: f32,
    scale: f32,
    leading: f32,
    size: f32,
    font: Vec<u8>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            char_space: 0.0,
            word_space: 0.0,
            scale: 1.0,
            leading: 0.0,
            size: 0.0,
            font: Vec::new(),
        }
    }
}

/// Where the last shown glyph ended and the font height there. Without widths the end is
/// unknown, and `array` keeps lopdf's space after a `TJ` array.
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
}

fn float(operands: &[Object], at: usize) -> f32 {
    operands
        .get(at)
        .and_then(|value| value.as_float().ok())
        .unwrap_or(0.0)
}

impl Reader {
    fn apply(&mut self, operator: &str, operands: &[Object], fonts: &Fonts<'_>) -> lopdf::Result<()> {
        match operator {
            "BT" => {
                self.line = Matrix::default();
                self.matrix = self.line;
                self.end = None;
            }
            "ET" => {
                self.newline();
                self.end = None;
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
                self.state.font = operands
                    .first()
                    .and_then(|name| name.as_name().ok())
                    .map(<[u8]>::to_vec)
                    .unwrap_or_default();
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
                        } else if let Ok(adjust) = item.as_float() {
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
        let breaks = self.end.is_some_and(|end| self.breaks(end));
        if breaks && !ends_blank(&self.text) && !shown.starts_with(char::is_whitespace) {
            self.text.push(' ');
        }
        self.text.push_str(&shown);
        self.matrix = start.moved(advance, 0.0);
        self.end = Some(End {
            x: self.matrix.e,
            y: self.matrix.f,
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
                .map(|pair| pair.iter().fold(0, |code, byte| code * 256 + u32::from(*byte)))
                .collect()
        } else {
            bytes.iter().map(|byte| u32::from(*byte)).collect()
        };
        let state = &self.state;
        codes
            .into_iter()
            .map(|code| {
                let word = if !widths.two_byte && code == 32 { state.word_space } else { 0.0 };
                (widths.of(code) / 1000.0 * state.size + state.char_space + word) * state.scale
            })
            .sum()
    }

    /// The font size measured along the text matrix's vertical axis.
    fn height(&self) -> f32 {
        self.state.size * self.matrix.c.hypot(self.matrix.d)
    }

    /// Whether the pen now sits a word gap or a line away from where the last glyph ended.
    fn breaks(&self, end: End) -> bool {
        let height = end.height.max(self.height());
        let length = self.matrix.a.hypot(self.matrix.b);
        if !end.measured {
            return end.array;
        }
        if height <= 0.0 || length <= 0.0 {
            return true;
        }
        let (x, y) = (self.matrix.e - end.x, self.matrix.f - end.y);
        let (along_x, along_y) = (self.matrix.a / length, self.matrix.b / length);
        let along = x * along_x + y * along_y;
        let across = (x * along_y - y * along_x).abs();
        across > LINE_GAP * height || along > WORD_GAP * height || along < -height
    }
}

fn ends_blank(text: &str) -> bool {
    text.is_empty() || text.ends_with(char::is_whitespace)
}
