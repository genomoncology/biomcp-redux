//! The text matrix and the graphics and text state a PDF content stream sets, as
//! PDF 32000-1 §8.4 and §9.3 define them, for placing words.
use super::fonts::number;
use lopdf::Object;

/// A text or line matrix `[a b c d e f]`.
#[derive(Clone, Copy)]
pub(super) struct Matrix {
    pub(super) a: f32,
    pub(super) b: f32,
    pub(super) c: f32,
    pub(super) d: f32,
    pub(super) e: f32,
    pub(super) f: f32,
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
    pub(super) fn from(operands: &[Object]) -> Option<Self> {
        let mut values = operands.iter().map(number);
        Some(Self {
            a: values.next()??,
            b: values.next()??,
            c: values.next()??,
            d: values.next()??,
            e: values.next()??,
            f: values.next()??,
        })
    }
    /// This matrix followed by `then`, as text space maps through the CTM to the page.
    pub(super) fn times(self, then: Self) -> Self {
        Self {
            a: self.a * then.a + self.b * then.c,
            b: self.a * then.b + self.b * then.d,
            c: self.c * then.a + self.d * then.c,
            d: self.c * then.b + self.d * then.d,
            e: self.e * then.a + self.f * then.c + then.e,
            f: self.e * then.b + self.f * then.d + then.f,
        }
    }
    /// This matrix moved by `(x, y)` in its own text space.
    pub(super) fn moved(self, x: f32, y: f32) -> Self {
        Self {
            e: self.e + self.a * x + self.c * y,
            f: self.f + self.b * x + self.d * y,
            ..self
        }
    }
}

/// The CTM and text state parameters that `q` and `Q` save and restore.
#[derive(Clone)]
pub(super) struct State {
    pub(super) ctm: Matrix,
    pub(super) char_space: f32,
    pub(super) word_space: f32,
    pub(super) scale: f32,
    pub(super) leading: f32,
    pub(super) size: f32,
    pub(super) font: Vec<u8>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            ctm: Matrix::default(),
            char_space: 0.0,
            word_space: 0.0,
            scale: 1.0,
            leading: 0.0,
            size: 0.0,
            font: Vec::new(),
        }
    }
}
